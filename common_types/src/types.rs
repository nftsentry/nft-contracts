use near_sdk::env;
use near_sdk::serde_json;
use crate::*;
use crate::policy::{LimitationData, LimitsInfoData, PolicyData};
use crate::prices::Price;
use crate::utils::{get_inventory_id, get_objects};

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
    fn sku_id(&self) -> String;
    fn token_id(&self) -> String;
    fn objects(&self) -> Vec<String>;
    fn object_hash(&self) -> String;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub skip_policies: Option<bool>,
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
#[serde(crate = "near_sdk::serde")]
pub struct ShrinkedTokenMetadata {
    // pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub object: Option<String>, // Object data string
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub from: Option<SourceLicenseMeta>,
}

impl ShrinkedTokenMetadata {
    pub fn get_objects(&self) -> ObjectData {
        return get_objects(self.object.as_ref())
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub previews: Option<String>, // URL/JSON with URLs to image previews
    pub object: Option<String>, // Object data string
    // pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    // pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    // pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    // pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub from: Option<SourceLicenseMeta>,
    pub sku_data: Option<SkuTokenData>,
}

impl TokenMetadata {
    pub fn get_objects(&self) -> ObjectData {
        return get_objects(self.object.as_ref())
    }

    pub fn shrink(&self) -> ShrinkedTokenMetadata {
        ShrinkedTokenMetadata{
            object: self.object.clone(),
            expires_at: self.expires_at.clone(),
            issued_at: self.issued_at.clone(),
            from: self.from.clone(),
            // title: self.title.clone(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ObjectData {
    pub items: Option<Vec<ObjectItem>>,
    // pub sets: Option<Vec<ObjectSet>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ObjectSet {
    id: String,
    objects: Option<Vec<String>>,
    title: Option<String>,
    active: Option<bool>,
    icon: Option<String>,
    description: Option<String>,
}

impl ObjectData {
    // pub fn filter_by_set_id(&self, set_id: String) -> ObjectData {
    //     if self.sets.is_none() {
    //         return ObjectData::default()
    //     }
    //     unsafe {
    //         let object_set = self.sets.clone().unwrap_unchecked().into_iter().find(|x| x.id == set_id);
    //         if object_set.is_none() {
    //             env::log_str(&("Set not found: ".to_string() + &set_id));
    //             return ObjectData::default()
    //         }
    //         let objects = object_set.clone().unwrap_unchecked().objects.unwrap_unchecked();
    //         return self.filter_by_objects(objects, object_set);
    //     }
    // }

    pub fn filter_by_objects(&self, objects: Vec<String>) -> ObjectData {
        if self.items.is_none() {
            return ObjectData{items: None}
        }
        let mut filtered: Vec<ObjectItem> = Vec::new();
        for i in self.items.as_ref().unwrap_or(&Vec::default()) {
            if objects.contains(&i.id) {
                filtered.push(i.clone())
            }
        }
        let new_obj_data: ObjectData = ObjectData{
            items: Some(filtered),
            // sets: None,
        };

        new_obj_data
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ObjectItem {
    pub link: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub active: Option<bool>,
    pub params: Option<HashMap<String, String>>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShrinkedLicenseData {
    pub exclusivity: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub personal_use: Option<bool>,
    pub commercial_use: Option<bool>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LicenseData {
    pub exclusivity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub personal_use: Option<bool>,
    pub commercial_use: Option<bool>,
    pub display_sublicensee: Option<bool>,
    pub hate_speech_termination: Option<bool>,
    pub creative_commons: Option<bool>,
    pub moral_use_restrictions: Option<bool>,
    pub template: Option<String>,
    pub pdf_url: Option<String>,
    pub version: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShrinkedTokenLicense {
    pub id: String,
    pub metadata: ShrinkedLicenseData,
    // pub from: Option<SourceLicenseMeta>,
    // pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    // pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenLicense {
    pub id: String,
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    // pub issuer_id: Option<AccountId>, // AccountId of the license issuer
    pub uri: Option<String>, // URL to associated pdf, preferably to decentralized, content-addressed storage
    pub metadata: LicenseData,
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
}

impl TokenLicense {
    pub fn shrink(&self) -> ShrinkedTokenLicense {
        return ShrinkedTokenLicense{
            // expires_at: self.expires_at.clone(),
            id: self.id.clone(),
            // issued_at: self.issued_at.clone(),
            // from: self.from.clone(),
            metadata: ShrinkedLicenseData{
                commercial_use: self.metadata.commercial_use.clone(),
                personal_use: self.metadata.personal_use.clone(),
                exclusivity: self.metadata.exclusivity.clone().unwrap_or(false),
            }
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SourceLicenseMeta {
    pub inventory_id: String,
    pub sku_id: Option<String>,
    pub issuer_id: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    pub asset_id: AssetId,
    pub license: Option<TokenLicense>,
    pub metadata: TokenMetadata,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    // pub approved_account_ids: HashMap<AccountId, u64>,
    //the next approval ID to give out.
    // pub next_approval_id: u64,
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
pub struct ShrinkedLicenseToken {
    //token ID
    pub token_id: TokenId,
    //asset id of the token
    pub asset_id: AssetId,
    //token metadata
    pub metadata: ShrinkedTokenMetadata,
    // license metadata
    pub license: Option<ShrinkedTokenLicense>,
}

impl ShrinkedLicenseToken {
    pub fn inventory_asset_license_sku(&self) -> (String, String, String, String) {
        if self.license.is_none() && self.metadata.from.is_none() {
            return (String::new(), String::new(), String::new(), String::new())
        }
        unsafe {
            let inv_id: String;
            let asset_id: String;
            // if self.metadata.from.is_none() {
            //     inv_id = self.license.as_ref().unwrap_unchecked().from.inventory_id.clone();
            //     asset_id = self.license.as_ref().unwrap_unchecked().from.asset_id.clone();
            // } else {
            inv_id = self.metadata.from.clone().unwrap_unchecked().inventory_id.clone();
            asset_id = self.asset_id.clone();
            // }
            return (
                inv_id,
                asset_id,
                self.license_id(),
                self.sku_id(),
            )
        }
    }
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
    // pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    // pub royalty: HashMap<AccountId, u32>,
}

impl LicenseToken {
    pub fn inventory_asset_license_sku(&self) -> (String, String, String, String) {
        if self.license.is_none() && self.metadata.from.is_none() {
           return (String::new(), String::new(), String::new(), String::new())
        }
        unsafe {
            let inv_id: String;
            let asset_id: String;
            // if self.metadata.from.is_none() {
            //     inv_id = self.license.as_ref().unwrap_unchecked().from.inventory_id.clone();
            //     asset_id = self.license.as_ref().unwrap_unchecked().from.asset_id.clone();
            // } else {
            inv_id = self.metadata.from.clone().unwrap_unchecked().inventory_id.clone();
            asset_id = self.asset_id.clone();
            // }
            return (
                inv_id,
                asset_id,
                self.license_id(),
                self.sku_id(),
            )
        }
    }

    pub fn shrink(&self) -> ShrinkedLicenseToken {
        return ShrinkedLicenseToken{
            asset_id: self.asset_id.clone(),
            token_id: self.token_id.clone(),
            license: if self.license.is_some() { Some(self.license.as_ref().unwrap().shrink()) } else {None},
            metadata: self.metadata.shrink()
        }
    }
}

impl LicenseGeneral for LicenseToken {
    fn is_exclusive(&self) -> bool {
        if self.license.is_none() {
            return false
        }
        unsafe {
            self.license.as_ref().unwrap_unchecked().metadata.exclusivity.unwrap_or(false)
        }
    }

    fn is_personal(&self) -> bool {
        if self.license.is_none() {
            return true
        }
        unsafe {
            let lic = self.license.as_ref().unwrap_unchecked();
            if lic.metadata.personal_use.clone().is_some() {
                return lic.metadata.personal_use.clone().unwrap()
            }
            if lic.metadata.commercial_use.clone().is_some() {
                !lic.metadata.commercial_use.clone().unwrap()
            } else {
                // Imply personal_use == false
                return false
            }
        }
    }

    fn is_commercial(&self) -> bool {
        if self.license.is_none() {
            return false
        }
        unsafe {
            let lic = self.license.as_ref().unwrap_unchecked();
            if lic.metadata.personal_use.clone().is_some() {
                return !lic.metadata.personal_use.clone().unwrap()
            }
            if lic.metadata.commercial_use.clone().is_some() {
                lic.metadata.commercial_use.clone().unwrap()
            } else {
                // Imply personal_use == false
                return true
            }
        }
    }

    fn license_id(&self) -> String {
        if self.license.is_none() {
            return String::new()
        }
        unsafe {
            self.license.as_ref().unwrap_unchecked().id.clone()
        }
    }

    fn license_title(&self) -> String {
        unsafe {
            if self.metadata.from.is_none() {
                return self.license.as_ref().unwrap_unchecked().title.as_ref().unwrap_unchecked().clone()
            } else {
                return self.metadata.title.clone().unwrap_or(String::new())
            }
        }
    }

    fn sku_id(&self) -> String {
        self.metadata.from.as_ref().unwrap().sku_id.clone().unwrap_or(String::new())
    }

    fn token_id(&self) -> String {
        self.token_id.clone()
    }

    fn objects(&self) -> Vec<String> {
        let obj_data = self.metadata.get_objects();
        let ids = obj_data.items.unwrap_or(Vec::new()).iter().map(|x| x.id.clone()).collect();
        return ids
    }

    fn object_hash(&self) -> String {
        let mut objects = self.objects();
        objects.sort();
        return objects.join(",")
    }
}

impl LicenseGeneral for ShrinkedLicenseToken {
    fn is_exclusive(&self) -> bool {
        if self.license.is_none() {
            return false
        }
        unsafe {
            self.license.as_ref().unwrap_unchecked().metadata.exclusivity
        }
    }

    fn is_personal(&self) -> bool {
        if self.license.is_none() {
            return true
        }
        unsafe {
            let lic = self.license.as_ref().unwrap_unchecked();
            if lic.metadata.personal_use.clone().is_some() {
                return lic.metadata.personal_use.clone().unwrap()
            }
            if lic.metadata.commercial_use.clone().is_some() {
                !lic.metadata.commercial_use.clone().unwrap()
            } else {
                // Imply personal_use == false
                return false
            }
        }
    }

    fn is_commercial(&self) -> bool {
        if self.license.is_none() {
            return false
        }
        unsafe {
            let lic = self.license.as_ref().unwrap_unchecked();
            if lic.metadata.personal_use.clone().is_some() {
                return !lic.metadata.personal_use.clone().unwrap()
            }
            if lic.metadata.commercial_use.clone().is_some() {
                lic.metadata.commercial_use.clone().unwrap()
            } else {
                // Imply personal_use == false
                return true
            }
        }
    }

    fn license_id(&self) -> String {
        if self.license.is_none() {
            return String::new()
        }
        unsafe {
            self.license.as_ref().unwrap_unchecked().id.clone()
        }
    }

    fn license_title(&self) -> String {
        return String::new()
    }

    fn sku_id(&self) -> String {
        self.metadata.from.as_ref().unwrap().sku_id.clone().unwrap_or(String::new())
    }

    fn token_id(&self) -> String {
        self.token_id.clone()
    }

    fn objects(&self) -> Vec<String> {
        let obj_data = self.metadata.get_objects();
        let ids = obj_data.items.unwrap_or(Vec::new()).iter().map(|x| x.id.clone()).collect();
        return ids
    }

    fn object_hash(&self) -> String {
        let mut objects = self.objects();
        objects.sort();
        return objects.join(",")
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
    pub price: Option<String>,
    pub license: LicenseData,
}

impl LicenseGeneral for InventoryLicense {
    fn is_exclusive(&self) -> bool {
        self.license.exclusivity.unwrap_or(false)
    }

    fn is_personal(&self) -> bool {
        return if self.license.personal_use.is_some() {
            self.license.personal_use.clone().unwrap()
        } else {
            !self.license.commercial_use.clone().unwrap_or_default()
        }
    }

    fn is_commercial(&self) -> bool {
        if self.license.personal_use.clone().is_some() {
            return !self.license.personal_use.clone().unwrap()
        }
        return self.license.commercial_use.clone().unwrap_or_default()
    }

    fn license_id(&self) -> String {
        self.license_id.clone()
    }

    fn license_title(&self) -> String {
        self.title.clone()
    }

    fn sku_id(&self) -> String {
        String::new()
    }
    fn token_id(&self) -> String {
        String::new()
    }
    fn objects(&self) -> Vec<String> {
        Vec::new()
    }
    fn object_hash(&self) -> String {
        String::new()
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
                starts_at: None,
                updated_at: None,
                uri: None,
            }),
            // approved_account_ids: Default::default(),
            // royalty: Default::default(),
            owner_id: AccountId::new_unchecked("alice".to_string()),
            asset_id: String::new(),
            metadata: token_metadata,
        };
        res
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SkuTokenData {
    pub title: String,
    pub params: Option<String> // Json-serialized AssetLicenseParams
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetLicense {
    pub sku_id: Option<String>,
    pub license_id: Option<String>,
    pub title: String,
    pub price: String,
    pub currency: Option<String>,
    pub active: Option<bool>,
    pub hidden: Option<bool>,
    pub limited_edition: Option<bool>,
    pub sole_limit: Option<i32>,
    pub objects: Option<Vec<String>>,
    pub params: Option<String> // Json-serialized AssetLicenseParams
}

pub const NEAR_CURRENCY: &str = "NEAR";

impl AssetLicense {
    pub fn get_near_cost(&self, near_usd_price: &Price) -> String {
        let mut currency = NEAR_CURRENCY.to_string();
        if let Some(new_currency) = &self.currency {
            currency = new_currency.to_string();
        }

        if currency != NEAR_CURRENCY.to_string() {
            // near cost = usd_price / near_price
            let near_cost: f64 = self.price.clone().parse::<f64>().unwrap() / near_usd_price.float();
            return format!("{:.6}", near_cost);
        }

        return self.price.clone()
    }

    pub fn get_params(&self) -> AssetLicenseParams {
        let res: AssetLicenseParams = serde_json::from_str(
            &self.params.clone().unwrap_or("{}".to_string())).unwrap_or_default();
        res
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetLicenseParams {
    pub icon: Option<String>,
    pub description: Option<String>,
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
    pub licenses: Option<Vec<AssetLicense>>,
    pub policy_rules: Option<Vec<LimitationData>>,
    pub upgrade_rules: Option<Vec<PolicyData>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
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
    pub policy_rules: Option<Vec<LimitationData>>,
    pub upgrade_rules: Option<Vec<PolicyData>>,
    // pub available_licenses: Option<Vec<InventoryLicenseAvailability>>
}

impl Default for JsonAssetToken {
    fn default() -> Self {
        JsonAssetToken{
            upgrade_rules: None,
            policy_rules: None,
            licenses: None,
            minter_id: AccountId::new_unchecked("alice".to_string()),
            owner_id: AccountId::new_unchecked("alice".to_string()),
            metadata: TokenMetadata::default(),
            token_id: String::new(),
            license_token_count: 0,
        }
    }
}

impl JsonAssetToken {
    pub fn issue_new_metadata(&self, sku_info: AssetLicense) -> TokenMetadata {
        let mut metadata = self.metadata.clone();
        metadata.issued_at = Some(env::block_timestamp_ms());
        metadata.updated_at = Some(env::block_timestamp_ms());
        metadata.starts_at = Some(env::block_timestamp_ms());
        metadata.sku_data = Some(SkuTokenData{
            title: sku_info.title.clone(),
            params: sku_info.params.clone(),
        });

        let from = SourceLicenseMeta{
            inventory_id: get_inventory_id(self.minter_id.clone().to_string()),
            sku_id: sku_info.sku_id.clone(),
            issuer_id: Some(self.minter_id.to_string()),
        };
        metadata.from = Some(from);

        if self.metadata.object.is_none() {
            return metadata
        }
        // Set metadata title to sku title
        metadata.title = if sku_info.title.is_empty() { metadata.title } else { Some(sku_info.title.clone()) };
        // Set metadata preview to sku icon
        let params = sku_info.get_params();
        if let Some(icon) = params.icon {
            if !icon.is_empty() {
                metadata.media = Some(icon);
            }
        }
        if let Some(desc) = params.description {
            if !desc.is_empty() {
                metadata.description = Some(desc);
            }
        }

        let sku_id = sku_info.sku_id.unwrap();
        unsafe {
            if self.metadata.object.as_ref().unwrap_unchecked().is_empty() {
                return metadata
            }
            let obj_data = self.metadata.get_objects();
            let obj_ids = self.licenses.as_ref().unwrap().iter().find(
                |&x| x.sku_id.as_ref().unwrap_or(&String::default()) == &sku_id
            ).expect("Not found by sku_id").objects.clone().unwrap();
            // let new_obj_data = obj_data.filter_by_set_id(set_id);
            let new_obj_data = obj_data.filter_by_objects(obj_ids);
            metadata.object = Some(serde_json::to_string(&new_obj_data).expect("Failed to serialize"));
            return metadata
        }
    }

    pub fn issue_new_license(&self, inv_license: Option<InventoryLicense>, sku_info: AssetLicense, token_id: String) -> LicenseToken {
        let metadata = self.issue_new_metadata(sku_info.clone());
        let license: Option<TokenLicense>;

        if let Some(inv_license) = inv_license {
            license = Some(TokenLicense{
                id: inv_license.license_id,
                metadata: inv_license.license.clone(),
                title: Some(inv_license.title),
                description: None,
                uri: inv_license.license.pdf_url,
                issued_at: Some(env::block_timestamp_ms()),
                starts_at: Some(env::block_timestamp_ms()),
                updated_at: Some(env::block_timestamp_ms()),
                expires_at: None,
            });
        } else {
            license = None
        }
        LicenseToken{
            asset_id: self.token_id.clone(),
            token_id: token_id.clone(),
            license,
            owner_id: AccountId::new_unchecked("alice".to_string()),
            metadata,
            // approved_account_ids: Default::default(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SKUAvailability {
    #[serde(flatten)]
    pub asset_license:    AssetLicense,
    pub available:            bool,
    pub upgrade_price: Option<String>,
    pub reason_not_available: Option<String>,
    pub additional_info: Option<HashMap<String, LimitsInfoData>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullInventory {
    pub inventory_licenses: Vec<InventoryLicense>,
    pub issued_licenses:    Vec<ShrinkedLicenseToken>,
    pub asset: Option<JsonAssetToken>,
}
