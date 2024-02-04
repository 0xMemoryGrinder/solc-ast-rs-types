use serde::{Deserialize, Serialize};

#[doc = "SourceUnitNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"SourceUnit\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SourceUnitNodeType {
    SourceUnit,
}

impl From<&SourceUnitNodeType> for SourceUnitNodeType {
    fn from(value: &SourceUnitNodeType) -> Self {
        value.clone()
    }
}

impl ToString for SourceUnitNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::SourceUnit => "SourceUnit".to_string(),
        }
    }
}

impl std::str::FromStr for SourceUnitNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "SourceUnit" => Ok(Self::SourceUnit),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for SourceUnitNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for SourceUnitNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for SourceUnitNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}