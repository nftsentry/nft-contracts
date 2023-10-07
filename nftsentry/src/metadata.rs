use crate::*;
pub type TokenId = String;
pub type AssetId = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTokenLicense {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub license: TokenLicense,
    // proposed license 
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
}


pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
    fn set_nft_metadata(&mut self, metadata: NFTContractMetadata) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }

    fn set_nft_metadata(&mut self, metadata: NFTContractMetadata) -> NFTContractMetadata {
        let sender = env::predecessor_account_id();
        if sender != self.owner_id && sender != env::current_account_id() && sender != self.inventory_id {
            env::panic_str("Only the owner or inventory can call this method")
        }
        self.metadata.replace(&metadata);
        metadata
    }
}

