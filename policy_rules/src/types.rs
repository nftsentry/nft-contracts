use near_sdk::env;
use crate::*;
use crate::policy::{Limitation, LimitsInfo, Policy};
use crate::prices::Price;
use crate::utils::{get_inventory_id};

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
    fn set_id(&self) -> String;
    fn sku_id(&self) -> String;
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
    pub from: Option<SourceLicenseMeta>,
    pub sku_data: Option<SkuTokenData>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ObjectData {
    items: Option<Vec<ObjectItem>>,
    sets: Option<Vec<ObjectSet>>,
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
    pub fn filter_by_set_id(&self, set_id: String) -> ObjectData {
        if self.sets.is_none() {
            return ObjectData::default()
        }
        unsafe {
            let object_set = self.sets.clone().unwrap_unchecked().into_iter().find(|x| x.id == set_id);
            if object_set.is_none() {
                env::log_str(&("Set not found: ".to_string() + &set_id));
                return ObjectData::default()
            }
            let objects = object_set.clone().unwrap_unchecked().objects.unwrap_unchecked();
            return self.filter_by_objects(objects, object_set);
        }
    }

    pub fn filter_by_objects(&self, objects: Vec<String>, _set: Option<ObjectSet>) -> ObjectData {
        let filtered: Vec<ObjectItem> = self.items.clone().unwrap().into_iter().filter(|x| objects.contains(&x.id)).collect();
        // let ids: Vec<String> = filtered.iter().map(|x| x.id.clone()).collect();

        // let new_set = if set.is_none() {
        //     ObjectSet{
        //         icon: None,
        //         objects: Some(ids.clone()),
        //         id: ids[0].clone() + &"_set".to_string(),
        //         title: None,
        //         description: None,
        //         active: Some(true),
        //     }
        // } else {
        //     unsafe { set.unwrap_unchecked().clone() }
        // };
        let new_obj_data: ObjectData = ObjectData{
            items: Some(filtered),
            // sets: Some(vec![new_set]),
            sets: None,
        };

        new_obj_data
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ObjectItem {
    link: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    id: String,
    title: Option<String>,
    icon: Option<String>,
    active: Option<bool>,
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
    pub set_id: String,
    pub sku_id: Option<String>,
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
    pub fn inventory_asset_license_sku(&self) -> (String, String, String, String) {
        if self.license.is_none() && self.metadata.from.is_none() {
           return (String::new(), String::new(), String::new(), String::new())
        }
        unsafe {
            let inv_id: String;
            let asset_id: String;
            if self.metadata.from.is_none() {
                inv_id = self.license.as_ref().unwrap_unchecked().from.inventory_id.clone();
                asset_id = self.license.as_ref().unwrap_unchecked().from.asset_id.clone();
            } else {
                inv_id = self.metadata.from.clone().unwrap_unchecked().inventory_id.clone();
                asset_id = self.metadata.from.clone().unwrap_unchecked().asset_id.clone();
            }
            return (
                inv_id,
                asset_id,
                self.license_id(),
                self.sku_id(),
            )
        }
    }

    pub fn as_inventory_license(&self, price: Option<String>) -> Option<InventoryLicense> {
        if self.license.is_none() {
            return None
        }
        unsafe {
            let license_data = &self.license.as_ref().unwrap_unchecked().metadata;
            let (_, _, lic_id, _) = self.inventory_asset_license_sku();
            let res = InventoryLicense {
                license_id: lic_id,
                title: self.license.as_ref().unwrap_unchecked().title.as_ref().unwrap().to_string(),
                license: license_data.clone(),
                price: price.clone(),
            };
            Some(res)
        }
    }
}

impl LicenseGeneral for LicenseToken {
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
            self.license.as_ref().unwrap_unchecked().metadata.personal_use
        }
    }

    fn is_commercial(&self) -> bool {
        if self.license.is_none() {
            return false
        }
        unsafe {
            !self.license.as_ref().unwrap_unchecked().metadata.personal_use
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

    fn set_id(&self) -> String {
        unsafe {
            if self.metadata.from.is_none() {
                self.license.as_ref().unwrap_unchecked().from.set_id.clone()
            } else {
                self.metadata.from.as_ref().unwrap_unchecked().set_id.clone()
            }
        }
    }

    fn sku_id(&self) -> String {
        unsafe {
            if self.metadata.from.is_none() {
                self.license.as_ref().unwrap_unchecked().from.sku_id.clone().unwrap_or(String::new())
            } else {
                self.metadata.from.as_ref().unwrap_unchecked().sku_id.clone().unwrap_or(String::new())
            }
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
    pub price: Option<String>,
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

    fn set_id(&self) -> String {
        String::new()
    }
    fn sku_id(&self) -> String {
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
                issuer_id: None,
                starts_at: None,
                updated_at: None,
                uri: None,
                from: SourceLicenseMeta{
                    asset_id: "asset".to_string(),
                    inventory_id: "inv".to_string(),
                    set_id: "set_id".to_string(),
                    sku_id: Some("sku_id".to_string()),
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SkuTokenData {
    pub sku_id: Option<String>,
    pub title: String,
    pub params: Option<String> // Json-serialized AssetLicenseParams
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetLicense {
    pub sku_id: Option<String>,
    pub license_id: Option<String>,
    pub title: String,
    pub price: Option<String>,
    pub currency: Option<String>,
    pub active: Option<bool>,
    pub sole_limit: Option<i32>,
    pub set_id: Option<String>,
    pub objects: Option<Vec<String>>,
    pub params: Option<String> // Json-serialized AssetLicenseParams
}

pub const NEAR_CURRENCY: &str = "NEAR";

impl AssetLicense {
    pub fn get_near_cost(&self, near_usd_price: Price) -> String {
        let mut currency = NEAR_CURRENCY.to_string();
        if let Some(new_currency) = self.currency.clone() {
            currency = new_currency;
        }

        if currency != NEAR_CURRENCY.to_string() {
            // near cost = usd_price / near_price
            let near_cost: f64 = self.price.clone().unwrap().parse::<f64>().unwrap() / near_usd_price.float();
            return format!("{:.6}", near_cost);
        }

        return self.price.clone().unwrap()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
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
    pub policy_rules: Option<Vec<Limitation>>,
    pub upgrade_rules: Option<Vec<Policy>>,
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
    pub policy_rules: Option<Vec<Limitation>>,
    pub upgrade_rules: Option<Vec<Policy>>,
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
            sku_id: sku_info.sku_id.clone(),
            title: sku_info.title.clone(),
            params: sku_info.params.clone(),
        });

        let from = SourceLicenseMeta{
            inventory_id: get_inventory_id(self.minter_id.clone().to_string()),
            sku_id: sku_info.sku_id.clone(),
            set_id: String::new(),
            asset_id: self.token_id.clone(),
        };
        metadata.from = Some(from);

        if self.metadata.object.is_none() {
            return metadata
        }

        let sku_id = sku_info.sku_id.clone().unwrap();
        unsafe {
            if self.metadata.object.as_ref().unwrap_unchecked().is_empty() {
                return metadata
            }
            let obj_data: ObjectData = serde_json::from_str(
                &self.metadata.object.clone().unwrap_unchecked()
            ).expect("Failed parse asset object data");
            let obj_ids = self.licenses.as_ref().unwrap().iter().find(
                |&x| x.sku_id.clone().unwrap_or(String::new()) == sku_id.clone()
            ).expect("Not found by sku_id").objects.clone().unwrap();
            // let new_obj_data = obj_data.filter_by_set_id(set_id);
            let new_obj_data = obj_data.filter_by_objects(obj_ids, None);
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
                from: metadata.from.clone().unwrap(),
                metadata: inv_license.license.clone(),
                title: Some(inv_license.title),
                description: None,
                uri: inv_license.license.pdf_url,
                issued_at: Some(env::block_timestamp_ms()),
                starts_at: Some(env::block_timestamp_ms()),
                updated_at: Some(env::block_timestamp_ms()),
                expires_at: None,
                issuer_id: Some(self.minter_id.clone()),
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
            approved_account_ids: Default::default(),
        }
    }

    pub fn migrate_to_sets(&mut self) {
        if self.licenses.is_none() {
            return
        }
        unsafe {
            if self.metadata.object.is_none() || self.metadata.object.as_ref().unwrap_unchecked().is_empty() {
                // Insert a default object
                let obj_data: ObjectData = ObjectData{
                    items: Some(vec![ObjectItem{
                        title: self.metadata.title.clone(),
                        link: self.metadata.media.clone(),
                        type_: "image".to_string(),
                        id: "default_object".to_string(),
                        icon: None,
                        active: Some(true),
                    }]),
                    // sets: Some(vec![ObjectSet{
                    //     id: "default_object_set".to_string(),
                    //     objects: Some(vec!["default_object".to_string()]),
                    //     title: self.metadata.title.clone(),
                    //     active: Some(true),
                    //     icon: None,
                    //     description: None,
                    // }]),
                    sets: None,
                };
                for (i, lic) in self.licenses.as_mut().unwrap_unchecked().iter_mut().enumerate() {
                    lic.sku_id = Some(format!("{}-{}-{}", env::block_timestamp(), self.token_id, i));
                    lic.active = Some(true)
                }
                let obj_data_raw = serde_json::to_string(&obj_data).expect("failed serialize obj data");
                // migrated to sets
                self.metadata.object = Some(obj_data_raw);
                return
            }
            // let mut obj_data: ObjectData = serde_json::from_str(&self.metadata.object.clone().unwrap_unchecked()).expect("Failed parse asset object data");
            // let mut obj_map: HashMap<Vec<String>, String> = HashMap::new();
            for (i, lic) in self.licenses.as_mut().unwrap_unchecked().iter_mut().enumerate() {
                if lic.sku_id.is_some() && !lic.sku_id.clone().unwrap().is_empty() {
                    continue
                }
                lic.sku_id = Some(format!("{}-{}-{}", env::block_timestamp(), self.token_id, i));
                lic.active = Some(true)
                // let old_set_id = obj_map.get(&lic.objects.clone().unwrap_unchecked());
                // if old_set_id.is_none() {
                //     let lic_objects = lic.objects.as_ref().unwrap_unchecked();
                //     let set_id = lic_objects.first().unwrap_or(&lic.license_id).to_owned() + "_set";
                //     obj_map.insert(lic.objects.clone().unwrap_unchecked(), set_id.clone());
                //     lic.set_id = Some(set_id.clone());
                // } else {
                //     // Re-insert existing set id
                //     lic.set_id = Some(old_set_id.unwrap_unchecked().clone());
                // }
            }

            // Add mapped objects -> set_id in object data
            // let mut obj_sets: Vec<ObjectSet> = Vec::new();
            // for (objects, set_id) in obj_map.iter() {
            //     let obj_set = ObjectSet{
            //         objects: Some(objects.clone()),
            //         id: set_id.clone(),
            //         icon: None,
            //         title: None,
            //         description: None,
            //         active: Some(true),
            //     };
            //     obj_sets.push(obj_set);
            // }
            // if obj_data.sets.is_none() {
            //     obj_data.sets = Some(obj_sets);
            // } else {
            //     obj_data.sets.as_mut().unwrap_unchecked().extend(obj_sets);
            // }
            // let obj_data_raw = serde_json::to_string(&obj_data).expect("failed serialize obj data");
            // migrated to sets
            // self.metadata.object = Some(obj_data_raw);
        }
    }
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
    pub asset: Option<JsonAssetToken>,
}
