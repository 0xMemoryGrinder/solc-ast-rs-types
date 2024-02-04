use serde::{Deserialize, Serialize};

use crate::{FunctionCall, SourceLocation};

#[doc = "EmitStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"eventCall\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"eventCall\": {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"EmitStatement\""]
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
pub struct EmitStatement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "eventCall")]
    pub event_call: FunctionCall,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: EmitStatementNodeType,
    pub src: SourceLocation,
}

impl From<&EmitStatement> for EmitStatement {
    fn from(value: &EmitStatement) -> Self {
        value.clone()
    }
}
#[doc = "EmitStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EmitStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EmitStatementNodeType {
    EmitStatement,
}

impl From<&EmitStatementNodeType> for EmitStatementNodeType {
    fn from(value: &EmitStatementNodeType) -> Self {
        value.clone()
    }
}

impl ToString for EmitStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EmitStatement => "EmitStatement".to_string(),
        }
    }
}

impl std::str::FromStr for EmitStatementNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "EmitStatement" => Ok(Self::EmitStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for EmitStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for EmitStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for EmitStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}