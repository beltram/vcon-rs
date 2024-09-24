use derive_more::{Deref, DerefMut, Into};

/// Uuid
///
/// See https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-uuid
///
/// # Usage
///
/// `#[serde(flatten)]` at declaration site
///
/// # json example
///
/// ```rust
/// # use serde_json::json;
/// # use vcon_types::Uuid;
/// let actual = Uuid::new(*b"abcdefghijklmnop");
/// # let actual_ser = serde_json::to_string(&actual).unwrap();
/// let expected = json!({ "uuid": "61626364-6566-8768-a96a-6b6c6d6e6f70" });
/// # let expected = serde_json::to_string(&expected).unwrap();
/// # assert_eq!(expected, actual_ser);
/// # let deser = serde_json::from_str::<Uuid>(&expected).unwrap();
/// # assert_eq!(actual, deser);
/// ```
/// # cbor example
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::Uuid;
/// let actual = Uuid::new(*b"abcdefghijklmnop");
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Map(vec![(Value::Text("uuid".into()), Value::Text("61626364-6566-8768-a96a-6b6c6d6e6f70".into()))]);
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<Uuid>(&expected).unwrap();
/// # assert_eq!(actual, deser);
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Uuid {
    #[cfg_attr(feature = "cbor", serde(with = "hyphenated"))]
    uuid: uuid::Uuid,
}

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
        Self {
            uuid: uuid::Uuid::new_v8(udf),
        }
    }
}
