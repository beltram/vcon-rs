#[cfg(ser)]
use crate::ExtensionObject;
use derive_more::{From, Into};

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct CivicAddress {
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub country: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub a1: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub a2: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub a3: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub a4: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub a5: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub a6: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub prd: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub pod: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub sts: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub hno: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub hns: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub lmk: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub loc: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub flr: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub nam: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub pc: Option<String>,
    #[cfg(ser)]
    #[cfg_attr(ser, serde(flatten))]
    pub extension_object: ExtensionObject,
}
