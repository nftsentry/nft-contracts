use near_sdk::{Gas, PromiseError};
use policy_rules::policy::ConfigInterface;
use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) -> Promise {
        if self.tokens_by_id.get(&token_id).is_some() {
            env::panic_str("Token already exists")
        }

        let (inventory_id, asset_id, _license_id) = metadata.inventory_asset_license();

        // Schedule calls to metadata and asset token
        let promise_meta: Promise = inventory_contract::ext(AccountId::new_unchecked(inventory_id.clone()))
            .with_static_gas(Gas(5*TGAS)).inventory_metadata();
        let promise_asset: Promise = inventory_contract::ext(AccountId::new_unchecked(inventory_id.clone()))
            .with_static_gas(Gas(5*TGAS)).asset_token(asset_id, None);
        let promise_inventory = promise_meta.and(promise_asset);
        // Then schedule call to self.callback

        let predecessor_id = env::predecessor_account_id();
        return promise_inventory.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .with_static_gas(Gas(15*TGAS))
                .on_nft_mint(
                    token_id, metadata, receiver_id, perpetual_royalties, predecessor_id
                )
        )
    }

    #[private]
    #[payable]
    pub fn on_nft_mint(
        &mut self,
        #[callback_result] metadata_res: Result<InventoryContractMetadata, PromiseError>,
        #[callback_result] asset_res: Result<JsonAssetToken, PromiseError>,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
        predecessor_id: AccountId,
    ) -> LicenseToken {

        let result = self.ensure_nft_mint(
            metadata_res, asset_res, token_id.clone(), metadata.clone(), receiver_id.clone()
        );

        if result.is_err() {
            Promise::new(predecessor_id.clone()).transfer(env::attached_deposit());
            unsafe {
                env::panic_str(result.unwrap_err_unchecked().as_str());
            }
        }
        let (lic_token, asset_id) = unsafe {result.unwrap_unchecked()};
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();
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
            token_id: token_id,
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            asset_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty: royalty.clone(),
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        self.tokens_by_id.insert(&token.token_id, &token);
        self.token_metadata_by_id.insert(&token.token_id, &metadata);
        //insert the token ID and license
        self.token_license_by_id.insert(&token.token_id, lic_token.license.as_ref().unwrap());

        //self.token_proposed_license_by_id.insert(&token_id, &proposed_license);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token.token_id);
        self.internal_add_token_to_asset(&token.asset_id, &token.token_id);

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

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes, Some(predecessor_id));

        // Log the serialized json.
        self.log_event(&nft_mint_log.to_string());

        lic_token
    }

    fn ensure_nft_mint(
        &self,
        metadata_res: Result<InventoryContractMetadata, PromiseError>,
        asset_res: Result<JsonAssetToken, PromiseError>,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        ) -> Result<(LicenseToken, String), String> {

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
            let (_inventory_id, asset_id, license_id) = metadata.inventory_asset_license();

            let full_inventory = self.get_full_inventory(asset, inv_metadata);
            let inv_license = full_inventory.inventory_licenses.iter().find(|x| x.license_id == license_id);
            if inv_license.is_none() {
                return Err("Inventory license not found by id".to_string())
            }
            let license = TokenLicense {
                title: Some(inv_license.unwrap_unchecked().title.clone()),
                metadata: Some(serde_json::to_string(&inv_license.unwrap_unchecked().license).unwrap_unchecked()),
                description: None,
                expires_at: None,
                issued_at: Some(env::block_timestamp_ms()),
                starts_at: Some(env::block_timestamp_ms()),
                issuer_id: Some(env::current_account_id()),
                reference: None,
                reference_hash: None,
                updated_at: None,
                uri: inv_license.unwrap_unchecked().license.pdf_url.clone(),
            };
            // let lic_token = inv_license.unwrap_unchecked().as_license_token(token_id);
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
                full_inventory,
                lic_token.clone(),
            );
            if !ok {
                return Err(reason);
            }

            Ok((lic_token, asset_id))
        }
    }

    }

