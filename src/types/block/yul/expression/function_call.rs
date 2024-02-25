use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, YulExpression, YulIdentifier};

#[doc = "YulFunctionCall"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"arguments\","]
#[doc = "    \"functionName\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"functionName\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulIdentifier\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulFunctionCall\""]
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
pub struct YulFunctionCall {
    pub arguments: Vec<YulExpression>,
    #[serde(rename = "functionName")]
    pub function_name: YulIdentifier,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulFunctionCallNodeType,
    pub src: SourceLocation,
}

impl From<&YulFunctionCall> for YulFunctionCall {
    fn from(value: &YulFunctionCall) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "YulFunctionCallNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulFunctionCall\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulFunctionCallNodeType {
    YulFunctionCall,
}

impl From<&YulFunctionCallNodeType> for YulFunctionCallNodeType {
    fn from(value: &YulFunctionCallNodeType) -> Self {
        value.clone()
    }
}

impl ToString for YulFunctionCallNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulFunctionCall => "YulFunctionCall".to_string(),
        }
    }
}

impl std::str::FromStr for YulFunctionCallNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulFunctionCall" => Ok(Self::YulFunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulFunctionCallNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulFunctionCallNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulFunctionCallNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
