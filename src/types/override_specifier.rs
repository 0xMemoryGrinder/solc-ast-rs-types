use serde::{Deserialize, Serialize};

use crate::{IdentifierPath, SourceLocation, UserDefinedTypeName};

#[doc = "OverrideSpecifier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"overrides\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"OverrideSpecifier\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overrides\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "          }"]
#[doc = "        }"]
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
pub struct OverrideSpecifier {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: OverrideSpecifierNodeType,
    pub overrides: OverrideSpecifierOverrides,
    pub src: SourceLocation,
}

impl From<&OverrideSpecifier> for OverrideSpecifier {
    fn from(value: &OverrideSpecifier) -> Self {
        value.clone()
    }
}


#[doc = "OverrideSpecifierOverrides"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OverrideSpecifierOverrides {
    UserDefinedTypeNames(Vec<UserDefinedTypeName>),
    IdentifierPaths(Vec<IdentifierPath>),
}
impl From<&OverrideSpecifierOverrides> for OverrideSpecifierOverrides {
    fn from(value: &OverrideSpecifierOverrides) -> Self {
        value.clone()
    }
}

impl From<Vec<UserDefinedTypeName>> for OverrideSpecifierOverrides {
    fn from(value: Vec<UserDefinedTypeName>) -> Self {
        Self::UserDefinedTypeNames(value)
    }
}

impl From<Vec<IdentifierPath>> for OverrideSpecifierOverrides {
    fn from(value: Vec<IdentifierPath>) -> Self {
        Self::IdentifierPaths(value)
    }
}


// Node type
#[doc = "OverrideSpecifierNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"OverrideSpecifier\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OverrideSpecifierNodeType {
    OverrideSpecifier,
}

impl From<&OverrideSpecifierNodeType> for OverrideSpecifierNodeType {
    fn from(value: &OverrideSpecifierNodeType) -> Self {
        value.clone()
    }
}

impl ToString for OverrideSpecifierNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::OverrideSpecifier => "OverrideSpecifier".to_string(),
        }
    }
}

impl std::str::FromStr for OverrideSpecifierNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "OverrideSpecifier" => Ok(Self::OverrideSpecifier),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for OverrideSpecifierNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for OverrideSpecifierNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for OverrideSpecifierNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}