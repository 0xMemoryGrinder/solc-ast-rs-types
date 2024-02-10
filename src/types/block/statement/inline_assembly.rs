use serde::{Deserialize, Serialize};

mod evm_version;
pub use evm_version::*;
mod external_references_item;
pub use external_references_item::*;
mod flags_item;
pub use flags_item::*;

use crate::types::{SourceLocation, YulBlock};

#[doc = "InlineAssembly"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"AST\","]
#[doc = "    \"evmVersion\","]
#[doc = "    \"externalReferences\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AST\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"evmVersion\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"homestead\","]
#[doc = "        \"tangerineWhistle\","]
#[doc = "        \"spuriousDragon\","]
#[doc = "        \"byzantium\","]
#[doc = "        \"constantinople\","]
#[doc = "        \"petersburg\","]
#[doc = "        \"istanbul\","]
#[doc = "        \"berlin\","]
#[doc = "        \"london\","]
#[doc = "        \"paris\","]
#[doc = "        \"shanghai\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"externalReferences\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"declaration\","]
#[doc = "          \"isOffset\","]
#[doc = "          \"isSlot\","]
#[doc = "          \"src\","]
#[doc = "          \"valueSize\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"declaration\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"isOffset\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"isSlot\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"src\": {"]
#[doc = "            \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "          },"]
#[doc = "          \"suffix\": {"]
#[doc = "            \"enum\": ["]
#[doc = "              \"slot\","]
#[doc = "              \"offset\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"valueSize\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"enum\": ["]
#[doc = "          \"memory-safe\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"InlineAssembly\""]
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
#[serde(deny_unknown_fields)]
pub struct InlineAssembly {
    #[serde(rename = "AST")]
    pub ast: YulBlock,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "evmVersion")]
    pub evm_version: InlineAssemblyEvmVersion,
    #[serde(rename = "externalReferences")]
    pub external_references: Vec<InlineAssemblyExternalReferencesItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<InlineAssemblyFlagsItem>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: InlineAssemblyNodeType,
    pub src: SourceLocation,
}

impl From<&InlineAssembly> for InlineAssembly {
    fn from(value: &InlineAssembly) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "InlineAssemblyNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"InlineAssembly\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyNodeType {
    InlineAssembly,
}

impl From<&InlineAssemblyNodeType> for InlineAssemblyNodeType {
    fn from(value: &InlineAssemblyNodeType) -> Self {
        value.clone()
    }
}

impl ToString for InlineAssemblyNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::InlineAssembly => "InlineAssembly".to_string(),
        }
    }
}

impl std::str::FromStr for InlineAssemblyNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "InlineAssembly" => Ok(Self::InlineAssembly),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for InlineAssemblyNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for InlineAssemblyNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for InlineAssemblyNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
