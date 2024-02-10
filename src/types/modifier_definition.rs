use serde::{Deserialize, Serialize};

use crate::types::{
    Block, OverrideSpecifier, ParameterList, SourceLocation, StructuredDocumentation, Visibility,
};

#[doc = "ModifierDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\","]
#[doc = "    \"virtual\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseModifiers\": {"]
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
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
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
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ModifierDefinition\""]
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
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
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
pub struct ModifierDefinition {
    #[serde(
        rename = "baseModifiers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_modifiers: Option<Vec<i64>>,
    pub body: Block,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: ModifierDefinitionNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    #[serde(rename = "virtual")]
    pub virtual_: bool,
    pub visibility: Visibility,
}

impl From<&ModifierDefinition> for ModifierDefinition {
    fn from(value: &ModifierDefinition) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "ModifierDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ModifierDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierDefinitionNodeType {
    ModifierDefinition,
}

impl From<&ModifierDefinitionNodeType> for ModifierDefinitionNodeType {
    fn from(value: &ModifierDefinitionNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ModifierDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ModifierDefinition => "ModifierDefinition".to_string(),
        }
    }
}

impl std::str::FromStr for ModifierDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ModifierDefinition" => Ok(Self::ModifierDefinition),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ModifierDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ModifierDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ModifierDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
