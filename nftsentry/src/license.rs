use crate::*;

use near_sdk::{log, Gas, PromiseError};
use policy_rules::policy::ConfigInterface;
use policy_rules::types::{FullInventory, InventoryLicense, LicenseGeneral, NFTUpdateLicenseResult, SourceLicenseMeta};
use policy_rules::utils::{balance_from_string, format_balance};

// const GAS_FOR_LICENSE_APPROVE: Gas = Gas(10_000_000_000_000);
// const NO_DEPOSIT: Balance = 0;
const MIN_GAS_FOR_LICENSE_APPROVE_CALL: Gas = Gas(100_000_000_000_000);


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_update_license(
        &mut self,  
        token_id: TokenId,
        new_license_id: String,
    ) -> Promise {
        let predecessor_id = env::predecessor_account_id();
        let token_opt = self.nft_token(token_id.clone());
        if token_opt.is_none() {
            env::panic_str("Token does not exist")
        }
        let token = unsafe {token_opt.unwrap_unchecked()};

        if predecessor_id != token.owner_id {
            env::panic_str("License can only be updated directly by the token owner");
        }
        let (inventory_id, asset_id, _license_id) = token.inventory_asset_license();
        let inventory_account_id = AccountId::new_unchecked(inventory_id.clone());

        // Schedule calls to metadata and asset token
        let promise_meta: Promise = inventory_contract::ext(inventory_account_id.clone())
            .inventory_metadata();
        let promise_asset: Promise = inventory_contract::ext(inventory_account_id.clone())
            .asset_token(asset_id);
        let promise_inventory = promise_meta.and(promise_asset);
        // Then schedule call to self.callback

        return promise_inventory.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .on_license_update(
                token_id, inventory_account_id, predecessor_id, new_license_id
            )
        )
    }

    #[payable]
    pub fn on_license_update(
        &mut self,
        #[callback_result] metadata_res: Result<InventoryContractMetadata, PromiseError>,
        #[callback_result] asset_res: Result<JsonAssetToken, PromiseError>,
        token_id: TokenId,
        inventory_id: AccountId,
        predecessor_id: AccountId,
        new_license_id: String,
    ) -> NFTUpdateLicenseResult {

        let initial_storage_usage = env::storage_usage();
        let result = self.ensure_update_license(
            metadata_res, asset_res, token_id.clone(), new_license_id
        );
        if result.is_err() {
            let _ = refund_deposit(0, Some(predecessor_id), None);
            unsafe {
                let msg = result.unwrap_err_unchecked();
                env::log_str( &format!("Error: {}", msg));
                return NFTUpdateLicenseResult{error: msg}
            }
        }
        let (license, price_diff) = unsafe{result.unwrap_unchecked()};
        //measure the initial storage being used on the contract
        let token = unsafe{self.nft_token(token_id.clone()).unwrap_unchecked()};
        let old_license = token.license.unwrap();

        self.internal_replace_license(&predecessor_id, &token_id, &license);

        // Construct the mint log as per the events standard.
        let nft_update_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftUpdateLicense(vec![NftUpdateLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            let result = refund_deposit(
                storage_usage - initial_storage_usage,
                Some(predecessor_id.clone()),
                Some(price_diff)
            );
            if result.is_err() {
                // Refund failed due to storage costs.
                // Rollback all changes!
                self.internal_replace_license(&token.owner_id, &token.token_id, &old_license);
                // Refund any deposit
                let _ = refund_deposit(0, Some(predecessor_id), None);

                let msg = result.unwrap_err();
                env::log_str( &format!("Error: {}", msg));
                return NFTUpdateLicenseResult{error: msg}
            }
        }

        self.process_fees(price_diff, inventory_id);

        // Log the serialized json.
        self.log_event(&nft_update_license_log.to_string());

        return NFTUpdateLicenseResult{error: String::new()}

    }

    fn ensure_update_license(
        &self,
        metadata_res: Result<InventoryContractMetadata, PromiseError>,
        asset_res: Result<JsonAssetToken, PromiseError>,
        token_id: TokenId,
        new_license_id: String,
    ) -> Result<(TokenLicense, Balance), String> {
        // 1. Check callback results first.
        if metadata_res.is_err() || asset_res.is_err() {
            return if metadata_res.is_err() {
                Err("Failed call inventory_metadata".to_string())
            } else {
                Err("Failed call asset_token".to_string())
            }
        }
        let token = self.nft_token(token_id.clone()).unwrap();
        let asset = unsafe{asset_res.unwrap_unchecked()};
        let new_asset_license = asset.licenses.as_ref().unwrap().into_iter().find(|x| x.license_id == new_license_id).expect("Asset license not found");
        let old_asset_license = asset.licenses.as_ref().unwrap().into_iter().find(|x| x.license_id == token.license_id()).expect("Asset license not found");
        let metadata = unsafe{metadata_res.unwrap_unchecked()};
        let (inv_id, asset_id, old_license_id) = token.inventory_asset_license();

        // Build full inventory for those.
        // First, populate licenses with actual prices from asset
        let full_inventory = self.get_full_inventory(asset.clone(), metadata.clone());
        let new_license = metadata.licenses.iter().find(|x| x.license_id == new_license_id).unwrap();
        let old_license = metadata.licenses.iter().find(|x| x.license_id == old_license_id).unwrap();

        // Check for valid deposit
        let must_attach = balance_from_string(
            new_asset_license.price.clone().unwrap_or(new_license.price.clone().unwrap())
        ) - balance_from_string(
            old_asset_license.price.clone().unwrap_or(old_license.price.clone().unwrap())
        );
        if env::attached_deposit() < must_attach {
            return Err(format!(
                "Attached deposit of {} NEAR is less than license price difference of {} NEAR",
                format_balance(env::attached_deposit()),
                format_balance(must_attach),
            ))
        }

        let lic = TokenLicense{
            id: new_license_id,
            title: Some(new_license.title.clone()),
            description: None,
            from: SourceLicenseMeta{
                asset_id: asset.token_id.clone(),
                inventory_id: inv_id.clone(),
                set_id: new_asset_license.set_id.clone().unwrap_or(String::new()),
                sku_id: new_asset_license.sku_id.clone().unwrap(),
            },
            issuer_id: Some(env::current_account_id()),
            uri: new_license.license.pdf_url.clone(),
            metadata: new_license.license.clone(),
            issued_at: Some(env::block_timestamp_ms()),
            starts_at: Some(env::block_timestamp_ms()),
            expires_at: None,
            updated_at: None,
        };
        let new_metadata = asset.issue_new_metadata(new_asset_license.set_id.clone().unwrap());

        let new_token = LicenseToken{
            owner_id: token.owner_id.clone(),
            license: Some(lic.clone()),
            metadata: new_metadata.clone(),
            asset_id: asset_id.clone(),
            token_id: token_id.clone(),
            approved_account_ids: token.approved_account_ids.clone(),
        };

        let result = self.policies.clone_with_additional(
            asset.policy_rules.clone().unwrap_or_default().clone()
        ).check_transition(full_inventory, token, new_token.clone());
        // Check result of transition attempt.
        if result.is_err() {
            env::panic_str(unsafe{result.unwrap_err_unchecked().as_str()})
        } else {
            let avail = result.unwrap();
            if !avail.result {
                env::panic_str(avail.reason_not_available.as_str())
            }
        }

        Ok((lic, must_attach))
    }

    pub fn get_full_inventory(&self, asset: JsonAssetToken, metadata: InventoryContractMetadata) -> FullInventory {
        // Build full inventory for those.
        // First, populate licenses with actual prices from asset
        let mut inventory_licenses: Vec<InventoryLicense> = Vec::new();
        if asset.licenses.is_some() {
            for asset_l in asset.licenses.as_ref().unwrap() {
                for inv_license in &metadata.licenses {
                    if inv_license.license_id == asset_l.license_id {
                        let mut price = inv_license.price.clone().unwrap();
                        if asset_l.price.is_some() {
                            price = asset_l.price.as_ref().unwrap().clone();
                        }
                        inventory_licenses.push(InventoryLicense{
                            license_id: inv_license.license_id.clone(),
                            price: Some(price),
                            title: asset_l.title.clone(),
                            license: inv_license.license.clone(),
                        })
                    }
                }
            }
        }
        let tokens = self.nft_tokens(
            None,
            Some(MAX_LIMIT),
            Some(FilterOpt{asset_id: Some(asset.token_id.clone()), account_id: None})
        );
        let full_inventory = FullInventory{
            inventory_licenses,
            issued_licenses: tokens,
            asset: Some(asset.clone()),
        };
        full_inventory
    }

    #[payable]
    pub fn nft_reject_license(&mut self, token_id: TokenId){
       //measure the initial storage being used on the contract
        assert_one_yocto(); // user need to sign for approve transaction

        let initial_storage_usage = env::storage_usage();

        let token = self.tokens_by_id.get(&token_id).expect("No token");
        let predecessor_id = env::predecessor_account_id();


        if predecessor_id != token.owner_id {
            panic!("Only the token owner can approve a license update");
        }

        self.internal_reject_license(&predecessor_id, &token_id); 

        // Construct the mint log as per the events standard.
        let nft_reject_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.

            event: EventLogVariant::NftRejectLicense(vec![NftRejectLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_reject_license_log.to_string());

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        let _ = refund_storage(initial_storage_usage, None, None);
    }


    #[payable]
    pub fn nft_approve_license(&mut self, token_id: TokenId){
       //measure the initial storage being used on the contract
        assert_one_yocto(); // user need to sign for approve transaction

        let initial_storage_usage = env::storage_usage();

        let token = self.tokens_by_id.get(&token_id).expect("No token");
        let predecessor_id = env::predecessor_account_id();


        if predecessor_id != token.owner_id {
            panic!("Only the token owner can approve a license update");
        }

        self.internal_update_license(&predecessor_id, &token_id); 

        // Construct the mint log as per the events standard.
        let nft_license_update_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.

            event: EventLogVariant::NftApproveLicense(vec![NftApproveLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_license_update_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            let _ = refund_deposit(storage_usage - initial_storage_usage, None, None);
        }
    }

    #[payable]
    pub fn nft_propose_license(&mut self, token_id: TokenId, proposed_license: TokenLicense){
       //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let predecessor_id = env::predecessor_account_id();
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        self.internal_propose_license(&predecessor_id, &token_id, &proposed_license);

        // Construct the mint log as per the events standard.
        let nft_propose_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftProposeLicense(vec![NftProposeLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_propose_license_log.to_string());

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        let _ = refund_storage(initial_storage_usage, None, None);
    }

    //get the information for a specific token ID
    pub fn nft_license(&self, token_id: TokenId) -> Option<JsonTokenLicense> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            if let Some(license) = self.token_license_by_id.get(&token_id) {
                //we return the JsonTokenLicense (wrapped by Some since we return an option)
                Some(JsonTokenLicense {
                    token_id,
                    owner_id: token.owner_id,
                    license,
                })
            } else {
                None
            }
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }

    //get the information for a specific token ID
    pub fn nft_proposed_license(&self, token_id: TokenId) -> Option<JsonTokenLicense> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            // let license = self.token_license_by_id.get(&token_id).unwrap();
            if let Some(license) = self.token_proposed_license_by_id.get(&token_id) {
            //we return the JsonTokenLicense (wrapped by Some since we return an option)
                Some(JsonTokenLicense {
                    token_id,
                    owner_id: token.owner_id,
                    license,
                })
            } else {
                None
            }
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }
    #[private]
    pub fn internal_propose_license(&mut self, account_id: &AccountId, token_id: &TokenId, proposed_license: &TokenLicense) {
        println!("==>internal_propose_license, account={}", account_id);
        if let Some(_license) = self.token_proposed_license_by_id.get(&token_id) {
            self.token_proposed_license_by_id.remove(&token_id);
        }
        self.token_proposed_license_by_id.insert(&token_id, &proposed_license);
    }

    #[private]
    pub fn internal_update_license(&mut self, account_id: &AccountId, token_id: &TokenId) {
        println!("==>internal_update_license, account={}", account_id);
        if let Some(proposed_license) = self.token_proposed_license_by_id.get(&token_id) {
            self.token_proposed_license_by_id.remove(&token_id );
            if let Some(_license) = self.token_license_by_id.get(&token_id) {
                self.token_license_by_id.remove(&token_id);
            }
            self.token_license_by_id.insert(&token_id, &proposed_license);
        } else {
            log!("No proposed license i the token");
            panic!("No propose license in the token");
        }
    }

    #[private]
    pub fn internal_reject_license(&mut self, account_id: &AccountId, token_id: &TokenId) {
        println!("==>internal_restore_license, account={}", account_id);
        if let Some(_proposed_license) = self.token_proposed_license_by_id.get(&token_id) {
            self.token_proposed_license_by_id.remove(&token_id );
        } else {
            log!("No proposed license in the token");
            panic!("No propose license in the token");
        }
    }

    #[private]
    pub fn internal_replace_license(&mut self, account_id: &AccountId, token_id: &TokenId, license: &TokenLicense) {
        println!("==>internal_replace_license, account={}", account_id);
        if let Some(_license) = self.token_license_by_id.get(&token_id) {
            self.token_license_by_id.remove(&token_id);

        }
        self.token_license_by_id.insert(&token_id, &license);
    }

    #[private]
    pub fn license_approval(
        sender_id: AccountId, 
        account_id: AccountId, 
        token_id: TokenId,
        approve: bool, 
        deposit: Balance, 
        gas_limit: Gas,
    ) -> bool {
        println!(
            "==>license_authorization, sender={}, account={}, token={}, deposit={}, gas_limit={}",
            sender_id, account_id, token_id, deposit, serde_json::to_string(&gas_limit).unwrap(),
        );
        assert_one_yocto();

        //get the GAS attached to the call
        let attached_gas = env::prepaid_gas();

        /*
            make sure that the attached gas is greater than the minimum GAS for NFT approval call.
            This is to ensure that the cross contract call to internal_update_license won't cause a prepaid GAS error.
        */
        assert!(
            attached_gas >= MIN_GAS_FOR_LICENSE_APPROVE_CALL,
            "You cannot attach less than {:?} Gas to nft_transfer_call",
            MIN_GAS_FOR_LICENSE_APPROVE_CALL
        );

        approve
    }
}

