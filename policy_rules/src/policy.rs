use crate::*;
use crate::types::*;
use std::collections::HashMap;
use minijinja::value::Value;

const LEVEL_INVENTORY: &str = "inventory";
const LEVEL_LICENSES: &str  = "licenses";

pub trait ConfigInterface {
    fn check_transition(&self, inventory: FullInventory, old: LicenseToken, new: InventoryLicense) -> Result<(bool, String), String>;
    fn check_new(&self, inventory: FullInventory, new: LicenseToken) -> (bool, String);
    fn check_state(&self, licenses: Vec<LicenseToken>) -> (bool, String);
    fn check_inventory_state(&self, licenses: Vec<InventoryLicense>) -> (bool, String);
    fn list_transitions(&self, inventory: FullInventory, from: LicenseToken) -> Vec<InventoryLicenseAvailability>;
    fn list_available(&self, inventory: FullInventory) -> Vec<InventoryLicenseAvailability>;
}

pub fn init_policies() -> AllPolicies {
    let raw = include_bytes!("rules.json").to_vec();
    let mut config: AllPolicies = serde_json::from_slice(raw.as_slice()).expect("Fail to parse rules.yaml");

    for (policy_name, pol) in &mut config.policies {
        // config.policies.get_mut(policy_name.as_str()).unwrap().name = Some(policy_name.clone());
        pol.name = Some(policy_name.clone());
    }

    config
    // CONFIG.lock().unwrap().limitations = config.limitations;
    // CONFIG.lock().unwrap().policies = config.policies;
    // CONFIG.lock().unwrap().version = config.version;
    //
    // CONFIG.lock().unwrap().clone()
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AllPolicies {
    pub version:     String,
    pub policies:    HashMap<String, Policy>,
    pub limitations: Vec<Limitation>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub name:       Option<String>,
    pub template:   String,
    pub upgrade_to: Vec<String>,
}

impl Policy {
    pub fn has_upgrade_to(&self, policy_name: String) -> bool {
        self.upgrade_to.contains(&policy_name)
    }
}

pub trait LimitCheck {
    fn check(&self, matched: Vec<&dyn LicenseGeneral>, l: &Limitation) -> (bool, String);
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct  CheckNewOpt {
    in_assets:   Option<bool>,
    in_licenses: Option<bool>,
    token_id:    Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FutureStateOpt {
    pub level: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Limitation {
    pub name:      String,
    pub level:     String,
    pub template:  String,
    pub max_count: Option<MaxCount>,
    pub exclusive: Option<Exclusive>,
    // Add another limit types
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MaxCount {
    pub count: i32,
}

impl LimitCheck for MaxCount {
    fn check(&self, matched: Vec<&dyn LicenseGeneral>, l: &Limitation) -> (bool, String) {
        if matched.len() > self.count as usize {
            let msg = format!(
                "Cannot upgrade to {}: Max count {}",
                l.name, self.count,
            );
            (false, msg)
        } else {
            (true, "".to_string())
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Exclusive {
    pub template: String,
}

impl LimitCheck for Exclusive {
    fn check(&self, all: Vec<&dyn LicenseGeneral>, l: &Limitation) -> (bool, String) {
        let mut count_excl = 0;
        for lic in &all {
            let res = exec_template(&self.template, *lic);
            if res.is_true() {
                count_excl += 1;
            }
        }
        if count_excl > 1 {
            return (false, format!("Count of {} cannot be greater than 1", l.name));
        }
        if count_excl == 1 && all.len() != count_excl {
            return (false, format!("There can be no other licenses, except for {} one.", l.name));
        }
        (true, "".to_string())
    }
}

impl Limitation {
    pub fn check(&self, licenses: &Vec<&dyn LicenseGeneral>) -> (bool, String) {
        let matched_licenses= self.find_all(licenses);
        let checks: Vec<Option<&dyn LimitCheck>> = vec![
            self.max_count.as_ref().map(|x| x as &dyn LimitCheck),
            self.exclusive.as_ref().map(|x| x as &dyn LimitCheck)
        ];

        for check in checks {
            if check.is_none() {
                continue
            }
            let must_check = check.unwrap();
            let (ok, reason) = must_check.check(matched_licenses.clone(), self);
            if !ok {
                return (ok, reason)
            }
        }
        return (true, "".to_string())
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
    fn check_transition(&self, inventory: FullInventory, old: LicenseToken, new: InventoryLicense) -> Result<(bool, String), String> {
        let old_inv_lic = old.as_inventory_license(None);
        if old_inv_lic.is_none() {
            return Err("Failed old.as_inventory_license()".to_string())
        }
        let policy_old = self.find_policy(old_inv_lic.unwrap())?;
        let policy_new = self.find_policy(new.clone())?;
        let exists = policy_old.has_upgrade_to(policy_new.name.as_ref().unwrap().clone());
        if !exists {
            return Ok((false, format!("No upgrade path to {}", policy_new.name.as_ref().unwrap())))
        } else {
            // Check restrictions
            // compute future state
            let future_state = self.get_future_state_with_transition(inventory, old, new);

            let (ok, reason) = self.check_future_state(
                future_state.issued_licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
                FutureStateOpt{level: LEVEL_LICENSES.to_string()}
            );
            return Ok((ok, reason))
        }
    }

    fn list_transitions(&self, inventory: FullInventory, from: LicenseToken) -> Vec<InventoryLicenseAvailability> {
        let mut res: Vec<InventoryLicenseAvailability> = Vec::new();
        for license in &inventory.inventory_licenses {
            let check_transition_res = self.check_transition(inventory.clone(), from.clone(), license.clone());

            let (can_upgrade, reason) = if check_transition_res.is_err() {
                (false, check_transition_res.unwrap_err())
            } else {
                check_transition_res.ok().unwrap()
            };
            res.push(InventoryLicenseAvailability{
                available: can_upgrade,
                reason_not_available: Some(reason.clone()),
                inventory_license: license.clone(),
            });
        }
        res
    }

    fn check_new(&self, inventory: FullInventory, new: LicenseToken) -> (bool, String) {
        // For asset_mint, nft_mint, update_licenses and
        // update inventory metadata (license list).
        let future_state = self.get_future_state_with_new(inventory.clone(), new.clone());
        self.check_future_state(
            future_state.issued_licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
            FutureStateOpt{level: LEVEL_LICENSES.to_string()}
        )
    }

    fn list_available(&self, inventory: FullInventory) -> Vec<InventoryLicenseAvailability> {
        let mut available: Vec<InventoryLicenseAvailability> = Vec::new();
        for inv_license in &inventory.inventory_licenses {
            let token = inv_license.as_license_token("0".to_string()).unwrap();
            let (res, reason) = self.check_new(inventory.clone(), token);
            available.push(InventoryLicenseAvailability{
                available: res,
                reason_not_available: Some(reason),
                inventory_license: inv_license.clone(),
            });
        }
        available
    }

    fn check_state(&self, licenses: Vec<LicenseToken>) -> (bool, String) {
        self.check_future_state(
            licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
            FutureStateOpt{level: LEVEL_LICENSES.to_string()},
        )
    }

    fn check_inventory_state(&self, licenses: Vec<InventoryLicense>) -> (bool, String) {
        self.check_future_state(
            licenses.iter().map(|x| x as &dyn LicenseGeneral).collect(),
            FutureStateOpt{level: LEVEL_INVENTORY.to_string()},
        )
    }
}

impl AllPolicies {
    fn find_policy(&self, from: InventoryLicense) -> Result<Policy, String> {
        let mut found: Option<&str> = None;
        for (pol_name, pol) in self.policies.iter() {
            let result = exec_template(&pol.template, &from);
            if result.is_true() {
                found = Some(pol_name.as_str());
            }
        }
        if found.is_some() {
            Ok(self.policies.get(found.unwrap()).unwrap().clone())
        } else {
            Err(format!("License policy not found for {}", from.title))
        }
    }

    pub fn get_future_state_with_transition(&self, inventory: FullInventory, old: LicenseToken, new: InventoryLicense) -> FullInventory {
        let mut future_state = inventory.clone();
        for token in future_state.issued_licenses.iter_mut() {
            if token.token_id == old.token_id {
                let meta = serde_json::to_string(&new.license.clone()).unwrap();
                let (inv_id, asset_id, _) = token.metadata.inventory_asset_license();
                token.license.as_mut().unwrap().metadata = Some(meta);
                token.license.as_mut().unwrap().title = Some(new.title.clone());
                token.metadata.extra = Some(extra_reference_for_asset_path(
                    token.metadata.extra.clone().unwrap_or("".to_string()),
                    inv_id, asset_id, new.license_id.clone(),
                ));
            }
        }
        // future_state.issued_licenses.push(new);
        future_state
    }

    pub fn get_future_state_with_new(&self, inventory: FullInventory, new: LicenseToken) -> FullInventory {
        let mut future_state = inventory.clone();
        future_state.issued_licenses.push(new);
        future_state
    }

    pub fn check_future_state(&self, licenses: Vec<&dyn LicenseGeneral>, opt: FutureStateOpt) -> (bool, String) {
        for l in &self.limitations {
            if l.level != opt.level {
                continue
            }
            let (ok, reason) = l.check(&licenses);
            if !ok {
                return (ok, reason)
            }
        }
        return (true, "".to_string())
    }
}

pub fn exec_template(template_str: &String, object: &dyn LicenseGeneral) -> Value {
    let env = minijinja::Environment::new();
    // env.add_template("tpl", &template_str).expect("Failed to add template");

    let expr = env.compile_expression(&template_str).expect("Failed parse expression");
    let context = minijinja::context!(
        is_personal => object.is_personal(),
        is_commercial => object.is_commercial(),
        is_exclusive => object.is_exclusive(),
    );

    let result = expr.eval(context).unwrap();
    result
}