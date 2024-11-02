use crate::{InlineContent, UrlReferencedContent, Uuid};
use derive_more::{From, Into};

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct RedactedReference {
    #[cfg_attr(ser, serde(rename = "type", skip_serializing_if = "Option::is_none"))]
    pub typ: Option<String>,
    #[cfg_attr(ser, serde(flatten))]
    pub vcon_reference: VconReference,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(
    ser,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum VconReference {
    Url {
        vcon_url_referenced: UrlReferencedContent,
    },
    Inline {
        inline_content: InlineContent,
    },
    Uuid {
        uuid: Uuid,
    },
}
