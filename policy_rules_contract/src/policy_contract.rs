use near_sdk::{near_bindgen};
use crate::*;
use policy_rules::policy::{AllPolicies, ConfigInterface, IsAvailableResponse, Limitation};
use policy_rules::types::{FullInventory, InventoryLicense, SKUAvailability, LicenseToken};

#[near_bindgen]
impl ConfigInterface for Contract {
    #[handle_result]
    fn check_transition(&self, inventory: FullInventory, old: LicenseToken, new: LicenseToken) -> Result<IsAvailableResponse, String> {
        self.policies.check_transition(inventory, old, new)
    }

    fn check_new(&self, inventory: FullInventory, new: LicenseToken) -> IsAvailableResponse {
        self.policies.check_new(inventory, new)
    }

    fn check_state(&self, licenses: Vec<LicenseToken>) -> IsAvailableResponse {
        self.policies.check_state(licenses)
    }

    fn check_inventory_state(&self, licenses: Vec<InventoryLicense>) -> IsAvailableResponse {
        self.policies.check_inventory_state(licenses)
    }

    fn list_transitions(&self, inventory: FullInventory, from: LicenseToken) -> Vec<SKUAvailability> {
        self.policies.list_transitions(inventory, from)
    }

    fn list_available(&self, inventory: FullInventory) -> Vec<SKUAvailability> {
        self.policies.list_available(inventory)
    }

    fn clone_with_additional(&self, l: Vec<Limitation>) -> AllPolicies {
        self.policies.clone_with_additional(l)
    }
}