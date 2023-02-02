use near_sdk::{PromiseError};
use near_sdk::serde::{Deserialize, Serialize};

use common_types::types::ExtendedInventoryMetadata;
use common_types::utils::refund_storage;

use crate::*;

/// This spec can be treated like a version of the standard.
pub const INVENTORY_METADATA_SPEC: &str = "inventory-1.0.0";

//The Json token is what will be returned from view calls.
#[derive(Serialize, Deserialize)]
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
    fn update_inventory_metadata(&mut self, metadata: InventoryContractMetadata) -> Promise;
    fn update_inventory_licenses(&mut self, licenses: Vec<InventoryLicense>) -> Promise;
    fn add_inventory_license(&mut self, license: InventoryLicense) -> ExtendedInventoryMetadata;
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
    fn update_inventory_metadata(&mut self, metadata: InventoryContractMetadata) -> Promise {
        self.ensure_owner();

        let check_state = policy_rules_contract::ext(self.policy_contract.clone())
            .with_unused_gas_weight(3).check_inventory_state(
            metadata.licenses.clone(),
        );
        let on_check_promise = check_state.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .with_unused_gas_weight(27)
                .on_update_inventory_metadata(
                    metadata.clone(),
                    env::predecessor_account_id(),
                )
        );
        on_check_promise

        // let initial_storage_usage = env::storage_usage();
        // let res = self._update_inventory_metadata(
        //     metadata, self.owner_id.clone(), "update_metadata".to_string()
        // );
        // let _ = refund_storage(initial_storage_usage, None, None);
        // res
    }

    #[payable]
    fn update_inventory_licenses(&mut self, licenses: Vec<InventoryLicense>) -> Promise {
        self.ensure_owner();

        let check_state = policy_rules_contract::ext(self.policy_contract.clone())
            .with_unused_gas_weight(3).check_inventory_state(
            licenses.clone()
        );
        let on_check_promise = check_state.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .with_unused_gas_weight(27)
                .on_update_inventory_licenses(
                    licenses.clone(),
                    env::predecessor_account_id(),
                )
        );
        on_check_promise
        // let res = self.policies.check_inventory_state(licenses.clone());
        // if !res.result {
        //     env::panic_str(res.reason_not_available.as_str())
        // }
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MetadataResult {
    #[serde(flatten)]
    pub metadata: Option<ExtendedInventoryMetadata>,
    pub error: Option<String>,
}

#[near_bindgen]
impl InventoryContract {
    #[private]
    pub(crate) fn _update_inventory_metadata(
        &mut self,
        metadata: InventoryContractMetadata) -> ExtendedInventoryMetadata {

        let old_metadata = self.metadata.get().unwrap();
        let old_minter = old_metadata.default_minter_id;
        let mut new_metadata = metadata.clone();
        new_metadata.default_minter_id = old_minter;

        self.metadata.replace(&new_metadata);
        ExtendedInventoryMetadata {
            metadata: self.metadata.get().unwrap(),
            asset_count: self.token_metadata_by_id.len(),
            owner_id: self.owner_id.clone(),
        }
    }

    pub fn on_update_inventory_licenses(
        &mut self,
        #[callback_result] check_state_res: Result<IsAvailableResponseData, PromiseError>,
        licenses: Vec<InventoryLicense>,
        predecessor_id: AccountId,
    ) -> MetadataResult {
        let initial_storage_usage = env::storage_usage();

        if check_state_res.is_err() {
            let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
            return MetadataResult{error: Some("Failed call check_inventory_state()".to_string()), metadata: None}
        } else {
            let result = check_state_res.unwrap();
            if !result.result {
                let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
                return MetadataResult{error: Some(result.reason_not_available), metadata: None}
            }
        }

        let mut meta = self.metadata.get().unwrap();
        meta.licenses = licenses.clone();
        self.metadata.replace(&meta);

        if licenses.len() > 0 {
            let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
        }

        MetadataResult {
            metadata: Some(ExtendedInventoryMetadata {
                metadata: self.metadata.get().unwrap(),
                asset_count: self.token_metadata_by_id.len(),
                owner_id: self.owner_id.clone(),
            }),
            error: None,
        }
    }

