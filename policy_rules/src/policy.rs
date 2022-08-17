use crate::*;
use crate::types::*;
use std::collections::HashMap;
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
    static ref policies_raw: Mutex<Vec<u8>> = Mutex::new(vec![]);
    static ref CONFIG: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AllPolicies {
    version:     String,
    policies:    HashMap<String, Policy>,
    limitations: Vec<Limitation>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    name:      String,
    template:  String,
    upgrade_to: Vec<String>,
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
    level: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Limitation {
    name:      String,
    level:     String,
    template:  String,
    max_count: Option<MaxCount>,
    exclusive: Option<Exclusive>,
    // Add another limit types
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MaxCount {
    count: i32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Exclusive {
    template: String,
}