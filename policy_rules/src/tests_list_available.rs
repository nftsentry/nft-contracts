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

        // println!("{}", serde_json::to_string_pretty(&availability).unwrap());
        assert_eq!(false, availability[2].available)
    }
}