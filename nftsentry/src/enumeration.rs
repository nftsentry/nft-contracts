use std::str::FromStr;
use crate::*;

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.tokens_by_id.len() as u128)
    }

    pub fn benefit_config(&self) -> Option<BenefitConfig> {
        return self.benefit_config.clone()
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>, filter_opt: Option<FilterOpt>) -> Vec<LicenseToken> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        let is_filter = filter_opt.as_ref().is_some();
        let is_asset_filter = is_filter && filter_opt.as_ref().unwrap().asset_id.is_some();
        let is_owner_filter = is_filter && filter_opt.as_ref().unwrap().account_id.is_some();
        self.tokens_by_id.keys()
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .filter(|x| !is_asset_filter || *filter_opt.as_ref().unwrap().asset_id.as_ref().unwrap() == x.asset_id)
            .filter(|x| !is_owner_filter || *filter_opt.as_ref().unwrap().account_id.as_ref().unwrap() == x.owner_id)
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    pub fn shrinked_nft_tokens_for_asset(&self, asset_id: String) -> Vec<ShrinkedLicenseToken> {
        let mut result: Vec<ShrinkedLicenseToken> = Vec::new();
        let tokens_opt = self.tokens_per_asset.get(&asset_id);
        if tokens_opt.is_none() {
            return Vec::new()
        }
        let tokens = tokens_opt.unwrap();
        for key in tokens.iter() {
            result.push(self.shrinked_nft_token(key))
        }
        result
    }

    pub fn nft_token_supply_for_asset(&self, asset_id: String) -> u64 {
        let tokens_for_asset = self.tokens_per_asset.get(&asset_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_asset) = tokens_for_asset {
            tokens_for_asset.len() as u64
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            0
        }
    }

    fn shrinked_nft_token(&self, token_id: TokenId) -> ShrinkedLicenseToken {
        //if there is some token ID in the tokens_by_id collection
        let token = self.tokens_by_id.get(&token_id).unwrap();
        ShrinkedLicenseToken {
            token_id,
            asset_id: token.asset_id,
            metadata: token.metadata.shrink(),
            license: if token.license.is_some() { Some(token.license.as_ref().unwrap().shrink()) } else {None},
        }
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    pub fn nft_token_id_max(&self) -> String {
        let mut max: i64 = 0;
        for key in self.tokens_by_id.keys() {
            let current = i64::from_str(&key);
            if current.is_err() {
                continue
            } else {
                let res = current.unwrap();
                if res > max {
                    max = res.clone()
                }
            }
        }
        max.to_string()
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<LicenseToken> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
