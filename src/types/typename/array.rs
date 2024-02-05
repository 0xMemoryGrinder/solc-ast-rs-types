use serde::{Deserialize, Serialize};

use crate::types::{Expression, SourceLocation, TypeDescriptions, TypeName};


#[doc = "ArrayTypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"baseType\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"length\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ArrayTypeName\""]
#[doc = "      ]"]
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
pub struct ArrayTypeName {
    #[serde(rename = "baseType")]
    pub base_type: TypeName,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<Expression>,
    #[serde(rename = "nodeType")]
    pub node_type: ArrayTypeNameNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&ArrayTypeName> for ArrayTypeName {
    fn from(value: &ArrayTypeName) -> Self {
        value.clone()
    }
}



// NodeType
#[doc = "ArrayTypeNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ArrayTypeName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ArrayTypeNameNodeType {
    ArrayTypeName,
}

impl From<&ArrayTypeNameNodeType> for ArrayTypeNameNodeType {
    fn from(value: &ArrayTypeNameNodeType) -> Self {
        value.clone()
    }
}

impl ToString for ArrayTypeNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ArrayTypeName => "ArrayTypeName".to_string(),
        }
    }
}

impl std::str::FromStr for ArrayTypeNameNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ArrayTypeName" => Ok(Self::ArrayTypeName),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ArrayTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ArrayTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ArrayTypeNameNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}