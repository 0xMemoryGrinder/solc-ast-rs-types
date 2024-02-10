use serde::{Deserialize, Serialize};

use crate::types::{Expression, SourceLocation, TypeDescriptions};

#[doc = "FunctionCallOptions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expression\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"names\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"options\","]
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
#[doc = "    \"expression\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
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
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"FunctionCallOptions\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"options\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Expression\""]
#[doc = "      }"]
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
pub struct FunctionCallOptions {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub expression: Box<Expression>,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue", default, skip_serializing_if = "Option::is_none")]
    pub is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    pub names: Vec<String>,
    #[serde(rename = "nodeType")]
    pub node_type: FunctionCallOptionsNodeType,
    pub options: Vec<Expression>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&FunctionCallOptions> for FunctionCallOptions {
    fn from(value: &FunctionCallOptions) -> Self {
        value.clone()
    }
}

// Node type
#[doc = "FunctionCallOptionsNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"FunctionCallOptions\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionCallOptionsNodeType {
    FunctionCallOptions,
}

impl From<&FunctionCallOptionsNodeType> for FunctionCallOptionsNodeType {
    fn from(value: &FunctionCallOptionsNodeType) -> Self {
        value.clone()
    }
}

impl ToString for FunctionCallOptionsNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionCallOptions => "FunctionCallOptions".to_string(),
        }
    }
}

impl std::str::FromStr for FunctionCallOptionsNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "FunctionCallOptions" => Ok(Self::FunctionCallOptions),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for FunctionCallOptionsNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for FunctionCallOptionsNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for FunctionCallOptionsNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
