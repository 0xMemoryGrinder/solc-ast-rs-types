use serde::{Deserialize, Serialize};

use crate::types::{Expression, SourceLocation, TypeDescriptions};

#[doc = "BinaryOperation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commonType\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"leftExpression\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"operator\","]
#[doc = "    \"rightExpression\","]
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
#[doc = "    \"commonType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
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
#[doc = "    \"leftExpression\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"BinaryOperation\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"+\","]
#[doc = "        \"-\","]
#[doc = "        \"*\","]
#[doc = "        \"/\","]
#[doc = "        \"%\","]
#[doc = "        \"**\","]
#[doc = "        \"&&\","]
#[doc = "        \"||\","]
#[doc = "        \"!=\","]
#[doc = "        \"==\","]
#[doc = "        \"<\","]
#[doc = "        \"<=\","]
#[doc = "        \">\","]
#[doc = "        \">=\","]
#[doc = "        \"^\","]
#[doc = "        \"&\","]
#[doc = "        \"|\","]
#[doc = "        \"<<\","]
#[doc = "        \">>\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"rightExpression\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
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
pub struct BinaryOperation {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "commonType")]
    pub common_type: TypeDescriptions,
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
    #[serde(rename = "leftExpression")]
    pub left_expression: Box<Expression>,
    #[serde(rename = "nodeType")]
    pub node_type: BinaryOperationNodeType,
    pub operator: BinaryOperationOperator,
    #[serde(rename = "rightExpression")]
    pub right_expression: Box<Expression>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl From<&BinaryOperation> for BinaryOperation {
    fn from(value: &BinaryOperation) -> Self {
        value.clone()
    }
}

#[doc = "BinaryOperationOperator"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"+\","]
#[doc = "    \"-\","]
#[doc = "    \"*\","]
#[doc = "    \"/\","]
#[doc = "    \"%\","]
#[doc = "    \"**\","]
#[doc = "    \"&&\","]
#[doc = "    \"||\","]
#[doc = "    \"!=\","]
#[doc = "    \"==\","]
#[doc = "    \"<\","]
#[doc = "    \"<=\","]
#[doc = "    \">\","]
#[doc = "    \">=\","]
#[doc = "    \"^\","]
#[doc = "    \"&\","]
#[doc = "    \"|\","]
#[doc = "    \"<<\","]
#[doc = "    \">>\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BinaryOperationOperator {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Substract,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "/")]
    Divide,
    #[serde(rename = "%")]
    Modulo,
    #[serde(rename = "**")]
    Exponentiation,
    #[serde(rename = "&&")]
    And,
    #[serde(rename = "||")]
    Or,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "<=")]
    LessEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = ">=")]
    GreaterEqual,
    #[serde(rename = "^")]
    BitwiseXor,
    #[serde(rename = "&")]
    BitwiseAnd,
    #[serde(rename = "|")]
    BitwiseOr,
    #[serde(rename = "<<")]
    ShiftLeft,
    #[serde(rename = ">>")]
    ShiftRight,
}

impl From<&BinaryOperationOperator> for BinaryOperationOperator {
    fn from(value: &BinaryOperationOperator) -> Self {
        value.clone()
    }
}

impl ToString for BinaryOperationOperator {
    fn to_string(&self) -> String {
        match *self {
            Self::Add => "+".to_string(),
            Self::Substract => "-".to_string(),
            Self::Multiply => "*".to_string(),
            Self::Divide => "/".to_string(),
            Self::Modulo => "%".to_string(),
            Self::Exponentiation => "**".to_string(),
            Self::And => "&&".to_string(),
            Self::Or => "||".to_string(),
            Self::NotEqual => "!=".to_string(),
            Self::Equal => "==".to_string(),
            Self::Less => "<".to_string(),
            Self::LessEqual => "<=".to_string(),
            Self::Greater => ">".to_string(),
            Self::GreaterEqual => ">=".to_string(),
            Self::BitwiseXor => "^".to_string(),
            Self::BitwiseAnd => "&".to_string(),
            Self::BitwiseOr => "|".to_string(),
            Self::ShiftLeft => "<<".to_string(),
            Self::ShiftRight => ">>".to_string(),
        }
    }
}

impl std::str::FromStr for BinaryOperationOperator {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Substract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            "%" => Ok(Self::Modulo),
            "**" => Ok(Self::Exponentiation),
            "&&" => Ok(Self::And),
            "||" => Ok(Self::Or),
            "!=" => Ok(Self::NotEqual),
            "==" => Ok(Self::Equal),
            "<" => Ok(Self::Less),
            "<=" => Ok(Self::LessEqual),
            ">" => Ok(Self::Greater),
            ">=" => Ok(Self::GreaterEqual),
            "^" => Ok(Self::BitwiseXor),
            "&" => Ok(Self::BitwiseAnd),
            "|" => Ok(Self::BitwiseOr),
            "<<" => Ok(Self::ShiftLeft),
            ">>" => Ok(Self::ShiftRight),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for BinaryOperationOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for BinaryOperationOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for BinaryOperationOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

// Node type
#[doc = "BinaryOperationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"BinaryOperation\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BinaryOperationNodeType {
    BinaryOperation,
}

impl From<&BinaryOperationNodeType> for BinaryOperationNodeType {
    fn from(value: &BinaryOperationNodeType) -> Self {
        value.clone()
    }
}

impl ToString for BinaryOperationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::BinaryOperation => "BinaryOperation".to_string(),
        }
    }
}

impl std::str::FromStr for BinaryOperationNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "BinaryOperation" => Ok(Self::BinaryOperation),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for BinaryOperationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for BinaryOperationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for BinaryOperationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
