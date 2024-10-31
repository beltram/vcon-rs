/// Signature
///
/// TODO: supposedly SHA-512 and base64Url encoded, discuss before implementing
///
/// See
/// - https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-signature
/// - https://ietf-wg-vcon.github.io/draft-ietf-vcon-vcon-container/draft-ietf-vcon-vcon-container.html#name-alg
///
/// # Usage
///
/// `#[serde(flatten)]` at declaration site
///
/// # json example
///
/// ```rust
/// # #[cfg(feature = "json")] {
/// # use vcon_types::Signature;
/// # use serde_json::json;
/// # vcon_types::expect_json_eq(
/// "2AIvIGCtbv0perc9zFNVybIUBUsNF3ahNqZp0mp9OxT3OqDQ6_8Z7jMzaPAWS2QZqW2knj5IF1Pn6Wtxa9zLbw".parse::<Signature>().unwrap(), // actual
/// json!({
///     "alg": "SHA-512",
///     "signature": "2AIvIGCtbv0perc9zFNVybIUBUsNF3ahNqZp0mp9OxT3OqDQ6_8Z7jMzaPAWS2QZqW2knj5IF1Pn6Wtxa9zLbw"
/// }), // expected
/// # )}
/// ```
/// # cbor example
///
/// ```rust
/// # #[cfg(feature = "cbor")] {
/// # use vcon_types::Signature;
/// # use ciborium::cbor;
/// # vcon_types::expect_cbor_eq(
/// "2AIvIGCtbv0perc9zFNVybIUBUsNF3ahNqZp0mp9OxT3OqDQ6_8Z7jMzaPAWS2QZqW2knj5IF1Pn6Wtxa9zLbw".parse::<Signature>().unwrap(), // actual
/// cbor!({ 
///     "alg" => "SHA-512",
///     "signature" => "2AIvIGCtbv0perc9zFNVybIUBUsNF3ahNqZp0mp9OxT3OqDQ6_8Z7jMzaPAWS2QZqW2knj5IF1Pn6Wtxa9zLbw" 
/// }) // expected
/// # .unwrap(),
/// # )}
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Signature {
    Sha256 {
        signature: [u8; Signature::SHA256_OUTPUT_SIZE],
    },
    Sha384 {
        signature: [u8; Signature::SHA384_OUTPUT_SIZE],
    },
    Sha512 {
        signature: [u8; Signature::SHA512_OUTPUT_SIZE],
    },
}

impl Signature {
    // cannot be access at compile time so we do debug assertions to verify we haven't messed up
    const SHA512_OUTPUT_SIZE: usize = 64;
    const SHA384_OUTPUT_SIZE: usize = 48;
    const SHA256_OUTPUT_SIZE: usize = 32;
    const B64: base64::engine::GeneralPurpose = base64::prelude::BASE64_URL_SAFE_NO_PAD;

    #[cfg(debug_assertions)]
    fn verify_hash_output_sizes() {
        debug_assert_eq!(
            Self::SHA512_OUTPUT_SIZE,
            <sha2::Sha512VarCore as sha2::digest::crypto_common::OutputSizeUser>::output_size()
        );
        debug_assert_eq!(
            Self::SHA384_OUTPUT_SIZE,
            <sha2::Sha384 as sha2::digest::crypto_common::OutputSizeUser>::output_size()
        );
        debug_assert_eq!(
            Self::SHA256_OUTPUT_SIZE,
            <sha2::Sha256VarCore as sha2::digest::crypto_common::OutputSizeUser>::output_size()
        );
    }

    fn signature(&self) -> String {
        use base64::Engine as _;
        match self {
            Signature::Sha256 { signature } => Self::B64.encode(signature),
            Signature::Sha384 { signature } => Self::B64.encode(signature),
            Signature::Sha512 { signature } => Self::B64.encode(signature),
        }
    }

    fn alg(&self) -> SignatureAlg {
        match self {
            Self::Sha256 { .. } => SignatureAlg::Sha256,
            Self::Sha384 { .. } => SignatureAlg::Sha384,
            Self::Sha512 { .. } => SignatureAlg::Sha512,
        }
    }
}

#[cfg(ser)]
impl serde::Serialize for Signature {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("alg", &self.alg())?;
        map.serialize_entry("signature", &self.signature())?;
        map.end()
    }
}

#[cfg(ser)]
impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Signature, D::Error> {
        struct SignatureVisitor;

        impl<'de> serde::de::Visitor<'de> for SignatureVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a Map of Signature with content and alg")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                use base64::Engine as _;
                use serde::de::Error as _;

                let (_, alg) = map
                    .next_entry::<String, SignatureAlg>()?
                    .ok_or(A::Error::custom("Invalid Signature alg serialization"))?;
                let (_, signature) = map
                    .next_entry::<String, String>()?
                    .ok_or(A::Error::custom("Invalid BinaryBody content serialization"))?;
                let signature = Signature::B64
                    .decode(signature.as_bytes())
                    .map_err(|_| A::Error::custom("Signature not Base64 encoded"))?;
                Ok(match alg {
                    SignatureAlg::Sha256 => Self::Value::Sha256 {
                        signature: signature.try_into().map_err(|_| {
                            A::Error::custom("Expected a SHA-256 signature to be 32 bytes long")
                        })?,
                    },
                    SignatureAlg::Sha384 => {
                        let signature = signature.try_into().map_err(|_| {
                            A::Error::custom("Expected a SHA-384 signature to be 48 bytes long")
                        })?;
                        Self::Value::Sha384 { signature }
                    }
                    SignatureAlg::Sha512 => {
                        let signature = signature.try_into().map_err(|_| {
                            A::Error::custom("Expected a SHA-512 signature to be 64 bytes long")
                        })?;
                        Self::Value::Sha512 { signature }
                    }
                })
            }
        }

        deserializer.deserialize_map(SignatureVisitor)
    }
}

impl std::str::FromStr for Signature {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(debug_assertions)]
        Self::verify_hash_output_sizes();

        use base64::Engine as _;
        let decoded = Self::B64
            .decode(s)
            .map_err(|_| Self::Err::from("signature not base64Url encoded"))?;
        Ok(match decoded.len() {
            Self::SHA256_OUTPUT_SIZE => {
                let signature = decoded
                    .try_into()
                    .map_err(|_| Self::Err::from("implementation error"))?;
                Self::Sha256 { signature }
            }
            Self::SHA384_OUTPUT_SIZE => {
                let signature = decoded
                    .try_into()
                    .map_err(|_| Self::Err::from("implementation error"))?;
                Self::Sha384 { signature }
            }
            Self::SHA512_OUTPUT_SIZE => {
                let signature = decoded
                    .try_into()
                    .map_err(|_| Self::Err::from("implementation error"))?;
                Self::Sha512 { signature }
            }
            _ => return Err("unexpected signature digest length".into()),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum SignatureAlg {
    Sha256,
    Sha384,
    Sha512,
}

#[cfg(ser)]
impl serde::Serialize for SignatureAlg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let alg = match self {
            SignatureAlg::Sha256 => "SHA-256",
            SignatureAlg::Sha384 => "SHA-384",
            SignatureAlg::Sha512 => "SHA-512",
        };
        serializer.serialize_str(alg)
    }
}

#[cfg(ser)]
impl<'de> serde::de::Deserialize<'de> for SignatureAlg {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "SHA-256" => Self::Sha256,
            "SHA-384" => Self::Sha384,
            "SHA-512" => Self::Sha512,
            _ => return Err(D::Error::custom("Unknown signature algorithm")),
        })
    }
}
