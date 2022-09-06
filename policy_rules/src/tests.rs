#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::policy::{init_policies};
    use crate::policy::ConfigInterface;
    use crate::utils::{balance_from_string, format_balance};
    use crate::types::{FullInventory, InventoryLicense, LicenseData};

    #[test]
    fn test_init_policies() {
        let _policies = init_policies();
        assert_eq!(_policies.version, "0.0.1");
        assert_eq!(_policies.limitations.len(), 3);
        assert_eq!(_policies.policies.len(), 4);
    }

    #[test]
    fn test_check_transition() {
        let policies = init_policies();

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
        let old_token = old_l.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![old_l.clone(), new_l.clone()],
            issued_licenses:    vec![old_token.clone()],
        };

        let res = policies.check_transition(
            inventory, old_token, new_l
        );
        assert_eq!(res.clone().err(), None);
        assert_eq!(res.clone().ok().unwrap(), (true, "".to_string()));
    }

    #[test]
    fn test_list_transitions() {
        let policies = init_policies();

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
        let personal_token = personal.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_token.clone()],
        };

        let available = policies.list_transitions(
            inventory, personal_token
        );
        let count = available.iter().filter(|x| x.available).count();
        assert_eq!(available.len(), 3);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_list_transitions_has_exclusive() {
        let policies = init_policies();

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
        let personal_exclusive_token = personal_exclusive.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let available = policies.list_transitions(
            inventory, personal_exclusive_token
        );
        let count = available.iter().filter(|x| x.available).count();
        assert_eq!(available.len(), 3);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_check_new_exclusive() {
        let policies = init_policies();

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
        let personal_token = personal.as_license_token("1".to_string());
        let personal_exclusive_token = personal_exclusive.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let (res, reason) = policies.check_new(
            inventory, personal_token
        );
        assert_eq!(res, false);
        assert_eq!(reason.contains("There can be no other licenses"), true);
    }

    #[test]
    fn test_check_inventory_state() {
        let policies = init_policies();

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
        let personal_exclusive_token = personal_exclusive.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let (res, _reason) = policies.check_inventory_state(
            inventory.inventory_licenses
        );
        assert_eq!(res, true);

        let inventory2 = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let (res2, reason2) = policies.check_inventory_state(
            inventory2.inventory_licenses
        );
        assert_eq!(res2, false);
        assert_eq!(reason2.contains("max count 1"), true)
    }

    #[test]
    fn test_balance_from_string() {
        let price1 = "0.3".to_string();
        let price2 = "0.1".to_string();
        let price3 = "1".to_string();
        let price4 = "0.0002".to_string();
        let price5 = "155.242".to_string();

        let balance1 = balance_from_string(price1.clone());
        let balance2 = balance_from_string(price2.clone());
        let balance3 = balance_from_string(price3.clone());
        let balance4 = balance_from_string(price4.clone());
        let balance5 = balance_from_string(price5.clone());
        assert_eq!(balance1, 300000000000000000000000);
        assert_eq!(balance2, 100000000000000000000000);
        assert_eq!(balance3, 1000000000000000000000000);
        assert_eq!(balance4, 200000000000000000000);
        assert_eq!(balance5, 155242000000000000000000000);

        assert_eq!(format_balance(balance1), price1);
        assert_eq!(format_balance(balance2), price2);
        assert_eq!(format_balance(balance3), price3);
        assert_eq!(format_balance(balance4), price4);
        assert_eq!(format_balance(balance5), price5);
    }
}