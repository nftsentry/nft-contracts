use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, ext_contract, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use policy_rules::policy::{AllPolicies, init_policies};
pub use policy_rules::types::{NFTContractMetadata, Token, TokenLicense, TokenMetadata};
pub use policy_rules::types::{LicenseToken, FilterOpt};
pub use policy_rules::utils::*;
use policy_rules::types::{AssetTokenOpt, InventoryContractMetadata, JsonAssetToken};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::license::*;

mod internal;
pub mod approval;
mod enumeration; 
pub mod metadata;
pub mod mint;
pub mod nft_core;
mod royalty; 
mod events;
pub mod license;
mod tests;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";
/// This spec can be treated like a version of the standard.
pub const NFT_LICENSE_SPEC: &str = "nftsentry-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_LICENSE_STANDARD_NAME: &str = "nepTBD";
pub const TGAS: u64 = 1_000_000_000_000;
pub const MAX_LIMIT: u64 = 1_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the token license for a given token ID
    pub token_license_by_id: UnorderedMap<TokenId, TokenLicense>,

    //keeps track of the token license for a given token ID
    pub token_proposed_license_by_id: UnorderedMap<TokenId, TokenLicense>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    pub policies: AllPolicies,
    pub disable_events: bool,
}

#[ext_contract(inventory_contract)]
pub trait InventoryContract {
    fn inventory_metadata(&self) -> InventoryContractMetadata;
    fn asset_token(&self, token_id: String, opt: Option<AssetTokenOpt>) -> Option<JsonAssetToken>;
    fn asset_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonAssetToken>;
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    TokenLicenseById,
    TokenProposedLicenseById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
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
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "NFTSentry Contract 0.0.1".to_string(),
                symbol: "SENTRY".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized.
        let policies = init_policies();
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            token_license_by_id: UnorderedMap::new(
                StorageKey::TokenLicenseById.try_to_vec().unwrap(),
            ),
            token_proposed_license_by_id: UnorderedMap::new(
                StorageKey::TokenProposedLicenseById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            disable_events: false,
            policies,
        };

        //return the Contract object
        this
    }
}