use crate::*;

#[near_bindgen]
impl InventoryContract {

    pub fn asset_add_licenses(
        &mut self, 
        token_id: AssetTokenId, 
        license: Option<AssetLicenses>,
    ) -> Option<AssetLicenses> {
        None
    }

    pub fn asset_update_licenses(
        &mut self,
        token_id: AssetTokenId,
        licenses: AssetLicenses,
    ) -> AssetLicenses {
        let _old_licenses = self.token_licenses_by_id.get(&token_id);
        self.token_licenses_by_id.remove(&token_id);

        let new_licenses = self.token_licenses_by_id.insert(&token_id, &licenses);
        new_licenses.unwrap()
    }

    pub fn asset_remove_license(
        &mut self, 
        token_id: AssetTokenId, 
        license_id: String,
    ) -> bool {
        false
    }

    pub fn asset_find_license(
        &mut self, 
        token_id: AssetTokenId, 
        license_id: String,
    ) -> Option<AssetLicense> {
        None
    }        
    
    pub fn asset_add_minter(
        &mut self, 
        token_id: AssetTokenId, 
        minter_id: AssetMinterContractId,
    ) -> Option<AssetLicense> {
        None
    } 
    
    pub fn asset_get_minter(
        &mut self, 
        token_id: AssetTokenId, 
    ) -> Option<AssetMinterContractId> {
        None
    }
}