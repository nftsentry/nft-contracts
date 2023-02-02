use common_types::utils::assert_one_yocto;
use crate::*;

#[near_bindgen]
impl InventoryContract {
    #[payable]
    pub fn asset_update_licenses(
        &mut self,
        token_id: String,
        licenses: Vec<AssetLicense>,
    ) -> Vec<AssetLicense> {
        let initial_storage_usage = env::storage_usage();

        self.ensure_owner();

        let _old_licenses = self.token_licenses_by_id.get(&token_id);
        self.token_licenses_by_id.remove(&token_id);

        self.token_licenses_by_id.insert(&token_id, &licenses);

        let _ = refund_storage(initial_storage_usage, None, None);

        licenses
    }

    #[payable]
    pub fn asset_delete(&mut self, token_id: String) {
        // let initial_storage_usage = env::storage_usage();

        assert_one_yocto();

        self.ensure_owner();
        let meta = self.token_metadata_by_id.get(&token_id);
        if meta.is_none() {
            env::panic_str("Token does not exist")
        }

        self.token_metadata_by_id.remove(&token_id);
        self.tokens_by_id.remove(&token_id);
        self.token_licenses_by_id.remove(&token_id);

        // let current_storage_usage = env::storage_usage();
        // TODO: need refund?
    }

    #[payable]
    pub fn asset_update_metadata(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
    ) {
        let initial_storage_usage = env::storage_usage();

        self.ensure_owner();

        let old_meta = self.token_metadata_by_id.get(&token_id);
        if old_meta.is_none() {
            env::panic_str("Token does not exist")
        }
        let old_asset = self.tokens_by_id.get(&token_id);
        if old_asset.unwrap().license_token_count > 0 {
            env::panic_str("Token already has issued licenses, impossible to change metadata")
        }
        self.token_metadata_by_id.remove(&token_id);
        self.token_metadata_by_id.insert(&token_id, &metadata);

        let _ = refund_storage(initial_storage_usage, None, None);
    }
}