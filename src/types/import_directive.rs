use serde::{Deserialize, Serialize};

mod symbol_alias_item;
pub use symbol_alias_item::*;

use crate::types::SourceLocation;

#[doc = "ImportDirective"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"absolutePath\","]
#[doc = "    \"file\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"scope\","]
#[doc = "    \"sourceUnit\","]
#[doc = "    \"src\","]
#[doc = "    \"symbolAliases\","]
#[doc = "    \"unitAlias\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"absolutePath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ImportDirective\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"sourceUnit\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"symbolAliases\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"foreign\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"foreign\": {"]
#[doc = "            \"$ref\": \"#/definitions/Identifier\""]
#[doc = "          },"]
#[doc = "          \"local\": {"]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"null\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"nameLocation\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"unitAlias\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportDirective {
    #[serde(rename = "absolutePath")]
    pub absolute_path: String,
    pub file: String,
    pub id: i64,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: ImportDirectiveNodeType,
    pub scope: i64,
    #[serde(rename = "sourceUnit")]
    pub source_unit: i64,
    pub src: SourceLocation,
    #[serde(rename = "symbolAliases")]
    pub symbol_aliases: Vec<ImportDirectiveSymbolAliasesItem>,
    #[serde(rename = "unitAlias")]
    pub unit_alias: String,
}

impl From<&ImportDirective> for ImportDirective {
    fn from(value: &ImportDirective) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "ImportDirectiveNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ImportDirective\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ImportDirectiveNodeType {
    ImportDirective,
}

impl From<&ImportDirectiveNodeType> for ImportDirectiveNodeType {
    fn from(value: &ImportDirectiveNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ImportDirectiveNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ImportDirective => "ImportDirective".to_string(),
        }
    }
}

impl std::str::FromStr for ImportDirectiveNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ImportDirective" => Ok(Self::ImportDirective),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ImportDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ImportDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ImportDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
