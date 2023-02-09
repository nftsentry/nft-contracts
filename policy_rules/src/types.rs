use crate::*;
use crate::policy::{LimitsInfo};
use common_types::types::{AssetLicense};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SKUAvailability {
    #[serde(flatten)]
    pub asset_license:    AssetLicense,
    pub available:            bool,
    pub upgrade_price: Option<String>,
    pub reason_not_available: Option<String>,
    pub additional_info: Option<HashMap<String, LimitsInfo>>,
}