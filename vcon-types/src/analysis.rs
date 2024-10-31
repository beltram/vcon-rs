#[cfg(ser)]
use crate::ExtensionObject;
use crate::{Content, ContentParameters, DialogIndex};
use derive_more::{From, Into};

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct AnalysisObject {
    #[cfg_attr(ser, serde(rename = "type"))]
    pub typ: String,
    pub dialog: DialogIndex,
    #[cfg_attr(ser, serde(flatten))]
    pub content_parameters: ContentParameters,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub vendor: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub product: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub schema: Option<String>,
    #[cfg_attr(ser, serde(flatten))]
    pub content: Content,
    #[cfg(ser)]
    #[cfg_attr(ser, serde(flatten))]
    pub extension_object: ExtensionObject,
}
