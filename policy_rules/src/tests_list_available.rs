#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests_list_available {
    use near_sdk::serde_json;
    use near_sdk::serde::{Deserialize, Serialize};
    use crate::policy::{init_policies};
    use crate::policy::{ConfigInterface};
    use common_types::types::{FullInventory};

    #[test]
    fn test_list_available() {
        let args = r#"{
  "inventory": {
    "inventory_licenses": [
      {
        "license_id": "38c6ba6e-792c-4b30-bfba-add480c323fb",
        "title": "personal_license",
        "price": "1",
        "license": {
          "perpetuity": false,
          "exclusivity": false,
          "personal_use": true,
          "commercial_use": null,
          "limited_display_sublicensee": true,
          "template": "level",
          "pdf_url": "https://veriken.mypinata.cloud/ipfs/Qma5LZ5thuBLfc5ZvToCHhpQSQ6vzB47iA7bZgCmUFQCfm"
        }
      },
      {
        "license_id": "16c461fb-e614-4ef8-8e4f-0420f15e5900",
        "title": "commercial_license",
        "price": "5",
        "license": {
          "perpetuity": false,
          "exclusivity": false,
          "commercial_use": null,
          "limited_display_sublicensee": true,
          "template": "level",
          "pdf_url": "https://veriken.mypinata.cloud/ipfs/Qmc2w8AFTPXsauQuKyZUB9Sx9EpCwZEoUKrUCTWTHNGXgA"
        }
      },
      {
        "license_id": "a780f062-3e58-42ad-8b6d-1c9f9bea0200",
        "title": "exclusive_license",
        "price": "5",
        "license": {
          "perpetuity": false,
          "exclusivity": true,
          "commercial_use": false,
          "limited_display_sublicensee": true,
          "template": "level",
          "pdf_url": "https://veriken.mypinata.cloud/ipfs/Qmc2w8AFTPXsauQuKyZUB9Sx9EpCwZEoUKrUCTWTHNGXgA"
        }
      }
    ],
    "issued_licenses": [
      {
        "token_id": "1",
        "owner_id": "nftsentry.testnet",
        "asset_id": "sunset_at_the_lake",
        "metadata": {
          "title": "Sunset at the lake",
          "description": "",
          "media": "https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o",
          "media_hash": "",
          "previews": "",
          "object": "{\"items\":[{\"id\":\"ba1117f1-3951-46ed-836f-022c1b62d1f1\",\"link\":\"https://veriken.xyz/api/v1/gate/assets/armenia1.rocketscience.testnet/sunset_at_the_lake/ba1117f1-3951-46ed-836f-022c1b62d1f1\",\"type\":\"image\",\"title\":\"sunset\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o\",\"params\":null}],\"sets\":[{\"id\":\"c729cb21-7561-4b84-878d-8730610ff84b\",\"objects\":[\"ba1117f1-3951-46ed-836f-022c1b62d1f1\"],\"title\":\"sunset\",\"active\":true,\"icon\":\"/assets/img/skuIcon.jpg\",\"description\":\"\"}]}",
          "copies": null,
          "issued_at": 1666762384312,
          "expires_at": null,
          "starts_at": 1666762384312,
          "updated_at": 1666762384312,
          "extra": "",
          "reference": "",
          "reference_hash": "",
          "from": {
            "inventory_id": "armenia1.rocketscience.testnet",
            "asset_id": "sunset_at_the_lake",
            "set_id": "",
            "sku_id": "2ee65332-8703-46c5-978c-af829695e440"
          },
          "sku_data": null
        },
        "approved_account_ids": {},
        "royalty": null,
        "license": {
          "id": "38c6ba6e-792c-4b30-bfba-add480c323fb",
          "title": "personal_license",
          "description": "",
          "issuer_id": "license_armenia1.rocketscience.testnet",
          "uri": "https://veriken.mypinata.cloud/ipfs/Qma5LZ5thuBLfc5ZvToCHhpQSQ6vzB47iA7bZgCmUFQCfm",
          "from": {
            "inventory_id": "armenia1.rocketscience.testnet",
            "asset_id": "sunset_at_the_lake",
            "set_id": "",
            "sku_id": "2ee65332-8703-46c5-978c-af829695e440"
          },
          "metadata": {
            "perpetuity": false,
            "exclusivity": false,
            "personal_use": true,
            "commercial_use": false,
            "limited_display_sublicensee": true,
            "template": "level",
            "pdf_url": "https://veriken.mypinata.cloud/ipfs/Qma5LZ5thuBLfc5ZvToCHhpQSQ6vzB47iA7bZgCmUFQCfm"
          },
          "issued_at": 1666762384312,
          "starts_at": 1666762384312
        }
      }
    ],
    "asset": {
      "token_id": "sunset_at_the_lake",
      "owner_id": "rocketscience.testnet",
      "metadata": {
        "title": "Sunset at the lake",
        "description": "",
        "media": "https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o",
        "media_hash": "",
        "previews": "{\"items\":[{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o\",\"type\":\"image\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o\"}]}",
        "object": "{\"items\":[{\"id\":\"ba1117f1-3951-46ed-836f-022c1b62d1f1\",\"link\":\"https://veriken.xyz/api/v1/gate/assets/armenia1.rocketscience.testnet/sunset_at_the_lake/ba1117f1-3951-46ed-836f-022c1b62d1f1\",\"type\":\"image\",\"title\":\"sunset\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYnSFnRuQA8xNjxd7abkE8kf53rm4zpAutBz6UKeT6H2o\",\"params\":null}],\"sets\":[{\"id\":\"c729cb21-7561-4b84-878d-8730610ff84b\",\"objects\":[\"ba1117f1-3951-46ed-836f-022c1b62d1f1\"],\"title\":\"sunset\",\"active\":true,\"icon\":\"/assets/img/skuIcon.jpg\",\"description\":\"\"}]}",
        "copies": null,
        "issued_at": null,
        "expires_at": null,
        "starts_at": null,
        "updated_at": null,
        "extra": "",
        "reference": "",
        "reference_hash": "",
        "from": null,
        "sku_data": null
      },
      "minter_id": "license_armenia1.rocketscience.testnet",
      "licenses": [
        {
          "sku_id": "2ee65332-8703-46c5-978c-af829695e440",
          "license_id": "38c6ba6e-792c-4b30-bfba-add480c323fb",
          "title": "personal_sku",
          "price": "4",
          "currency": "USD",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "ba1117f1-3951-46ed-836f-022c1b62d1f1"
          ],
          "params": "{\"description\":null,\"attr\":{}}"
        },
        {
          "sku_id": "c4ccc6c1-fe2c-4903-99d8-b8dee1418f7b",
          "license_id": "16c461fb-e614-4ef8-8e4f-0420f15e5900",
          "title": "commercial_sku",
          "price": "12",
          "currency": "USD",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "ba1117f1-3951-46ed-836f-022c1b62d1f1"
          ],
          "params": "{\"description\":null,\"attr\":{}}"
        },
        {
          "sku_id": "7d001da2-9ab3-4d41-8e7b-17ced1c9a8ea",
          "license_id": "a780f062-3e58-42ad-8b6d-1c9f9bea0200",
          "title": "exclusive_sku",
          "price": "12",
          "currency": "USD",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "objects": [
            "ba1117f1-3951-46ed-836f-022c1b62d1f1"
          ],
          "params": "{\"description\":null,\"attr\":{}}"
        }
      ],
      "upgrade_rules": null,
      "license_token_count": 1
    }
  }
}"#;
        #[derive(Serialize, Deserialize, Clone)]
        #[serde(crate = "near_sdk::serde")]
        struct ListAvailableOpt {
            pub inventory: FullInventory,
        }
        let _policies = init_policies();
        let l_args: ListAvailableOpt = serde_json::from_str(args).expect("Failed parse");
        let availability = _policies.list_available(
            l_args.inventory.clone(), None, None
        );

        // println!("{}", serde_json::to_string_pretty(&availability[1]).unwrap());
        assert_eq!(false, availability[2].available)
    }

    #[test]
    fn test_list_available_sole_limit() {
        let args = r#"{
  "inventory": {
    "inventory_licenses": [
      {
        "license_id": "7a41b643-2df5-4cb6-a161-22933aa893a2",
        "title": "Personal Use License",
        "price": null,
        "license": {
          "exclusivity": false,
          "commercial_use": false,
          "template": "level",
          "pdf_url": "https://veriken.mypinata.cloud/ipfs/QmNjEw25uTTFNgTnNcM2gAZczahERvDwhcJEHV9qnuAe17",
          "version": ""
        }
      },
      {
        "license_id": "69f5335a-b3ba-4b32-b05f-006149e8673d",
        "title": "Commercial Use License Agreement",
        "price": null,
        "license": {
          "exclusivity": false,
          "commercial_use": true,
          "template": "level",
          "pdf_url": "https://veriken.mypinata.cloud/ipfs/QmX5Nh6DEqWkQxsY6LzXNTjbYFtPuAe6mdesrw2StkRQ1x",
          "version": ""
        }
      }
    ],
    "issued_licenses": [],
    "asset": {
      "token_id": "ocean_during_sunset",
      "owner_id": "veriken_demo.testnet",
      "metadata": {
        "title": "Ocean During Sunset",
        "description": "Ocean During Sunset by Dey Kheireddine",
        "media": "https://veriken.mypinata.cloud/ipfs/QmYSh9JhnFP2Cwf1RWNgK6kLkaB2rtqZyoB1SqpxSWPCkw",
        "media_hash": "",
        "previews": "{\"items\":[{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmYSh9JhnFP2Cwf1RWNgK6kLkaB2rtqZyoB1SqpxSWPCkw\",\"type\":\"image\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYSh9JhnFP2Cwf1RWNgK6kLkaB2rtqZyoB1SqpxSWPCkw\"},{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmVBU1H4tEtmuTSjjG1eF7bNpLSgPq5g8mo9jkijY9xhGY\",\"type\":\"video\",\"icon\":\"/assets/icons/video-play-roll-icon.png\"},{\"link\":\"https://youtu.be/MDuErap6raQ\",\"type\":\"video\",\"icon\":\"https://img.youtube.com/vi/MDuErap6raQ/0.jpg\"}]}",
        "object": "{\"items\":[{\"id\":\"422256f6-db35-4dcc-bcc3-64709d53f547\",\"link\":\"https://veriken.xyz/api/v1/gate/assets/shopify_video_catalog.veriken_demo.testnet/ocean_during_sunset/422256f6-db35-4dcc-bcc3-64709d53f547\",\"type\":\"video\",\"title\":\"720p\",\"icon\":\"/assets/icons/video-play-roll-icon.png\",\"params\":null}],\"sets\":[]}",
        "copies": null,
        "issued_at": null,
        "expires_at": null,
        "starts_at": null,
        "updated_at": null,
        "extra": "{\"_options\":{\"list\":[{\"name\":\"License\",\"value\":null,\"lock\":true,\"ext\":{\"shopify\":true}},{\"name\":\"Max Resolution\",\"value\":\"\",\"lock\":false,\"ext\":{\"shopify\":true}}],\"ext\":{\"shopify\":true},\"shopify\":true}}",
        "reference": "",
        "reference_hash": "",
        "from": null,
        "sku_data": null
      },
      "minter_id": "product_shopify_video_catalog.veriken_demo.testnet",
      "licenses": [
        {
          "sku_id": "f9b521d8-cd4d-412d-817a-cd11f4c0329b",
          "license_id": "7a41b643-2df5-4cb6-a161-22933aa893a2",
          "title": "Ocean During Sunset Personal Use 720p",
          "price": "2",
          "currency": "USD",
          "sole_limit": 250,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "422256f6-db35-4dcc-bcc3-64709d53f547"
          ],
          "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYSh9JhnFP2Cwf1RWNgK6kLkaB2rtqZyoB1SqpxSWPCkw\",\"description\":null,\"attr\":{\"list\":[{\"name\":\"License\",\"value\":\"7a41b643-2df5-4cb6-a161-22933aa893a2\",\"lock\":true,\"ext\":{}},{\"name\":\"Max Resolution\",\"value\":\"720p\",\"lock\":true,\"ext\":{}}],\"ext\":{}}}"
        },
        {
          "sku_id": "28d86ea1-4b04-4a95-b0b9-560086098fbf",
          "license_id": "69f5335a-b3ba-4b32-b05f-006149e8673d",
          "title": "Ocean During Sunset Commercial Use 720p",
          "price": "10",
          "currency": "USD",
          "sole_limit": 100,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "422256f6-db35-4dcc-bcc3-64709d53f547"
          ],
          "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmYSh9JhnFP2Cwf1RWNgK6kLkaB2rtqZyoB1SqpxSWPCkw\",\"description\":null,\"attr\":{\"list\":[{\"name\":\"License\",\"value\":\"69f5335a-b3ba-4b32-b05f-006149e8673d\",\"lock\":true,\"ext\":{}},{\"name\":\"Max Resolution\",\"value\":\"720p\",\"lock\":true,\"ext\":{}}],\"ext\":{}}}"
        }
      ],
      "upgrade_rules": null,
      "license_token_count": 0
    }
  }
}"#;
        #[derive(Serialize, Deserialize, Clone)]
        #[serde(crate = "near_sdk::serde")]
        struct ListAvailableOpt {
            pub inventory: FullInventory,
        }
        let _policies = init_policies();
        let l_args: ListAvailableOpt = serde_json::from_str(args).expect("Failed parse");
        // l_args.inventory.issued_licenses[0].license.unwrap()
        let availability = _policies.list_available(
            l_args.inventory.clone(), None, None
        );

        let res1 = availability[0].additional_info.as_ref().unwrap().get("f9b521d8-cd4d-412d-817a-cd11f4c0329b").unwrap();
        let res2 = availability[1].additional_info.as_ref().unwrap().get("28d86ea1-4b04-4a95-b0b9-560086098fbf").unwrap();

        assert_eq!(res1.remains, 250);
        assert_eq!(res1.total, 250);
        assert_eq!(res1.issued, 0);

        assert_eq!(res2.remains, 100);
        assert_eq!(res2.total, 100);
        assert_eq!(res2.issued, 0);
    }
}