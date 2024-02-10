use serde::{Deserialize, Serialize};

mod function_list_item;
pub use function_list_item::*;
mod library_name;
pub use library_name::*;

use crate::types::{SourceLocation, TypeName};

#[doc = "UsingForDirective"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"functionList\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"function\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"function\": {"]
#[doc = "                \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"definition\","]
#[doc = "              \"operator\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"definition\": {"]
#[doc = "                \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "              },"]
#[doc = "              \"operator\": {"]
#[doc = "                \"enum\": ["]
#[doc = "                  \"&\","]
#[doc = "                  \"|\","]
#[doc = "                  \"^\","]
#[doc = "                  \"~\","]
#[doc = "                  \"+\","]
#[doc = "                  \"-\","]
#[doc = "                  \"*\","]
#[doc = "                  \"/\","]
#[doc = "                  \"%\","]
#[doc = "                  \"==\","]
#[doc = "                  \"!=\","]
#[doc = "                  \"<\","]
#[doc = "                  \"<=\","]
#[doc = "                  \">\","]
#[doc = "                  \">=\""]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"global\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"libraryName\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"UsingForDirective\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UsingForDirective {
    #[serde(
        rename = "functionList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub function_list: Vec<UsingForDirectiveFunctionListItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    pub id: i64,
    #[serde(
        rename = "libraryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_name: Option<UsingForDirectiveLibraryName>,
    #[serde(rename = "nodeType")]
    pub node_type: UsingForDirectiveNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<TypeName>,
}

impl From<&UsingForDirective> for UsingForDirective {
    fn from(value: &UsingForDirective) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "UsingForDirectiveNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"UsingForDirective\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UsingForDirectiveNodeType {
    UsingForDirective,
}
impl From<&UsingForDirectiveNodeType> for UsingForDirectiveNodeType {
    fn from(value: &UsingForDirectiveNodeType) -> Self {
        value.clone()
    }
}
impl ToString for UsingForDirectiveNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::UsingForDirective => "UsingForDirective".to_string(),
        }
    }
}
impl std::str::FromStr for UsingForDirectiveNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "UsingForDirective" => Ok(Self::UsingForDirective),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UsingForDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UsingForDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UsingForDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
