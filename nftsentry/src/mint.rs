use near_sdk::{Gas, PromiseError};
use policy_rules::policy::ConfigInterface;
use policy_rules::types::{InventoryLicense, NFTMintResult, SourceLicenseMeta};
use policy_rules::utils::{balance_from_string, format_balance};
use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        inventory_id: AccountId,
        asset_id: String,
        license_id: String,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) -> Promise {
        if self.tokens_by_id.get(&token_id).is_some() {
            env::panic_str("Token already exists")
        }

        // Schedule calls to metadata and asset token
        let promise_meta: Promise = inventory_contract::ext(inventory_id.clone())
            .with_unused_gas_weight(3).inventory_metadata();
        let promise_asset: Promise = inventory_contract::ext(inventory_id.clone())
            .with_unused_gas_weight(3).asset_token(asset_id.clone());
        let promise_inventory = promise_meta.and(promise_asset);
        // Then schedule call to self.callback

        let predecessor_id = env::predecessor_account_id();
        return promise_inventory.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .with_unused_gas_weight(7)
                .on_nft_mint(
                    token_id, inventory_id, license_id, receiver_id, perpetual_royalties, predecessor_id
                )
        )
    }

    #[private]
    #[payable]
    pub fn on_nft_mint(
        &mut self,
        #[callback_result] metadata_res: Result<ExtendedInventoryMetadata, PromiseError>,
        #[callback_result] asset_res: Result<JsonAssetToken, PromiseError>,
        token_id: TokenId,
        inventory_id: AccountId,
        license_id: String,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
        predecessor_id: AccountId,
    ) -> NFTMintResult {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let result = self.ensure_nft_mint(
            metadata_res, asset_res, token_id.clone(),  inventory_id.clone(),license_id.clone(), receiver_id.clone()
        );

        if result.is_err() {
            let _ = refund_deposit(0, Some(predecessor_id), None);
            unsafe {
                let msg = result.unwrap_err_unchecked();
                env::log_str( &format!("Error: {}", msg));
                // env::panic_str(result.unwrap_err_unchecked().as_str());
                return NFTMintResult{
                    license_token: None,
                    error: msg,
                }
            }
        }
        let (lic_token, inv_license, asset, inventory_owner) = unsafe {result.unwrap_unchecked()};

        // we add an optional parameter for perpetual royalties
        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function:
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }
        }

        //specify the token struct that contains the owner ID
        let token = Token {
            token_id,
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            asset_id: asset.token_id.clone(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty: royalty.clone(),
        };


        // ----- Token mint start -----
        let mint_result = self.internal_mint(lic_token.clone());
        if mint_result.is_err() {
            let msg = unsafe{mint_result.unwrap_err_unchecked()};
            env::log_str( &format!("Error: {}", msg));
            return NFTMintResult{license_token: None, error: msg.to_string()}
        }
        // ----- Token mint end -----

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token.token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much.
        let result = refund_deposit(
            required_storage_in_bytes,
            Some(predecessor_id.clone()),
            Some(balance_from_string(inv_license.price.clone())),
        );
        if result.is_err() {
            // Refund failed due to storage costs.
            // Rollback all changes!
            self.internal_remove_token_from_owner(&token.owner_id, &token.token_id);
            self.tokens_by_id.remove(&token.token_id);
            self.token_license_by_id.remove(&token.token_id);
            self.token_metadata_by_id.remove(&token.token_id);
            // Refund any deposit
            let _ = refund_deposit(0, Some(predecessor_id), None);

            let msg = result.unwrap_err();
            env::log_str( &format!("Error: {}", msg));
            return NFTMintResult{license_token:None, error:msg}
        }

        inventory_contract::ext(inventory_id.clone()).with_static_gas(Gas::ONE_TERA * 3).on_nft_mint(
            asset.token_id.clone(), self.token_metadata_by_id.len()
        );
        self.process_fees(balance_from_string(inv_license.price), inventory_owner);

        // Log the serialized json.
        self.log_event(&nft_mint_log.to_string());

        NFTMintResult{
            license_token: Some(lic_token),
            error: String::new(),
        }
    }

    fn ensure_nft_mint(
        &self,
        metadata_res: Result<ExtendedInventoryMetadata, PromiseError>,
        asset_res: Result<JsonAssetToken, PromiseError>,
        token_id: TokenId,
        inventory_id: AccountId,
        license_id: String,
        receiver_id: AccountId,
        ) -> Result<(LicenseToken, InventoryLicense, JsonAssetToken, AccountId), String> {

        // 1. Check callback results first.
        if metadata_res.is_err() || asset_res.is_err() {
            if metadata_res.is_err() {
                return Err("Failed call inventory_metadata".to_string())
            } else {
                return Err("Failed call asset_token".to_string())
            }
        }

        unsafe {
            let asset = asset_res.unwrap_unchecked();
            let inv_metadata = metadata_res.unwrap_unchecked();
            let asset_id = asset.token_id.clone();

            let full_inventory = self.get_full_inventory(asset.clone(), inv_metadata.metadata.clone());
            let inv_license = full_inventory.inventory_licenses.iter().find(|x| x.license_id == license_id);
            if inv_license.is_none() {
                return Err("Inventory license not found by id".to_string())
            }
            let deposit = env::attached_deposit();
            let price = balance_from_string(inv_license.unwrap_unchecked().price.clone());
            if deposit < price {
                return Err(format!(
                    "Attached deposit of {} NEAR is less than license price of {} NEAR",
                    format_balance(deposit),
                    inv_license.unwrap_unchecked().price.clone(),
                ))
            }

            let license = TokenLicense {
                id: license_id.clone(),
                title: Some(inv_license.unwrap_unchecked().title.clone()),
                metadata: inv_license.unwrap_unchecked().license.clone(),
                from: SourceLicenseMeta{
                    asset_id: asset_id.clone(),
                    inventory_id: inventory_id.clone().to_string(),
                },
                description: None,
                expires_at: None,
                issued_at: Some(env::block_timestamp_ms()),
                starts_at: Some(env::block_timestamp_ms()),
                issuer_id: Some(env::current_account_id()),
                updated_at: None,
                uri: inv_license.unwrap_unchecked().license.pdf_url.clone(),
            };
            // let lic_token = inv_license.unwrap_unchecked().as_license_token(token_id);
            let metadata = asset.metadata.issue_new_metadata();

            let lic_token = LicenseToken {
                token_id: token_id.clone(),
                metadata,
                asset_id: asset_id.clone(),
                owner_id: receiver_id.clone(),
                approved_account_ids: HashMap::new(),
                royalty: HashMap::new(),
                license: Some(license.clone()),
            };
            let (ok, reason) = self.policies.check_new(
                full_inventory.clone(),
                lic_token.clone(),
            );
            if !ok {
                return Err(reason);
            }

            Ok((lic_token, inv_license.unwrap_unchecked().clone(), asset, inv_metadata.owner_id.clone()))
        }
    }

    #[private]
    pub fn process_fees(&self, base_deposit: Balance, master_account: AccountId) {
        // base_deposit -> 97.5% base_account, 2.5% benefit
        let benefit_fee_milli_percent = if self.benefit_config.is_some() {
            unsafe {self.benefit_config.clone().unwrap_unchecked().fee_milli_percent_amount}
        } else {
            0
        };
        let benefit_fee = base_deposit / (1e5 as u128) * (benefit_fee_milli_percent as u128);
        let base_amount = base_deposit - benefit_fee;

        Promise::new(master_account).transfer(base_amount);
        if benefit_fee != 0 {
            unsafe {
                Promise::new(self.benefit_config.clone().unwrap_unchecked().account_id).transfer(benefit_fee);
            }
        }
    }

    #[private]
    pub(crate) fn internal_mint(&mut self, lic_token: LicenseToken) -> Result<(), String> {
        let token = Token{
            token_id: lic_token.token_id.clone(),
            asset_id: lic_token.asset_id.clone(),
            owner_id: lic_token.owner_id.clone(),
            approved_account_ids: lic_token.approved_account_ids.clone(),
            next_approval_id: 0,
            royalty: lic_token.royalty.clone(),
        };
        let exists = self.tokens_by_id.insert(&lic_token.token_id, &token);
        if exists.is_some() {
            let msg = "Token already exists";
            env::log_str( &format!("Error: {}",msg));
            return Err(msg.to_string())
        }
        self.token_metadata_by_id.insert(&lic_token.token_id, &lic_token.metadata);
        //insert the token ID and license
        self.token_license_by_id.insert(&lic_token.token_id, lic_token.license.as_ref().unwrap());

        //self.token_proposed_license_by_id.insert(&token_id, &proposed_license);

        self.internal_add_token_to_owner(&lic_token.owner_id, &lic_token.token_id);
        Ok(())
    }
}

