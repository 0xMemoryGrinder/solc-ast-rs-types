use serde::{Deserialize, Serialize};

mod enum_value;
pub use enum_value::*;

use crate::types::{SourceLocation, StructuredDocumentation};

#[doc = "EnumDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"canonicalName\","]
#[doc = "    \"id\","]
#[doc = "    \"members\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonicalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"members\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/EnumValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"EnumDefinition\""]
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
pub struct EnumDefinition {
    #[serde(rename = "canonicalName")]
    pub canonical_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    pub id: i64,
    pub members: Vec<EnumValue>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: EnumDefinitionNodeType,
    pub src: SourceLocation,
}

impl From<&EnumDefinition> for EnumDefinition {
    fn from(value: &EnumDefinition) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "EnumDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EnumDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EnumDefinitionNodeType {
    EnumDefinition,
}

impl From<&EnumDefinitionNodeType> for EnumDefinitionNodeType {
    fn from(value: &EnumDefinitionNodeType) -> Self {
        value.clone()
    }
}

impl ToString for EnumDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EnumDefinition => "EnumDefinition".to_string(),
        }
    }
}

impl std::str::FromStr for EnumDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "EnumDefinition" => Ok(Self::EnumDefinition),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for EnumDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for EnumDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for EnumDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
