use derive_more::{Deref, DerefMut, From, Into};

/// Date
///
/// See https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#section-2.2-2
///
/// # json example
///
/// ```rust
/// # #[cfg(feature = "json")] {
/// # use vcon_types::Date;
/// # use serde_json::json;
/// # vcon_types::expect_json_eq(
/// "2022-09-23T23:24:59Z".parse::<Date>().unwrap(), // actual
/// json!("2022-09-23T23:24:59Z"), // expected
/// # )}
/// ```
/// # cbor example
///
/// ```rust
/// # #[cfg(feature = "cbor")] {
/// # use vcon_types::Date;
/// # use ciborium::cbor;
/// # vcon_types::expect_cbor_eq(
/// "2022-09-23T23:24:59Z".parse::<Date>().unwrap(), // actual
/// cbor!("2022-09-23T23:24:59Z") // expected
/// # .unwrap(),
/// # )}
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into, Deref, DerefMut)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize), serde(transparent))]
pub struct Date {
    #[cfg_attr(ser, serde(with = "time::serde::rfc3339"))]
    date: time::OffsetDateTime,
}

impl std::str::FromStr for Date {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
            .map_err(|e| Box::<dyn std::error::Error>::from(format!("{e:?}")))?;
        Ok(Self { date })
    }
}
