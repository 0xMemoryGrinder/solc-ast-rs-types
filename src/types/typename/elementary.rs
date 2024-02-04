use serde::{Deserialize, Serialize};

use crate::{SourceLocation, StateMutability, TypeDescriptions};

#[doc = "ElementaryTypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ElementaryTypeName\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateMutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/StateMutability\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementaryTypeName {
    pub id: i64,
    pub name: String,
    #[serde(rename = "nodeType")]
    pub node_type: ElementaryTypeNameNodeType,
    pub src: SourceLocation,
    #[serde(
        rename = "stateMutability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_mutability: Option<StateMutability>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&ElementaryTypeName> for ElementaryTypeName {
    fn from(value: &ElementaryTypeName) -> Self {
        value.clone()
    }
}



// Node type
#[doc = "ElementaryTypeNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ElementaryTypeName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ElementaryTypeNameNodeType {
    ElementaryTypeName,
}

impl From<&ElementaryTypeNameNodeType> for ElementaryTypeNameNodeType {
    fn from(value: &ElementaryTypeNameNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ElementaryTypeNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ElementaryTypeName => "ElementaryTypeName".to_string(),
        }
    }
}

impl std::str::FromStr for ElementaryTypeNameNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ElementaryTypeName" => Ok(Self::ElementaryTypeName),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ElementaryTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ElementaryTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ElementaryTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}