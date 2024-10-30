#![cfg(ser)]

use derive_more::{Deref, DerefMut, From, Into};

// Flatten at declaration site
#[cfg(feature = "json")]
#[derive(
    Default, Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize, From, Into, Deref, DerefMut,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ExtensionObject(crate::JsonAnyValue);

#[cfg(feature = "cbor")]
#[derive(
    Default, Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize, From, Into, Deref, DerefMut,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ExtensionObject(crate::CborAnyValue);
