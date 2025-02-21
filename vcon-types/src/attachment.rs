use crate::{Content, ContentParameters, Date, PartyIndex};
use derive_more::{From, Into};

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Attachment {
    #[cfg_attr(ser, serde(rename = "type"))]
    pub typ: String,
    pub start: Date,
    pub party: PartyIndex,
    #[cfg_attr(ser, serde(flatten))]
    pub content_parameters: ContentParameters,
    #[cfg_attr(ser, serde(flatten))]
    pub content: Content,
    #[cfg(json)]
    #[cfg_attr(json, serde(flatten))]
    pub extension_object: crate::JsonAnyValue,
}
