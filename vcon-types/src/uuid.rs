use derive_more::{Deref, DerefMut, Into};

/// Uuid
///
/// See https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-uuid
///
/// # json example
///
/// ```rust
/// # #[cfg(feature = "json")] {
/// # use vcon_types::Uuid;
/// # use serde_json::json;
/// # vcon_types::expect_json_eq(
/// Uuid::new(*b"abcdefghijklmnop"), // actual
/// json!("61626364-6566-8768-a96a-6b6c6d6e6f70"), // expected
/// # )}
/// ```
/// # cbor example
///
/// ```rust
/// # #[cfg(feature = "cbor")] {
/// # use vcon_types::Uuid;
/// # use ciborium::cbor;
/// # vcon_types::expect_cbor_eq(
/// Uuid::new(*b"abcdefghijklmnop"), // actual
/// cbor!({ "uuid" => "61626364-6566-8768-a96a-6b6c6d6e6f70" }) // expected
/// # .unwrap(),
/// # )}
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[repr(transparent)]
pub struct Uuid(#[cfg_attr(feature = "cbor", serde(with = "hyphenated"))] uuid::Uuid);

// overcomes troubles with cbor because in uuid serde_support a &str is expected whereas only a String works
pub mod hyphenated {
    pub fn serialize<S: serde::Serializer>(
        u: &uuid::Uuid,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(u.as_hyphenated(), serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<uuid::Uuid, D::Error> {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        let uuid = uuid::Uuid::parse_str(&s).map_err(|_| {
            use serde::de::Error as _;
            D::Error::custom("Invalid uuid")
        })?;
        Ok(uuid)
    }
}

impl Uuid {
    pub fn new(udf: [u8; 16]) -> Self {
        Self(uuid::Uuid::new_v8(udf))
    }
}
