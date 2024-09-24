use derive_more::{Deref, DerefMut, From, Into};

/// Mime type
///
/// # Usage
///
/// `#[serde(flatten)]` at declaration site
///
/// # json example
///
/// ```rust
/// # use serde_json::json;
/// # use vcon_types::Mime;
/// let actual = Mime::from("text/plain".to_string());
/// # let actual_ser = serde_json::to_string(&actual).unwrap();
/// let expected = json!({ "mimetype": "text/plain" });
/// # let expected = serde_json::to_string(&expected).unwrap();
/// # assert_eq!(expected, actual_ser);
/// # let deser = serde_json::from_str::<Mime>(&expected).unwrap();
/// # assert_eq!(actual, deser);
/// ```
/// # cbor example
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::Mime;
/// let actual = Mime::from("text/plain".to_string());
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Map(vec![(Value::Text("mimetype".into()), Value::Text("text/plain".into()))]);
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<Mime>(&actual_ser).unwrap();
/// # assert_eq!(actual, deser);
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Mime {
    mimetype: String,
}
