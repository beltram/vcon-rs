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
/// # #[cfg(feature = "json")] {
/// # use vcon_types::Url;
/// # use serde_json::json;
/// # vcon_types::expect_json_eq(
/// "https://github.com/".parse::<Url>().unwrap(), // actual
/// json!({ "url": "https://github.com/" }), // expected
/// # )}
/// ```
/// # cbor example
///
/// ```rust
/// # #[cfg(feature = "cbor")] {
/// # use vcon_types::Url;
/// # use ciborium::cbor;
/// # vcon_types::expect_cbor_eq(
/// "https://github.com/".parse::<Url>().unwrap(), // actual
/// cbor!({ "url" => "https://github.com/" }) // expected
/// # .unwrap(),
/// # )}
/// ```
///
/// # Notes
///
/// * Only 'https' scheme is supported
/// * Urls adhere to WHATWG URL Standard, so ones without a path will have a trailing slash appended to them
///
/// ```rust
/// # use vcon_types::Url;
/// assert!("http://github.com/".parse::<Url>().is_err());
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Url {
    url: url::Url,
}

impl std::str::FromStr for Url {
    type Err = Box<dyn core::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = url::Url::parse(s)?;
        if url.scheme() != "https" {
            return Err("Url scheme MUST be 'https'".into());
        }
        Ok(Self { url })
    }
}
