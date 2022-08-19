#[cfg(test)]
mod tests {
    use crate::policy::{CONFIG, init_policies};
    use crate::policy::ConfigInterface;
    use crate::types::{FullInventory, InventoryLicense, LicenseData};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_init_policies() {
        let _policies = init_policies();
        assert_eq!(CONFIG.try_lock().is_err(), false);
        assert_eq!(CONFIG.lock().unwrap().version, "0.0.1");
        assert_eq!(CONFIG.lock().unwrap().limitations.len(), 3);
        assert_eq!(CONFIG.lock().unwrap().policies.len(), 4);
    }

    #[test]
    fn test_check_transition() {
        let _policies = init_policies();

        let old_l = InventoryLicense{
            title: "lic1".to_string(),
            price: "1".to_string(),
            license_id: "lic_id".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: false,
                commercial_use: false,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let new_l = InventoryLicense{
            title: "lic2".to_string(),
            price: "1".to_string(),
            license_id: "lic_id2".to_string(),
            license: LicenseData{
                personal_use: false,
                exclusivity: false,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let old_token = old_l.as_license_token("1".to_string()).expect("");
        let inventory = FullInventory{
            inventory_licenses: vec![old_l.clone(), new_l.clone()],
            issued_licenses:    vec![old_token.clone()],
        };

        let res = CONFIG.lock().unwrap().check_transition(
            inventory, old_token, new_l
        );
        assert_eq!(res.clone().err(), None);
        assert_eq!(res.clone().ok().unwrap(), (true, "".to_string()));
    }

    #[test]
    fn test_list_transitions() {
        let _policies = init_policies();

        let personal = InventoryLicense{
            title: "lic1".to_string(),
            price: "1".to_string(),
            license_id: "lic_id".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: false,
                commercial_use: false,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let commercial = InventoryLicense{
            title: "lic2".to_string(),
            price: "1".to_string(),
            license_id: "lic_id2".to_string(),
            license: LicenseData{
                personal_use: false,
                exclusivity: false,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_exclusive = InventoryLicense{
            title: "lic3".to_string(),
            price: "1".to_string(),
            license_id: "lic_id3".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: true,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_token = personal.as_license_token("1".to_string()).expect("");
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_token.clone()],
        };

        let available = CONFIG.lock().unwrap().list_transitions(
            inventory, personal_token
        );
        let count = available.iter().filter(|x| x.available).count();
        assert_eq!(available.len(), 3);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_list_transitions_has_exclusive() {
        let _policies = init_policies();

        let personal = InventoryLicense{
            title: "lic1".to_string(),
            price: "1".to_string(),
            license_id: "lic_id".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: false,
                commercial_use: false,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let commercial = InventoryLicense{
            title: "lic2".to_string(),
            price: "1".to_string(),
            license_id: "lic_id2".to_string(),
            license: LicenseData{
                personal_use: false,
                exclusivity: false,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_exclusive = InventoryLicense{
            title: "lic3".to_string(),
            price: "1".to_string(),
            license_id: "lic_id3".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: true,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_exclusive_token = personal_exclusive.as_license_token("1".to_string()).expect("");
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let available = CONFIG.lock().unwrap().list_transitions(
            inventory, personal_exclusive_token
        );
        let count = available.iter().filter(|x| x.available).count();
        assert_eq!(available.len(), 3);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_check_new_exclusive() {
        let _policies = init_policies();

        let personal = InventoryLicense{
            title: "lic1".to_string(),
            price: "1".to_string(),
            license_id: "lic_id".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: false,
                commercial_use: false,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let commercial = InventoryLicense{
            title: "lic2".to_string(),
            price: "1".to_string(),
            license_id: "lic_id2".to_string(),
            license: LicenseData{
                personal_use: false,
                exclusivity: false,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_exclusive = InventoryLicense{
            title: "lic3".to_string(),
            price: "1".to_string(),
            license_id: "lic_id3".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: true,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_token = personal.as_license_token("1".to_string()).expect("");
        let personal_exclusive_token = personal_exclusive.as_license_token("1".to_string()).expect("");
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let (res, reason) = CONFIG.lock().unwrap().check_new(
            inventory, personal_token
        );
        assert_eq!(res, false);
        assert_eq!(reason.contains("There can be no other licenses"), true);
    }

    #[test]
    fn test_check_inventory_state() {
        let _policies = init_policies();

        let personal = InventoryLicense{
            title: "lic1".to_string(),
            price: "1".to_string(),
            license_id: "lic_id".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: false,
                commercial_use: false,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let commercial = InventoryLicense{
            title: "lic2".to_string(),
            price: "1".to_string(),
            license_id: "lic_id2".to_string(),
            license: LicenseData{
                personal_use: false,
                exclusivity: false,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_exclusive = InventoryLicense{
            title: "lic3".to_string(),
            price: "1".to_string(),
            license_id: "lic_id3".to_string(),
            license: LicenseData{
                personal_use: true,
                exclusivity: true,
                commercial_use: true,
                i_agree: true,
                limited_display_sublicensee: false,
                template: None,
                perpetuity: true,
                pdf_url: None
            }
        };
        let personal_exclusive_token = personal_exclusive.as_license_token("1".to_string()).expect("");
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let (res, reason) = CONFIG.lock().unwrap().check_inventory_state(
            inventory.inventory_licenses
        );
        assert_eq!(res, true);
    }
}