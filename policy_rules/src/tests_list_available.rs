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
    fn test_list_available_fix_sku() {
        let args = r#"{
  "inventory": {
    "inventory_licenses": [],
    "issued_licenses": [
      {
        "token_id": "1",
        "owner_id": "chrisclason2.testnet",
        "asset_id": "waves_by_enrique_hoyos",
        "metadata": {
          "title": "Waves by Enrique Hoyos",
          "description": "Video file of waves",
          "media": "https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85",
          "media_hash": "",
          "previews": "{\"items\":[{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmacYkaYUjCur57gp7zmmSPByS1Y5YE8EQpRdw2RyET4tp\",\"type\":\"video\",\"icon\":\"/assets/icons/video-play-roll-icon.png\"}]}",
          "object": "{\"items\":[{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"type\":\"video\",\"id\":\"509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"title\":\"720p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"},{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/d7490ac0-f87f-47a5-b60b-181357c3b005\",\"type\":\"video\",\"id\":\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"title\":\"1080p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"},{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/666ed016-82ce-48d6-9913-d4cd274e425b\",\"type\":\"video\",\"id\":\"666ed016-82ce-48d6-9913-d4cd274e425b\",\"title\":\"4k\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"}],\"sets\":[{\"id\":\"4db1a9da-08b7-4445-933f-f4846f565d13\",\"objects\":[\"509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"666ed016-82ce-48d6-9913-d4cd274e425b\"],\"title\":\"Commercial Use - 4k, 1080p, 720p\",\"active\":true,\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"\"}]}",
          "copies": null,
          "issued_at": 1670948531741,
          "expires_at": null,
          "starts_at": 1670948531741,
          "updated_at": 1670948531741,
          "extra": "{\"artist\":\"Enrique Hoyos\",\"resolutions\":\"720p,1080p,4k\",\"duration\":\"33 seconds\"}",
          "reference": "",
          "reference_hash": "",
          "from": null,
          "sku_data": null
        },
        "approved_account_ids": {},
        "royalty": null,
        "license": {
          "id": "b76cbb5d-b00a-457a-8b57-d824b14f379a",
          "title": "Commercial Use License",
          "description": "",
          "issuer_id": "license_nature_videos.veriken_demo.testnet",
          "uri": "https://veriken.mypinata.cloud/ipfs/QmPTrL86SBGk4UgaWUMi4othVdvLvJx2ZqfgFKcVQH3SxZ",
          "from": {
            "inventory_id": "nature_videos.veriken_demo.testnet",
            "asset_id": "waves_by_enrique_hoyos",
            "set_id": "4db1a9da-08b7-4445-933f-f4846f565d13",
            "sku_id": ""
          },
          "metadata": {
            "exclusivity": false,
            "commercial_use": null,
            "template": "level",
            "pdf_url": "https://veriken.mypinata.cloud/ipfs/QmPTrL86SBGk4UgaWUMi4othVdvLvJx2ZqfgFKcVQH3SxZ",
            "version": ""
          },
          "issued_at": 1670948531741,
          "starts_at": 1670948531741
        }
      },
      {
        "token_id": "2",
        "owner_id": "chrisclason.testnet",
        "asset_id": "waves_by_enrique_hoyos",
        "metadata": {
          "title": "Waves by Enrique Hoyos",
          "description": "Video file of waves",
          "media": "https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85",
          "media_hash": "",
          "previews": "{\"items\":[{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmacYkaYUjCur57gp7zmmSPByS1Y5YE8EQpRdw2RyET4tp\",\"type\":\"video\",\"icon\":\"/assets/icons/video-play-roll-icon.png\"}]}",
          "object": "{\"items\":[{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"type\":\"video\",\"id\":\"509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"title\":\"720p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"},{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/d7490ac0-f87f-47a5-b60b-181357c3b005\",\"type\":\"video\",\"id\":\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"title\":\"1080p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"},{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/666ed016-82ce-48d6-9913-d4cd274e425b\",\"type\":\"video\",\"id\":\"666ed016-82ce-48d6-9913-d4cd274e425b\",\"title\":\"4k\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"}],\"sets\":null}",
          "copies": null,
          "issued_at": 1671636296487,
          "expires_at": null,
          "starts_at": 1671636296487,
          "updated_at": 1671636296487,
          "extra": "{\"artist\":\"Enrique Hoyos\",\"duration\":\"33 seconds\"}",
          "reference": "",
          "reference_hash": "",
          "from": null,
          "sku_data": null
        },
        "approved_account_ids": {},
        "royalty": null,
        "license": {
          "id": "b76cbb5d-b00a-457a-8b57-d824b14f379a",
          "title": "Waves by Enrique Hoyes - 720p",
          "description": "",
          "issuer_id": "license_nature_videos.veriken_demo.testnet",
          "uri": "https://veriken.mypinata.cloud/ipfs/QmPTrL86SBGk4UgaWUMi4othVdvLvJx2ZqfgFKcVQH3SxZ",
          "from": {
            "inventory_id": "nature_videos.veriken_demo.testnet",
            "asset_id": "waves_by_enrique_hoyos",
            "set_id": "",
            "sku_id": "1671635190898345070-waves_by_enrique_hoyos-2"
          },
          "metadata": {
            "exclusivity": false,
            "commercial_use": null,
            "template": "level",
            "pdf_url": "https://veriken.mypinata.cloud/ipfs/QmPTrL86SBGk4UgaWUMi4othVdvLvJx2ZqfgFKcVQH3SxZ",
            "version": ""
          },
          "issued_at": 1671636296487,
          "starts_at": 1671636296487
        }
      },
      {
        "token_id": "5",
        "owner_id": "chrisclason4.testnet",
        "asset_id": "waves_by_enrique_hoyos",
        "metadata": {
          "title": "Waves by Enrique Hoyos",
          "description": "Video file of waves",
          "media": "https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85",
          "media_hash": "",
          "previews": "{\"items\":[{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmacYkaYUjCur57gp7zmmSPByS1Y5YE8EQpRdw2RyET4tp\",\"type\":\"video\",\"icon\":\"/assets/icons/video-play-roll-icon.png\"}]}",
          "object": "{\"items\":[{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"type\":\"video\",\"id\":\"509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"title\":\"720p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"},{\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/d7490ac0-f87f-47a5-b60b-181357c3b005\",\"type\":\"video\",\"id\":\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"title\":\"1080p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\"}],\"sets\":null}",
          "copies": null,
          "issued_at": 1672426111546,
          "expires_at": null,
          "starts_at": 1672426111546,
          "updated_at": 1672426111546,
          "extra": "{\"artist\":\"Enrique Hoyos\",\"duration\":\"33 seconds\"}",
          "reference": "",
          "reference_hash": "",
          "from": {
            "inventory_id": "nature_videos.veriken_demo.testnet",
            "asset_id": "waves_by_enrique_hoyos",
            "set_id": "",
            "sku_id": "1672155093505119973-waves_by_enrique_hoyos-1"
          },
          "sku_data": {
            "sku_id": "1672155093505119973-waves_by_enrique_hoyos-1",
            "title": "Waves by Enrique Hoyes - 1080p",
            "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"A 33 second 1080p video clip of waves that includes a commercial use license. Also includes lower resolutions\",\"attr\":{\"resolutions\":\"720p, 1080p\"}}"
          }
        },
        "approved_account_ids": {},
        "royalty": null,
        "license": {
          "id": "b76cbb5d-b00a-457a-8b57-d824b14f379a",
          "title": "Commercial Use License",
          "description": "",
          "issuer_id": "license_nature_videos.veriken_demo.testnet",
          "uri": "https://veriken.mypinata.cloud/ipfs/QmPTrL86SBGk4UgaWUMi4othVdvLvJx2ZqfgFKcVQH3SxZ",
          "from": {
            "inventory_id": "nature_videos.veriken_demo.testnet",
            "asset_id": "waves_by_enrique_hoyos",
            "set_id": "",
            "sku_id": "1672155093505119973-waves_by_enrique_hoyos-1"
          },
          "metadata": {
            "exclusivity": false,
            "commercial_use": null,
            "template": "level",
            "pdf_url": "https://veriken.mypinata.cloud/ipfs/QmPTrL86SBGk4UgaWUMi4othVdvLvJx2ZqfgFKcVQH3SxZ",
            "version": ""
          },
          "issued_at": 1672426111546,
          "starts_at": 1672426111546,
          "updated_at": 1672426111546
        }
      }
    ],
    "asset": {
      "token_id": "waves_by_enrique_hoyos",
      "owner_id": "veriken_demo.testnet",
      "metadata": {
        "title": "Waves by Enrique Hoyos",
        "description": "Video file of waves",
        "media": "https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85",
        "media_hash": "",
        "previews": "{\"items\":[{\"link\":\"https://veriken.mypinata.cloud/ipfs/QmacYkaYUjCur57gp7zmmSPByS1Y5YE8EQpRdw2RyET4tp\",\"type\":\"video\",\"icon\":\"/assets/icons/video-play-roll-icon.png\"}]}",
        "object": "{\"items\":[{\"id\":\"509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"type\":\"video\",\"title\":\"720p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"params\":null},{\"id\":\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/d7490ac0-f87f-47a5-b60b-181357c3b005\",\"type\":\"video\",\"title\":\"1080p\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"params\":null},{\"id\":\"666ed016-82ce-48d6-9913-d4cd274e425b\",\"link\":\"https://veriken.xyz/api/v1/gate/assets/nature_videos.veriken_demo.testnet/waves_by_enrique_hoyos/666ed016-82ce-48d6-9913-d4cd274e425b\",\"type\":\"video\",\"title\":\"4k\",\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"params\":null}],\"sets\":[{\"id\":\"cae9f350-d47e-4820-852f-ac8d1d14c3d3\",\"objects\":[\"509afcf0-36c9-4f5d-a3bc-227d8c331053\"],\"title\":\"Commercial Use License - 720p\",\"active\":true,\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"\"},{\"id\":\"f9585955-6bd9-4c41-8c32-3ba31e8d497c\",\"objects\":[\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"509afcf0-36c9-4f5d-a3bc-227d8c331053\"],\"title\":\"Commercial Use License - 1080p, 720p\",\"active\":true,\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"\"},{\"id\":\"4db1a9da-08b7-4445-933f-f4846f565d13\",\"objects\":[\"509afcf0-36c9-4f5d-a3bc-227d8c331053\",\"d7490ac0-f87f-47a5-b60b-181357c3b005\",\"666ed016-82ce-48d6-9913-d4cd274e425b\"],\"title\":\"Commercial Use - 4k, 1080p, 720p\",\"active\":true,\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"\"}]}",
        "copies": null,
        "issued_at": null,
        "expires_at": null,
        "starts_at": null,
        "updated_at": null,
        "extra": "{\"artist\":\"Enrique Hoyos\",\"duration\":\"33 seconds\"}",
        "reference": "",
        "reference_hash": "",
        "from": null,
        "sku_data": null
      },
      "minter_id": "license_nature_videos.veriken_demo.testnet",
      "licenses": [
        {
          "sku_id": "22c0ffc8-c111-4651-8b67-7e65c432e20e",
          "license_id": null,
          "title": "Waves by Enrique Hoyes - 720p",
          "price": "2",
          "currency": "NEAR",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "509afcf0-36c9-4f5d-a3bc-227d8c331053"
          ],
          "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"A 33 second 720p video clip of waves that includes a commercial use license. \",\"attr\":{\"Max Resolution\":\"720p\",\"Commercial Use\":\"Yes\"}}"
        },
        {
          "sku_id": "59488ab2-6483-4126-8eb5-e53370a0bbb7",
          "license_id": null,
          "title": "Waves by Enrique Hoyes - 1080p",
          "price": "5",
          "currency": "NEAR",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "d7490ac0-f87f-47a5-b60b-181357c3b005",
            "509afcf0-36c9-4f5d-a3bc-227d8c331053"
          ],
          "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"A 33 second 1080p video clip of waves that includes a commercial use license. Also includes lower resolutions\",\"attr\":{\"Max Resolution\":\"1080p\",\"Commercial Use\":\"Yes\"}}"
        },
        {
          "sku_id": "d3e14bec-e9e8-4746-91ff-130babcd59a5",
          "license_id": null,
          "title": "Waves by Enrique Hoyes - 4k",
          "price": "10",
          "currency": "NEAR",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "509afcf0-36c9-4f5d-a3bc-227d8c331053",
            "d7490ac0-f87f-47a5-b60b-181357c3b005",
            "666ed016-82ce-48d6-9913-d4cd274e425b"
          ],
          "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":\"A 33 second 4k video clip of waves that includes a commercial use license. Also includes lower resolutions\",\"attr\":{\"Max Resolution\":\"4k\",\"Commercial Use\":\"Yes\"}}"
        },
        {
          "sku_id": "4e6a1e55-b891-476b-a8ad-a2a66a3a7ec3",
          "license_id": null,
          "title": "Waves By Enrique Hoyes - 4k Personal Use",
          "price": "0.5",
          "currency": "USD",
          "sole_limit": null,
          "limited_edition": false,
          "active": true,
          "set_id": "",
          "objects": [
            "666ed016-82ce-48d6-9913-d4cd274e425b",
            "509afcf0-36c9-4f5d-a3bc-227d8c331053",
            "d7490ac0-f87f-47a5-b60b-181357c3b005"
          ],
          "params": "{\"icon\":\"https://veriken.mypinata.cloud/ipfs/QmSox9T3DQvoGKeNJtigBh3HPMEWGdB3vNACZrkKs6oe85\",\"description\":null,\"attr\":{\"Max Resolution\":\"4k\",\"Commercial Use\":\"No\"}}"
        }
      ],
      "upgrade_rules": null,
      "license_token_count": 3
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

        // println!("{}", serde_json::to_string_pretty(&availability[1]).unwrap());
        assert_eq!(true, availability[0].available);
        assert_eq!(true, availability[1].available);
        assert_eq!(true, availability[2].available);
        assert_eq!(true, availability[3].available);
    }
}