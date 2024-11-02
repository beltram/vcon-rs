use crate::{body::BodyEncoding, dialog::Duration, Date, DialogIndex, PartyIndex, Uuid};
use derive_more::{From, Into};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Analysis {
    #[cfg_attr(ser, serde(rename = "type"))]
    pub typ: String,
    pub dialog: DialogIndex,
    pub body: AnalysisBody,
    pub encoding: BodyEncoding,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub vendor: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub schema: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub product: Option<String>,
    // #[cfg(ser)]
    // #[cfg_attr(ser, serde(flatten))]
    // pub extension_object: ExtensionObject,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct AnalysisBody {
    pub metadata: Metadata,
    pub results: Results,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Metadata {
    pub transaction_key: String,
    pub request_id: Uuid,
    pub sha256: String, // TODO: type depending alg
    pub created: Date,
    pub duration: Duration,
    pub channels: u32,
    pub models: Vec<Uuid>,
    pub model_info: HashMap<Uuid, ModelInfo>,
}

impl std::hash::Hash for Metadata {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.transaction_key.hash(state);
        self.request_id.hash(state);
        self.sha256.hash(state);
        self.created.hash(state);
        self.channels.hash(state);
        self.models.hash(state);
        // TODO: handle duration & model info
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Results {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Channel {
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Alternative {
    pub transcript: String,
    // #[cfg_attr(ser, serde(with = "crate::serde_float"))]
    pub confidence: f32,
    pub words: Vec<Word>,
    #[cfg_attr(ser, serde(rename = "paragraphs"))]
    pub paragraph: Paragraph,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Word {
    pub word: String,
    pub start: f32,
    pub end: f32,
    pub confidence: f32,
    pub speaker: PartyIndex,
    pub speaker_confidence: f32,
    pub punctuated_word: String,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Paragraph {
    pub transcript: String,
    pub paragraphs: Vec<InnerParagraph>,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct InnerParagraph {
    pub sentences: Vec<Sentence>,
    pub speaker: u32,
    pub num_words: u32,
    pub start: f32,
    pub end: f32,
}

#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct Sentence {
    pub text: String,
    pub start: f32,
    pub end: f32,
}
