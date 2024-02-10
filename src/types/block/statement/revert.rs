use serde::{Deserialize, Serialize};

use crate::types::{FunctionCall, SourceLocation};

#[doc = "RevertStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"errorCall\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"errorCall\": {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"RevertStatement\""]
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
#[serde(deny_unknown_fields)]
pub struct RevertStatement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "errorCall")]
    pub error_call: FunctionCall,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: RevertStatementNodeType,
    pub src: SourceLocation,
}

impl From<&RevertStatement> for RevertStatement {
    fn from(value: &RevertStatement) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "RevertStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"RevertStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RevertStatementNodeType {
    RevertStatement,
}

impl From<&RevertStatementNodeType> for RevertStatementNodeType {
    fn from(value: &RevertStatementNodeType) -> Self {
        value.clone()
    }
}

impl ToString for RevertStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::RevertStatement => "RevertStatement".to_string(),
        }
    }
}

impl std::str::FromStr for RevertStatementNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "RevertStatement" => Ok(Self::RevertStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for RevertStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for RevertStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for RevertStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
