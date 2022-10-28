use crate::*;

#[near_bindgen]
impl InventoryContract {
    pub fn on_nft_mint(&mut self, token_id: String, token_count: u64) -> Option<String> {
        let token_opt = self.tokens_by_id.get(&token_id);
        if token_opt.is_none() {
            return Some("Token does not exist".to_string())
        }
        let token = unsafe {token_opt.unwrap_unchecked()};
        if env::predecessor_account_id() != token.minter_id {
            return Some("Forbidden: call must be from the license contract".to_string())
        }

        self._on_nft_mint(token.clone(), token_count);

        None
    }

    #[private]
    pub fn _on_nft_mint(&mut self, token: AssetToken, token_count: u64) {
        let mut copy = token.clone();
        copy.license_token_count = token_count;

        self.tokens_by_id.remove(&copy.token_id);
        self.tokens_by_id.insert(&copy.token_id, &copy);
    }
}