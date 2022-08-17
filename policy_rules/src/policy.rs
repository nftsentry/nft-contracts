use crate::*;
use crate::types::*;
use std::collections::HashMap;
use std::ops::Deref;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

pub trait ConfigInterface {
    fn check_transition(old: InventoryLicense, new: InventoryLicense) -> (bool, String);
    fn check_new(inventory: InventoryContractMetadata, new: InventoryLicense, opt: CheckNewOpt) -> (bool, String);
    fn check_state(inventory: InventoryContractMetadata, opt: CheckNewOpt) -> (bool, String);
    fn check_inventory_state(licenses: Vec<InventoryLicense>) -> (bool, String);
    fn list_transitions(inventory: InventoryContractMetadata, from: InventoryLicense) -> Vec<InventoryLicenseAvailability>;
    fn list_available(inventory: FullInventory) -> Vec<InventoryLicenseAvailability>;
}

lazy_static! {
    static ref POLICIES_RAW: Mutex<Vec<u8>> = Mutex::new(include_bytes!("rules.yaml").to_vec());
    pub static ref CONFIG: Mutex<AllPolicies> = Mutex::new(AllPolicies::default());
}

pub fn init_policies() -> AllPolicies {
    let raw = POLICIES_RAW.lock().unwrap();
    let config: AllPolicies = serde_yaml::from_slice(raw.as_slice()).expect("Fail to parse rules.yaml");
    CONFIG.lock().unwrap().limitations = config.limitations;
    CONFIG.lock().unwrap().policies = config.policies;
    CONFIG.lock().unwrap().version = config.version;

    CONFIG.lock().unwrap().clone()
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct AllPolicies {
    pub version:     String,
    pub policies:    HashMap<String, Policy>,
    pub limitations: Vec<Limitation>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub name:       Option<String>,
    pub template:   String,
    pub upgrade_to: Vec<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct  CheckNewOpt {
    in_assets:   Option<bool>,
    in_licenses: Option<bool>,
    token_id:    Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FutureStateOpt {
    pub level: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Limitation {
    pub name:      String,
    pub level:     String,
    pub template:  String,
    pub max_count: Option<MaxCount>,
    pub exclusive: Option<Exclusive>,
    // Add another limit types
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MaxCount {
    pub count: i32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Exclusive {
    pub template: String,
}