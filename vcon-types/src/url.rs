use derive_more::{Deref, DerefMut, Into};

/// Url
///
/// See https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-url
///
/// # Usage
///
/// `#[serde(flatten)]` at declaration site
///
/// # json example
///
/// ```rust
/// # use serde_json::json;
/// # use vcon_types::Url;
/// let actual = Url::try_from("https://github.com/").unwrap();
/// # let actual_ser = serde_json::to_string(&actual).unwrap();
/// let expected = json!({ "url": "https://github.com/" });
/// # let expected = serde_json::to_string(&expected).unwrap();
/// # assert_eq!(expected, actual_ser);
/// # let deser = serde_json::from_str::<Url>(&expected).unwrap();
/// # assert_eq!(actual, deser);
/// ```
/// # cbor example
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::Url;
/// let actual = Url::try_from("https://github.com/").unwrap();
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Map(vec![(Value::Text("url".into()), Value::Text("https://github.com/".into()))]);
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<Url>(&actual_ser).unwrap();
/// # assert_eq!(actual, deser);
/// ```
/// # Notes
///
/// * Only 'https' scheme is supported
/// * Urls adhere to WHATWG URL Standard, so ones without a path will have a trailing slash appended to them
///
/// ```rust
/// # use vcon_types::Url;
/// assert!(Url::try_from("http://github.com/").is_err());
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Url {
    url: url::Url,
}

impl TryFrom<&str> for Url {
    type Error = Box<dyn core::error::Error>;

    fn try_from(u: &str) -> Result<Self, Self::Error> {
        let url = url::Url::parse(u)?;
        if url.scheme() != "https" {
            return Err("Url scheme MUST be 'https'".into());
        }
        Ok(Self { url })
    }
}
