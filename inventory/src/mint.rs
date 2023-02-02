use common_types::policy::{LimitationData, PolicyData};
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
        licenses: Option<Vec<AssetLicense>>,
        policy_rules: Option<Vec<LimitationData>>,
        upgrade_rules: Option<Vec<PolicyData>>,
    ) -> JsonAssetToken {
        let initial_storage_usage = env::storage_usage();

        let event = self.internal_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            minter_id.clone(),
            licenses.clone(),
            policy_rules.clone(),
            upgrade_rules.clone(),
        );
        
        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        let _ = refund_storage(initial_storage_usage, None, None);

        // Log the serialized json.
        self.log_event(&event.to_string());

        JsonAssetToken {
            token_id: token_id.clone(),
            owner_id: receiver_id,
            minter_id: minter_id,
            metadata,
            licenses: licenses.clone(),
            license_token_count: 0,
            policy_rules: policy_rules.clone(),
            upgrade_rules: upgrade_rules.clone(),
        }
    }

    pub(crate) fn internal_mint(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        minter_id: AccountId,
        licenses: Option<Vec<AssetLicense>>,
        policy_rules: Option<Vec<LimitationData>>,
        upgrade_rules: Option<Vec<PolicyData>>,
    ) -> EventLog {
        self.ensure_owner();

        let token = AssetToken {
            token_id: token_id.clone(),
            owner_id: receiver_id,
            minter_id: minter_id.clone(),
            license_token_count: 0,
            policy_rules: policy_rules.clone(),
            upgrade_rules: upgrade_rules.clone(),
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
        asset_mint_log
    }

    #[private]
    pub(crate) fn asset_replace(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        licenses: Option<Vec<AssetLicense>>,
        policy_rules: Option<Vec<LimitationData>>,
        upgrade_rules: Option<Vec<PolicyData>>,
    ) {
        let old_token = unsafe{self.tokens_by_id.get(&token_id).unwrap_unchecked()};
        let token = AssetToken {
            token_id: token_id.clone(),
            owner_id: old_token.owner_id,
            minter_id: old_token.minter_id.clone(),
            license_token_count: old_token.license_token_count,
            policy_rules: policy_rules.clone(),
            upgrade_rules: upgrade_rules.clone(),
        };

        self.tokens_by_id.insert(&token_id, &token);
        self.token_licenses_by_id.insert(&token_id, unsafe{licenses.as_ref().unwrap_unchecked()});
        self.token_metadata_by_id.insert(&token_id, &metadata);
    }

    #[payable]
    pub fn batch_mint(
        &mut self,
        tokens: Vec<JsonAssetToken>,
    ) {
        let initial_storage_usage = env::storage_usage();
        let mut events: Vec<EventLog> = Vec::new();
        for asset in tokens {
            let event = self.internal_mint(
                asset.token_id.clone(),
                asset.metadata.clone(),
                asset.owner_id,
                asset.minter_id,
                asset.licenses,
                asset.policy_rules,
                asset.upgrade_rules,
            );
            events.push(event);
        }
        let _ = refund_storage(initial_storage_usage, None, None);

        for event in events {
            self.log_event(&event.to_string());
        }
    }
}