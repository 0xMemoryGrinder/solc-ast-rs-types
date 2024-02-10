use serde::{Deserialize, Serialize};

mod source_unit_node_type;
pub use source_unit_node_type::*;
mod source_unit_nodes_item;
use crate::types::SourceLocation;
pub use source_unit_nodes_item::*;

#[doc = "SourceUnit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SourceUnit\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"absolutePath\","]
#[doc = "    \"exportedSymbols\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"nodes\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"absolutePath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"experimentalSolidity\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"exportedSymbols\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"license\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"SourceUnit\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ContractDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ImportDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/PragmaDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/StructDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/UserDefinedValueTypeDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/UsingForDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "          }"]
#[doc = "        ]"]
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
pub struct SourceUnit {
    #[serde(rename = "absolutePath")]
    pub absolute_path: String,
    #[serde(
        rename = "experimentalSolidity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_solidity: Option<bool>,
    #[serde(rename = "exportedSymbols", skip_serializing_if = "Option::is_none")]
    pub exported_symbols: Option<std::collections::HashMap<String, Vec<i64>>>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: SourceUnitNodeType,
    pub nodes: Vec<SourceUnitNodesItem>,
    pub src: SourceLocation,
}
impl From<&SourceUnit> for SourceUnit {
    fn from(value: &SourceUnit) -> Self {
        value.clone()
    }
}
