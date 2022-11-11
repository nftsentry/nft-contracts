#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::AccountId;
    use crate::policy::{init_policies, Limitation, MaxCount};
    use crate::policy::{ConfigInterface, LEVEL_LICENSES};
    use crate::utils::{balance_from_string, format_balance};
    use crate::types::{AssetLicense, FullInventory, InventoryLicense, JsonAssetToken, LicenseData, TokenMetadata};

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
        let avail = res.unwrap();
        assert_eq!(avail.result, true);
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

        let res = policies.check_new(
            inventory, personal_token
        );
        assert_eq!(res.result, false);
        assert_eq!(res.reason_not_available.contains("There can be no other licenses"), true);
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

        let res = policies.check_inventory_state(
            inventory.inventory_licenses
        );
        assert_eq!(res.result, true);

        let inventory2 = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
        };

        let res2 = policies.check_inventory_state(
            inventory2.inventory_licenses
        );
        assert_eq!(res2.result, false);
        assert_eq!(res2.reason_not_available.contains("max count 1"), true)
    }

    #[test]
    fn test_check_new_limitation_count() {
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
        let personal_token2 = personal.as_license_token("2".to_string());
        let personal_token3 = personal.as_license_token("3".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_token.clone(), personal_token2.clone(), personal_token3.clone()],
        };

        let new_limits = vec![Limitation{
            exclusive: None,
            max_count: Some(MaxCount{count: 3}),
            template: "true".to_string(),
            name: "3count".to_string(),
            level: LEVEL_LICENSES.to_string(),
        }];

        let res = policies.clone_with_additional(new_limits.clone()).check_new(
            inventory, personal_token.clone()
        );
        assert_eq!(res.result, false);
        assert_eq!(res.reason_not_available.contains("Cannot set more 3count: max count 3"), true);


        let inventory2 = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_token.clone(), personal_token2.clone()],
        };

        let res = policies.clone_with_additional(new_limits.clone()).check_new(
            inventory2.clone(), personal_token.clone()
        );
        assert_eq!(res.result, true);
        let add_info = res.additional_info.unwrap();
        let limit_info = add_info.get("3count").expect("3count must be filled");
        assert_eq!(limit_info.remains == 0, true);

        let res = policies.clone_with_additional(new_limits.clone()).list_available(
            inventory2
        );
        let count3pers = res[0].additional_info.as_ref().unwrap().get("3count").unwrap();
        let count3comm = res[1].additional_info.as_ref().unwrap().get("3count").unwrap();

        assert_eq!(count3pers.remains == 1, true);
        assert_eq!(count3pers.issued == 2, true);
        assert_eq!(count3comm.remains == 1, true);
        assert_eq!(count3comm.issued == 0, true);
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

    #[test]
    fn test_issue_new_metadata() {
        let mut json_asset = JsonAssetToken{
            metadata: TokenMetadata{
                title: None,
                description: None,
                media: None,
                previews: Some("preview".to_string()),
                object: Some("{\"items\":
                [
                  {\"id\": \"1\", \"type\": \"image\"},
                  {\"id\": \"2\", \"type\": \"video\"},
                  {\"id\": \"3\", \"type\": \"model\"}
                ]}".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None
            },
            licenses: Some(vec![
                AssetLicense{
                    objects: Some(
                        vec!["1".to_string(), "2".to_string(), "3".to_string()]
                    ),
                    license_id: "id1".to_string(),
                    price: None,
                    title: "id1 title".to_string(),
                },
                AssetLicense{
                    objects: Some(
                        vec!["4".to_string(), "3".to_string(), "2".to_string()]
                    ),
                    license_id: "id2".to_string(),
                    price: None,
                    title: "id2 title".to_string(),
                },
            ]),
            license_token_count: 2,
            token_id: "asset_normal".to_string(),
            owner_id: AccountId::new_unchecked("rocketscience".to_string()),
            policy_rules: None,
            minter_id: AccountId::new_unchecked("license_rocketscience".to_string()),
        };

        let new_meta = json_asset.issue_new_metadata(vec!["1".to_string(), "2".to_string()]);

        assert_eq!(new_meta.object.clone().unwrap().contains("\"1\""), true);
        assert_eq!(new_meta.object.clone().unwrap().contains("\"2\""), true);
        assert_eq!(new_meta.object.clone().unwrap().contains("\"3\""), false);
        assert_eq!(new_meta.object.clone().unwrap().contains("\"4\""), false);

        json_asset.metadata.object = Some("".to_string());
        let new_meta = json_asset.issue_new_metadata(Vec::default());

        assert_eq!(new_meta.object.unwrap().is_empty(), true);
        // println!("{}", serde_json::to_string(&new_meta.object).unwrap())
    }
}