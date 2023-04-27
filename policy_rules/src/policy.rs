use std::collections::{BTreeMap, HashMap};
use std::string::ToString;
use near_sdk::serde_json;
use minijinja::value::{Value};

use crate::*;
use common_types::types::*;
use crate::types::SKUAvailability;

pub const LEVEL_INVENTORY: &str = "inventory";
pub const LEVEL_LICENSES: &str = "licenses";
pub const DEFAULT_TEMPLATE: &str = "true";

pub trait ConfigInterface {
    fn check_transition(
        &self, inventory: FullInventory, old: ShrinkedLicenseToken, new: ShrinkedLicenseToken, policy_rules: Option<Vec<Limitation>>, upgrade_rules: Option<Vec<Policy>>
    ) -> Result<IsAvailableResponse, String>;
    fn check_new(
        &self, inventory: FullInventory, new: ShrinkedLicenseToken, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> IsAvailableResponse;
    fn check_state(&self, licenses: Vec<ShrinkedLicenseToken>) -> IsAvailableResponse;
    fn check_inventory_state(&self, licenses: Vec<InventoryLicense>) -> IsAvailableResponse;
    fn list_transitions(
        &self, inventory: FullInventory, from: ShrinkedLicenseToken, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> Vec<SKUAvailability>;
    fn list_available(
        &self, inventory: FullInventory, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> Vec<SKUAvailability>;
    // fn clone_with_additional(&self, l: Vec<Limitation>) -> AllPolicies;
}

pub fn init_policies() -> AllPolicies {
    let raw = include_bytes!("rules.json").to_vec();
    let mut config: AllPolicies = serde_json::from_slice(raw.as_slice()).expect("Fail to parse rules.yaml");

    for (policy_name, pol) in &mut config.policies {
        // config.policies.get_mut(policy_name.as_str()).unwrap().name = Some(policy_name.clone());
        pol.name = Some(policy_name.clone());
    }

    config
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AllPolicies {
    pub version: String,
    pub policies: HashMap<String, Policy>,
    pub limitations: Vec<Limitation>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub name: Option<String>,
    pub template: String,
    pub upgrade_to: Vec<String>,
    pub user_defined: Option<bool>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct IsAvailableResponse {
    pub result: bool,
    pub reason_not_available: String,
    pub additional_info: Option<HashMap<String, LimitsInfo>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LimitsInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub scope: String,
    pub name: String,
    pub remains: i32,
    pub total: i32,
    pub issued: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Context {
    pub full: FullInventory,
}

impl Policy {
    pub fn has_upgrade_to(&self, policy_name: String) -> bool {
        self.upgrade_to.contains(&policy_name)
    }
}

pub trait LimitCheck {
    fn check(&self, matched: Vec<&dyn LicenseGeneral>, l: &Limitation, ctx: Context) -> IsAvailableResponse;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PolicyOpt {
    set_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CheckNewOpt {
    in_assets: Option<bool>,
    in_licenses: Option<bool>,
    token_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FutureStateOpt {
    pub level: String,
    pub ctx: Context,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Limitation {
    pub name: String,
    pub display_name: Option<String>,
    pub scope: String,
    pub level: String,
    pub template: String,
    pub max_count: Option<MaxCount>,
    pub exclusive: Option<Exclusive>,
    // Add another limit types
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MaxCount {
    pub count: i32,
}

impl LimitCheck for MaxCount {
    fn check(&self, matched: Vec<&dyn LicenseGeneral>, l: &Limitation, _: Context) -> IsAvailableResponse {
        let name = if l.display_name.is_none() { l.name.clone() } else { l.display_name.as_ref().unwrap().clone() };
        if matched.len() > self.count as usize {
            let msg = format!(
                "Cannot set more {}: max count {}",
                name.clone(), self.count,
            );
            IsAvailableResponse{result: false, reason_not_available: msg, additional_info: None}
        } else {
            let info = LimitsInfo{
                remains: self.count - matched.len() as i32,
                total:   self.count,
                issued:  matched.len() as i32,
                type_:    "max_count".to_string(),
                scope:   l.scope.clone(),
                name,
            };
            let infos: HashMap<String, LimitsInfo> = vec![("check".to_string(), info)].into_iter().collect();
            IsAvailableResponse{result: true, reason_not_available: "".to_string(), additional_info: Some(infos)}
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Exclusive {}

impl LimitCheck for Exclusive {
    fn check(&self, matched: Vec<&dyn LicenseGeneral>, l: &Limitation, ctx: Context) -> IsAvailableResponse {
        let count_excl = matched.len();

        let mut exclusive_by_set: HashMap<String, i32> = HashMap::new();
        let mut exclusive_by_objects: HashMap<String, i32> = HashMap::new();
        let mut exclusives_by_token_id: HashMap<String, bool> = HashMap::new();

        // Go over minted exclusive SKUs:
        // Search for exclusive object duplicates
        for lic in matched {
            exclusives_by_token_id.insert(lic.token_id(), true);

            let by_set_exists = exclusive_by_set.contains_key(&lic.object_hash());
            if by_set_exists {
                let msg = format!("Count of {} cannot be greater than 1", l.name);
                return IsAvailableResponse{result: false, reason_not_available: msg, additional_info: None}
            } else {
                exclusive_by_set.insert(lic.object_hash(), 1);
            }

            for object_id in lic.objects() {
                let by_object_exists = exclusive_by_objects.contains_key(&object_id);
                if by_object_exists {
                    // exclusiveByObjects[objectID] += 1
                    let msg = format!("Count of {} for object {} cannot be greater than 1", l.name, object_id);
                    return IsAvailableResponse { result: false, reason_not_available: msg, additional_info: None }
                } else {
                    exclusive_by_objects.insert(object_id, 1);
                }
            }
        }

        let mut remain_to_check: Vec<ShrinkedLicenseToken> = Vec::new();
        for lic in ctx.full.issued_licenses {
            if !exclusives_by_token_id.contains_key(&lic.token_id()) {
                remain_to_check.push(lic.clone())
            }
        }

        for lic in remain_to_check {
            //	if _, ok := exclusiveBySet[lic.ObjectsHash()]; ok {
            //		msg := fmt.Sprintf("Count of %v cannot be greater than 1", l.Name)
            //		return &IsAvailableResponse{Result: false, ReasonNotAvailable: msg}, nil
            //	} else {
            //	}
            //
            for object_id in lic.objects() {
                if exclusive_by_objects.contains_key(&object_id) {
                    let msg = format!("Count of {} for object {} cannot be greater than 1", l.name, object_id);
                    return IsAvailableResponse{result: false, reason_not_available: msg, additional_info: None}
                } else {
                }
            }
        }
        let info = LimitsInfo{
            remains: 1 - count_excl as i32,
            total:   1,
            issued:  count_excl as i32,
            type_:   "exclusive".to_string(),
            scope:   l.scope.clone(),
            name: if l.display_name.is_none() { l.name.clone() } else { l.display_name.as_ref().unwrap().clone() }
        };
        let infos: HashMap<String, LimitsInfo> = vec![("check".to_string(), info)].into_iter().collect();
        IsAvailableResponse{result: true, reason_not_available: "".to_string(), additional_info: Some(infos)}
    }
}

impl Limitation {
    pub fn check(&self, licenses: &Vec<&dyn LicenseGeneral>, ctx: Context) -> IsAvailableResponse {
        let matched_licenses = self.find_all(licenses);
        let checks: Vec<Option<&dyn LimitCheck>> = vec![
            self.max_count.as_ref().map(|x| x as &dyn LimitCheck),
            self.exclusive.as_ref().map(|x| x as &dyn LimitCheck),
        ];

        let mut infos: HashMap<String, LimitsInfo> = HashMap::new();
        for check in checks {
            if check.is_none() {
                continue;
            }
            unsafe {
                let must_check = check.unwrap_unchecked();
                let res = must_check.check(matched_licenses.clone(), self, ctx.clone());
                if !res.result {
                    return res;
                }
                infos.insert(
                    self.name.clone(),
                    res.additional_info.unwrap_unchecked().get("check").unwrap_unchecked().clone()
                );
            }
        }
        return IsAvailableResponse{result: true, reason_not_available: String::new(), additional_info: Some(infos)};
    }

    fn find_all<'a>(&'a self, licenses: &Vec<&'a dyn LicenseGeneral>) -> Vec<&dyn LicenseGeneral> {
        let mut list: Vec<&dyn LicenseGeneral> = Vec::new();
        for license in licenses {
            let result = exec_template(&self.template, *license);
            if result.is_true() {
                list.push(*license);
            }
        }
        list
    }
}

impl ConfigInterface for AllPolicies {
    fn check_transition(
        &self, inventory: FullInventory, old: ShrinkedLicenseToken, new: ShrinkedLicenseToken,
        policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> Result<IsAvailableResponse, String> {
        // Take into account asset licenses with sets
        // and old/new license_token set_id, compare them etc.
        let cloned = self.clone_with_optional(policy_rules, upgrade_rules);
        unsafe {
            // let old_asset_license = inventory.asset.clone().unwrap_or_default().licenses.unwrap_or_default()
            //     .iter().find(|x| x.license_id == old.license_id());

            if old.sku_id() == new.sku_id() {
                // Try to find asset_license with set_id == new.set_id
                let new_asset_licenses = inventory.asset.clone().unwrap_or_default().licenses.unwrap_or_default();
                let new_asset_license = new_asset_licenses.iter().find(
                    |x| x.sku_id.clone().unwrap() == new.sku_id()
                );
                // If not found - then no upgrade.
                if new_asset_license.is_none() {
                    let msg = "No upgrade path with the current set to license '".to_string() + &new.license_title() + "' and id = " + &new.license_id();
                    return Ok(IsAvailableResponse { result: false, reason_not_available: msg, additional_info: None });
                }
            } else {
                // Then try to search policy rule from user-defined
                // Currently not implemented
                let msg = "No upgrade path between different sets.".to_string();
                return Ok(IsAvailableResponse { result: false, reason_not_available: msg, additional_info: None });
            }

            let policy_old = cloned.find_policy(&old.clone())?;
            let policy_new = cloned.find_policy(&new.clone())?;
            let exists = policy_old.has_upgrade_to(policy_new.name.as_ref().unwrap_unchecked().clone());
            if !exists {
                let msg = "No upgrade path to ".to_string() + &policy_new.name.as_ref().unwrap_unchecked().clone();
                return Ok(IsAvailableResponse{result: false, reason_not_available: msg, additional_info: None});
            } else {
                // Check restrictions
                // compute future state
                let future_state = cloned.get_future_state_with_transition(inventory, old, new);

                let ctx = Context{full: future_state.clone()};
                let res = cloned.check_future_state(
                    future_state.issued_licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
                    FutureStateOpt { level: LEVEL_LICENSES.to_string(), ctx },
                );
                return Ok(res);
            }
        }
    }

    fn check_new(
        &self, inventory: FullInventory, new: ShrinkedLicenseToken, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> IsAvailableResponse {
        // For asset_mint, nft_mint, update_licenses and
        // update inventory licenses (metadata).
        let all_limits = self.get_all_limit_rules_from_asset(
            inventory.asset.as_ref().unwrap(), policy_rules,
        );

        let cloned = self.clone_with_optional(Some(all_limits), upgrade_rules);
        let future_state = cloned.get_future_state_with_new(inventory.clone(), new.clone());
        let ctx = Context{full: future_state.clone()};
        let mut available = cloned.check_future_state(
            future_state.issued_licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
            FutureStateOpt { level: LEVEL_LICENSES.to_string(), ctx },
        );

        // Filter exclusive and others not related
        if available.additional_info.is_none() {
            return available
        }
        let mut additional_info = available.additional_info.unwrap();
        let is_exclusive = new.is_exclusive();
        let new_sku_id = new.sku_id();
        let mut min_key_by_type: HashMap<String, (i32, String)> = HashMap::new();
        for (k, v) in additional_info.clone() {
            if v.type_ == "exclusive".to_string() && !is_exclusive {
                additional_info.remove(&k);
            }
            if v.type_ != "exclusive".to_string() && is_exclusive {
                additional_info.remove(&k);
            }

            if &v.scope == "sku" && k != new_sku_id {
                additional_info.remove(&k);
                continue
            }

            if min_key_by_type.contains_key(&v.type_.clone()) {
                let (current, _) = min_key_by_type.get(&v.type_.clone()).unwrap();
                if v.remains.clone() < current.clone() {
                    min_key_by_type.insert(
                        v.type_.clone(),
                        (v.remains.clone(), k)
                    );
                }
            } else {
                min_key_by_type.insert(
                    v.type_.clone(),
                    (v.remains.clone(), k),
                );
            }
        }
        let keys: Vec<&String> = min_key_by_type.iter().map(|(_, (_, key))| key).collect();
        available.additional_info = Some(
            additional_info.into_iter().filter(|(x, _)| keys.contains(&x)).collect()
        );
        available
    }

    fn check_state(&self, licenses: Vec<ShrinkedLicenseToken>) -> IsAvailableResponse {
        let ctx = Context{full: FullInventory{issued_licenses: licenses.clone(), inventory_licenses: Vec::new(), asset: None}};
        self.check_future_state(
            licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
            FutureStateOpt { level: LEVEL_LICENSES.to_string(), ctx },
        )
    }

    fn check_inventory_state(&self, licenses: Vec<InventoryLicense>) -> IsAvailableResponse {
        let ctx = Context{full: FullInventory{issued_licenses: Vec::new(), inventory_licenses: licenses.clone(), asset: None}};
        self.check_future_state(
            licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
            FutureStateOpt { level: LEVEL_INVENTORY.to_string(), ctx },
        )
    }

    fn list_transitions(
        &self, inventory: FullInventory, from: ShrinkedLicenseToken, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> Vec<SKUAvailability> {
        let cloned = self.clone_with_optional(policy_rules, upgrade_rules);
        let mut result: Vec<SKUAvailability> = Vec::new();
        for license in &inventory.inventory_licenses {
            let mut lic_token = license.as_license_token("token".to_string());
            lic_token.metadata.from = Some(SourceLicenseMeta{
                asset_id: from.asset_id.clone(),
                sku_id: Some(from.sku_id()),
                inventory_id: "".to_owned(),
                issuer_id: None,
            });
            let new_lic_token = lic_token.shrink();

            let check_transition_res = cloned.check_transition(
                inventory.clone(), from.clone(), new_lic_token, None, None,
            );

            unsafe {
                let res = if check_transition_res.is_err() {
                    IsAvailableResponse{result: false, reason_not_available: check_transition_res.unwrap_err_unchecked(), additional_info: None}
                } else {
                    check_transition_res.unwrap_unchecked()
                };
                result.push(SKUAvailability {
                    available: res.result,
                    reason_not_available: Some(res.reason_not_available.clone()),
                    asset_license: AssetLicense::default(),
                    upgrade_price: None,
                    additional_info: res.additional_info,
                });
            }
        }
        result
    }

    fn list_available(
        &self, inventory: FullInventory, policy_rules: Option<Vec<Limitation>>,
        upgrade_rules: Option<Vec<Policy>>) -> Vec<SKUAvailability> {

        let all_limits = self.get_all_limit_rules_from_asset(
            inventory.asset.as_ref().unwrap(), policy_rules,
        );

        let cloned = self.clone_with_optional(Some(all_limits), upgrade_rules);
        // Make issued map by sku ID
        let mut issued_map: HashMap<String, i32> = HashMap::new();
        for lic in &inventory.issued_licenses {
            if issued_map.contains_key(&lic.sku_id()) {
                *issued_map.get_mut(&lic.sku_id()).unwrap() += 1;
            } else {
                issued_map.insert(lic.sku_id(), 1);
            }
        }

        // Build inventory license map by license_id
        let mut inventory_licenses: HashMap<String, InventoryLicense> = HashMap::new();
        for lic in &inventory.inventory_licenses {
            inventory_licenses.insert(lic.license_id(), lic.clone());
        }

        let mut available: Vec<SKUAvailability> = Vec::new();

        for asset_license in &inventory.asset.clone().expect("Expect asset in inventory").licenses.unwrap_or_default() {
            // Optional inventory license
            let mut inv_license: Option<InventoryLicense> = None;
            if !asset_license.license_id.clone().unwrap_or_default().is_empty() {
                inv_license = inventory_licenses.get(&asset_license.license_id.clone().unwrap_or_default()).cloned();
            }

            // pretend like we "mint" a new token
            let token = inventory.asset.as_ref().unwrap().issue_new_license(inv_license, asset_license.clone(), "0".to_string()).shrink();
            // check this new token if it is available to mint
            let mut res = cloned.check_new(inventory.clone(), token, None, None);

            if res.additional_info.is_some() {
                unsafe {
                    for (_k, mut v) in res.additional_info.as_mut().unwrap_unchecked() {
                        v.issued = *issued_map.get(&asset_license.sku_id.clone().unwrap()).unwrap_or(&0);
                        v.remains += 1
                    }
                }
            }

            available.push(SKUAvailability {
                available: res.result,
                reason_not_available: Some(res.reason_not_available),
                asset_license: asset_license.clone(),
                upgrade_price: None,
                additional_info: res.additional_info.clone(),
            });
        }
        available
    }

}

impl AllPolicies {
    fn find_policy(&self, from: &dyn LicenseGeneral) -> Result<Policy, String> {
        let mut found: String = String::new();
        for (pol_name, pol) in self.policies.iter() {
            let result = exec_template(&pol.template, from);
            if result.is_true() {
                found = pol_name.clone();
            }
        }
        unsafe {
            if found.len() > 0 {
                Ok(self.policies.get(&found).unwrap_unchecked().clone())
            } else {
                Err("License policy not found for ".to_string() + &from.license_title())
            }
        }
    }

    fn get_all_limit_rules_from_asset(&self, asset: &JsonAssetToken, limit_rules: Option<Vec<Limitation>>) -> Vec<Limitation> {
        let mut all = limit_rules.unwrap_or(Vec::new());
        for sku in asset.licenses.as_ref().unwrap_or(&Vec::new()) {
            if sku.sole_limit.is_some() {
                all.push(max_count_from_sku(sku))
            }
        }
        return all
    }

    pub fn clone_with_additional(&self, mut l: Vec<Limitation>) -> Self {
        let mut policies = self.clone();
        policies.limitations.append(&mut l);
        return policies;
    }
    pub fn clone_with_policies(&self, p: Vec<Policy>) -> Self {
        let mut policies = self.clone();
        for policy in p {
            policies.policies.insert(policy.name.clone().unwrap(), policy.clone());
        }
        return policies;
    }

    pub fn clone_with_optional(&self, l: Option<Vec<Limitation>>, p: Option<Vec<Policy>>) -> Self {
        let mut cloned = self.clone();
        if l.is_some() {
            cloned = cloned.clone_with_additional(l.unwrap());
        }
        if p.is_some() {
            cloned = cloned.clone_with_policies(p.unwrap());
        }
        cloned
    }

    // fn find_policy_set_id(&self, from: &dyn LicenseGeneral, opt: PolicyOpt) -> Result<Policy, String> {
    //     let mut found: String = String::new();
    //     for (pol_name, pol) in self.policies.iter() {
    //         let result = exec_template(&pol.template, from);
    //         let mut bool_res = result.is_true();
    //
    //         // Apply additional constraints
    //         if let Some(ref set_id) = opt.set_id {
    //             let inner = exec_template(&("set_id == ".to_string() + &set_id), from);
    //             bool_res = bool_res && inner.is_true();
    //         }
    //
    //         if bool_res {
    //             found = pol_name.clone();
    //         }
    //     }
    //     unsafe {
    //         if found.len() > 0 {
    //             Ok(self.policies.get(&found).unwrap_unchecked().clone())
    //         } else {
    //             Err(format!("License policy not found for {}", from.license_title()))
    //         }
    //     }
    // }

    pub unsafe fn get_future_state_with_transition(&self, inventory: FullInventory, old: ShrinkedLicenseToken, new: ShrinkedLicenseToken) -> FullInventory {
        let mut future_state = inventory.clone();
        for token in future_state.issued_licenses.iter_mut() {
            if token.token_id == old.token_id {
                let (inv_id, asset_id, lic_id, _sku) = token.inventory_asset_license_sku();
                if token.license.is_some() {
                    token.license.as_mut().unwrap_unchecked().metadata = new.license.clone().unwrap_unchecked().metadata;
                    // token.license.as_mut().unwrap_unchecked().title = Some(new.license_title());
                    token.license.as_mut().unwrap_unchecked().id = lic_id.clone();
                    // token.license.as_mut().unwrap_unchecked().from.inventory_id = inv_id.clone();
                    // token.license.as_mut().unwrap_unchecked().from.asset_id = asset_id.clone();
                    // token.license.as_mut().unwrap_unchecked().from.sku_id = Some(new.sku_id());
                    // token.license.as_mut().unwrap_unchecked().from.set_id = new.set_id();
                }
                token.metadata.from.as_mut().unwrap_unchecked().inventory_id = inv_id.clone();
                token.metadata.from.as_mut().unwrap_unchecked().asset_id = asset_id.clone();
                token.metadata.from.as_mut().unwrap_unchecked().sku_id = Some(new.sku_id());
            }
        }
        // future_state.issued_licenses.push(new);
        future_state
    }

    pub fn get_future_state_with_new(&self, inventory: FullInventory, new: ShrinkedLicenseToken) -> FullInventory {
        let mut future_state = inventory.clone();
        future_state.issued_licenses.push(new);
        future_state
    }

    pub fn check_future_state(&self, licenses: Vec<&dyn LicenseGeneral>, opt: FutureStateOpt) -> IsAvailableResponse {
        let mut infos: HashMap<String, LimitsInfo> = HashMap::new();
        for l in &self.limitations {
            if l.level != opt.level {
                continue;
            }
            let res = l.check(&licenses, opt.ctx.clone());
            if !res.result {
                return res;
            }
            unsafe {
                for (k, v) in res.additional_info.unwrap_unchecked() {
                    infos.insert(k, v);
                }
            }
        }
        return IsAvailableResponse{result: true, reason_not_available: String::new(), additional_info: Some(infos)};
    }

    pub fn filter_by_limits(&self, result: IsAvailableResponse, new: &dyn LicenseGeneral) -> IsAvailableResponse {
        if result.additional_info.is_none() {
            return result
        }
        let mut add_info = unsafe {result.additional_info.unwrap_unchecked()};

        let new_as_general = vec![new];
        for l in &self.limitations {
            let matched = l.find_all(&new_as_general);
            if matched.len() != 1 {
                add_info.remove(&l.name);
            }
        }
        // Filter by minimum remains
        let (min_name, min) = unsafe { add_info.iter().min_by_key(
                |&(_k, v)| v.remains
            ).unwrap_unchecked()
        };
        let new_result = IsAvailableResponse {
            additional_info: Some(
                vec![(min_name.to_string(), min.clone())].into_iter().collect()
            ),
            result: result.result,
            reason_not_available: result.reason_not_available,
        };
        return new_result
    }
}

pub fn max_count_from_sku(sku: &AssetLicense) -> Limitation {
    let limit = Limitation{
        level: LEVEL_LICENSES.to_string(),
        name:  sku.sku_id.clone().unwrap_or_default(),
        display_name:  Some(sku.title.clone()),
        scope: "sku".to_string(),
        exclusive: None,
        max_count: Some(MaxCount{
            count: sku.sole_limit.unwrap(),
        }),
        template: format!("sku_id == '{}'", &sku.sku_id.as_ref().expect("Sku ID null")),
    };
    limit
}

pub fn exec_template(template_str: &String, object: &dyn LicenseGeneral) -> Value {
    let env = minijinja::Environment::new();
    // env.add_template("tpl", &template_str).expect("Failed to add template");

    unsafe {
        let expr = env.compile_expression(&template_str).unwrap();
        // let context = minijinja::context!(
        //     is_personal => object.is_personal(),
        //     is_commercial => object.is_commercial(),
        //     is_exclusive => object.is_exclusive(),
        // );
        let mut context = BTreeMap::default();
        context.insert("is_personal", Value::from_serializable(&object.is_personal()));
        context.insert("is_commercial", Value::from_serializable(&object.is_commercial()));
        context.insert("is_exclusive", Value::from_serializable(&object.is_exclusive()));
        context.insert("sku_id", Value::from_serializable(&object.sku_id()));
        context.insert("token_id", Value::from_serializable(&object.token_id()));
        context.insert("license_id", Value::from_serializable(&object.license_id()));
        context.insert("license_title", Value::from_serializable(&object.license_title()));

        let result = expr.eval(context).unwrap_unchecked();
        result
    }
}