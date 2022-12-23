use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, Timestamp, AccountId, Promise};

pub type DurationSec = u32;
pub type AssetId = String;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceData {
    // #[serde(with = "u64_dec_format")]
    pub timestamp: Timestamp,
    pub recency_duration_sec: DurationSec,

    pub prices: Vec<AssetOptionalPrice>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetPrice {
    pub asset_id: AssetId,
    pub price: Price,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetOptionalPrice {
    pub asset_id: AssetId,
    pub price: Option<Price>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Price {
    // #[serde(with = "u128_dec_format")]
    pub multiplier: String,
    pub decimals: u8,
}

impl Price {
    pub fn string_price(&self) -> String {
        // price: { multiplier: '13564', decimals: 28 }
        // should be "1.3564"
        // == 13564 / (10**(28-24))
        return format!("{}", self.float())
    }

    pub fn float(&self) -> f64 {
        // price: { multiplier: '13564', decimals: 28 }
        // should be "1.3564"
        // == 13564 / (10**(28-24))
        let denominator = (10 as f64).powf(self.decimals as f64 - 24.0);
        let nominator: f64 = self.multiplier.parse().unwrap();
        return nominator / denominator
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Asset {
    pub reports: Vec<Report>,
    // pub emas: Vec<AssetEma>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Report {
    pub oracle_id: AccountId,
    // #[serde(with = "u64_dec_format")]
    pub timestamp: String,
    pub price: Price,
}

#[ext_contract(price_oracle_contract)]
pub trait PriceOracleContract {
    fn get_price_data(&self, asset_ids: Option<Vec<AssetId>>) -> PriceData;
    fn get_asset(&self, asset_id: AssetId) -> Option<Asset>;
}

pub fn oracle_account() -> AccountId {
    return if env::current_account_id().to_string().ends_with("near") {
        AccountId::new_unchecked("priceoracle.near".to_owned())
    } else {
        AccountId::new_unchecked("priceoracle.testnet".to_owned())
    }
}

pub fn wrap_account() -> String {
    return if env::current_account_id().to_string().ends_with("near") {
        "wrap.near".to_owned()
    } else {
        "wrap.testnet".to_owned()
    }
}

pub fn get_near_price(gas_weight: u64) -> Promise {
    let promise = price_oracle_contract::ext(oracle_account())
        .with_unused_gas_weight(gas_weight)
        .get_asset(wrap_account());
    return promise
}

