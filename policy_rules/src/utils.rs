use near_sdk::{Balance, AccountId, env, Promise};

pub fn refund_storage(initial_storage: u64, predecessor_id: Option<AccountId>, charged_price: Option<Balance>) -> Result<(), String> {
    let new_storage_usage = env::storage_usage();
    let mut storage_usage_diff =  0 as near_sdk::StorageUsage;
    if new_storage_usage > initial_storage {
        storage_usage_diff = new_storage_usage - initial_storage;
        let log_message = format!("Storage usage increased by {} bytes", storage_usage_diff);
        env::log_str(&log_message);
    }
    return refund_deposit(storage_usage_diff, predecessor_id, charged_price)
}

//refund the initial deposit based on the amount of storage that was used up
pub fn refund_deposit(storage_used: u64, predecessor_id: Option<AccountId>, charged_price: Option<Balance>) -> Result<(), String> {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    if required_cost > attached_deposit - charged_price.unwrap_or(0) {
        let msg = &format!(
            "Must attach {} NEAR to cover storage",
            format_balance(required_cost)
        );
        if predecessor_id.is_some() {
            return Err(msg.to_string())
        } else {
            env::panic_str(msg)
        }
    }

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost - charged_price.unwrap_or(0);

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    let predecessor_account_id = predecessor_id.unwrap_or(env::predecessor_account_id());
    env::log_str(&format!("Refund {} NEAR to {}", format_balance(refund), predecessor_account_id));
    if refund > 1 {
        Promise::new(predecessor_account_id).transfer(refund);
    }
    Ok(())
}

//Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage)
pub fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}

pub fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR",
    )
}

pub fn balance_from_string(s: String) -> near_sdk::Balance {
    let float: f64 = s.parse().unwrap();
    let half_yocto = float * 1e12;
    let half = half_yocto as u128;
    use std::ops::Mul;
    let near = half.mul(1e12 as u128);
    return near
}

pub fn format_balance(b: near_sdk::Balance) -> String {
    let half = b / 1e12 as u128;
    let float = (half as f64) / 1e12;
    float.to_string()
}

pub fn get_inventory_id(minter_id: String) -> String {
    let splitted: Vec<&str> = minter_id.split("_").collect();
    splitted[1..].join("_")
}