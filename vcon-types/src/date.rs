use derive_more::{Deref, DerefMut, From, Into};

/// Date
///
/// See https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#section-2.2-2
///
/// # json example
///
/// ```rust
/// # use serde_json::json;
/// # use vcon_types::Date;
/// let actual = "2022-09-23T23:24:59Z".parse::<Date>().unwrap();
/// # let actual_ser = serde_json::to_string(&actual).unwrap();
/// let expected = json!("2022-09-23T23:24:59Z");
/// # let expected = serde_json::to_string(&expected).unwrap();
/// # assert_eq!(expected, actual_ser);
/// # let deser = serde_json::from_str::<Date>(&expected).unwrap();
/// # assert_eq!(actual, deser);
/// ```
/// # cbor example
///
/// ```rust
/// # use ciborium::Value;
/// # use vcon_types::Date;
/// let actual = "2022-09-23T23:24:59Z".parse::<Date>().unwrap();
/// # let actual_ser = Value::serialized(&actual).unwrap();
/// let expected = Value::Text("2022-09-23T23:24:59Z".into());
/// # assert_eq!(expected, actual_ser);
/// # let deser = Value::deserialized::<Date>(&actual_ser).unwrap();
/// # assert_eq!(actual, deser);
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
