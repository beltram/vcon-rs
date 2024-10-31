#[cfg(ser)]
use crate::ExtensionObject;
use crate::{Date, PartyIndex};
use derive_more::{From, Into};

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct PartyEvent {
    pub party: PartyIndex,
    pub event: Event,
    pub time: Date,
    #[cfg(ser)]
    #[cfg_attr(ser, serde(flatten))]
    pub extension_object: ExtensionObject,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(
    ser,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Event {
    Join,
    Drop,
    Hold,
    Unhold,
    Mute,
    Unmute,
}
