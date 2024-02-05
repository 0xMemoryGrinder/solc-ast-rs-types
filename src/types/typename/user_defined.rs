use serde::{Deserialize, Serialize};

use crate::types::{IdentifierPath, SourceLocation, TypeDescriptions};


#[doc = "UserDefinedTypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"referencedDeclaration\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contractScope\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"UserDefinedTypeName\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pathNode\": {"]
#[doc = "      \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "    },"]
#[doc = "    \"referencedDeclaration\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserDefinedTypeName {
    #[serde(rename = "contractScope", default)]
    pub contract_scope: (),
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: UserDefinedTypeNameNodeType,
    #[serde(rename = "pathNode", default, skip_serializing_if = "Option::is_none")]
    pub path_node: Option<IdentifierPath>,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&UserDefinedTypeName> for UserDefinedTypeName {
    fn from(value: &UserDefinedTypeName) -> Self {
        value.clone()
    }
}


// Node type
#[doc = "UserDefinedTypeNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserDefinedTypeName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UserDefinedTypeNameNodeType {
    UserDefinedTypeName,
}

impl From<&UserDefinedTypeNameNodeType> for UserDefinedTypeNameNodeType {
    fn from(value: &UserDefinedTypeNameNodeType) -> Self {
        value.clone()
    }
}

impl ToString for UserDefinedTypeNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::UserDefinedTypeName => "UserDefinedTypeName".to_string(),
        }
    }
}

impl std::str::FromStr for UserDefinedTypeNameNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "UserDefinedTypeName" => Ok(Self::UserDefinedTypeName),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for UserDefinedTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for UserDefinedTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for UserDefinedTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}