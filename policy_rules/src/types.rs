use near_sdk::env;
use crate::*;
use crate::policy::{Limitation, LimitsInfo};

pub type TokenId = String;
pub type AssetId = String;
//defines the payout type we'll be returning as a part of the royalty standards.

/// This spec can be treated like a version of the standard.
pub const INVENTORY_METADATA_SPEC: &str = "inventory-1.0.0";

pub trait LicenseGeneral {
    fn is_exclusive(&self) -> bool;
    fn is_personal(&self) -> bool;
    fn is_commercial(&self) -> bool;
    fn license_id(&self) -> String;
    fn license_title(&self) -> String;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenExtra {
    asset_id_path: String
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[derive(Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub previews: Option<String>, // URL/JSON with URLs to image previews
    pub object: Option<String>, // URL to an object
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

impl TokenMetadata {
    pub fn issue_new_metadata(&self) -> TokenMetadata {
        let mut metadata = self.clone();
        metadata.issued_at = Some(env::block_timestamp_ms());
        metadata.updated_at = Some(env::block_timestamp_ms());
        metadata.starts_at = Some(env::block_timestamp_ms());
        metadata
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LicenseData {
    pub i_agree: bool,
    pub perpetuity: bool,
    pub exclusivity: bool,
    pub personal_use: bool,
    pub commercial_use: bool,
    pub limited_display_sublicensee: bool,
    pub template: Option<String>,
    pub pdf_url: Option<String>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenLicense {
    pub id: String,
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub issuer_id: Option<AccountId>, // AccountId of the license issuer
    pub uri: Option<String>, // URL to associated pdf, preferably to decentralized, content-addressed storage
    pub metadata: LicenseData,
    pub from: SourceLicenseMeta,
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SourceLicenseMeta {
    pub inventory_id: String,
    pub asset_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    pub asset_id: AssetId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //the next approval ID to give out.
    pub next_approval_id: u64,
    //keep track of the royalty percentages for the token in a hash map
    // pub royalty: HashMap<AccountId, u32>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FilterOpt {
    pub account_id: Option<AccountId>,
    pub asset_id: Option<AssetId>,
}

//The Json token is what will be returned from view calls.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LicenseToken {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //asset id of the token
    pub asset_id: AssetId,
    //token metadata
    pub metadata: TokenMetadata,
    // license metadata
    pub license: Option<TokenLicense>,
    // proposed license
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    // pub royalty: HashMap<AccountId, u32>,
}

impl LicenseToken {
    pub fn inventory_asset_license(&self) -> (String, String, String) {
        if self.license.is_none() {
           return (String::new(), String::new(), String::new())
        }
        unsafe {
            return (
                self.license.as_ref().unwrap_unchecked().from.inventory_id.clone(),
                self.license.as_ref().unwrap_unchecked().from.asset_id.clone(),
                self.license.as_ref().unwrap_unchecked().id.clone()
            )
        }
    }

    pub fn as_inventory_license(&self, price: Option<String>) -> Option<InventoryLicense> {
        if self.license.is_none() {
            return None
        }
        unsafe {
            let license_data = &self.license.as_ref().unwrap_unchecked().metadata;
            let (_, _, lic_id) = self.inventory_asset_license();
            let res = InventoryLicense {
                license_id: lic_id,
                title: self.license.as_ref().unwrap_unchecked().title.as_ref().unwrap().to_string(),
                license: license_data.clone(),
                price: if price.is_some() { price.unwrap_unchecked() } else { String::new() },
            };
            Some(res)
        }
    }
}

impl LicenseGeneral for LicenseToken {
    fn is_exclusive(&self) -> bool {
        unsafe {
            self.license.as_ref().unwrap_unchecked().metadata.exclusivity
        }
    }

    fn is_personal(&self) -> bool {
        unsafe {
            self.license.as_ref().unwrap_unchecked().metadata.personal_use
        }
    }

    fn is_commercial(&self) -> bool {
        unsafe {
            !self.license.as_ref().unwrap_unchecked().metadata.personal_use
        }
    }

    fn license_id(&self) -> String {
        unsafe {
            self.license.as_ref().unwrap_unchecked().id.clone()
        }
    }

    fn license_title(&self) -> String {
        unsafe {
            self.license.as_ref().unwrap_unchecked().title.as_ref().unwrap_unchecked().clone()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTMintResult {
    pub license_token: Option<LicenseToken>,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTUpdateLicenseResult {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTokenLicense {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub license: TokenLicense,
    // proposed license
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryLicense {
    pub license_id: String,
    pub title: String,
    pub price: String,
    pub license: LicenseData,
}

impl LicenseGeneral for InventoryLicense {
    fn is_exclusive(&self) -> bool {
        self.license.exclusivity
    }

    fn is_personal(&self) -> bool {
        self.license.personal_use
    }

    fn is_commercial(&self) -> bool {
        !self.license.personal_use
    }

    fn license_id(&self) -> String {
        self.license_id.clone()
    }

    fn license_title(&self) -> String {
        self.title.clone()
    }
}

impl InventoryLicense {
    pub fn as_license_token(&self, token_id: String) -> LicenseToken {
        let mut token_metadata = TokenMetadata::default();
        token_metadata.title = Some(self.title.clone());
        let res = LicenseToken {
            token_id,
            license: Some(TokenLicense {
                id: self.license_id.clone(),
                title: Some(self.title.clone()),
                metadata: self.license.clone(),
                description: None,
                expires_at: None,
                issued_at: None,
                issuer_id: None,
                starts_at: None,
                updated_at: None,
                uri: None,
                from: SourceLicenseMeta{
                    asset_id: "asset".to_string(),
                    inventory_id: "inv".to_string(),
                }
            }),
            approved_account_ids: Default::default(),
            // royalty: Default::default(),
            owner_id: AccountId::new_unchecked("alice".to_string()),
            asset_id: String::new(),
            metadata: token_metadata,
        };
        res
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetLicense {
    pub license_id: String,
    pub title: String,
    pub price: Option<String>,
    pub objects: Option<Vec<String>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtendedInventoryMetadata {
    pub asset_count: u64,
    pub owner_id: AccountId,

    #[serde(flatten)]
    pub metadata: InventoryContractMetadata,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryContractMetadata {
    pub spec: String,                // required, essentially a version like "nft-1.0.0"
    pub name: String,                // required, ex. "Mosaics"
    pub description: Option<String>, // required, ex. "Mosaics"
    pub symbol: String,              // required, ex. "MOSIAC"
    pub icon: Option<String>,        // Data URL
    pub background_image: Option<String>,
    // pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    // pub reference: Option<String>, // URL to a JSON file with more info
    // pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub licenses: Vec<InventoryLicense>,            // required, ex. "MOSIAC"
    pub default_minter_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
//#[derive(Debug)]
pub struct AssetToken {
    // token_id: String,
    pub token_id: String,
    //owner of the token
    pub owner_id: AccountId,
    //minter of the token
    pub minter_id: AccountId,
    pub license_token_count: u64,
    pub policy_rules: Option<Vec<Limitation>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonAssetToken {
    //token ID
    pub token_id: String,
    //owner of the token
    pub owner_id: AccountId,
    // minter_id
    pub minter_id: AccountId,
    pub license_token_count: u64,
    //token metadata
    pub metadata: TokenMetadata,
    // license metadata
    pub licenses: Option<Vec<AssetLicense>>,
    pub policy_rules: Option<Vec<Limitation>>
    // pub available_licenses: Option<Vec<InventoryLicenseAvailability>>
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryLicenseAvailability {
    pub inventory_license:    InventoryLicense,
    pub available:            bool,
    pub upgrade_price: Option<String>,
    pub reason_not_available: Option<String>,
    pub additional_info: Option<HashMap<String, LimitsInfo>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullInventory {
    pub inventory_licenses: Vec<InventoryLicense>,
    pub issued_licenses:    Vec<LicenseToken>,
}
