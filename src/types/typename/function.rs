use serde::{Deserialize, Serialize};

use crate::types::{ParameterList, SourceLocation, StateMutability, TypeDescriptions, Visibility};

#[doc = "FunctionTypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameterTypes\","]
#[doc = "    \"returnParameterTypes\","]
#[doc = "    \"src\","]
#[doc = "    \"stateMutability\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"FunctionTypeName\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameterTypes\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"returnParameterTypes\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateMutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/StateMutability\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"$ref\": \"#/definitions/Visibility\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionTypeName {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: FunctionTypeNameNodeType,
    #[serde(rename = "parameterTypes")]
    pub parameter_types: ParameterList,
    #[serde(rename = "returnParameterTypes")]
    pub return_parameter_types: ParameterList,
    pub src: SourceLocation,
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    pub visibility: Visibility,
}

impl From<&FunctionTypeName> for FunctionTypeName {
    fn from(value: &FunctionTypeName) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "FunctionTypeNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"FunctionTypeName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionTypeNameNodeType {
    FunctionTypeName,
}

impl From<&FunctionTypeNameNodeType> for FunctionTypeNameNodeType {
    fn from(value: &FunctionTypeNameNodeType) -> Self {
        value.clone()
    }
}

impl ToString for FunctionTypeNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionTypeName => "FunctionTypeName".to_string(),
        }
    }
}

impl std::str::FromStr for FunctionTypeNameNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "FunctionTypeName" => Ok(Self::FunctionTypeName),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for FunctionTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for FunctionTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for FunctionTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
