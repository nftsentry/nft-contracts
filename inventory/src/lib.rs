// use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128};
use near_sdk::{env, near_bindgen, ext_contract, AccountId, CryptoHash, PanicOnDefault};

pub use crate::metadata::*;
pub use crate::events::*;
pub use crate::mint::*;
pub use policy_rules::*;
pub use policy_rules::types::{TokenMetadata, AssetToken};
pub use policy_rules::types::{AssetLicense, InventoryLicenseAvailability, FilterOpt};
pub use policy_rules::types::{InventoryContractMetadata, InventoryLicense};
pub use policy_rules::types::{LicenseToken, TokenId, JsonAssetToken};
use policy_rules::policy::{AllPolicies, ConfigInterface, init_policies};
use policy_rules::utils::{refund_storage};

mod enumeration;
mod internal;
pub mod metadata;
pub mod mint;
pub mod nft_callbacks;
mod events;
mod tests;
mod asset;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct InventoryContract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<String>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<String, AssetToken>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<String, TokenMetadata>,

    //keeps track of the asset minter for a given token ID
    pub token_licenses_by_id: UnorderedMap<String, Vec<AssetLicense>>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<InventoryContractMetadata>,

    pub policies: AllPolicies,
    pub disable_events: bool,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    AssetPerOwner,
    AssetPerOwnerInner { account_id_hash: CryptoHash },
    AssetById,
    AssetMetadata,
    AssetMetadataById,
    AssetMinterById,
    AssetLicensesById,
    InventoryContractMetadata,
    // TokensPerType,
    // TokensPerTypeInner { token_type_hash: CryptoHash },
    // TokenTypesLocked,
}

#[ext_contract(license_contract)]
pub trait LicenseContract {
    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>, filter_opt: Option<FilterOpt>) -> Vec<LicenseToken>;
    fn nft_token(&self, token_id: TokenId) -> Option<LicenseToken>;
}

#[near_bindgen]
impl InventoryContract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            InventoryContractMetadata {
                spec: "inventory-1.0.0".to_string(),
                name: "NFTSentry InventoryContract 0.0.2".to_string(),
                symbol: "SENTRY".to_string(),
                icon: None,
                background_image: None,
                description: None,
                licenses: Vec::new(),
                default_minter_id: "".to_string(),
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: InventoryContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized.
        let policies = init_policies();
        let res = policies.check_inventory_state(metadata.licenses.clone());
        if !res.result {
            env::panic_str(res.reason_not_available.as_str())
        }
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::AssetPerOwner.try_to_vec().unwrap()),

            tokens_by_id: LookupMap::new(StorageKey::AssetById.try_to_vec().unwrap()),
            
            token_metadata_by_id: UnorderedMap::new(StorageKey::AssetMetadataById.try_to_vec().unwrap()),
            token_licenses_by_id: UnorderedMap::new(StorageKey::AssetLicensesById.try_to_vec().unwrap()),
            
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::InventoryContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            disable_events: false,
            policies,
        };

        //return the Contract object
        this
    }

    #[init]
    #[payable]
    pub fn restore(owner_id: AccountId, metadata: InventoryContractMetadata, tokens: Vec<JsonAssetToken>) -> Self {
        // let initial_storage_usage = env::storage_usage();
        // Restore metadata
        let mut this = Self::new(owner_id, metadata.clone());

        let logs = this._restore_data(metadata, tokens);

        //calculate the required storage which was the used - initial
        // let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        // let _ = refund_deposit(required_storage_in_bytes, None, None);

        for log in logs {
            this.log_event(&log.to_string())
        }

        this
    }

    #[payable]
    pub fn set_state(&mut self, metadata: InventoryContractMetadata, tokens: Vec<JsonAssetToken>) {
        self.ensure_owner();

        let initial_storage_usage = env::storage_usage();
        // Restore metadata & data
        let logs = self._restore_data(metadata, tokens);

        let _ = refund_storage(initial_storage_usage, None, None);

        for log in logs {
            self.log_event(&log.to_string())
        }
    }

    #[payable]
    fn _restore_data(&mut self, metadata: InventoryContractMetadata, tokens: Vec<JsonAssetToken>) -> Vec<EventLog> {
        let mut logs: Vec<EventLog> = Vec::new();

        self._update_inventory_metadata(metadata);

        for token_src in tokens {
            // -- migration block
            let mut token = token_src.clone();
            token.migrate_to_sets();
            // -- end migration block
            let exists = self.tokens_by_id.contains_key(&token.token_id);
            if exists {
                self.asset_replace(
                    token.token_id.clone(),
                    token.metadata.clone(),
                    token.licenses,
                    token.policy_rules.clone(),
                    token.upgrade_rules.clone(),
                );
            } else {
                let event = self.internal_mint(
                    token.token_id.clone(),
                    token.metadata.clone(),
                    token.owner_id.clone(),
                    token.minter_id.clone(),
                    token.licenses,
                    token.policy_rules.clone(),
                    token.upgrade_rules.clone(),
                );
                logs.push(event);
            }
            let asset_token = self.tokens_by_id.get(&token.token_id);
            self._on_nft_mint(asset_token.unwrap().clone(), token.license_token_count);

        }

        logs
    }

    fn ensure_owner(&self) {
        let sender = env::predecessor_account_id();
        if sender != self.owner_id && sender != env::current_account_id() {
            env::panic_str("Unauthorized")
        }
    }
}