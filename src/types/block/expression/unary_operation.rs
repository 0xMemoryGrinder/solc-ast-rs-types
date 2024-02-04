use serde::{Deserialize, Serialize};

use crate::{Expression, SourceLocation, TypeDescriptions};


#[doc = "UnaryOperation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"operator\","]
#[doc = "    \"prefix\","]
#[doc = "    \"src\","]
#[doc = "    \"subExpression\","]
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
#[doc = "    \"function\": {"]
#[doc = "      \"type\": \"integer\""]
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
#[doc = "        \"UnaryOperation\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"++\","]
#[doc = "        \"--\","]
#[doc = "        \"-\","]
#[doc = "        \"!\","]
#[doc = "        \"delete\","]
#[doc = "        \"~\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"prefix\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"subExpression\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
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
pub struct UnaryOperation {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<i64>,
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
    pub node_type: UnaryOperationNodeType,
    pub operator: UnaryOperationOperator,
    pub prefix: bool,
    pub src: SourceLocation,
    #[serde(rename = "subExpression")]
    pub sub_expression: Box<Expression>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&UnaryOperation> for UnaryOperation {
    fn from(value: &UnaryOperation) -> Self {
        value.clone()
    }
}


#[doc = "UnaryOperationOperator"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"++\","]
#[doc = "    \"--\","]
#[doc = "    \"-\","]
#[doc = "    \"!\","]
#[doc = "    \"delete\","]
#[doc = "    \"~\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UnaryOperationOperator {
    #[serde(rename = "++")]
    Increment,
    #[serde(rename = "--")]
    Decrement,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "!")]
    Not,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "~")]
    BitwiseNegation,
}

impl From<&UnaryOperationOperator> for UnaryOperationOperator {
    fn from(value: &UnaryOperationOperator) -> Self {
        value.clone()
    }
}

impl ToString for UnaryOperationOperator {
    fn to_string(&self) -> String {
        match *self {
            Self::Increment => "++".to_string(),
            Self::Decrement => "--".to_string(),
            Self::Minus => "-".to_string(),
            Self::Not => "!".to_string(),
            Self::Delete => "delete".to_string(),
            Self::BitwiseNegation => "~".to_string(),
        }
    }
}

impl std::str::FromStr for UnaryOperationOperator {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "++" => Ok(Self::Increment),
            "--" => Ok(Self::Decrement),
            "-" => Ok(Self::Minus),
            "!" => Ok(Self::Not),
            "delete" => Ok(Self::Delete),
            "~" => Ok(Self::BitwiseNegation),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for UnaryOperationOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for UnaryOperationOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for UnaryOperationOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}



// Node type
#[doc = "UnaryOperationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"UnaryOperation\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UnaryOperationNodeType {
    UnaryOperation,
}

impl From<&UnaryOperationNodeType> for UnaryOperationNodeType {
    fn from(value: &UnaryOperationNodeType) -> Self {
        value.clone()
    }
}

impl ToString for UnaryOperationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::UnaryOperation => "UnaryOperation".to_string(),
        }
    }
}

impl std::str::FromStr for UnaryOperationNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "UnaryOperation" => Ok(Self::UnaryOperation),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for UnaryOperationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for UnaryOperationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for UnaryOperationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}