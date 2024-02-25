use serde::{Deserialize, Serialize};

mod statement;
pub use statement::*;
mod expression;
pub use expression::*;
mod yul;
pub use yul::*;

use crate::types::SourceLocation;

#[doc = "Block"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Block\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"statements\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Statement\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: BlockNodeType,
    pub src: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<Statement>>,
}

impl From<&Block> for Block {
    fn from(value: &Block) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "BlockNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Block\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BlockNodeType {
    Block,
}

impl From<&BlockNodeType> for BlockNodeType {
    fn from(value: &BlockNodeType) -> Self {
        value.clone()
    }
}

impl ToString for BlockNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "Block".to_string(),
        }
    }
}

impl std::str::FromStr for BlockNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "Block" => Ok(Self::Block),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for BlockNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for BlockNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for BlockNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
