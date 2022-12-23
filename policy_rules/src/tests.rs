#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::AccountId;
    use crate::policy::{init_policies, Limitation, MaxCount};
    use crate::policy::{ConfigInterface, LEVEL_LICENSES};
    use crate::prices::Price;
    use crate::utils::{balance_from_string, format_balance, get_inventory_id};
    use crate::types::{AssetLicense, FullInventory, InventoryLicense, JsonAssetToken, LicenseData, TokenMetadata};

    #[test]
    fn test_init_policies() {
        let _policies = init_policies();
        assert_eq!(_policies.version, "0.0.1");
        assert_eq!(_policies.limitations.len(), 3);
        assert_eq!(_policies.policies.len(), 4);
    }

    fn asset_license(sku_id: &str, license_id: &str, title: &str) -> AssetLicense {
        AssetLicense{
            sku_id: Some(sku_id.to_string()),
            objects: None,
            params: None,
            set_id: None,
            license_id: Some(license_id.to_string()),
            price: None,
            title: title.to_string(),
        }
    }

    fn sample_asset_token() -> JsonAssetToken {
        JsonAssetToken{
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
                ],
                  \"sets\": [
                    {\"id\": \"set1\", \"objects\": [\"1\",\"2\"]},
                    {\"id\": \"set2\", \"objects\": [\"2\",\"3\"]}
                  ]
                }".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
                from: None,
                sku_data: None,
            },
            licenses: Some(vec![
                asset_license("set1", "id1", "title1"),
                asset_license("set2", "id1", "title1"),
                asset_license("set1", "id2", "title2"),
                asset_license("set2", "id2", "title2"),
            ]),
            license_token_count: 2,
            token_id: "asset_normal".to_string(),
            owner_id: AccountId::new_unchecked("rocketscience".to_string()),
            minter_id: AccountId::new_unchecked("license_rocketscience".to_string()),
            policy_rules: None,
            upgrade_rules: None,
        }
    }

    #[test]
    fn test_check_transition() {
        let policies = init_policies();

        let old_l = InventoryLicense{
            title: "lic1".to_string(),
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
        let mut asset_token = sample_asset_token();
        asset_token.metadata.object = None;
        asset_token.licenses = Some(vec![
            asset_license("set_id", "lic_id", "lic1"),
            asset_license("set_id", "lic_id2", "lic2"),
        ]);
        let new_lic_token = new_l.as_license_token("token".to_string());
        let old_token = old_l.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![old_l.clone(), new_l.clone()],
            issued_licenses:    vec![old_token.clone()],
            asset: Some(asset_token),
        };

        let res = policies.check_transition(
            inventory, old_token, new_lic_token
        );
        assert_eq!(res.clone().err(), None);
        let avail = res.unwrap();
        assert_eq!(avail.result, true);
    }

    #[test]
    fn test_check_transition_mul_sets() {
        let policies = init_policies();

        let mut json_asset = sample_asset_token();

        let old_l = InventoryLicense{
            title: "title1".to_string(),
            price: Some("1".to_string()),
            license_id: "id1".to_string(),
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
            title: "title2".to_string(),
            price: Some("1".to_string()),
            license_id: "id2".to_string(),
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

        json_asset.licenses = Some(vec![
            asset_license("sku1", "id1", "title1"),
            asset_license("sku2", "id1", "title1"),
            asset_license("sku3", "id2", "title2"),
            asset_license("sku4", "id2", "title2"),
        ]);

        let licenses = json_asset.licenses.clone().unwrap();
        let mut old_token = old_l.as_license_token("1".to_string());
        old_token.metadata = json_asset.issue_new_metadata(licenses[0].clone());

        // Simulate upgrade of the same token to another set
        let mut new_lic_token = new_l.as_license_token("1".to_string());
        new_lic_token.metadata = json_asset.issue_new_metadata(licenses[1].clone());

        let inventory = FullInventory{
            inventory_licenses: vec![old_l.clone(), new_l.clone()],
            issued_licenses:    vec![old_token.clone()],
            asset: Some(json_asset.clone()),
        };

        let res = policies.check_transition(
            inventory, old_token, new_lic_token
        );
        assert_eq!(res.clone().err(), None);
        let avail = res.unwrap();
        assert_eq!(avail.result, false);
        assert_eq!(avail.reason_not_available.contains("between different sets"), true);
    }

    #[test]
    fn test_list_transitions() {
        let policies = init_policies();

        let personal = InventoryLicense{
            title: "lic1".to_string(),
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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

        let mut asset_token = sample_asset_token();
        asset_token.metadata.object = None;
        asset_token.licenses = Some(vec![
            asset_license("set_id", "lic_id", "lic1"),
            asset_license("set_id", "lic_id2", "lic2"),
            asset_license("set_id", "lic_id3", "lic3"),
        ]);

        let personal_token = personal.as_license_token("1".to_string());
        let inventory = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_token.clone()],
            asset: Some(asset_token),
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            asset: None,
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            asset: None,
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            asset: None,
        };

        let res = policies.check_inventory_state(
            inventory.inventory_licenses
        );
        assert_eq!(res.result, true);

        let inventory2 = FullInventory{
            inventory_licenses: vec![personal.clone(), commercial.clone(), personal_exclusive.clone(), personal_exclusive.clone()],
            issued_licenses:    vec![personal_exclusive_token.clone()],
            asset: None,
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            price: Some("1".to_string()),
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
            asset: None,
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
            asset: None,
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
    fn test_price_string() {
        let price_near = Price{
            multiplier: 13564.to_string(),
            decimals: 28,
        };
        let str = price_near.string_price();

        // let price_btc = Price {
        //     multiplier: 1664197.to_string(),
        //     decimals: 10,
        // };
        // let str_btc = price_btc.string_price();

        assert_eq!(str, "1.3564".to_string());
        // assert_eq!(str_btc, "16641.97".to_string());
    }

    #[test]
    fn test_get_inventory_id() {
        let some = "license_i45.rocketscience.testnet";
        let inv_id = get_inventory_id(some.to_string());
        assert_eq!(inv_id, "i45.rocketscience.testnet".to_string());

        let some2 = "license_i45_awesome.rocketscience.testnet";
        let inv_id2 = get_inventory_id(some2.to_string());
        assert_eq!(inv_id2, "i45_awesome.rocketscience.testnet".to_string());
    }

    #[test]
    fn test_migrate_to_sku() {
        let mut json_asset = JsonAssetToken{
            metadata: TokenMetadata{
                title: None,
                description: None,
                media: None,
                previews: Some("preview".to_string()),
                object: Some("{
                  \"items\":[
                    {\"id\":\"ba1117f1-3951-46ed-836f-022c1b62d1f1\",\"link\":\"http://localhost:8082/api/v1/gate/assets/i51.rocketscience.testnet/sunset_at_the_lake/ba1117f1-3951-46ed-836f-022c1b62d1f1\",\"type\":\"image\",\"title\":\"sunset\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o\"}\
                  ],
                  \"sets\":[
                    {\"id\":\"set_id1\",\"objects\":[\"ba1117f1-3951-46ed-836f-022c1b62d1f1\"]}
                  ]
                }".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
                from: None,
                sku_data: None,
            },
            licenses: Some(vec![
                AssetLicense{
                    objects: Some(vec!["ba1117f1-3951-46ed-836f-022c1b62d1f1".to_string()]),
                    set_id: None,
                    sku_id: Some("sku1".to_string()),
                    license_id: Some("some_id".to_string()),
                    price: Some("2".to_string()),
                    title: "personal".to_string(),
                    params: None,
                },
                AssetLicense{
                    objects: Some(vec!["ba1117f1-3951-46ed-836f-022c1b62d1f1".to_string()]),
                    set_id: None,
                    sku_id: Some("sku2".to_string()),
                    license_id: Some("some_id2".to_string()),
                    price: Some("5".to_string()),
                    title: "commercial".to_string(),
                    params: None,
                },
            ]),
            license_token_count: 2,
            token_id: "asset_normal".to_string(),
            owner_id: AccountId::new_unchecked("rocketscience".to_string()),
            minter_id: AccountId::new_unchecked("license_rocketscience".to_string()),
            policy_rules: None,
            upgrade_rules: None,
        };

        json_asset.migrate_to_sets();

        assert_eq!(json_asset.licenses.clone().unwrap()[0].sku_id.clone().unwrap(), "sku1");
        assert_eq!(json_asset.licenses.clone().unwrap()[0].objects.clone().unwrap().len(), 1);
        assert_eq!(json_asset.licenses.clone().unwrap()[1].sku_id.clone().unwrap(), "sku2");
        assert_eq!(json_asset.licenses.clone().unwrap()[1].objects.clone().unwrap().len(), 1);
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
                ],
                  \"sets\": [
                    {\"id\": \"set1\", \"objects\": [\"1\",\"2\",\"3\"]},
                    {\"id\": \"set2\", \"objects\": [\"4\",\"3\",\"2\"]}
                  ]
                }".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
                from: None,
                sku_data: None,
            },
            licenses: Some(vec![
                AssetLicense{
                    objects: Some(vec!["1".to_string(), "2".to_string()]),
                    sku_id: Some("sku1".to_string()),
                    set_id: Some("set1".to_string()),
                    license_id: Some("id1".to_string()),
                    price: None,
                    title: "id1 title".to_string(),
                    params: None,
                },
                AssetLicense{
                    objects: None,
                    sku_id: Some("sku2".to_string()),
                    params: None,
                    set_id: Some("set2".to_string()),
                    license_id: Some("id2".to_string()),
                    price: None,
                    title: "id2 title".to_string(),
                },
            ]),
            license_token_count: 2,
            token_id: "asset_normal".to_string(),
            owner_id: AccountId::new_unchecked("rocketscience".to_string()),
            minter_id: AccountId::new_unchecked("license_rocketscience".to_string()),
            policy_rules: None,
            upgrade_rules: None,
        };

        let licenses = json_asset.licenses.clone().unwrap();
        let new_meta = json_asset.issue_new_metadata(licenses[0].clone());

        println!("{}", serde_json::to_string(&new_meta.object).unwrap());
        assert_eq!(new_meta.object.clone().unwrap().contains("\"1\""), true);
        assert_eq!(new_meta.object.clone().unwrap().contains("\"2\""), true);
        assert_eq!(new_meta.object.clone().unwrap().contains("\"3\""), false);
        assert_eq!(new_meta.object.clone().unwrap().contains("\"4\""), false);

        json_asset.metadata.object = Some("".to_string());
        let new_meta = json_asset.issue_new_metadata(licenses[0].clone());

        assert_eq!(new_meta.object.unwrap().is_empty(), true);
        // println!("{}", serde_json::to_string(&new_meta.object).unwrap())
    }
}