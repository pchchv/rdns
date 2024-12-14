//! Configuration types for all security options in hickory-dns

use serde::Deserialize;

/// Key pair configuration for DNSSEC keys for signing a zone
#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(deny_unknown_fields)]
pub struct KeyConfig {
    /// file path to the key
    pub key_path: String,
    /// password to use to read the key
    pub password: Option<String>,
    /// the type of key stored, see `Algorithm`
    pub algorithm: String,
    /// the name to use when signing records, e.g. ns.example.com
    pub signer_name: Option<String>,
    /// specify that this key should be used for signing a zone
    pub is_zone_signing_key: Option<bool>,
    /// specifies that this key can be used for dynamic updates in the zone
    pub is_zone_update_auth: Option<bool>,
}