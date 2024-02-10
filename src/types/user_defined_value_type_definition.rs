use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, TypeName};

#[doc = "UserDefinedValueTypeDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"underlyingType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonicalName\": {"]
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
#[doc = "        \"UserDefinedValueTypeDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"underlyingType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserDefinedValueTypeDefinition {
    #[serde(
        rename = "canonicalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub canonical_name: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: UserDefinedValueTypeDefinitionNodeType,
    pub src: SourceLocation,
    #[serde(rename = "underlyingType")]
    pub underlying_type: TypeName,
}

impl From<&UserDefinedValueTypeDefinition> for UserDefinedValueTypeDefinition {
    fn from(value: &UserDefinedValueTypeDefinition) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "UserDefinedValueTypeDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserDefinedValueTypeDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UserDefinedValueTypeDefinitionNodeType {
    UserDefinedValueTypeDefinition,
}

impl From<&UserDefinedValueTypeDefinitionNodeType> for UserDefinedValueTypeDefinitionNodeType {
    fn from(value: &UserDefinedValueTypeDefinitionNodeType) -> Self {
        value.clone()
    }
}

impl ToString for UserDefinedValueTypeDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::UserDefinedValueTypeDefinition => "UserDefinedValueTypeDefinition".to_string(),
        }
    }
}

impl std::str::FromStr for UserDefinedValueTypeDefinitionNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "UserDefinedValueTypeDefinition" => Ok(Self::UserDefinedValueTypeDefinition),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for UserDefinedValueTypeDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for UserDefinedValueTypeDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for UserDefinedValueTypeDefinitionNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
