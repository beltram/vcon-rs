use crate::{InlineContent, Mime, Signature, Url};
use derive_more::{From, Into};
use crate::signature::SignatureAlg;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(ser, derive(serde::Serialize), serde(untagged))]
pub enum Content {
    Inline(InlineContent),
    UrlReferenced(UrlReferencedContent),
}

#[cfg(ser)]
impl<'de> serde::Deserialize<'de> for Content {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Content, D::Error> {
        struct ContentVisitor;

        impl<'de> serde::de::Visitor<'de> for ContentVisitor {
            type Value = Content;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a Map of Content with content and encoding")
            }

            #[cfg(feature = "json")]
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                use crate::body::BodyEncoding;
                use serde::de::Error as _;

                let (mut encoding, mut body, mut url, mut signature, mut alg) =
                    (None, None, None, None, None);

                while let Ok(Some((k, v))) = map.next_entry::<String, serde_json::Value>() {
                    match k.as_str() {
                        "encoding" => encoding = Some(v),
                        "body" => body = Some(v),
                        "url" => url = Some(v),
                        "signature" => signature = Some(v),
                        "alg" => alg = Some(v),
                        _ => {}
                    }
                }

                Ok(if let Some((encoding, body)) = encoding.zip(body) {
                    let encoding =
                        <BodyEncoding as serde::Deserialize>::deserialize(encoding).unwrap();
                    let inline_content = match encoding {
                        BodyEncoding::Base64Url => {
                            let body = <String as serde::Deserialize>::deserialize(body)
                                .map_err(A::Error::custom)?;
                            
                            use base64::Engine as _;
                            let body = InlineContent::B64
                                .decode(body.as_bytes())
                                .map_err(A::Error::custom)?;
                            InlineContent::BinaryBase64Url(body)
                        }
                        BodyEncoding::None => {
                            let body = <String as serde::Deserialize>::deserialize(body)
                                .map_err(A::Error::custom)?;
                            InlineContent::TextNone(body)
                        }
                        BodyEncoding::Json => {
                            let body = <String as serde::Deserialize>::deserialize(body)
                                .map_err(A::Error::custom)?;
                            InlineContent::TextJson(body)
                        }
                    };
                    Self::Value::Inline(inline_content)
                } else if let Some(((url, signature), alg)) = url.zip(signature).zip(alg) {
                    let url =
                        <Url as serde::Deserialize>::deserialize(url).map_err(A::Error::custom)?;
                    let alg = <SignatureAlg as serde::Deserialize>::deserialize(alg)
                        .map_err(A::Error::custom)?;
                    let signature = <String as serde::Deserialize>::deserialize(signature)
                        .map_err(A::Error::custom)?;
                    let signature =
                        Signature::try_from((alg, signature)).map_err(A::Error::custom)?;
                    Self::Value::UrlReferenced(UrlReferencedContent { url, signature })
                } else {
                    return Err(A::Error::custom(
                        "Invalid Content, must be either Inline or UrlReferenced",
                    ));
                })
            }

            #[cfg(feature = "cbor")]
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                todo!()
            }
        }

        deserializer.deserialize_map(ContentVisitor)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct UrlReferencedContent {
    pub url: Url,
    #[cfg_attr(ser, serde(flatten))]
    pub signature: Signature,
}

/// Flatten at declaration-site
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, From, Into)]
#[cfg_attr(ser, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
pub struct ContentParameters {
    #[cfg_attr(ser, serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub mime: Option<Mime>,
    #[cfg_attr(ser, serde(skip_serializing_if = "Option::is_none"))]
    pub filename: Option<String>,
}
