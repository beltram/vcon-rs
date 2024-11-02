#[cfg(ser)]
use crate::ExtensionObject;
use crate::{CivicAddress, Uuid};
use derive_more::{From, Into};

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct PartyObject {
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub tel: Option<String>,
    #[cfg_attr(ser, serde(rename = "str", skip_serializing_if = "Option::is_none"))]
    pub string: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub mailto: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub validation: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub gmlpos: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub civic_address: Option<CivicAddress>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub uuid: Option<Uuid>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub role: Option<String>,
    #[cfg(ser)]
    #[cfg_attr(ser, serde(flatten))]
    pub extension_object: ExtensionObject,
}
