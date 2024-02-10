use serde::{Deserialize, Serialize};

mod inheritance_specifier;
pub use inheritance_specifier::*;
mod contract_kind;
pub use contract_kind::*;
mod nodes;
pub use nodes::*;

use crate::types::{SourceLocation, StructuredDocumentation};

#[doc = "ContractDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"abstract\","]
#[doc = "    \"baseContracts\","]
#[doc = "    \"contractDependencies\","]
#[doc = "    \"contractKind\","]
#[doc = "    \"fullyImplemented\","]
#[doc = "    \"id\","]
#[doc = "    \"linearizedBaseContracts\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"nodes\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"abstract\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"baseContracts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/InheritanceSpecifier\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"canonicalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"contractDependencies\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"contractKind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"contract\","]
#[doc = "        \"interface\","]
#[doc = "        \"library\""]
#[doc = "      ]"]
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
#[doc = "    \"fullyImplemented\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalFunctionIDs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"linearizedBaseContracts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
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
#[doc = "        \"ContractDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EventDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ModifierDefinition\""]
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
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"usedErrors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"usedEvents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractDefinition {
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    #[serde(rename = "baseContracts")]
    pub base_contracts: Vec<InheritanceSpecifier>,
    #[serde(
        rename = "canonicalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub canonical_name: Option<String>,
    #[serde(rename = "contractDependencies")]
    pub contract_dependencies: Vec<i64>,
    #[serde(rename = "contractKind")]
    pub contract_kind: ContractDefinitionContractKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(rename = "fullyImplemented")]
    pub fully_implemented: bool,
    pub id: i64,
    #[serde(
        rename = "internalFunctionIDs",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub internal_function_i_ds: std::collections::HashMap<String, i64>,
    #[serde(rename = "linearizedBaseContracts")]
    pub linearized_base_contracts: Vec<i64>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: ContractDefinitionNodeType,
    pub nodes: Vec<ContractDefinitionNodesItem>,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(rename = "usedErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub used_errors: Vec<i64>,
    #[serde(rename = "usedEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub used_events: Vec<i64>,
}

impl From<&ContractDefinition> for ContractDefinition {
    fn from(value: &ContractDefinition) -> Self {
        value.clone()
    }
}

// Node Type
#[doc = "ContractDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ContractDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ContractDefinitionNodeType {
    ContractDefinition,
}

impl From<&ContractDefinitionNodeType> for ContractDefinitionNodeType {
    fn from(value: &ContractDefinitionNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ContractDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ContractDefinition => "ContractDefinition".to_string(),
        }
    }
}

impl std::str::FromStr for ContractDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ContractDefinition" => Ok(Self::ContractDefinition),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ContractDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ContractDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ContractDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
