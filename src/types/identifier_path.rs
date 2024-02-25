use serde::{Deserialize, Serialize};

use crate::types::SourceLocation;

#[doc = "IdentifierPath"]
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
#[doc = "    \"referencedDeclaration\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocations\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"IdentifierPath\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"referencedDeclaration\": {"]
#[doc = "      \"type\": \"integer\""]
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
pub struct IdentifierPath {
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name_locations: Vec<String>,
    #[serde(rename = "nodeType")]
    pub node_type: IdentifierPathNodeType,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,
    pub src: SourceLocation,
}

impl From<&IdentifierPath> for IdentifierPath {
    fn from(value: &IdentifierPath) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "IdentifierPathNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"IdentifierPath\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IdentifierPathNodeType {
    IdentifierPath,
}

impl From<&IdentifierPathNodeType> for IdentifierPathNodeType {
    fn from(value: &IdentifierPathNodeType) -> Self {
        value.clone()
    }
}

impl ToString for IdentifierPathNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::IdentifierPath => "IdentifierPath".to_string(),
        }
    }
}

impl std::str::FromStr for IdentifierPathNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "IdentifierPath" => Ok(Self::IdentifierPath),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for IdentifierPathNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for IdentifierPathNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for IdentifierPathNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
