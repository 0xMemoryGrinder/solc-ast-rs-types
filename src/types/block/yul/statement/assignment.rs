use serde::{Deserialize, Serialize};

use crate::{SourceLocation, YulExpression, YulIdentifier};


#[doc = "YulAssignment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"value\","]
#[doc = "    \"variableNames\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulAssignment\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"variableNames\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulIdentifier\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulAssignment {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulAssignmentNodeType,
    pub src: SourceLocation,
    pub value: YulExpression,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<YulIdentifier>,
}

impl From<&YulAssignment> for YulAssignment {
    fn from(value: &YulAssignment) -> Self {
        value.clone()
    }
}



// Node type
#[doc = "YulAssignmentNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulAssignment\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulAssignmentNodeType {
    YulAssignment,
}

impl From<&YulAssignmentNodeType> for YulAssignmentNodeType {
    fn from(value: &YulAssignmentNodeType) -> Self {
        value.clone()
    }
}

impl ToString for YulAssignmentNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulAssignment => "YulAssignment".to_string(),
        }
    }
}

impl std::str::FromStr for YulAssignmentNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulAssignment" => Ok(Self::YulAssignment),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulAssignmentNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulAssignmentNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulAssignmentNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}