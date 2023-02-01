use near_sdk::{AccountId, env, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8};
use policy_rules::policy::{AllPolicies, init_policies};

pub mod policy_contract;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {
    // contract owner
    pub owner_id: AccountId,
    pub policies: AllPolicies,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(owner_id)
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized.
        let policies = init_policies();
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            owner_id,
            policies,
        };

        this
    }

    pub fn get_policies(&self) -> AllPolicies {
        self.policies.clone()
    }

    pub fn clean(&self, keys: Vec<Base64VecU8>) {
        let sender = env::predecessor_account_id();
        if sender != self.owner_id && sender != env::current_account_id() {
            env::panic_str("Unauthorized")
        }
        for key in keys.iter() {
            env::storage_remove(&key.0);
        }
    }
}
