use serde::{Deserialize, Serialize};

use crate::types::{Expression, IdentifierPath, SourceLocation, UserDefinedTypeName};

#[doc = "InheritanceSpecifier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"baseName\","]
#[doc = "    \"id\","]
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
#[doc = "    \"baseName\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"InheritanceSpecifier\""]
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
pub struct InheritanceSpecifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<Expression>>,
    #[serde(rename = "baseName")]
    pub base_name: InheritanceSpecifierBaseName,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: InheritanceSpecifierNodeType,
    pub src: SourceLocation,
}

impl From<&InheritanceSpecifier> for InheritanceSpecifier {
    fn from(value: &InheritanceSpecifier) -> Self {
        value.clone()
    }
}

#[doc = "InheritanceSpecifierBaseName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InheritanceSpecifierBaseName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

impl From<&InheritanceSpecifierBaseName> for InheritanceSpecifierBaseName {
    fn from(value: &InheritanceSpecifierBaseName) -> Self {
        value.clone()
    }
}

impl From<UserDefinedTypeName> for InheritanceSpecifierBaseName {
    fn from(value: UserDefinedTypeName) -> Self {
        Self::UserDefinedTypeName(value)
    }
}

impl From<IdentifierPath> for InheritanceSpecifierBaseName {
    fn from(value: IdentifierPath) -> Self {
        Self::IdentifierPath(value)
    }
}

// Node type
#[doc = "InheritanceSpecifierNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"InheritanceSpecifier\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InheritanceSpecifierNodeType {
    InheritanceSpecifier,
}

impl From<&InheritanceSpecifierNodeType> for InheritanceSpecifierNodeType {
    fn from(value: &InheritanceSpecifierNodeType) -> Self {
        value.clone()
    }
}

impl ToString for InheritanceSpecifierNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::InheritanceSpecifier => "InheritanceSpecifier".to_string(),
        }
    }
}

impl std::str::FromStr for InheritanceSpecifierNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "InheritanceSpecifier" => Ok(Self::InheritanceSpecifier),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for InheritanceSpecifierNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for InheritanceSpecifierNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for InheritanceSpecifierNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
