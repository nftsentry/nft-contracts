use crate::*;
// use near_sdk::{require};
use near_sdk::serde::{Deserialize, Serialize};
use policy_rules::utils::refund_deposit;

/// This spec can be treated like a version of the standard.
pub const INVENTORY_METADATA_SPEC: &str = "inventory-1.0.0";

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: String,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    // license metadata
    // pub license: Option<TokenLicense>,
    // proposed license 
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    // pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    // pub royalty: HashMap<AccountId, u32>,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTokenLicense {
    //token ID
    pub token_id: String,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    // pub license: TokenLicense,
    // proposed license 
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
}


pub trait InventoryMetadata {
    //view call for returning the contract metadata
    fn inventory_metadata(&self) -> InventoryContractMetadata;
    fn inventory_licenses(&self) -> InventoryLicenses;
    fn update_inventory_licenses(&mut self, licenses: InventoryLicenses) -> InventoryContractMetadata;
    fn add_inventory_license(&mut self, license: InventoryLicense) -> InventoryContractMetadata;
}

#[near_bindgen]
impl InventoryMetadata for InventoryContract {
    fn inventory_metadata(&self) -> InventoryContractMetadata {
        self.metadata.get().unwrap()
    }

    fn inventory_licenses(&self) -> InventoryLicenses {
        let met = self.metadata.get().unwrap();
        met.licenses
    }

    #[payable]
    fn update_inventory_licenses(&mut self, licenses: InventoryLicenses) -> InventoryContractMetadata {
        let (ok, reason) = self.policies.check_inventory_state(licenses.clone());
        if !ok {
            env::panic_str(reason.as_str())
        }

        let initial_storage_usage = env::storage_usage();

        let mut meta = self.metadata.get().unwrap();
        meta.licenses = licenses.clone();
        self.metadata.replace(&meta);

        if licenses.len() > 0 {
            let new_storage_usage = env::storage_usage();
            let storage_usage_diff = new_storage_usage - initial_storage_usage;
            let log_message = format!("Storage usage increased by {} bytes", storage_usage_diff);
            env::log_str(&log_message);
            let _ = refund_deposit(storage_usage_diff, None, None);
        }

        self.metadata.get().unwrap()
    }
    #[payable]
    fn add_inventory_license(&mut self, license: InventoryLicense) -> InventoryContractMetadata {
        let initial_storage_usage = env::storage_usage();


        let mut meta = self.metadata.get().unwrap();
        meta.licenses.push(license);
        self.metadata.replace(&meta);

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        let _ = refund_deposit(required_storage_in_bytes, None, None);

        self.metadata.get().unwrap()
    }
}

