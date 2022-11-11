use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use policy_rules::types::ExtendedInventoryMetadata;
use policy_rules::utils::{refund_storage};

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
    fn inventory_metadata(&self) -> ExtendedInventoryMetadata;
    fn update_inventory_metadata(&mut self, metadata: InventoryContractMetadata) -> ExtendedInventoryMetadata;
    fn update_inventory_licenses(&mut self, licenses: Vec<InventoryLicense>) -> ExtendedInventoryMetadata;
    fn add_inventory_license(&mut self, license: InventoryLicense) -> ExtendedInventoryMetadata;
}

#[near_bindgen]
impl InventoryContract {
    #[private]
    pub(crate) fn _update_inventory_metadata(&mut self, metadata: InventoryContractMetadata) -> ExtendedInventoryMetadata {
        let res = self.policies.check_inventory_state(metadata.licenses.clone());
        if !res.result {
            env::panic_str(res.reason_not_available.as_str())
        }
        let old_metadata = self.metadata.get().unwrap();
        let old_minter = old_metadata.default_minter_id;
        let mut new_metadata = metadata.clone();
        new_metadata.default_minter_id = old_minter;

        self.metadata.replace(&new_metadata);
        ExtendedInventoryMetadata{
            metadata: self.metadata.get().unwrap(),
            asset_count: self.token_metadata_by_id.len(),
            owner_id: self.owner_id.clone(),
        }
    }
}

#[near_bindgen]
impl InventoryMetadata for InventoryContract {
    fn inventory_metadata(&self) -> ExtendedInventoryMetadata {
        ExtendedInventoryMetadata{
            metadata: self.metadata.get().unwrap(),
            asset_count: self.token_metadata_by_id.len(),
            owner_id: self.owner_id.clone(),
        }
    }

    #[payable]
    fn update_inventory_metadata(&mut self, metadata: InventoryContractMetadata) -> ExtendedInventoryMetadata {
        self.ensure_owner();

        let initial_storage_usage = env::storage_usage();

        let res = self._update_inventory_metadata(metadata);

        let _ = refund_storage(initial_storage_usage, None, None);

        res
    }

    #[payable]
    fn update_inventory_licenses(&mut self, licenses: Vec<InventoryLicense>) -> ExtendedInventoryMetadata {
        self.ensure_owner();

        let res = self.policies.check_inventory_state(licenses.clone());
        if !res.result {
            env::panic_str(res.reason_not_available.as_str())
        }

        let initial_storage_usage = env::storage_usage();

        let mut meta = self.metadata.get().unwrap();
        meta.licenses = licenses.clone();
        self.metadata.replace(&meta);

        if licenses.len() > 0 {
            let _ = refund_storage(initial_storage_usage, None, None);
        }

        ExtendedInventoryMetadata{
            metadata: self.metadata.get().unwrap(),
            asset_count: self.token_metadata_by_id.len(),
            owner_id: self.owner_id.clone(),
        }
    }
    #[payable]
    fn add_inventory_license(&mut self, license: InventoryLicense) -> ExtendedInventoryMetadata {
        let initial_storage_usage = env::storage_usage();


        let mut meta = self.metadata.get().unwrap();
        meta.licenses.push(license);
        self.metadata.replace(&meta);

        let _ = refund_storage(initial_storage_usage, None, None);

        ExtendedInventoryMetadata{
            metadata: self.metadata.get().unwrap(),
            asset_count: self.token_metadata_by_id.len(),
            owner_id: self.owner_id.clone(),
        }
    }
}

