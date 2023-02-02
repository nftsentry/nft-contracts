use std::collections::HashMap;

use crate::*;
use crate::types::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AllPolicies {
    pub version: String,
    pub policies: HashMap<String, PolicyData>,
    pub limitations: Vec<LimitationData>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PolicyData {
    pub name: Option<String>,
    pub template: String,
    pub upgrade_to: Vec<String>,
    pub user_defined: Option<bool>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct IsAvailableResponseData {
    pub result: bool,
    pub reason_not_available: String,
    pub additional_info: Option<HashMap<String, LimitsInfoData>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LimitsInfoData {
    #[serde(rename = "type")]
    pub type_: String,
    pub remains: i32,
    pub total: i32,
    pub issued: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Context {
    pub full: FullInventory,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LimitationData {
    pub name: String,
    pub level: String,
    pub template: String,
    pub max_count: Option<MaxCountData>,
    pub exclusive: Option<ExclusiveData>,
    // Add another limit types
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MaxCountData {
    pub count: i32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExclusiveData {}
