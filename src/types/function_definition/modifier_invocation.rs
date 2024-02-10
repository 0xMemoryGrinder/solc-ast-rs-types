use serde::{Deserialize, Serialize};

mod modifier_invocation_kind;
pub use modifier_invocation_kind::*;
mod modifier_name;
pub use modifier_name::*;

use crate::types::{Expression, SourceLocation};

#[doc = "ModifierInvocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"modifierName\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Expression\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"modifierInvocation\","]
#[doc = "        \"baseConstructorSpecifier\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modifierName\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Identifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ModifierInvocation\""]
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
pub struct ModifierInvocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<Expression>>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModifierInvocationKind>,
    #[serde(rename = "modifierName")]
    pub modifier_name: ModifierInvocationModifierName,
    #[serde(rename = "nodeType")]
    pub node_type: ModifierInvocationNodeType,
    pub src: SourceLocation,
}

impl From<&ModifierInvocation> for ModifierInvocation {
    fn from(value: &ModifierInvocation) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "ModifierInvocationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ModifierInvocation\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierInvocationNodeType {
    ModifierInvocation,
}

impl From<&ModifierInvocationNodeType> for ModifierInvocationNodeType {
    fn from(value: &ModifierInvocationNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ModifierInvocationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ModifierInvocation => "ModifierInvocation".to_string(),
        }
    }
}

impl std::str::FromStr for ModifierInvocationNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ModifierInvocation" => Ok(Self::ModifierInvocation),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ModifierInvocationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ModifierInvocationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ModifierInvocationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
