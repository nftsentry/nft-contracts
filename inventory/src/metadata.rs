use crate::*;
use near_sdk::{require};
use near_sdk::serde::{Deserialize, Serialize};

pub type AssetTokenId = String;
pub type AssetMinterContractId = String;
pub type AssetLicenses = Vec<AssetLicense>;
pub type InventoryLicenses = Vec<InventoryLicense>;

/// This spec can be treated like a version of the standard.
pub const INVENTORY_METADATA_SPEC: &str = "inventory-1.0.0";

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryLicense {
    pub license_id: String,
    pub title: String,
    pub price: Balance,
    pub license: LicenseData,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetLicense {
    pub license_id: String,
    pub title: String,
    pub price: Balance,
//    pub license: LicenseData,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub licenses: InventoryLicenses,            // required, ex. "MOSIAC"
}

impl InventoryContractMetadata {
    pub fn assert_valid(&self) {
        require!(self.spec == INVENTORY_METADATA_SPEC, "Spec is not inventory metadata");
        require!(
            self.reference.is_some() == self.reference_hash.is_some(),
            "Reference and reference hash must be present"
        );
        if let Some(reference_hash) = &self.reference_hash {
            require!(reference_hash.0.len() == 32, "Hash has to be 32 bytes");
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetTokenMetadata {
    // The title for this token
    pub title: Option<String>, 
    // The free-form description for this token
    pub description: Option<String>, 
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>, 
    // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub media_hash: Option<Base64VecU8>, 
    // number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u64>, 
    // When token was issued or minted, Unix epoch in milliseconds
    pub issued_at: Option<u64>, 
    // When token expires, Unix epoch in milliseconds
    pub expires_at: Option<u64>,
    // When token starts being valid, Unix epoch in milliseconds 
    pub starts_at: Option<u64>, 
    // When token was last updated, Unix epoch in milliseconds
    pub updated_at: Option<u64>, 
    // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub extra: Option<String>, 
    // URL to an off-chain JSON file with more info.
    pub reference: Option<String>, 
    // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub reference_hash: Option<Base64VecU8>, 
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct LicenseData {
    pub i_agree: bool,
    pub perpetuity: Option<bool>,
    pub exclusivity: Option<bool>,
    pub personal_use: bool,
    pub commercial_use: Option<bool>,
    pub limited_display_sublicensee: Option<bool>,
    pub template: Option<String>,
    pub pdf_url: Option<String>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[derive(PartialEq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenLicense {
    // pub test: u8,
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub issuer_id: Option<AccountId>, // AccountId of the license issuer
    pub uri: Option<String>, // URL to associated pdf, preferably to decentralized, content-addressed storage
    pub metadata: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
//#[derive(Debug)]
//#[derive(BorshDeserialize, BorshSerialize)]
pub struct AssetToken {
    // token_id: AssetTokenId,
    pub token_id: AssetTokenId,
    //owner of the token
    pub owner_id: AccountId,
    //minter of the token
    pub minter_id: Option<AccountId>,
    //token metadata
    pub metadata: AssetTokenMetadata,
    // list of approved licenses available for this token
    pub licenses: Option<AssetLicenses>,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
//#[derive(Debug)]
//#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonAssetToken {
    //token ID
    pub token_id: AssetTokenId,
    //owner of the token
    pub owner_id: AccountId,
    // minter_id
    pub minter_id: Option<AccountId>,
    //token metadata
    pub metadata: AssetTokenMetadata,
    // license metadata
    pub licenses: Option<AssetLicenses>,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: AssetTokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: AssetTokenMetadata,
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
    pub token_id: AssetTokenId,
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
        let initial_storage_usage = env::storage_usage();

        let mut meta = self.metadata.get().unwrap();
        meta.licenses = licenses.clone();
        self.metadata.replace(&meta);

        if licenses.len() > 0 {
            let new_storage_usage = env::storage_usage();
            let storage_usage_diff = new_storage_usage - initial_storage_usage;
            let log_message = format!("Storage usage increased by {} bytes", storage_usage_diff);
            env::log_str(&log_message);
            refund_deposit(storage_usage_diff);
        }
//        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
//        refund_deposit(required_storage_in_bytes);

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
        refund_deposit(required_storage_in_bytes);
    

        self.metadata.get().unwrap()
    }
}

