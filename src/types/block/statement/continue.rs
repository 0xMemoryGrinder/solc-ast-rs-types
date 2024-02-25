use serde::{Deserialize, Serialize};

use crate::types::SourceLocation;

#[doc = "Continue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Continue\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Continue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: ContinueNodeType,
    pub src: SourceLocation,
}

impl From<&Continue> for Continue {
    fn from(value: &Continue) -> Self {
        value.clone()
    }
}

// Node Type
#[doc = "ContinueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Continue\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ContinueNodeType {
    Continue,
}

impl From<&ContinueNodeType> for ContinueNodeType {
    fn from(value: &ContinueNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ContinueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Continue => "Continue".to_string(),
        }
    }
}

impl std::str::FromStr for ContinueNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "Continue" => Ok(Self::Continue),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ContinueNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ContinueNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ContinueNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
