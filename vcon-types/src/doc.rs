#[cfg(feature = "cbor")]
pub fn expect_cbor_eq<T>(actual: T, expected: ciborium::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + std::cmp::PartialEq,
{
    use ciborium::Value;
    let actual_ser = Value::serialized(&actual).unwrap();
    assert_eq!(expected, actual_ser);
    let deser = Value::deserialized::<T>(&actual_ser).unwrap();
    assert_eq!(actual, deser);
}

#[cfg(feature = "json")]
pub fn expect_json_eq<T>(actual: T, expected: serde_json::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + std::cmp::PartialEq,
{
    let actual_ser = serde_json::to_string(&actual).unwrap();
    let expected = serde_json::to_string(&expected).unwrap();
    assert_eq!(expected, actual_ser);
    let deser = serde_json::from_str::<T>(&expected).unwrap();
    assert_eq!(actual, deser);
}
