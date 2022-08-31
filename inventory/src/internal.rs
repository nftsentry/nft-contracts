use crate::*;
use near_sdk::{CryptoHash};

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
}