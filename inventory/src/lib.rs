// use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
};

pub use crate::metadata::*;
pub use crate::events::*;
pub use crate::mint::*;

use crate::internal::*;

/* 
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::license::*;

pub mod approval;
pub mod nft_core;
mod royalty; 
pub mod license;
*/
mod enumeration; 
mod internal;
pub mod metadata;
pub mod mint;
mod events;
mod tests;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";
/// This spec can be treated like a version of the standard.
pub const NFT_LICENSE_SPEC: &str = "nftsentry-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_LICENSE_STANDARD_NAME: &str = "nepTBD";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct InventoryContract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<AssetTokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<AssetTokenId, AssetToken>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<AssetTokenId, AssetTokenMetadata>,

    //keeps track of the asset minter for a given token ID
    pub token_minter_by_id: UnorderedMap<AssetTokenId, AssetMinterContractID>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<InventoryContractMetadata>,

    pub disable_events: bool,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    InventoryContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
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
    pub fn new(owner_id: AccountId, metadata: InventoryContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadataById.try_to_vec().unwrap()),
            // token_minter_by_id: UnorderedMap::new(StorageKey::TokenMinterById.try_to_vec().unwrap()),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::InventoryContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            disable_events: false,
        };

        //return the Contract object
        this
    }

    //get the information for a specific token ID
    pub fn asset_token(&self, token_id: AssetTokenId) -> Option<JsonToken> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
        //    let license = self.token_license_by_id.get(&token_id);
        //    let proposed_license = self.token_proposed_license_by_id.get(&token_id).unwrap();
            //we return the JsonToken (wrapped by Some since we return an option)
            Some(JsonToken {
                token_id,
                owner_id: token.owner_id,
                metadata,
            //    license,
            //    proposed_license,     TODO: show proposed license. If proposed license
            //    approved_account_ids: token.approved_account_ids,
            //    royalty: token.royalty,
            })
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }

    // get information abount the contract
    pub fn inventory_contract_metadata(&self) -> Option<InventoryContractMetadata> {
        self.metadata.get()
    }
}