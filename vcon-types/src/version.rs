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
/// # use serde_json::json;
/// # use vcon_types::VconVersion;
/// let actual = VconVersion::from("v1.0".to_string());
/// # let actual_ser = serde_json::to_string(&actual).unwrap();
/// let expected = json!({ "vcon": "v1.0" });
/// # let expected = serde_json::to_string(&expected).unwrap();
/// # assert_eq!(expected, actual_ser);
/// # let deser = serde_json::from_str::<VconVersion>(&expected).unwrap();
/// # assert_eq!(actual, deser);
/// ```
/// # cbor example
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::VconVersion;
/// let actual = VconVersion::from("v1.0".to_string());
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Map(vec![(Value::Text("vcon".into()), Value::Text("v1.0".into()))]);
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<VconVersion>(&actual_ser).unwrap();
/// # assert_eq!(actual, deser);
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
pub struct VconVersion {
    vcon: String,
}
