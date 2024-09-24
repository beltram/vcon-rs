#![cfg(ser)]

use derive_more::{Deref, DerefMut, From, Into};

// Flatten at declaration site
#[derive(
    Default, Debug, Clone, serde::Serialize, serde::Deserialize, From, Into, Deref, DerefMut,
)]
#[cfg_attr(feature = "json", derive(Eq, PartialEq))]
#[repr(transparent)]
#[serde(transparent)]
pub struct ExtensionObject(std::collections::HashMap<String, crate::AnyValue>);

impl std::hash::Hash for ExtensionObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, _v) in &self.0 {
            k.hash(state);
            #[cfg(feature = "json")]
            _v.hash(state);
        }
    }
}

#[cfg(feature = "cbor")]
impl PartialEq<Self> for ExtensionObject {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}

#[cfg(feature = "cbor")]
impl Eq for ExtensionObject {}
