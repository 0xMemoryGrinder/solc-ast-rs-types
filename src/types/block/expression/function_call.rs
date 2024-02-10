use serde::{Deserialize, Serialize};

use crate::types::{Expression, SourceLocation, TypeDescriptions};

#[doc = "FunctionCall"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"arguments\","]
#[doc = "    \"expression\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"kind\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"names\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"tryCall\","]
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
#[doc = "    \"arguments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Expression\""]
#[doc = "      }"]
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
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"functionCall\","]
#[doc = "        \"typeConversion\","]
#[doc = "        \"structConstructorCall\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lValueRequested\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nameLocations\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"FunctionCall\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"tryCall\": {"]
#[doc = "      \"type\": \"boolean\""]
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
pub struct FunctionCall {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub arguments: Vec<Expression>,
    pub expression: Box<Expression>,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    pub kind: FunctionCallKind,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(
        rename = "nameLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name_locations: Vec<String>,
    pub names: Vec<String>,
    #[serde(rename = "nodeType")]
    pub node_type: FunctionCallNodeType,
    pub src: SourceLocation,
    #[serde(rename = "tryCall")]
    pub try_call: bool,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&FunctionCall> for FunctionCall {
    fn from(value: &FunctionCall) -> Self {
        value.clone()
    }
}

#[doc = "FunctionCallKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"functionCall\","]
#[doc = "    \"typeConversion\","]
#[doc = "    \"structConstructorCall\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionCallKind {
    #[serde(rename = "functionCall")]
    FunctionCall,
    #[serde(rename = "typeConversion")]
    TypeConversion,
    #[serde(rename = "structConstructorCall")]
    StructConstructorCall,
}

impl From<&FunctionCallKind> for FunctionCallKind {
    fn from(value: &FunctionCallKind) -> Self {
        value.clone()
    }
}

impl ToString for FunctionCallKind {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionCall => "functionCall".to_string(),
            Self::TypeConversion => "typeConversion".to_string(),
            Self::StructConstructorCall => "structConstructorCall".to_string(),
        }
    }
}

impl std::str::FromStr for FunctionCallKind {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "functionCall" => Ok(Self::FunctionCall),
            "typeConversion" => Ok(Self::TypeConversion),
            "structConstructorCall" => Ok(Self::StructConstructorCall),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for FunctionCallKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for FunctionCallKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for FunctionCallKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

// Node type
#[doc = "FunctionCallNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"FunctionCall\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionCallNodeType {
    FunctionCall,
}

impl From<&FunctionCallNodeType> for FunctionCallNodeType {
    fn from(value: &FunctionCallNodeType) -> Self {
        value.clone()
    }
}

impl ToString for FunctionCallNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionCall => "FunctionCall".to_string(),
        }
    }
}

impl std::str::FromStr for FunctionCallNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "FunctionCall" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for FunctionCallNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for FunctionCallNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for FunctionCallNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
