use crate::*;

pub trait FactoryContract {
    // fn deploy_contract(&self, account_id: AccountId, amount: U128);
    fn deploy_contract_code(&self, prefix: AccountId, code: Vec<u8>) -> Promise;
    fn deploy_contract_str(&self, prefix: AccountId, code: String) -> Promise;
    // fn simple_call(&mut self, account_id: AccountId, message: String);
    // fn complex_call(&mut self, account_id: AccountId, message: String) -> Promise;
}

#[near_bindgen]
impl FactoryContract for InventoryContract {
    // TODO: Including this method increases inventory contract size by ~385kb
    // fn deploy_contract(&self, account_id: AccountId, amount: U128) {
    //     Promise::new(account_id)
    //         .create_account()
    //         .transfer(amount.0)
    //         .add_full_access_key(env::signer_account_pk())
    //         .deploy_contract(
    //             include_bytes!("../../nftsentry/res/nftsentry.wasm").to_vec(),
    //         );
    // }

    fn deploy_contract_code(&self, prefix: AccountId, code: Vec<u8>) -> Promise {
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );
        let initial_balance = code.len() as u128 * 10e19 as u128 + 10e23 as u128;
        Promise::new(subaccount_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(initial_balance)
            .deploy_contract(code)
    }

    fn deploy_contract_str(&self, prefix: AccountId, code: String) -> Promise {
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );
        let initial_balance = code.len() as u128 * 10e19 as u128 + 10e23 as u128;
        Promise::new(subaccount_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(initial_balance)
            .deploy_contract(code.as_bytes().to_vec())
    }

    // fn simple_call(&mut self, account_id: AccountId, message: String) {
    //     ext_status_message::ext(account_id).set_status(message);
    // }
    // fn complex_call(&mut self, account_id: AccountId, message: String) -> Promise {
    //     // 1) call status_message to record a message from the signer.
    //     // 2) call status_message to retrieve the message of the signer.
    //     // 3) return that message as its own result.
    //     // Note, for a contract to simply call another contract (1) is sufficient.
    //     ext_status_message::ext(account_id.clone())
    //         .set_status(message)
    //         .then(Self::ext(env::current_account_id()).get_result(account_id))
    // }
    //
    // #[handle_result]
    // fn get_result(
    //     &self,
    //     account_id: AccountId,
    //     #[callback_result] set_status_result: Result<(), PromiseError>,
    // ) -> Result<Promise, &'static str> {
    //     match set_status_result {
    //         Ok(_) => Ok(ext_status_message::ext(account_id).get_status(env::signer_account_id())),
    //         Err(_) => Err("Failed to set status"),
    //     }
    // }
}