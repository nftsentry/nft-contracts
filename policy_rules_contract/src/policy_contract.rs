use near_sdk::{near_bindgen};
use crate::*;
use policy_rules::policy::{ConfigInterface, IsAvailableResponse, Limitation, Policy};
use common_types::types::{FullInventory, InventoryLicense, ShrinkedLicenseToken};
use policy_rules::types::{SKUAvailability};

#[near_bindgen]
impl ConfigInterface for Contract {
    #[handle_result]
    fn check_transition(
        &self, inventory: FullInventory, old: ShrinkedLicenseToken,
        new: ShrinkedLicenseToken, policy_rules: Option<Vec<Limitation>>, upgrade_rules: Option<Vec<Policy>>) -> Result<IsAvailableResponse, String> {
        self.policies.check_transition(inventory, old, new, policy_rules, upgrade_rules)
    }

    fn check_new(
        &self, inventory: FullInventory, new: ShrinkedLicenseToken,
        policy_rules: Option<Vec<Limitation>>, upgrade_rules: Option<Vec<Policy>>) -> IsAvailableResponse {
        self.policies.check_new(inventory, new, policy_rules, upgrade_rules)
    }

    fn check_state(&self, licenses: Vec<ShrinkedLicenseToken>) -> IsAvailableResponse {
        self.policies.check_state(licenses)
    }

    fn check_inventory_state(&self, licenses: Vec<InventoryLicense>) -> IsAvailableResponse {
        self.policies.check_inventory_state(licenses)
    }

    fn list_transitions(
        &self, inventory: FullInventory, from: ShrinkedLicenseToken,
        policy_rules: Option<Vec<Limitation>>, upgrade_rules: Option<Vec<Policy>>) -> Vec<SKUAvailability> {
        self.policies.list_transitions(inventory, from, policy_rules, upgrade_rules)
    }

    fn list_available(
        &self, inventory: FullInventory, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> Vec<SKUAvailability> {
        self.policies.list_available(inventory, policy_rules, upgrade_rules)
    }
}