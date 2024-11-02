use derive_more::{Deref, DerefMut, From, Into};

/// Vcon version
///
/// # Usage
///
/// `#[serde(flatten)]` at declaration site
///
/// # json example
///
/// ```rust
/// # #[cfg(feature = "json")] {
/// # use vcon_types::VconVersion;
/// # use serde_json::json;
/// # vcon_types::expect_json_eq(
/// VconVersion::from("v1.0"), // actual
/// json!({ "vcon": "v1.0" }), // expected
/// # )}
/// ```
/// # cbor example
///
/// ```rust
/// # #[cfg(feature = "cbor")] {
/// # use vcon_types::VconVersion;
/// # use ciborium::cbor;
/// # vcon_types::expect_cbor_eq(
/// VconVersion::from("v1.0"), // actual
/// cbor!({ "vcon" => "v1.0" }) // expected
/// # .unwrap(),
/// # )}
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
pub struct VconVersion {
    vcon: String,
}

impl From<&str> for VconVersion {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

/// See https://www.ietf.org/archive/id/draft-petrie-vcon-04.html#section-4.1.1-2.1.2
impl Default for VconVersion {
    fn default() -> Self {
        "0.0.1".into()
    }
}
