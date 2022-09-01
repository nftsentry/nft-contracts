use policy_rules::utils::refund_deposit;
use crate::*;

#[near_bindgen]
impl InventoryContract {
    #[payable]
    pub fn asset_mint(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        minter_id: AccountId,
        licenses: Option<AssetLicenses>,
    ) -> JsonAssetToken {
        
        let initial_storage_usage = env::storage_usage();

        let sender_id = env::predecessor_account_id();
        if sender_id != self.owner_id {
            // sender must be the owner of the contract
            env::panic_str("Unauthorized");
        }

        let token = AssetToken {
            token_id: token_id.clone(),
            owner_id: receiver_id,
            minter_id: minter_id.clone(),
            license_token_count: 0,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        let exists = self.tokens_by_id.insert(&token.token_id, &token);
        if exists.is_some() {
            env::panic_str("Token already exists")
        }

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token.token_id, &metadata);
        // Insert the token ID and list of available licenses  
        if licenses.is_some() {
            //insert the token ID and license
            self.token_licenses_by_id.insert(&token.token_id, unsafe{licenses.as_ref().unwrap_unchecked()});
        }

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token.token_id);

        // Construct the mint log as per the events standard.
        let asset_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::AssetMint(vec![AssetMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token.token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };
        
        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        let _ = refund_deposit(required_storage_in_bytes, None, None);

        // Log the serialized json.
        self.log_event(&asset_mint_log.to_string());

        JsonAssetToken {
            token_id: token_id.clone(),
            owner_id: token.owner_id,
            minter_id: token.minter_id,
            metadata,
            licenses: licenses.clone(),
            license_token_count: 0,
        }
    }
    

}