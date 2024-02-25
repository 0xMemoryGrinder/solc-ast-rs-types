use serde::{Deserialize, Serialize};

use crate::types::{ParameterList, SourceLocation, StructuredDocumentation};

#[doc = "EventDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"anonymous\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"anonymous\": {"]
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
#[doc = "    \"eventSelector\": {"]
#[doc = "      \"type\": \"string\""]
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
#[doc = "        \"EventDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
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
pub struct EventDefinition {
    pub anonymous: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "eventSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_selector: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: EventDefinitionNodeType,
    pub parameters: ParameterList,
    pub src: SourceLocation,
}

impl From<&EventDefinition> for EventDefinition {
    fn from(value: &EventDefinition) -> Self {
        value.clone()
    }
}

// Node Type
#[doc = "EventDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EventDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EventDefinitionNodeType {
    EventDefinition,
}

impl From<&EventDefinitionNodeType> for EventDefinitionNodeType {
    fn from(value: &EventDefinitionNodeType) -> Self {
        value.clone()
    }
}

impl ToString for EventDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EventDefinition => "EventDefinition".to_string(),
        }
    }
}

impl std::str::FromStr for EventDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "EventDefinition" => Ok(Self::EventDefinition),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for EventDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for EventDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for EventDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
