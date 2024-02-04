use serde::{Deserialize, Serialize};


#[doc = "LiteralKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"bool\","]
#[doc = "    \"number\","]
#[doc = "    \"string\","]
#[doc = "    \"hexString\","]
#[doc = "    \"unicodeString\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiteralKind {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "hexString")]
    HexString,
    #[serde(rename = "unicodeString")]
    UnicodeString,
}

impl From<&LiteralKind> for LiteralKind {
    fn from(value: &LiteralKind) -> Self {
        value.clone()
    }
}

impl ToString for LiteralKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Bool => "bool".to_string(),
            Self::Number => "number".to_string(),
            Self::String => "string".to_string(),
            Self::HexString => "hexString".to_string(),
            Self::UnicodeString => "unicodeString".to_string(),
        }
    }
}

impl std::str::FromStr for LiteralKind {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "bool" => Ok(Self::Bool),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "hexString" => Ok(Self::HexString),
            "unicodeString" => Ok(Self::UnicodeString),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for LiteralKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for LiteralKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for LiteralKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}