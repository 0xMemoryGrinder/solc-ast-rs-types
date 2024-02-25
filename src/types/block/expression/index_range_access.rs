use serde::{Deserialize, Serialize};

use crate::types::{Expression, SourceLocation, TypeDescriptions};

#[doc = "IndexRangeAccess"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"baseExpression\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"argumentTypes\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"baseExpression\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"endExpression\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isConstant\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isLValue\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isPure\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"lValueRequested\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"IndexRangeAccess\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"startExpression\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
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
pub struct IndexRangeAccess {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "baseExpression")]
    pub base_expression: Box<Expression>,
    #[serde(rename = "endExpression", default)]
    pub end_expression: Box<Option<Expression>>,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "nodeType")]
    pub node_type: IndexRangeAccessNodeType,
    pub src: SourceLocation,
    #[serde(rename = "startExpression", default)]
    pub start_expression: Box<Option<Expression>>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&IndexRangeAccess> for IndexRangeAccess {
    fn from(value: &IndexRangeAccess) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "IndexRangeAccessNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"IndexRangeAccess\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IndexRangeAccessNodeType {
    IndexRangeAccess,
}

impl From<&IndexRangeAccessNodeType> for IndexRangeAccessNodeType {
    fn from(value: &IndexRangeAccessNodeType) -> Self {
        value.clone()
    }
}

impl ToString for IndexRangeAccessNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::IndexRangeAccess => "IndexRangeAccess".to_string(),
        }
    }
}

impl std::str::FromStr for IndexRangeAccessNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "IndexRangeAccess" => Ok(Self::IndexRangeAccess),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for IndexRangeAccessNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for IndexRangeAccessNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for IndexRangeAccessNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
