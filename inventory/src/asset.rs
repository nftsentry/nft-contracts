use policy_rules::utils::refund_deposit;
use crate::*;

#[near_bindgen]
impl InventoryContract {
    #[payable]
    pub fn asset_update_licenses(
        &mut self,
        token_id: String,
        licenses: AssetLicenses,
    ) -> AssetLicenses {
        let initial_storage_usage = env::storage_usage();

        let _old_licenses = self.token_licenses_by_id.get(&token_id);
        self.token_licenses_by_id.remove(&token_id);

        self.token_licenses_by_id.insert(&token_id, &licenses);

        let new_storage_usage = env::storage_usage();
        if new_storage_usage > initial_storage_usage {
            let storage_usage_diff = new_storage_usage - initial_storage_usage;
            let log_message = format!("Storage usage increased by {} bytes", storage_usage_diff);
            env::log_str(&log_message);
            let _ = refund_deposit(storage_usage_diff, None, None);
        } else {
            let _ = refund_deposit(0, None, None);
        }

        licenses
    }
}