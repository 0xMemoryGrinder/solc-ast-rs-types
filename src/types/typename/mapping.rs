use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, TypeDescriptions, TypeName};

#[doc = "Mapping"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"keyType\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"valueType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"keyName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"keyNameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"keyType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Mapping\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"valueName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"valueNameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"valueType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mapping {
    pub id: i64,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(
        rename = "keyNameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_name_location: Option<String>,
    #[serde(rename = "keyType")]
    pub key_type: Box<TypeName>,
    #[serde(rename = "nodeType")]
    pub node_type: MappingNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "valueName", default, skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
    #[serde(
        rename = "valueNameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub value_name_location: Option<String>,
    #[serde(rename = "valueType")]
    pub value_type: Box<TypeName>,
}

impl From<&Mapping> for Mapping {
    fn from(value: &Mapping) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "MappingNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Mapping\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MappingNodeType {
    Mapping,
}

impl From<&MappingNodeType> for MappingNodeType {
    fn from(value: &MappingNodeType) -> Self {
        value.clone()
    }
}

impl ToString for MappingNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Mapping => "Mapping".to_string(),
        }
    }
}

impl std::str::FromStr for MappingNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "Mapping" => Ok(Self::Mapping),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for MappingNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for MappingNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for MappingNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
