use std::collections::HashMap;
use crate::*;
use near_sdk::{CryptoHash, PromiseError};
use policy_rules::policy::{ConfigInterface};
use policy_rules::types::FullInventory;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

#[near_bindgen]
impl InventoryContract {
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &String,
    ) {
        //get the set of tokens for the given account
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::AssetPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        tokens_set.insert(token_id);

        //we insert that set for the given account ID. 
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }
/*  Function is not used in this project yet 
    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &AssetTokenId,
    ) {
        //we get the set of tokens that the owner has
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("Token should be owned by the sender");

        //we remove the the token_id from the set of tokens
        tokens_set.remove(token_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            // if the token set is not empty, we simply insert it back for the account ID.
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }
*/
    pub fn full_inventory_for_asset_callback(
        &self,
        #[callback_result] call_result: Result<Vec<LicenseToken>, PromiseError>,
        asset: &mut JsonAssetToken) -> JsonAssetToken {

        if call_result.is_err() {
            env::panic_str("Failed call previous nft_tokens!");
        }
        let mut inv_metadata = self.inventory_metadata();
        let tokens = call_result.unwrap();

        let mut asset_lic_map: HashMap<String, &AssetLicense> = HashMap::new();
        if asset.licenses.is_some() {
            for asset_license in asset.licenses.as_ref().unwrap() {
                asset_lic_map.insert(asset_license.license_id.clone(), &asset_license);
            }
        }
        let mut full_inventory = FullInventory{
            issued_licenses: tokens,
            inventory_licenses: Vec::new(),
        };
        for lic in inv_metadata.licenses.iter_mut() {
            if asset_lic_map.contains_key(&lic.license_id.clone()) {
                let asset_license = asset_lic_map.get(&lic.license_id);
                if asset_license.unwrap().price.is_some() {
                    lic.price = asset_license.unwrap().price.as_ref().unwrap().clone();
                }
                full_inventory.inventory_licenses.push(lic.clone());
            }
        }

        let available = self.policies.list_available(full_inventory);
        asset.available_licenses = Some(available);
        asset.clone()
    }

    pub fn get_available_list_for_asset(&self, asset: &JsonAssetToken) -> Promise {
        // Now call asset.minter_id.nft_tokens(asset_id=asset.token_id)
        let filter = FilterOpt{account_id: None, asset_id: Some(asset.token_id.clone())};
        let mut asset_mut = asset.clone();
        let promise: Promise = license_contract::ext(asset.minter_id.clone()).nft_tokens(
            None, None, Some(filter)
        ).then(
            Self::ext(env::current_account_id()).full_inventory_for_asset_callback(
                &mut asset_mut,
            )
        );

        promise.as_return()
    }
}