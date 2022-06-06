#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::borsh::maybestd::collections::HashMap;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{AccountId, testing_env};
    use near_sdk::env;
    use crate::{Contract, Token, TokenMetadata, TokenLicense, LicenseData};
    use crate::approval::NonFungibleTokenCore;
    use crate::mint::*;
    // use crate::license::*;

    use crate::nft_core::NonFungibleTokenCore as NFTCore;
 

    const MINT_STORAGE_COST: u128 = 637000000000000000000000;

    /// Returns a pre-defined account_id from a list of 6.
    pub fn test_accounts(id: usize) -> AccountId {
    AccountId::new_unchecked(
        ["nft.lsheiba.testnet", "lsheiba.testnet", "nftsentry.testnet", "kibernetika.testnet"][id].to_string(),
    )
}


    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    
    fn sample_token_license() -> TokenLicense {
        // TODO std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("error")
        TokenLicense {
            title: Some("NFTSentry License #1".into()),
            description: Some("First NFTSentry License Template".into()),
            issuer_id: None,
            uri: Some("https://bafybeihjuk544ww4e5qexjlrfyzdl6mkht6rk6cmbfvbosknvrjni364x4.ipfs.nftstorage.link".into()), // URL to associated pdf, preferably to decentralized, content-addressed storage
            metadata: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
            issued_at: None, // When token was issued or minted, Unix epoch in milliseconds
            expires_at: None, // When token expires, Unix epoch in milliseconds
            starts_at: None, // When token starts being valid, Unix epoch in milliseconds
            updated_at: None, // When token was last updated, Unix epoch in milliseconds
            reference: None, // URL to an off-chain JSON file with more info.
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
                }
    }

    fn sample_license_data() -> String {
        let license_data = LicenseData {
            i_agree: true,
            perpetuity: true,
            exclusivity: true,
            personal_use: true,
            commercial_use: false,
            limited_display_sublicensee: true,
            template: Some("v1".into()),
            pdf_url: Some("https://nftstorage.link/ipfs/bafybeifrvo2ksl7mz6coxx2iie6l73pgd2wkptnict5ypsoe4xenlufdhm".into()),
        };

        let serialized = serde_json::to_string(&license_data).unwrap();
        serialized
    }

    #[test]
    fn test_serialize_license_data() {
        let license_data = sample_license_data();
        println!("==> LicenseData: {}", license_data);
        assert_ne!(license_data.len(), 0);
    }

    #[test]
    fn test1() {
        println!("==> test1");
        let mut context = get_context(test_accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(test_accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(test_accounts(0))
            .build());

        let token_id = "token-1".to_string();

        let _token = contract.nft_mint(
            token_id.clone(),
            sample_token_metadata(),
            sample_token_license(),
            test_accounts(0),
            None,
        );

        let authorized_id = Some(test_accounts(0).to_string());
        contract.nft_propose_license(authorized_id, token_id.clone(),sample_token_license(), test_accounts(0));
        let authorized_id = Some(test_accounts(0).to_string());
        contract.nft_approve_license(authorized_id, token_id.clone(), test_accounts(0));
        let authorized_id = Some(test_accounts(0).to_string());
        contract.nft_update_license(authorized_id, token_id.clone(),sample_token_license(), test_accounts(0));

        let authorized_id = test_accounts(0);
        let token_id = "token-1".to_string();
        let proposed_license  = sample_token_license();
        
        contract.internal_propose_license(&authorized_id, &token_id, &proposed_license);
        let out = contract.nft_proposed_license(token_id);
        println!("{}", serde_json::to_string(&out).unwrap());

        let authorized_id = test_accounts(0);
        let token_id = "token-1".to_string();
        
        contract.internal_update_license(&authorized_id, &token_id);
        let out = contract.nft_license(token_id);
        println!("{}", serde_json::to_string(&out).unwrap());

        // assert_eq!(token.token_id, token_id);

        assert!(true)
    }

    #[test]
    fn test_mint() {
        println!("==> test_mint");
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());

        let token_id = "token-1".to_string();
        contract.nft_mint(
            token_id.clone(),
            sample_token_metadata(),
            sample_token_license(),
            accounts(0),
            None,
        );

        let out = contract.nft_token(token_id);

        println!("TokenJson: {}", serde_json::to_string(&out).unwrap());

        assert!(true)

    //    assert_eq!(token.token_id, token_id);
    //    assert_eq!(token.owner_id, accounts(0));
    //    assert_eq!(token.metadata, sample_token_metadata());
    //    assert_eq!(token.approved_account_ids, HashMap::new());
    }

    #[test]
    fn test_approve() {
        println!("==> test_approve");
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), sample_token_metadata(), sample_token_license(), accounts(0), None);

        // alice approves bob
        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(150000000000000000000)
        .predecessor_account_id(accounts(0))
        .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
        .storage_usage(env::storage_usage())
        .account_balance(env::account_balance())
        .is_view(true)
        .attached_deposit(0)
        .build());
        assert!(contract.nft_is_approved(
            token_id.clone(), accounts(1), Some(0))
        );
    }

/*   

    #[test]
        fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());

        let token_id = "0".to_string();
        let token = contract.nft_mint(
            token_id.clone(),
            sample_token_metadata(),
            accounts(0),
            None,
        );
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0));
        assert_eq!(token.metadata, sample_token_metadata());
        assert_eq!(token.approved_account_ids, HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), sample_token_metadata(), accounts(0), None);

        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(1)
        .predecessor_account_id(accounts(0))
        .build());
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
        .storage_usage(env::storage_usage())
        .account_balance(env::account_balance())
        .is_view(true)
        .attached_deposit(0)
        .build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1));
            assert_eq!(token.metadata, sample_token_metadata());
            assert_eq!(token.approved_account_ids, HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    #[test]
    fn test_approve() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), sample_token_metadata(), accounts(0), None);

        // alice approves bob
        testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(150000000000000000000)
        .predecessor_account_id(accounts(0))
        .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
        .storage_usage(env::storage_usage())
        .account_balance(env::account_balance())
        .is_view(true)
        .attached_deposit(0)
        .build());
        assert!(contract.nft_is_approved(
            token_id.clone(), accounts(1), Some(0))
        );
    }

    #[test]
    fn test_revoke() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(MINT_STORAGE_COST)
    .predecessor_account_id(accounts(0))
    .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), sample_token_metadata(),
                          accounts(0), None);

        // alice approves bob
        testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(150000000000000000000)
    .predecessor_account_id(accounts(0))
    .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(1)
    .predecessor_account_id(accounts(0))
    .build());
        contract.nft_revoke(token_id.clone(), accounts(1));
        testing_env!(context
    .storage_usage(env::storage_usage())
    .account_balance(env::account_balance())
    .is_view(true)
    .attached_deposit(0)
    .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
    }

    #[test]
    fn test_revoke_all() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(MINT_STORAGE_COST)
    .predecessor_account_id(accounts(0))
    .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), sample_token_metadata(),
                          accounts(0), None);

        // alice approves bob
        testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(150000000000000000000)
    .predecessor_account_id(accounts(0))
    .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(1)
    .predecessor_account_id(accounts(0))
    .build());
        contract.nft_revoke_all(token_id.clone());
        testing_env!(context
    .storage_usage(env::storage_usage())
    .account_balance(env::account_balance())
    .is_view(true)
    .attached_deposit(0)
    .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }
*/
    
}

