use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};
use crate::*;

/// Enum that represents the data type of the EventLog.
/// The enum can either be an NftMint or an NftTransfer.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    AssetMint(Vec<AssetMintLog>),
    AssetUpdateLicense(Vec<AssetUpdateLicenseLog>),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,
    
    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

/// An event log to capture token minting
///
/// Arguments
/// * `owner_id`: "account.near"
/// * `token_ids`: ["1", "abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetMintLog {
    pub owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetUpdateLicenseLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<String>,

    pub owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nep_format_vector() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"asset_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]},{"owner_id":"user1.near","token_ids":["meme"]}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AssetMint(vec![
                AssetMintLog {
                    owner_id: "foundation.near".to_owned(),
                    token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
                    memo: None,
                },
                AssetMintLog {
                    owner_id: "user1.near".to_owned(),
                    token_ids: vec!["meme".to_string()],
                    memo: None,
                },
            ]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_asset_mint() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"asset_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AssetMint(vec![AssetMintLog {
                owner_id: "foundation.near".to_owned(),
                token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
                memo: None,
            }]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_asset_add_license_all_fields() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"asset_add_license","data":[{"authorized_id":"market.near","owner_id":"foundation.near","token_ids":["token"],"memo":"Go Team!"}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AssetAddLicense(vec![AssetAddLicenseLog {
                authorized_id: Some("market.near".to_string()),
                owner_id: "foundation.near".to_string(),
                token_ids: vec!["token".to_string()],
                memo: Some("Go Team!".to_owned()),
            }]),
        };
        assert_eq!(expected, log.to_string());
    }
    #[test]
    fn nep_format_asset_remove_license_all_fields() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"asset_remove_license","data":[{"authorized_id":"market.near","owner_id":"foundation.near","token_ids":["token"],"memo":"Go Team!"}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AssetRemoveLicense(vec![AssetRemoveLicenseLog {
                authorized_id: Some("market.near".to_string()),
                owner_id: "foundation.near".to_string(),
                token_ids: vec!["token".to_string()],
                memo: Some("Go Team!".to_owned()),
            }]),
        };
        assert_eq!(expected, log.to_string());
    }
    #[test]
    fn nep_format_asset_update_license_all_fields() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"asset_update_license","data":[{"authorized_id":"market.near","owner_id":"foundation.near","token_ids":["token"],"memo":"Go Team!"}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AssetUpdateLicense(vec![AssetUpdateLicenseLog {
                authorized_id: Some("market.near".to_string()),
                owner_id: "foundation.near".to_string(),
                token_ids: vec!["token".to_string()],
                memo: Some("Go Team!".to_owned()),
            }]),
        };
        assert_eq!(expected, log.to_string());
    }
}

#[near_bindgen]
impl InventoryContract {
    pub(crate) fn log_event(&self, msg: &str) {
        if !self.disable_events {
            env::log_str(msg);
        }
    }
}