    pub fn on_update_inventory_metadata(
        &mut self,
        #[callback_result] check_state_res: Result<IsAvailableResponseData, PromiseError>,
        metadata: InventoryContractMetadata,
        predecessor_id: AccountId,
    ) -> MetadataResult {
        let initial_storage_usage = env::storage_usage();

        if check_state_res.is_err() {
            let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
            return MetadataResult{error: Some("Failed call check_inventory_state()".to_string()), metadata: None}
        } else {
            let result = check_state_res.unwrap();
            if !result.result {
                let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
                return MetadataResult{error: Some(result.reason_not_available), metadata: None}
            }
        }

        let result = self._update_inventory_metadata(metadata.clone());

        let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
        MetadataResult {
            metadata: Some(result),
            error: None,
        }

    }

    // pub fn on_new(
    //     &mut self,
    //     #[callback_result] check_state_res: Result<IsAvailableResponseData, PromiseError>,
    //     owner_id: AccountId,
    //     policy_contract: AccountId,
    //     metadata: InventoryContractMetadata,
    //     predecessor_id: AccountId,
    // ) -> Self {
    //     let initial_storage_usage = env::storage_usage();
    //
    //     if check_state_res.is_err() {
    //         let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
    //         // return Err("Failed call check_inventory_state()".to_string())
    //         env::panic_str("Failed call check_inventory_state()")
    //     } else {
    //         let result = check_state_res.unwrap();
    //         if !result.result {
    //             let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
    //             // return Err(result.reason_not_available)
    //             env::panic_str(&result.reason_not_available)
    //         }
    //     }
    //
    //     let this = Self {
    //         //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
    //         tokens_per_owner: LookupMap::new(StorageKey::AssetPerOwner.try_to_vec().unwrap()),
    //
    //         tokens_by_id: LookupMap::new(StorageKey::AssetById.try_to_vec().unwrap()),
    //
    //         token_metadata_by_id: UnorderedMap::new(StorageKey::AssetMetadataById.try_to_vec().unwrap()),
    //         token_licenses_by_id: UnorderedMap::new(StorageKey::AssetLicensesById.try_to_vec().unwrap()),
    //
    //         //set the owner_id field equal to the passed in owner_id.
    //         owner_id,
    //         metadata: LazyOption::new(
    //             StorageKey::InventoryContractMetadata.try_to_vec().unwrap(),
    //             Some(&metadata),
    //         ),
    //         policy_contract: policy_contract.clone(),
    //     };
    //
    //     //return the Contract object
    //     this
    // }

    // pub fn on_restore(
    //     &mut self,
    //     #[callback_result] check_state_res: Result<IsAvailableResponseData, PromiseError>,
    //     owner_id: AccountId,
    //     policy_contract: AccountId,
    //     metadata: InventoryContractMetadata,
    //     predecessor_id: AccountId,
    // ) -> Self {
    //     let initial_storage_usage = env::storage_usage();
    //
    //     if check_state_res.is_err() {
    //         let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
    //         // return Err("Failed call check_inventory_state()".to_string())
    //         env::panic_str("Failed call check_inventory_state()")
    //     } else {
    //         let result = check_state_res.unwrap();
    //         if !result.result {
    //             let _ = refund_storage(initial_storage_usage, Some(predecessor_id), None);
    //             // return Err(result.reason_not_available)
    //             env::panic_str(&result.reason_not_available)
    //         }
    //     }
    //
    //     let this = Self {
    //         //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
    //         tokens_per_owner: LookupMap::new(StorageKey::AssetPerOwner.try_to_vec().unwrap()),
    //         tokens_by_id: LookupMap::new(StorageKey::AssetById.try_to_vec().unwrap()),
    //         token_metadata_by_id: UnorderedMap::new(StorageKey::AssetMetadataById.try_to_vec().unwrap()),
    //         token_licenses_by_id: UnorderedMap::new(StorageKey::AssetLicensesById.try_to_vec().unwrap()),
    //
    //         //set the owner_id field equal to the passed in owner_id.
    //         owner_id,
    //         metadata: LazyOption::new(
    //             StorageKey::InventoryContractMetadata.try_to_vec().unwrap(),
    //             Some(&metadata),
    //         ),
    //         policy_contract: policy_contract.clone(),
    //     };
    //
    //     //return the Contract object
    //     this
    // }
}