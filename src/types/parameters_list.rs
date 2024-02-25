use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, VariableDeclaration};

#[doc = "ParameterList"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ParameterList\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "      }"]
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
pub struct ParameterList {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: ParameterListNodeType,
    pub parameters: Vec<VariableDeclaration>,
    pub src: SourceLocation,
}

impl From<&ParameterList> for ParameterList {
    fn from(value: &ParameterList) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "ParameterListNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ParameterList\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParameterListNodeType {
    ParameterList,
}

impl From<&ParameterListNodeType> for ParameterListNodeType {
    fn from(value: &ParameterListNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ParameterListNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ParameterList => "ParameterList".to_string(),
        }
    }
}

impl std::str::FromStr for ParameterListNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ParameterList" => Ok(Self::ParameterList),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ParameterListNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ParameterListNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ParameterListNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
