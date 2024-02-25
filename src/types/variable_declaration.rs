use serde::{Deserialize, Serialize};

mod mutability;
pub use mutability::*;
mod storage_location;
pub use storage_location::*;
mod type_descriptions;
pub use type_descriptions::*;

use crate::types::{
    Expression, OverrideSpecifier, SourceLocation, StructuredDocumentation, TypeName, Visibility,
};

#[doc = "VariableDeclaration"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"constant\","]
#[doc = "    \"id\","]
#[doc = "    \"mutability\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\","]
#[doc = "    \"stateVariable\","]
#[doc = "    \"storageLocation\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseFunctions\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"constant\": {"]
#[doc = "      \"type\": \"boolean\""]
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
#[doc = "    \"indexed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"mutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/Mutability\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"VariableDeclaration\""]
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
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateVariable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"storageLocation\": {"]
#[doc = "      \"$ref\": \"#/definitions/StorageLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"typeName\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TypeName\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
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
pub struct VariableDeclaration {
    #[serde(
        rename = "baseFunctions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_functions: Option<Vec<i64>>,
    pub constant: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "functionSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_selector: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    pub mutability: Mutability,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: VariableDeclarationNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(rename = "stateVariable")]
    pub state_variable: bool,
    #[serde(rename = "storageLocation")]
    pub storage_location: StorageLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<TypeName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Expression>,
    pub visibility: Visibility,
}

impl From<&VariableDeclaration> for VariableDeclaration {
    fn from(value: &VariableDeclaration) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "VariableDeclarationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"VariableDeclaration\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VariableDeclarationNodeType {
    VariableDeclaration,
}

impl From<&VariableDeclarationNodeType> for VariableDeclarationNodeType {
    fn from(value: &VariableDeclarationNodeType) -> Self {
        value.clone()
    }
}

impl ToString for VariableDeclarationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::VariableDeclaration => "VariableDeclaration".to_string(),
        }
    }
}

impl std::str::FromStr for VariableDeclarationNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "VariableDeclaration" => Ok(Self::VariableDeclaration),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for VariableDeclarationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for VariableDeclarationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for VariableDeclarationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
