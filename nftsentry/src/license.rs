use crate::*;

use near_sdk::{log,Gas};

// const GAS_FOR_LICENSE_APPROVE: Gas = Gas(10_000_000_000_000);
// const NO_DEPOSIT: Balance = 0;
const MIN_GAS_FOR_LICENSE_APPROVE_CALL: Gas = Gas(100_000_000_000_000);


#[near_bindgen]
impl Contract {


    #[payable]
    pub fn nft_update_license(
        &mut self,  
        token_id: TokenId, 
        license: TokenLicense, 
    ){

        let predecessor_id = env::predecessor_account_id();
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        if predecessor_id != token.owner_id {
            panic!("License can only be updated directly by the token owner");
        }

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();


        self.internal_replace_license(&predecessor_id,&token_id,&license);

        // Construct the mint log as per the events standard.
        let nft_approve_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftUpdateLicense(vec![NftUpdateLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_approve_license_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
    }


    #[payable]
    pub fn nft_reject_license(&mut self, token_id: TokenId){
       //measure the initial storage being used on the contract
        assert_one_yocto(); // user need to sign for approve transaction

        let initial_storage_usage = env::storage_usage();

        let token = self.tokens_by_id.get(&token_id).expect("No token");
        let predecessor_id = env::predecessor_account_id();


        if predecessor_id != token.owner_id {
            panic!("Only the token owner can approve a license update");
        }

        self.internal_reject_license(&predecessor_id, &token_id); 

        // Construct the mint log as per the events standard.
        let nft_reject_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.

            event: EventLogVariant::NftRejectLicense(vec![NftRejectLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_reject_license_log.to_string());


        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
    }


    #[payable]
    pub fn nft_approve_license(&mut self, token_id: TokenId){
       //measure the initial storage being used on the contract
        assert_one_yocto(); // user need to sign for approve transaction

        let initial_storage_usage = env::storage_usage();

        let token = self.tokens_by_id.get(&token_id).expect("No token");
        let predecessor_id = env::predecessor_account_id();
        
        
        if predecessor_id != token.owner_id {
            panic!("Only the token owner can approve a license update");
        }
    
              
        self.internal_update_license(&predecessor_id, &token_id); 
    
        // Construct the mint log as per the events standard.
        let nft_license_update_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.

            event: EventLogVariant::NftApproveLicense(vec![NftApproveLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_license_update_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
    }

    #[payable]
    pub fn nft_propose_license(&mut self, token_id: TokenId, proposed_license: TokenLicense){
       //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let predecessor_id = env::predecessor_account_id();
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        self.internal_propose_license(&predecessor_id, &token_id, &proposed_license);

        // Construct the mint log as per the events standard.
        let nft_propose_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftProposeLicense(vec![NftProposeLicenseLog {
                owner_id: token.owner_id.to_string(),
                // Owner of the token.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        self.log_event(&nft_propose_license_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
    }

    //get the information for a specific token ID
    pub fn nft_license(&self, token_id: TokenId) -> Option<JsonTokenLicense> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            if let Some(license) = self.token_license_by_id.get(&token_id) {
                //we return the JsonTokenLicense (wrapped by Some since we return an option)
                Some(JsonTokenLicense {
                    token_id,
                    owner_id: token.owner_id,
                    license,
                })
            } else {
                None
            }
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }

    //get the information for a specific token ID
    pub fn nft_proposed_license(&self, token_id: TokenId) -> Option<JsonTokenLicense> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            // let license = self.token_license_by_id.get(&token_id).unwrap();
            if let Some(license) = self.token_proposed_license_by_id.get(&token_id) {
            //we return the JsonTokenLicense (wrapped by Some since we return an option)
                Some(JsonTokenLicense {
                    token_id,
                    owner_id: token.owner_id,
                    license,
                })
            } else {
                None
            }
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }
    #[private]
    pub fn internal_propose_license(&mut self, account_id: &AccountId, token_id: &TokenId, proposed_license: &TokenLicense) {
        println!("==>internal_propose_license, account={}", account_id);
        if let Some(_license) = self.token_proposed_license_by_id.get(&token_id) {
            self.token_proposed_license_by_id.remove(&token_id);
        }
        self.token_proposed_license_by_id.insert(&token_id, &proposed_license);
    }

    #[private]
    pub fn internal_update_license(&mut self, account_id: &AccountId, token_id: &TokenId) {
        println!("==>internal_update_license, account={}", account_id);
        if let Some(proposed_license) = self.token_proposed_license_by_id.get(&token_id) {
            self.token_proposed_license_by_id.remove(&token_id );
            if let Some(_license) = self.token_license_by_id.get(&token_id) {
                self.token_license_by_id.remove(&token_id);
            }
            self.token_license_by_id.insert(&token_id, &proposed_license);
        } else {
            log!("No proposed license i the token");
            panic!("No propose license in the token");
        }
    }

    #[private]
    pub fn internal_reject_license(&mut self, account_id: &AccountId, token_id: &TokenId) {
        println!("==>internal_restore_license, account={}", account_id);
        if let Some(_proposed_license) = self.token_proposed_license_by_id.get(&token_id) {
            self.token_proposed_license_by_id.remove(&token_id );
        } else {
            log!("No proposed license in the token");
            panic!("No propose license in the token");
        }
    }

    #[private]
    pub fn internal_replace_license(&mut self, account_id: &AccountId, token_id: &TokenId, license: &TokenLicense) {
        println!("==>internal_replace_license, account={}", account_id);
        if let Some(_license) = self.token_license_by_id.get(&token_id) {
            self.token_license_by_id.remove(&token_id);

        }
        self.token_license_by_id.insert(&token_id, &license);
    }

    #[private]
    pub fn license_approval(
        sender_id: AccountId, 
        account_id: AccountId, 
        token_id: TokenId,
        approve: bool, 
        deposit: Balance, 
        gas_limit: Gas,
    ) -> bool {
        println!(
            "==>license_authorization, sender={}, account={}, token={}, deposit={}, gas_limit={}",
            sender_id, account_id, token_id, deposit, serde_json::to_string(&gas_limit).unwrap(),
        );
        assert_one_yocto();

        //get the GAS attached to the call
        let attached_gas = env::prepaid_gas();

        /*
            make sure that the attached gas is greater than the minimum GAS for NFT approval call.
            This is to ensure that the cross contract call to internal_update_license won't cause a prepaid GAS error.
        */
        assert!(
            attached_gas >= MIN_GAS_FOR_LICENSE_APPROVE_CALL,
            "You cannot attach less than {:?} Gas to nft_transfer_call",
            MIN_GAS_FOR_LICENSE_APPROVE_CALL
        );

        approve
    }
}

