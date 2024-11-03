use crate::{Content, ContentParameters, Date, PartyEvent, PartyIndex};
use derive_more::{From, Into};

pub type DialogIndex = u32;

/// Dialog
///
/// See
/// - https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-dialog-object
#[derive(Debug, Clone, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct DialogObject {
    pub start: Date,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub party_history: Option<Vec<PartyEvent>>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub campaign: Option<String>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub interaction: Option<String>,
    #[cfg_attr(ser, serde(flatten))]
    pub dialog: Dialog,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    ser,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase", tag = "type")
)]
pub enum Dialog {
    Recording {
        #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
        duration: Option<Duration>,
        parties: DialogParties,
        #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
        originator: Option<PartyIndex>,
        #[cfg_attr(ser, serde(flatten))]
        content_parameters: ContentParameters,
        #[cfg_attr(ser, serde(flatten))]
        content: Content,
    },
    Text {
        #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
        duration: Option<Duration>,
        parties: DialogParties,
        #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
        originator: Option<PartyIndex>,
        #[cfg_attr(ser, serde(flatten))]
        content_parameters: ContentParameters,
        #[cfg_attr(ser, serde(flatten))]
        content: Content,
    },
    Transfer {
        transferee: PartyIndex,
        transferor: PartyIndex,
        transfer_target: PartyIndex,
        original: DialogIndex,
        #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
        consultation: Option<DialogIndex>,
        target_dialog: DialogIndex,
    },
    Incomplete {
        disposition: String,
    },
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum DialogParties {
    Index(PartyIndex),
    List(Vec<PartyIndex>),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Duration {
    Int(u32),
    Float(f32),
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use vcon_types::{
        Content, ContentParameters, Dialog, DialogObject, DialogParties, Duration, InlineContent,
    };

    #[test]
    #[ignore]
    fn json_text() {
        let actual = DialogObject {
            start: "2022-09-23T23:24:59Z".parse().unwrap(),
            party_history: Default::default(),
            campaign: Default::default(),
            interaction: Default::default(),
            dialog: Dialog::Text {
                duration: Some(Duration::Int(0)),
                parties: DialogParties::List(vec![0, 1]),
                content_parameters: ContentParameters {
                    mime: Some("text/plain".to_string().into()),
                    filename: None,
                },
                originator: Default::default(),
                content: Content::Inline(InlineContent::TextNone("Hi Bob".into())),
            },
            // extension_object: Default::default(),
        };
        let actual_ser = serde_json::to_value(&actual).unwrap();
        let expected = json!({
            "start": "2022-09-23T23:24:59Z",
            "type": "text",
            "duration": 0,
            "parties": [0, 1],
            "mimetype": "text/plain",
            "encoding": "none",
            "body": "Hi Bob"
        });
        assert_eq!(expected, actual_ser);
        let deser = serde_json::from_value::<DialogObject>(expected).unwrap();
        assert_eq!(actual, deser);
    }
}
