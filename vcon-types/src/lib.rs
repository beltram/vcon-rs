//! see https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html

// TODO: remove once finished
#![allow(dead_code)]

mod address;
mod analysis;
mod attachment;
mod body;
mod content;
mod date;
mod dialog;
#[cfg(feature = "doctest")]
mod doc;
mod event;
mod extension_object;
mod mime;
mod party;
mod reference;
mod signature;
mod url;
mod uuid;
mod version;

#[cfg(all(feature = "doctest", feature = "cbor"))]
pub use doc::expect_cbor_eq;
#[cfg(all(feature = "doctest", feature = "json"))]
pub use doc::expect_json_eq;

#[cfg(ser)]
pub use extension_object::ExtensionObject;
pub use {
    address::CivicAddress,
    analysis::Analysis,
    attachment::Attachment,
    body::InlineContent,
    content::{Content, ContentParameters, UrlReferencedContent},
    date::Date,
    dialog::{Dialog, DialogIndex, DialogObject, DialogParties},
    event::{Event, PartyEvent},
    mime::Mime,
    party::Party,
    reference::{RedactedReference, VconReference},
    signature::Signature,
    url::Url,
    uuid::Uuid,
    version::VconVersion,
};

#[cfg(feature = "json")]
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct JsonAnyValue(serde_json::Value);

#[cfg(feature = "cbor")]
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct CborAnyValue(ciborium::Value);

#[cfg(feature = "cbor")]
impl Default for CborAnyValue {
    fn default() -> Self {
        todo!()
    }
}

#[cfg(feature = "cbor")]
impl Eq for CborAnyValue {}

#[cfg(feature = "cbor")]
impl std::hash::Hash for CborAnyValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

#[cfg(all(feature = "cbor", feature = "json"))]
compile_error!("feature \"cbor\" and feature \"json\" cannot be enabled at the same time");

type PartyIndex = u32;

#[derive(Debug, Clone, PartialEq, derive_more::From, derive_more::Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Vcon {
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    #[cfg_attr(ser, serde(flatten))]
    pub version: VconVersion,
    pub uuid: Uuid,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub subject: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub created_at: Option<Date>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub updated_at: Option<Date>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub redacted: Option<OrEmpty<RedactedReference>>,
    // FIXME: typo in draft
    #[cfg_attr(
        ser,
        serde(rename = "ammended", skip_serializing_if = "Option::is_none")
    )]
    pub amended: Option<OrEmpty<RedactedReference>>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub group: Option<Vec<VconReference>>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub parties: Option<Vec<Party>>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub dialog: Option<Vec<DialogObject>>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub attachments: Option<Vec<Attachment>>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub analysis: Option<Vec<Analysis>>,
    // #[cfg(ser)]
    // #[cfg_attr(ser, serde(flatten))]
    // pub extension_object: ExtensionObject,
}

/// Flatten at declaration site
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum OrEmpty<T> {
    Some(T),
    #[cfg(feature = "json")]
    None(JsonAnyValue),
    #[cfg(feature = "cbor")]
    None(CborAnyValue),
}
