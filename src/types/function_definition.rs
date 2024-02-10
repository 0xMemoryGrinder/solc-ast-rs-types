use serde::{Deserialize, Serialize};

mod function_kind;
pub use function_kind::*;
mod modifier_invocation;
pub use modifier_invocation::*;
mod state_mutability;
pub use state_mutability::*;

use crate::types::{
    Block, OverrideSpecifier, ParameterList, SourceLocation, StructuredDocumentation, Visibility,
};

#[doc = "FunctionDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"implemented\","]
#[doc = "    \"kind\","]
#[doc = "    \"modifiers\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"returnParameters\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\","]
#[doc = "    \"stateMutability\","]
#[doc = "    \"virtual\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseFunctions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"body\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Block\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
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
#[doc = "    \"functionSelector\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"implemented\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"function\","]
#[doc = "        \"receive\","]
#[doc = "        \"constructor\","]
#[doc = "        \"fallback\","]
#[doc = "        \"freeFunction\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modifiers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ModifierInvocation\""]
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
#[doc = "        \"FunctionDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overrides\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/OverrideSpecifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"returnParameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateMutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/StateMutability\""]
#[doc = "    },"]
#[doc = "    \"virtual\": {"]
#[doc = "      \"type\": \"boolean\""]
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
pub struct FunctionDefinition {
    #[serde(
        rename = "baseFunctions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub base_functions: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "functionSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_selector: Option<String>,
    pub id: i64,
    pub implemented: bool,
    pub kind: FunctionDefinitionKind,
    pub modifiers: Vec<ModifierInvocation>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: FunctionDefinitionNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub parameters: ParameterList,
    #[serde(rename = "returnParameters")]
    pub return_parameters: ParameterList,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
    #[serde(rename = "virtual")]
    pub virtual_: bool,
    pub visibility: Visibility,
}

impl From<&FunctionDefinition> for FunctionDefinition {
    fn from(value: &FunctionDefinition) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "FunctionDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"FunctionDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionDefinitionNodeType {
    FunctionDefinition,
}
impl From<&FunctionDefinitionNodeType> for FunctionDefinitionNodeType {
    fn from(value: &FunctionDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for FunctionDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionDefinition => "FunctionDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for FunctionDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "FunctionDefinition" => Ok(Self::FunctionDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
