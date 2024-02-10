use serde::{Deserialize, Serialize};

use crate::types::SourceLocation;

#[doc = "YulLiteralHexValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"hexValue\","]
#[doc = "    \"kind\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"hexValue\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"number\","]
#[doc = "        \"string\","]
#[doc = "        \"bool\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulLiteral\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulLiteralHexValue {
    #[serde(rename = "hexValue")]
    pub hex_value: String,
    pub kind: YulLiteralHexValueKind,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulLiteralHexValueNodeType,
    pub src: SourceLocation,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl From<&YulLiteralHexValue> for YulLiteralHexValue {
    fn from(value: &YulLiteralHexValue) -> Self {
        value.clone()
    }
}

#[doc = "YulLiteralHexValueKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"number\","]
#[doc = "    \"string\","]
#[doc = "    \"bool\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLiteralHexValueKind {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bool")]
    Bool,
}

impl From<&YulLiteralHexValueKind> for YulLiteralHexValueKind {
    fn from(value: &YulLiteralHexValueKind) -> Self {
        value.clone()
    }
}

impl ToString for YulLiteralHexValueKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Number => "number".to_string(),
            Self::String => "string".to_string(),
            Self::Bool => "bool".to_string(),
        }
    }
}

impl std::str::FromStr for YulLiteralHexValueKind {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "bool" => Ok(Self::Bool),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulLiteralHexValueKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulLiteralHexValueKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulLiteralHexValueKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

// Node type
#[doc = "YulLiteralHexValueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulLiteral\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLiteralHexValueNodeType {
    YulLiteral,
}

impl From<&YulLiteralHexValueNodeType> for YulLiteralHexValueNodeType {
    fn from(value: &YulLiteralHexValueNodeType) -> Self {
        value.clone()
    }
}

impl ToString for YulLiteralHexValueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulLiteral => "YulLiteral".to_string(),
        }
    }
}

impl std::str::FromStr for YulLiteralHexValueNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulLiteral" => Ok(Self::YulLiteral),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulLiteralHexValueNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulLiteralHexValueNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulLiteralHexValueNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
