use crate::*;

#[near_bindgen]
impl InventoryContract {
    #[payable]
    pub fn asset_mint(
        &mut self,
        token_id: AssetTokenId,
        metadata: AssetTokenMetadata,
        receiver_id: AccountId,
        licenses: Option<AssetLicenses>,
    ) -> JsonAssetToken{
        
        let initial_storage_usage = env::storage_usage();

        let token = AssetToken {
            token_id: token_id.clone(),
            owner_id: receiver_id,
            metadata: metadata.clone(),
            minter_id: None,
            licenses: licenses,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token.token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token.token_id, &metadata);
        // Insert the token ID and list of available licenses  
        if let Some(ref licenses) = token.licenses {
            //insert the token ID and license
            self.token_licenses_by_id.insert(&token.token_id, &licenses);
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
        refund_deposit(required_storage_in_bytes);

        // Log the serialized json.
        self.log_event(&asset_mint_log.to_string());

        JsonAssetToken{
            token_id: token_id.clone(),
            owner_id: token.owner_id,
            minter_id: token.minter_id,
            metadata,
            licenses: token.licenses,
        }
    }

}