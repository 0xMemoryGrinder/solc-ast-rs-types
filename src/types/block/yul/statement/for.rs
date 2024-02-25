use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, YulBlock, YulExpression};

#[doc = "YulForLoop"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"condition\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"post\","]
#[doc = "    \"pre\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulForLoop\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"post\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"pre\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
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
pub struct YulForLoop {
    pub body: YulBlock,
    pub condition: YulExpression,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulForLoopNodeType,
    pub post: YulBlock,
    pub pre: YulBlock,
    pub src: SourceLocation,
}

impl From<&YulForLoop> for YulForLoop {
    fn from(value: &YulForLoop) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "YulForLoopNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulForLoop\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulForLoopNodeType {
    YulForLoop,
}

impl From<&YulForLoopNodeType> for YulForLoopNodeType {
    fn from(value: &YulForLoopNodeType) -> Self {
        value.clone()
    }
}

impl ToString for YulForLoopNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulForLoop => "YulForLoop".to_string(),
        }
    }
}

impl std::str::FromStr for YulForLoopNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulForLoop" => Ok(Self::YulForLoop),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulForLoopNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulForLoopNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulForLoopNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
