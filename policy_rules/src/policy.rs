use std::borrow::Borrow;
use crate::*;
use crate::types::*;
use std::collections::HashMap;
use std::ops::Deref;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
use minijinja::value::Value;

pub trait ConfigInterface {
    fn check_transition(old: InventoryLicense, new: InventoryLicense) -> (bool, String);
    fn check_new(inventory: InventoryContractMetadata, new: InventoryLicense, opt: CheckNewOpt) -> (bool, String);
    fn check_state(inventory: InventoryContractMetadata, opt: CheckNewOpt) -> (bool, String);
    fn check_inventory_state(licenses: Vec<InventoryLicense>) -> (bool, String);
    fn list_transitions(inventory: InventoryContractMetadata, from: InventoryLicense) -> Vec<InventoryLicenseAvailability>;
    fn list_available(inventory: FullInventory) -> Vec<InventoryLicenseAvailability>;
}

lazy_static! {
    static ref POLICIES_RAW: Mutex<Vec<u8>> = Mutex::new(include_bytes!("rules.yaml").to_vec());
    pub static ref CONFIG: Mutex<AllPolicies> = Mutex::new(AllPolicies::default());
}

pub fn init_policies() -> AllPolicies {
    // println!("{:?}", CONFIG.is_poisoned());
    let raw = POLICIES_RAW.lock().unwrap();
    let mut config: AllPolicies = serde_yaml::from_slice(raw.as_slice()).expect("Fail to parse rules.yaml");

    for (policy_name, pol) in &mut config.policies {
        // config.policies.get_mut(policy_name.as_str()).unwrap().name = Some(policy_name.clone());
        pol.name = Some(policy_name.clone());
    }

    CONFIG.lock().unwrap().limitations = config.limitations;
    CONFIG.lock().unwrap().policies = config.policies;
    CONFIG.lock().unwrap().version = config.version;

    CONFIG.lock().unwrap().clone()
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
    pub fn check(&self, licenses: Vec<&dyn LicenseGeneral>) -> (bool, String) {
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

    fn find_all<'a>(&'a self, licenses: Vec<&'a dyn LicenseGeneral>) -> Vec<&dyn LicenseGeneral> {
        let mut list: Vec<&dyn LicenseGeneral> = Vec::new();
        for license in licenses {
            let result = exec_template(&self.template, license);
            if result.is_true() {
                list.push(license);
            }
        }
        list
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

    pub fn check_transition(&self, inventory: FullInventory, old: LicenseToken, new: InventoryLicense) -> Result<(bool, String), String> {
        let old_inv_lic = old.as_inventory_license(None);
        if old_inv_lic.is_none() {
            return Err("Failed old.as_inventory_license()".to_string())
        }
        let policy_old = self.find_policy(old_inv_lic.unwrap())?;
        let policy_new = self.find_policy(new)?;
        let exists = policy_old.has_upgrade_to(policy_new.name.as_ref().unwrap().clone());
        if !exists {
            return Err(format!("No upgrade path to {}", policy_new.name.as_ref().unwrap()))
        } else {
            // check future state
            return Ok((true, "".to_string()))
        }

    }
}

pub fn exec_template(template_str: &String, object: &dyn LicenseGeneral) -> Value {
    let mut env = minijinja::Environment::new();
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