use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, YulBlock, YulTypedName};

#[doc = "YulFunctionDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"returnVariables\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulFunctionDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulTypedName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"returnVariables\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulTypedName\""]
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
#[serde(deny_unknown_fields)]
pub struct YulFunctionDefinition {
    pub body: YulBlock,
    pub name: String,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulFunctionDefinitionNodeType,
    pub parameters: Vec<YulTypedName>,
    #[serde(rename = "returnVariables")]
    pub return_variables: Vec<YulTypedName>,
    pub src: SourceLocation,
}

impl From<&YulFunctionDefinition> for YulFunctionDefinition {
    fn from(value: &YulFunctionDefinition) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "YulFunctionDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulFunctionDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulFunctionDefinitionNodeType {
    YulFunctionDefinition,
}

impl From<&YulFunctionDefinitionNodeType> for YulFunctionDefinitionNodeType {
    fn from(value: &YulFunctionDefinitionNodeType) -> Self {
        value.clone()
    }
}

impl ToString for YulFunctionDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulFunctionDefinition => "YulFunctionDefinition".to_string(),
        }
    }
}

impl std::str::FromStr for YulFunctionDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulFunctionDefinition" => Ok(Self::YulFunctionDefinition),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulFunctionDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulFunctionDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulFunctionDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
