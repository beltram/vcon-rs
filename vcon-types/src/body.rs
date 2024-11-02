#[cfg(feature = "cbor")]
pub const BASE_64_URL_TAG: u64 = 21;

/// Binary body
///
/// See https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-body
///
/// # Usage
///
/// `#[serde(flatten)]` at declaration site
///
/// # json example (binary)
///
/// ```rust
/// // # use serde_json::json;
/// // # use vcon_types::InlineContent;
/// // let actual = InlineContent::BinaryBase64Url(b"abcd".to_vec());
/// // # let actual_ser = serde_json::to_string(&actual).unwrap();
/// // let expected = json!({
/// //     "encoding": "base64url",
/// //     "body": "YWJjZA"
/// // });
/// // # let expected = serde_json::to_string(&expected).unwrap();
/// // # assert_eq!(expected, actual_ser);
/// // # let deser = serde_json::from_str::<InlineContent>(&expected).unwrap();
/// // # assert_eq!(actual, deser);
/// ```
///
/// # cbor example (binary)
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::InlineContent;
/// let actual = InlineContent::BinaryBase64Url(b"abcd".to_vec());
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Map(vec![
///     (Value::Text("encoding".into()), Value::Text("base64url".into())),
///     (Value::Text("body".into()), Value::Tag(21, Box::new(Value::Bytes(b"YWJjZA".to_vec()))))
/// ]);
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<InlineContent>(&actual_ser).unwrap();
/// # assert_eq!(actual, deser);
/// ```
///
/// # json example (text)
///
/// ```rust
/// // # use serde_json::json;
/// // # use vcon_types::InlineContent;
/// // let actual = InlineContent::TextNone("abcd".into());
/// // # let actual_ser = serde_json::to_string(&actual).unwrap();
/// // let expected = json!({
/// //     "encoding": "none",
/// //     "body": "abcd"
/// // });
/// // # let expected = serde_json::to_string(&expected).unwrap();
/// // # assert_eq!(expected, actual_ser);
/// // # let deser = serde_json::from_str::<InlineContent>(&expected).unwrap();
/// // # assert_eq!(actual, deser);
/// ```
///
/// # cbor example (text)
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::InlineContent;
/// let actual = InlineContent::TextNone("abcd".into());
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Map(vec![
///     (Value::Text("encoding".into()), Value::Text("none".into())),
///     (Value::Text("body".into()), Value::Text("abcd".into()))
/// ]);
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<InlineContent>(&actual_ser).unwrap();
/// # assert_eq!(actual, deser);
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum InlineContent {
    BinaryBase64Url(Vec<u8>),
    TextJson(String),
    TextNone(String),
}

impl InlineContent {
    const B64: base64::engine::GeneralPurpose = base64::prelude::BASE64_URL_SAFE_NO_PAD;
}

#[cfg(ser)]
impl serde::Serialize for InlineContent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bb = serializer.serialize_map(Some(2))?;
        match self {
            InlineContent::BinaryBase64Url(bytes) => {
                bb.serialize_entry("encoding", &BodyEncoding::Base64Url)?;
                use base64::Engine as _;
                let b64 = Self::B64.encode(bytes);
                #[cfg(feature = "json")]
                {
                    bb.serialize_entry("body", &b64)?;
                }
                #[cfg(feature = "cbor")]
                {
                    // TODO: we should not have to b64 encode it but that's what the spec currently says so...
                    let b64 = ciborium::Value::Bytes(b64.into_bytes());
                    let value = ciborium::tag::Required::<_, BASE_64_URL_TAG>(b64);
                    bb.serialize_entry("body", &value)?;
                }
            }
            InlineContent::TextNone(body) => {
                bb.serialize_entry("encoding", &BodyEncoding::None)?;
                bb.serialize_entry("body", &body)?;
            }
            InlineContent::TextJson(_) => unimplemented!(),
        }
        use serde::ser::SerializeMap as _;
        bb.end()
    }
}

#[cfg(ser)]
impl<'de> serde::Deserialize<'de> for InlineContent {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InlineContent, D::Error> {
        struct BinaryBodyVisitor;

        impl<'de> serde::de::Visitor<'de> for BinaryBodyVisitor {
            type Value = InlineContent;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a Map of BinaryBody with content and encoding")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::Error as _;
                let (_, encoding) =
                    map.next_entry::<String, BodyEncoding>()?
                        .ok_or(A::Error::custom(
                            "Invalid BinaryBody encoding serialization",
                        ))?;
                Ok(match encoding {
                    BodyEncoding::Base64Url => {
                        use base64::Engine as _;
                        use serde::de::Error as _;
                        #[cfg(feature = "json")]
                        {
                            let (_, value) = map
                                .next_entry::<String, String>()?
                                .ok_or(A::Error::custom("Invalid Body serialization"))?;
                            let value = InlineContent::B64
                                .decode(&value)
                                .map_err(|e| A::Error::custom(format!("{e:?}")))?;
                            Self::Value::BinaryBase64Url(value)
                        }
                        #[cfg(feature = "cbor")]
                        {
                            let (_, value) = map
                                .next_entry::<String, ciborium::tag::Required<ciborium::Value, BASE_64_URL_TAG>>()?
                                .ok_or(A::Error::custom("Invalid Body serialization"))?;
                            let value = value
                                .0
                                .into_bytes()
                                .map_err(|_| A::Error::custom("Binary Body should be bytes"))?;
                            let value = InlineContent::B64
                                .decode(value)
                                .map_err(|e| A::Error::custom(format!("{e:?}")))?;
                            Self::Value::BinaryBase64Url(value)
                        }
                    }
                    BodyEncoding::None => {
                        #[cfg(feature = "json")]
                        {
                            let (_, value) = map
                                .next_entry::<String, String>()?
                                .ok_or(A::Error::custom("Invalid Body serialization"))?;
                            Self::Value::TextNone(value)
                        }
                        #[cfg(feature = "cbor")]
                        {
                            let (_, value) = map
                                .next_entry::<String, String>()?
                                .ok_or(A::Error::custom("Invalid Body serialization"))?;
                            Self::Value::TextNone(value)
                        }
                    }
                    BodyEncoding::Json => unimplemented!(),
                })
            }
        }

        deserializer.deserialize_map(BinaryBodyVisitor)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) enum BodyEncoding {
    Base64Url,
    None,
    Json,
}

#[cfg(ser)]
impl serde::Serialize for BodyEncoding {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let alg = match self {
            Self::Base64Url => "base64url",
            Self::Json => "json",
            Self::None => "none",
        };
        serializer.serialize_str(alg)
    }
}

#[cfg(ser)]
impl<'de> serde::de::Deserialize<'de> for BodyEncoding {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "base64url" => Self::Base64Url,
            "none" => Self::None,
            "json" => Self::Json,
            _ => return Err(D::Error::custom("Unknown binary encoding")),
        })
    }
}
