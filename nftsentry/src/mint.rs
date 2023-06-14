use std::str::FromStr;
use near_sdk::{Gas, PromiseError};
use common_types::prices::{Asset, get_near_price};
use common_types::types::{NFTMintResult};
use common_types::utils::{balance_from_string, format_balance};
use crate::*;

const SLIPPAGE_PERCENTS: i32 = 3;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintOpt {
    pub is_gift: bool,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        mut token_id: TokenId,
        asset_id: String,
        sku_id: Option<String>,
        receiver_id: AccountId,
    ) -> Promise {
        if self.tokens_by_id.get(&token_id).is_some() {
            let max_token = self.nft_token_id_max();
            token_id = (i64::from_str(&max_token).unwrap() + 1).to_string();
        }

        // Schedule calls to metadata and asset token
        let promise_meta: Promise = inventory_contract::ext(self.inventory_id.clone())
            .with_unused_gas_weight(1).inventory_metadata();
        let promise_asset: Promise = inventory_contract::ext(self.inventory_id.clone())
            .with_unused_gas_weight(1).asset_token(asset_id.clone());
        let promise_price = get_near_price(3);
        let promise_inventory = promise_meta
            .and(promise_asset)
            .and(promise_price);
        // Then schedule call to self.callback

        let predecessor_id = env::predecessor_account_id();
        // Important: pass is_gift: false to make sure that price checks and charging work
        let opt = MintOpt{is_gift: false};
        return promise_inventory.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .with_unused_gas_weight(28)
                .on_nft_mint(
                    token_id, sku_id, receiver_id, predecessor_id, opt,
                )
        )
    }

    #[payable]
    pub fn nft_mint_owner(
        &mut self,
        mut token_id: TokenId,
        asset_id: String,
        sku_id: Option<String>,
        receiver_id: AccountId,
        opts: MintOpt,
    ) -> Promise {
        let sender = env::predecessor_account_id();
        if sender != self.owner_id && sender != env::current_account_id() && sender != self.inventory_id {
            env::panic_str("Only the owner or inventory can call this method")
        }

        if self.tokens_by_id.get(&token_id).is_some() {
            let max_token = self.nft_token_id_max();
            token_id = (i64::from_str(&max_token).unwrap() + 1).to_string();
        }

        // Schedule calls to metadata and asset token
        let promise_meta: Promise = inventory_contract::ext(self.inventory_id.clone())
            .with_unused_gas_weight(3).inventory_metadata();
        let promise_asset: Promise = inventory_contract::ext(self.inventory_id.clone())
            .with_unused_gas_weight(3).asset_token(asset_id.clone());
        let promise_price = get_near_price(3);
        let promise_inventory = promise_meta
            .and(promise_asset)
            .and(promise_price);
        // Then schedule call to self.callback

        let predecessor_id = env::predecessor_account_id();
        // Important: pass is_gift: false to make sure that price checks and charging work
        return promise_inventory.then(
            Self::ext(env::current_account_id())
                .with_attached_deposit(env::attached_deposit())
                .with_unused_gas_weight(27)
                .on_nft_mint(
                    token_id, sku_id, receiver_id, predecessor_id, opts,
                )
        )
    }

    #[private]
    #[payable]
    pub fn on_nft_mint(
        &mut self,
        #[callback_result] metadata_res: Result<ExtendedInventoryMetadata, PromiseError>,
        #[callback_result] asset_res: Result<JsonAssetToken, PromiseError>,
        #[callback_result] price_res: Result<Option<Asset>, PromiseError>,
        token_id: TokenId,
        sku_id: Option<String>,
        receiver_id: AccountId,
        predecessor_id: AccountId,
        opts: MintOpt,
    ) -> PromiseOrValue<NFTMintResult> {
        let result = self.ensure_nft_mint(
            metadata_res, asset_res, price_res, token_id.clone(), sku_id,
            receiver_id.clone(), predecessor_id.clone(), opts,
        );

        if result.is_err() {
            let _ = refund_deposit(0, Some(predecessor_id), None);
            unsafe {
                let msg = result.unwrap_err_unchecked();
                env::log_str( &format!("Error: {}", msg));
                // env::panic_str(result.unwrap_err_unchecked().as_str());
                return PromiseOrValue::Value(NFTMintResult{
                    license_token: None,
                    error: msg,
                })
            }
        }

        PromiseOrValue::Promise(result.unwrap())
    }

    fn ensure_nft_mint(
        &self,
        metadata_res: Result<ExtendedInventoryMetadata, PromiseError>,
        asset_res: Result<JsonAssetToken, PromiseError>,
        price_res: Result<Option<Asset>, PromiseError>,
        token_id: TokenId,
        sku_id: Option<String>,
        receiver_id: AccountId,
        predecessor_id: AccountId,
        opts: MintOpt,
        ) -> Result<Promise, String> {

        // 1. Check callback results first.
        if metadata_res.is_err() || asset_res.is_err() || price_res.is_err() {
            return if metadata_res.is_err() {
                Err("Failed call inventory_metadata".to_string())
            } else if asset_res.is_err() {
                Err("Failed call asset_token".to_string())
            } else {
                Err("Failed call priceoracle.get_asset".to_string())
            }
        }

        unsafe {
            let asset = asset_res.unwrap_unchecked();
            let inv_metadata = metadata_res.unwrap_unchecked();

            let near_price_data = price_res.unwrap_unchecked().unwrap();
            let near_price = &near_price_data.reports.last().unwrap().price;
            env::log_str(&format!("NEAR price: {} USD", near_price.string_price()));

            let asset_license_opt = asset.licenses.as_ref().unwrap().iter().find(
                |x| x.sku_id == sku_id
            );

            if asset_license_opt.is_none() {
                return Err(format!("Asset license not found by sku_id {}", sku_id.unwrap_or(String::new())))
            }
            let mut asset_license = (*asset_license_opt.unwrap()).clone();

            let full_inventory = self.get_full_inventory(asset.clone(), inv_metadata.metadata.clone());
            let inv_license = inv_metadata.metadata.licenses.iter().find(
                |x| asset_license.license_id.is_some() && Some(&x.license_id) == asset_license.license_id.as_ref()
            ).cloned();

            // Re-calculate and re-assign a price
            // let price_currency = asset_license.price.clone().unwrap();
            let price_str = asset_license.get_near_cost(near_price);
            asset_license.price = price_str.clone();

            let deposit = env::attached_deposit();
            let mut price = balance_from_string(price_str.clone());
            if deposit < price && !opts.is_gift {
                let reserved_price = deposit - balance_from_string("0.1".to_string());
                let minimum_price = price * (100 - SLIPPAGE_PERCENTS) as u128 / 100;
                if reserved_price < minimum_price {
                    return Err(format!(
                        "Attached deposit of {} NEAR is less than SKU price of {} NEAR (with {}% slippage)",
                        format_balance(deposit),
                        price_str,
                        SLIPPAGE_PERCENTS,
                    ))
                }
                price = reserved_price;

            }

            let mut lic_token = asset.issue_new_license(inv_license, asset_license, token_id);
            lic_token.owner_id = receiver_id;

            let promise_new: Promise = policy_rules_contract::ext(self.policy_contract.clone())
                .with_unused_gas_weight(38).check_new(
                full_inventory,
                lic_token.shrink(),
                asset.policy_rules,
                asset.upgrade_rules,
            );
            let on_check_promise = promise_new.then(
                Self::ext(env::current_account_id())
                    .with_attached_deposit(deposit)
                    .with_unused_gas_weight(8)
                    .on_check_new_receiver(
                        lic_token,
                        price, asset.token_id,
                        inv_metadata.owner_id,
                        predecessor_id,
                        opts,
                    )
            );
            Ok(on_check_promise)
        }
    }

    #[private]
    #[payable]
    pub fn on_check_new_receiver(
        &mut self,
        #[callback_result] check_new_res: Result<IsAvailableResponseData, PromiseError>,
        lic_token: LicenseToken,
        asset_license_price: Balance,
        asset_id: String,
        inventory_owner: AccountId,
        predecessor_id: AccountId,
        opts: MintOpt,
    ) -> NFTMintResult {
        // measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();
        // we add an optional parameter for perpetual royalties
        // create a royalty map to store in the token
        // let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function:
        // if let Some(perpetual_royalties) = perpetual_royalties {
        //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
        // assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

        //iterate through the perpetual royalties and insert the account and amount in the royalty map
        // for (account, amount) in perpetual_royalties {
        //     royalty.insert(account, amount);
        // }
        // }
        if check_new_res.is_err() {
            let _ = refund_deposit(0, Some(predecessor_id.clone()), None);
            return NFTMintResult{
                license_token: None,
                error: "Failed call check_new()".to_string(),
            }
        } else {
            let res = check_new_res.unwrap();
            if !res.result {
                let _ = refund_deposit(0, Some(predecessor_id.clone()), None);
                return NFTMintResult{
                    license_token: None,
                    error: res.reason_not_available,
                }
            }
        }

        //specify the token struct that contains the owner ID
        let token = Token {
            token_id: lic_token.token_id.clone(),
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: lic_token.owner_id.clone(),
            asset_id: asset_id.clone(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            license: lic_token.license.clone(),
            metadata: lic_token.metadata.clone(),
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            // royalty: royalty.clone(),
        };


        // ----- Token mint start -----
        let mint_result = self.internal_mint(lic_token.clone());
        if mint_result.is_err() {
            let _ = refund_deposit(0, Some(predecessor_id), None);
            let msg = unsafe{mint_result.unwrap_err_unchecked()};
            env::log_str( &format!("Error: {}", msg));
            return NFTMintResult{license_token: None, error: msg.to_string()}
        }
        // ----- Token mint end -----

        // If token is a gift -> no charged prices
        let charged_price = if opts.is_gift {
            None
        } else {
            Some(asset_license_price.clone())
        };
        // refund any excess storage if the user attached too much.
        let result = refund_storage(
            initial_storage_usage,
            Some(predecessor_id.clone()),
            charged_price,
        );
        if result.is_err() {
            // Refund failed due to storage costs.
            // Rollback all changes!
            self.internal_remove_token_from_owner(&token.owner_id, &token.token_id);
            self.internal_remove_token_from_asset(&token.asset_id, &token.token_id);
            self.tokens_by_id.remove(&token.token_id);
            // self.token_license_by_id.remove(&token.token_id);
            // self.token_metadata_by_id.remove(&token.token_id);
            // Refund any deposit
            let _ = refund_deposit(0, Some(predecessor_id), None);

            let msg = result.unwrap_err();
            env::log_str( &format!("Error: {}", msg));
            return NFTMintResult{license_token:None, error:msg}
        }

        let license_sold = self.nft_token_supply_for_asset(asset_id.clone());
        inventory_contract::ext(self.inventory_id.clone()).with_static_gas(Gas::ONE_TERA * 3).on_nft_mint(
            asset_id, license_sold
        );
        // Process fees only if it is not a gift
        if !opts.is_gift {
            self.process_fees(asset_license_price.clone(), inventory_owner);
        }

        // Log the serialized json.
        self.log_event(&mint_result.unwrap().to_string());

        NFTMintResult{
            license_token: Some(lic_token),
            error: String::new(),
        }
    }

    #[private]
    pub fn process_fees(&self, base_deposit: Balance, master_account: AccountId) {
        // base_deposit -> 97.5% base_account, 2.5% benefit
        let benefit_fee_milli_percent = if self.benefit_config.is_some() {
            unsafe {self.benefit_config.clone().unwrap_unchecked().fee_milli_percent_amount}
        } else {
            0
        };
        let benefit_fee = base_deposit / (1e5 as u128) * (benefit_fee_milli_percent as u128);
        let base_amount = base_deposit - benefit_fee;

        Promise::new(master_account).transfer(base_amount);
        if benefit_fee != 0 {
            unsafe {
                Promise::new(self.benefit_config.clone().unwrap_unchecked().account_id).transfer(benefit_fee);
            }
        }
    }

    #[private]
    pub(crate) fn internal_mint(&mut self, lic_token: LicenseToken) -> Result<EventLog, String> {
        let token = Token{
            token_id: lic_token.token_id.clone(),
            asset_id: lic_token.asset_id.clone(),
            owner_id: lic_token.owner_id.clone(),
            approved_account_ids: lic_token.approved_account_ids.clone(),
            next_approval_id: 0,
            license: lic_token.license,
            metadata: lic_token.metadata,
            // royalty: lic_token.royalty.clone(),
        };
        let exists = self.tokens_by_id.insert(&lic_token.token_id, &token);
        if exists.is_some() {
            let msg = "Token already exists";
            env::log_str( &format!("Error: {}",msg));
            return Err(msg.to_string())
        }
        // self.token_metadata_by_id.insert(&lic_token.token_id, &lic_token.metadata);
        //insert the token ID and license
        // if lic_token.license.is_some() {
        //     self.token_license_by_id.insert(&lic_token.token_id, lic_token.license.as_ref().unwrap());
        // }

        //self.token_proposed_license_by_id.insert(&token_id, &proposed_license);

        self.internal_add_token_to_owner(&lic_token.owner_id, &lic_token.token_id);
        self.internal_add_token_to_asset(&lic_token.asset_id, &lic_token.token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token.token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };
        Ok(nft_mint_log)
    }
}

