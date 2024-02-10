#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ArrayTypeName" => Ok(Self::ArrayTypeName),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ArrayTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ArrayTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ArrayTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Assignment"]
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
#[doc = "    \"leftHandSide\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"operator\","]
#[doc = "    \"rightHandSide\","]
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
#[doc = "    \"leftHandSide\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Assignment\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"=\","]
#[doc = "        \"+=\","]
#[doc = "        \"-=\","]
#[doc = "        \"*=\","]
#[doc = "        \"/=\","]
#[doc = "        \"%=\","]
#[doc = "        \"|=\","]
#[doc = "        \"&=\","]
#[doc = "        \"^=\","]
#[doc = "        \">>=\","]
#[doc = "        \"<<=\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"rightHandSide\": {"]
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
pub struct Assignment {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "leftHandSide")]
    pub left_hand_side: Box<Expression>,
    #[serde(rename = "nodeType")]
    pub node_type: AssignmentNodeType,
    pub operator: AssignmentOperator,
    #[serde(rename = "rightHandSide")]
    pub right_hand_side: Box<Expression>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&Assignment> for Assignment {
    fn from(value: &Assignment) -> Self {
        value.clone()
    }
}
#[doc = "AssignmentNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Assignment\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AssignmentNodeType {
    Assignment,
}
impl From<&AssignmentNodeType> for AssignmentNodeType {
    fn from(value: &AssignmentNodeType) -> Self {
        value.clone()
    }
}
impl ToString for AssignmentNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Assignment => "Assignment".to_string(),
        }
    }
}
impl std::str::FromStr for AssignmentNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Assignment" => Ok(Self::Assignment),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for AssignmentNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssignmentNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssignmentNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "AssignmentOperator"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"=\","]
#[doc = "    \"+=\","]
#[doc = "    \"-=\","]
#[doc = "    \"*=\","]
#[doc = "    \"/=\","]
#[doc = "    \"%=\","]
#[doc = "    \"|=\","]
#[doc = "    \"&=\","]
#[doc = "    \"^=\","]
#[doc = "    \">>=\","]
#[doc = "    \"<<=\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AssignmentOperator {
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "+=")]
    Add,
    #[serde(rename = "-=")]
    Substract,
    #[serde(rename = "*=")]
    Multiply,
    #[serde(rename = "/=")]
    Divide,
    #[serde(rename = "%=")]
    Modulo,
    #[serde(rename = "|=")]
    BitwiseOr,
    #[serde(rename = "&=")]
    BitwiseAnd,
    #[serde(rename = "^=")]
    BitwiseXor,
    #[serde(rename = ">>=")]
    ShiftRight,
    #[serde(rename = "<<=")]
    ShiftLeft,
}
impl From<&AssignmentOperator> for AssignmentOperator {
    fn from(value: &AssignmentOperator) -> Self {
        value.clone()
    }
}
impl ToString for AssignmentOperator {
    fn to_string(&self) -> String {
        match *self {
            Self::Equal => "=".to_string(),
            Self::Add => "+=".to_string(),
            Self::Substract => "-=".to_string(),
            Self::Multiply => "*=".to_string(),
            Self::Divide => "/=".to_string(),
            Self::Modulo => "%=".to_string(),
            Self::BitwiseOr => "|=".to_string(),
            Self::BitwiseAnd => "&=".to_string(),
            Self::BitwiseXor => "^=".to_string(),
            Self::ShiftRight => ">>=".to_string(),
            Self::ShiftLeft => "<<=".to_string(),
        }
    }
}
impl std::str::FromStr for AssignmentOperator {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "=" => Ok(Self::Equal),
            "+=" => Ok(Self::Add),
            "-=" => Ok(Self::Substract),
            "*=" => Ok(Self::Multiply),
            "/=" => Ok(Self::Divide),
            "%=" => Ok(Self::Modulo),
            "|=" => Ok(Self::BitwiseOr),
            "&=" => Ok(Self::BitwiseAnd),
            "^=" => Ok(Self::BitwiseXor),
            ">>=" => Ok(Self::ShiftRight),
            "<<=" => Ok(Self::ShiftLeft),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for AssignmentOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssignmentOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssignmentOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "BinaryOperation" => Ok(Self::BinaryOperation),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for BinaryOperationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BinaryOperationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BinaryOperationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
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
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BinaryOperationOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BinaryOperationOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Block"]
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
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Block\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"statements\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Statement\""]
#[doc = "          }"]
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
pub struct Block {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: BlockNodeType,
    pub src: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<Statement>>,
}
impl From<&Block> for Block {
    fn from(value: &Block) -> Self {
        value.clone()
    }
}
#[doc = "BlockNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Block\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BlockNodeType {
    Block,
}
impl From<&BlockNodeType> for BlockNodeType {
    fn from(value: &BlockNodeType) -> Self {
        value.clone()
    }
}
impl ToString for BlockNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "Block".to_string(),
        }
    }
}
impl std::str::FromStr for BlockNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Block" => Ok(Self::Block),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for BlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Break"]
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
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Break\""]
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
pub struct Break {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: BreakNodeType,
    pub src: SourceLocation,
}
impl From<&Break> for Break {
    fn from(value: &Break) -> Self {
        value.clone()
    }
}
#[doc = "BreakNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Break\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BreakNodeType {
    Break,
}
impl From<&BreakNodeType> for BreakNodeType {
    fn from(value: &BreakNodeType) -> Self {
        value.clone()
    }
}
impl ToString for BreakNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Break => "Break".to_string(),
        }
    }
}
impl std::str::FromStr for BreakNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Break" => Ok(Self::Break),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for BreakNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BreakNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BreakNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Conditional"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"condition\","]
#[doc = "    \"falseExpression\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"trueExpression\","]
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
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"falseExpression\": {"]
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
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Conditional\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"trueExpression\": {"]
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
pub struct Conditional {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub condition: Box<Expression>,
    #[serde(rename = "falseExpression")]
    pub false_expression: Box<Expression>,
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
    pub node_type: ConditionalNodeType,
    pub src: SourceLocation,
    #[serde(rename = "trueExpression")]
    pub true_expression: Box<Expression>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&Conditional> for Conditional {
    fn from(value: &Conditional) -> Self {
        value.clone()
    }
}
#[doc = "ConditionalNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Conditional\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ConditionalNodeType {
    Conditional,
}
impl From<&ConditionalNodeType> for ConditionalNodeType {
    fn from(value: &ConditionalNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ConditionalNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Conditional => "Conditional".to_string(),
        }
    }
}
impl std::str::FromStr for ConditionalNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Conditional" => Ok(Self::Conditional),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ConditionalNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ConditionalNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ConditionalNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Continue"]
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
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Continue\""]
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
pub struct Continue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: ContinueNodeType,
    pub src: SourceLocation,
}
impl From<&Continue> for Continue {
    fn from(value: &Continue) -> Self {
        value.clone()
    }
}
#[doc = "ContinueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Continue\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ContinueNodeType {
    Continue,
}
impl From<&ContinueNodeType> for ContinueNodeType {
    fn from(value: &ContinueNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ContinueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Continue => "Continue".to_string(),
        }
    }
}
impl std::str::FromStr for ContinueNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Continue" => Ok(Self::Continue),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ContinueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ContinueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ContinueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ContractDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"abstract\","]
#[doc = "    \"baseContracts\","]
#[doc = "    \"contractDependencies\","]
#[doc = "    \"contractKind\","]
#[doc = "    \"fullyImplemented\","]
#[doc = "    \"id\","]
#[doc = "    \"linearizedBaseContracts\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"nodes\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"abstract\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"baseContracts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/InheritanceSpecifier\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"canonicalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"contractDependencies\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"contractKind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"contract\","]
#[doc = "        \"interface\","]
#[doc = "        \"library\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fullyImplemented\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalFunctionIDs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"linearizedBaseContracts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ContractDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EventDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ModifierDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/StructDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/UserDefinedValueTypeDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/UsingForDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"usedErrors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"usedEvents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractDefinition {
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    #[serde(rename = "baseContracts")]
    pub base_contracts: Vec<InheritanceSpecifier>,
    #[serde(
        rename = "canonicalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub canonical_name: Option<String>,
    #[serde(rename = "contractDependencies")]
    pub contract_dependencies: Vec<i64>,
    #[serde(rename = "contractKind")]
    pub contract_kind: ContractDefinitionContractKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(rename = "fullyImplemented")]
    pub fully_implemented: bool,
    pub id: i64,
    #[serde(
        rename = "internalFunctionIDs",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub internal_function_i_ds: std::collections::HashMap<String, i64>,
    #[serde(rename = "linearizedBaseContracts")]
    pub linearized_base_contracts: Vec<i64>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: ContractDefinitionNodeType,
    pub nodes: Vec<ContractDefinitionNodesItem>,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(rename = "usedErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub used_errors: Vec<i64>,
    #[serde(rename = "usedEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub used_events: Vec<i64>,
}
impl From<&ContractDefinition> for ContractDefinition {
    fn from(value: &ContractDefinition) -> Self {
        value.clone()
    }
}
#[doc = "ContractDefinitionContractKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"contract\","]
#[doc = "    \"interface\","]
#[doc = "    \"library\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ContractDefinitionContractKind {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library,
}
impl From<&ContractDefinitionContractKind> for ContractDefinitionContractKind {
    fn from(value: &ContractDefinitionContractKind) -> Self {
        value.clone()
    }
}
impl ToString for ContractDefinitionContractKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Contract => "contract".to_string(),
            Self::Interface => "interface".to_string(),
            Self::Library => "library".to_string(),
        }
    }
}
impl std::str::FromStr for ContractDefinitionContractKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "contract" => Ok(Self::Contract),
            "interface" => Ok(Self::Interface),
            "library" => Ok(Self::Library),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ContractDefinitionContractKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ContractDefinitionContractKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ContractDefinitionContractKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ContractDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ContractDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ContractDefinitionNodeType {
    ContractDefinition,
}
impl From<&ContractDefinitionNodeType> for ContractDefinitionNodeType {
    fn from(value: &ContractDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ContractDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ContractDefinition => "ContractDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for ContractDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ContractDefinition" => Ok(Self::ContractDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ContractDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ContractDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ContractDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ContractDefinitionNodesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EventDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ModifierDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/StructDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedValueTypeDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UsingForDirective\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContractDefinitionNodesItem {
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}
impl From<&ContractDefinitionNodesItem> for ContractDefinitionNodesItem {
    fn from(value: &ContractDefinitionNodesItem) -> Self {
        value.clone()
    }
}
impl From<EnumDefinition> for ContractDefinitionNodesItem {
    fn from(value: EnumDefinition) -> Self {
        Self::EnumDefinition(value)
    }
}
impl From<ErrorDefinition> for ContractDefinitionNodesItem {
    fn from(value: ErrorDefinition) -> Self {
        Self::ErrorDefinition(value)
    }
}
impl From<EventDefinition> for ContractDefinitionNodesItem {
    fn from(value: EventDefinition) -> Self {
        Self::EventDefinition(value)
    }
}
impl From<FunctionDefinition> for ContractDefinitionNodesItem {
    fn from(value: FunctionDefinition) -> Self {
        Self::FunctionDefinition(value)
    }
}
impl From<ModifierDefinition> for ContractDefinitionNodesItem {
    fn from(value: ModifierDefinition) -> Self {
        Self::ModifierDefinition(value)
    }
}
impl From<StructDefinition> for ContractDefinitionNodesItem {
    fn from(value: StructDefinition) -> Self {
        Self::StructDefinition(value)
    }
}
impl From<UserDefinedValueTypeDefinition> for ContractDefinitionNodesItem {
    fn from(value: UserDefinedValueTypeDefinition) -> Self {
        Self::UserDefinedValueTypeDefinition(value)
    }
}
impl From<UsingForDirective> for ContractDefinitionNodesItem {
    fn from(value: UsingForDirective) -> Self {
        Self::UsingForDirective(value)
    }
}
impl From<VariableDeclaration> for ContractDefinitionNodesItem {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}
#[doc = "DoWhileStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"condition\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Block\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Statement\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"DoWhileStatement\""]
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
pub struct DoWhileStatement {
    pub body: DoWhileStatementBody,
    pub condition: Expression,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: DoWhileStatementNodeType,
    pub src: SourceLocation,
}
impl From<&DoWhileStatement> for DoWhileStatement {
    fn from(value: &DoWhileStatement) -> Self {
        value.clone()
    }
}
#[doc = "DoWhileStatementBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Statement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DoWhileStatementBody {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Block>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Statement>,
}
impl From<&DoWhileStatementBody> for DoWhileStatementBody {
    fn from(value: &DoWhileStatementBody) -> Self {
        value.clone()
    }
}
#[doc = "DoWhileStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"DoWhileStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DoWhileStatementNodeType {
    DoWhileStatement,
}
impl From<&DoWhileStatementNodeType> for DoWhileStatementNodeType {
    fn from(value: &DoWhileStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for DoWhileStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::DoWhileStatement => "DoWhileStatement".to_string(),
        }
    }
}
impl std::str::FromStr for DoWhileStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "DoWhileStatement" => Ok(Self::DoWhileStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DoWhileStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DoWhileStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DoWhileStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ElementaryTypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ElementaryTypeName\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateMutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/StateMutability\""]
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
pub struct ElementaryTypeName {
    pub id: i64,
    pub name: String,
    #[serde(rename = "nodeType")]
    pub node_type: ElementaryTypeNameNodeType,
    pub src: SourceLocation,
    #[serde(
        rename = "stateMutability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_mutability: Option<StateMutability>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&ElementaryTypeName> for ElementaryTypeName {
    fn from(value: &ElementaryTypeName) -> Self {
        value.clone()
    }
}
#[doc = "ElementaryTypeNameExpression"]
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
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"typeName\""]
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
#[doc = "        \"ElementaryTypeNameExpression\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"typeName\": {"]
#[doc = "      \"$ref\": \"#/definitions/ElementaryTypeName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementaryTypeNameExpression {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
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
    pub node_type: ElementaryTypeNameExpressionNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    pub type_name: ElementaryTypeName,
}
impl From<&ElementaryTypeNameExpression> for ElementaryTypeNameExpression {
    fn from(value: &ElementaryTypeNameExpression) -> Self {
        value.clone()
    }
}
#[doc = "ElementaryTypeNameExpressionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ElementaryTypeNameExpression\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ElementaryTypeNameExpressionNodeType {
    ElementaryTypeNameExpression,
}
impl From<&ElementaryTypeNameExpressionNodeType> for ElementaryTypeNameExpressionNodeType {
    fn from(value: &ElementaryTypeNameExpressionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ElementaryTypeNameExpressionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ElementaryTypeNameExpression => "ElementaryTypeNameExpression".to_string(),
        }
    }
}
impl std::str::FromStr for ElementaryTypeNameExpressionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ElementaryTypeNameExpression" => Ok(Self::ElementaryTypeNameExpression),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ElementaryTypeNameExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ElementaryTypeNameExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ElementaryTypeNameExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ElementaryTypeNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ElementaryTypeName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ElementaryTypeNameNodeType {
    ElementaryTypeName,
}
impl From<&ElementaryTypeNameNodeType> for ElementaryTypeNameNodeType {
    fn from(value: &ElementaryTypeNameNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ElementaryTypeNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ElementaryTypeName => "ElementaryTypeName".to_string(),
        }
    }
}
impl std::str::FromStr for ElementaryTypeNameNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ElementaryTypeName" => Ok(Self::ElementaryTypeName),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ElementaryTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ElementaryTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ElementaryTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "EmitStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"eventCall\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"eventCall\": {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"EmitStatement\""]
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
pub struct EmitStatement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "eventCall")]
    pub event_call: FunctionCall,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: EmitStatementNodeType,
    pub src: SourceLocation,
}
impl From<&EmitStatement> for EmitStatement {
    fn from(value: &EmitStatement) -> Self {
        value.clone()
    }
}
#[doc = "EmitStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EmitStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EmitStatementNodeType {
    EmitStatement,
}
impl From<&EmitStatementNodeType> for EmitStatementNodeType {
    fn from(value: &EmitStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for EmitStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EmitStatement => "EmitStatement".to_string(),
        }
    }
}
impl std::str::FromStr for EmitStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "EmitStatement" => Ok(Self::EmitStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EmitStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EmitStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EmitStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "EnumDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"canonicalName\","]
#[doc = "    \"id\","]
#[doc = "    \"members\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonicalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"members\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/EnumValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"EnumDefinition\""]
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
pub struct EnumDefinition {
    #[serde(rename = "canonicalName")]
    pub canonical_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    pub id: i64,
    pub members: Vec<EnumValue>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: EnumDefinitionNodeType,
    pub src: SourceLocation,
}
impl From<&EnumDefinition> for EnumDefinition {
    fn from(value: &EnumDefinition) -> Self {
        value.clone()
    }
}
#[doc = "EnumDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EnumDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EnumDefinitionNodeType {
    EnumDefinition,
}
impl From<&EnumDefinitionNodeType> for EnumDefinitionNodeType {
    fn from(value: &EnumDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for EnumDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EnumDefinition => "EnumDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for EnumDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "EnumDefinition" => Ok(Self::EnumDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EnumDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EnumDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EnumDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "EnumValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"EnumValue\""]
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
pub struct EnumValue {
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: EnumValueNodeType,
    pub src: SourceLocation,
}
impl From<&EnumValue> for EnumValue {
    fn from(value: &EnumValue) -> Self {
        value.clone()
    }
}
#[doc = "EnumValueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EnumValue\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EnumValueNodeType {
    EnumValue,
}
impl From<&EnumValueNodeType> for EnumValueNodeType {
    fn from(value: &EnumValueNodeType) -> Self {
        value.clone()
    }
}
impl ToString for EnumValueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EnumValue => "EnumValue".to_string(),
        }
    }
}
impl std::str::FromStr for EnumValueNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "EnumValue" => Ok(Self::EnumValue),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EnumValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EnumValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EnumValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ErrorDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nameLocation\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"errorSelector\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ErrorDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
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
pub struct ErrorDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "errorSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_selector: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "nameLocation")]
    pub name_location: String,
    #[serde(rename = "nodeType")]
    pub node_type: ErrorDefinitionNodeType,
    pub parameters: ParameterList,
    pub src: SourceLocation,
}
impl From<&ErrorDefinition> for ErrorDefinition {
    fn from(value: &ErrorDefinition) -> Self {
        value.clone()
    }
}
#[doc = "ErrorDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ErrorDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ErrorDefinitionNodeType {
    ErrorDefinition,
}
impl From<&ErrorDefinitionNodeType> for ErrorDefinitionNodeType {
    fn from(value: &ErrorDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ErrorDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ErrorDefinition => "ErrorDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for ErrorDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ErrorDefinition" => Ok(Self::ErrorDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ErrorDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ErrorDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ErrorDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "EventDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"anonymous\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"anonymous\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"eventSelector\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"EventDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
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
pub struct EventDefinition {
    pub anonymous: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "eventSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_selector: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: EventDefinitionNodeType,
    pub parameters: ParameterList,
    pub src: SourceLocation,
}
impl From<&EventDefinition> for EventDefinition {
    fn from(value: &EventDefinition) -> Self {
        value.clone()
    }
}
#[doc = "EventDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"EventDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EventDefinitionNodeType {
    EventDefinition,
}
impl From<&EventDefinitionNodeType> for EventDefinitionNodeType {
    fn from(value: &EventDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for EventDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::EventDefinition => "EventDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for EventDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "EventDefinition" => Ok(Self::EventDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EventDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EventDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EventDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Expression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Assignment\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BinaryOperation\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Conditional\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElementaryTypeNameExpression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCallOptions\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IndexAccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IndexRangeAccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Literal\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/MemberAccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/NewExpression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TupleExpression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UnaryOperation\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expression {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Assignment>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<BinaryOperation>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<Conditional>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<ElementaryTypeNameExpression>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<FunctionCall>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_5: Option<FunctionCallOptions>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_6: Option<Identifier>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_7: Option<IndexAccess>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_8: Option<IndexRangeAccess>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_9: Option<Literal>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_10: Option<MemberAccess>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_11: Option<NewExpression>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_12: Option<TupleExpression>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_13: Option<UnaryOperation>,
}
impl From<&Expression> for Expression {
    fn from(value: &Expression) -> Self {
        value.clone()
    }
}
#[doc = "ExpressionStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expression\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"expression\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ExpressionStatement\""]
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
pub struct ExpressionStatement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub expression: Expression,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: ExpressionStatementNodeType,
    pub src: SourceLocation,
}
impl From<&ExpressionStatement> for ExpressionStatement {
    fn from(value: &ExpressionStatement) -> Self {
        value.clone()
    }
}
#[doc = "ExpressionStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ExpressionStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ExpressionStatementNodeType {
    ExpressionStatement,
}
impl From<&ExpressionStatementNodeType> for ExpressionStatementNodeType {
    fn from(value: &ExpressionStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ExpressionStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ExpressionStatement => "ExpressionStatement".to_string(),
        }
    }
}
impl std::str::FromStr for ExpressionStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ExpressionStatement" => Ok(Self::ExpressionStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ExpressionStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ExpressionStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ExpressionStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ForStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Block\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Statement\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"initializationExpression\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ExpressionStatement\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/VariableDeclarationStatement\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isSimpleCounterLoop\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"loopExpression\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ExpressionStatement\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ForStatement\""]
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
pub struct ForStatement {
    pub body: ForStatementBody,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Expression>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(
        rename = "initializationExpression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initialization_expression: Option<ForStatementInitializationExpression>,
    #[serde(
        rename = "isSimpleCounterLoop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_simple_counter_loop: Option<bool>,
    #[serde(
        rename = "loopExpression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loop_expression: Option<ExpressionStatement>,
    #[serde(rename = "nodeType")]
    pub node_type: ForStatementNodeType,
    pub src: SourceLocation,
}
impl From<&ForStatement> for ForStatement {
    fn from(value: &ForStatement) -> Self {
        value.clone()
    }
}
#[doc = "ForStatementBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Statement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForStatementBody {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Block>,
    #[serde(flatten, default)]
    pub subtype_1: Box<Option<Statement>>,
}
impl From<&ForStatementBody> for ForStatementBody {
    fn from(value: &ForStatementBody) -> Self {
        value.clone()
    }
}
#[doc = "ForStatementInitializationExpression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExpressionStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclarationStatement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ForStatementInitializationExpression {
    ExpressionStatement(ExpressionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}
impl From<&ForStatementInitializationExpression> for ForStatementInitializationExpression {
    fn from(value: &ForStatementInitializationExpression) -> Self {
        value.clone()
    }
}
impl From<ExpressionStatement> for ForStatementInitializationExpression {
    fn from(value: ExpressionStatement) -> Self {
        Self::ExpressionStatement(value)
    }
}
impl From<VariableDeclarationStatement> for ForStatementInitializationExpression {
    fn from(value: VariableDeclarationStatement) -> Self {
        Self::VariableDeclarationStatement(value)
    }
}
#[doc = "ForStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ForStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ForStatementNodeType {
    ForStatement,
}
impl From<&ForStatementNodeType> for ForStatementNodeType {
    fn from(value: &ForStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ForStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ForStatement => "ForStatement".to_string(),
        }
    }
}
impl std::str::FromStr for ForStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ForStatement" => Ok(Self::ForStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ForStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ForStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ForStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "functionCall" => Ok(Self::FunctionCall),
            "typeConversion" => Ok(Self::TypeConversion),
            "structConstructorCall" => Ok(Self::StructConstructorCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionCallKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionCallKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionCallKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "FunctionCall" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionCallNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionCallNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionCallNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "FunctionCallOptions" => Ok(Self::FunctionCallOptions),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionCallOptionsNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionCallOptionsNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionCallOptionsNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "FunctionDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"implemented\","]
#[doc = "    \"kind\","]
#[doc = "    \"modifiers\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"returnParameters\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\","]
#[doc = "    \"stateMutability\","]
#[doc = "    \"virtual\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseFunctions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"body\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Block\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"functionSelector\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"implemented\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"function\","]
#[doc = "        \"receive\","]
#[doc = "        \"constructor\","]
#[doc = "        \"fallback\","]
#[doc = "        \"freeFunction\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modifiers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ModifierInvocation\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"FunctionDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overrides\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/OverrideSpecifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"returnParameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateMutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/StateMutability\""]
#[doc = "    },"]
#[doc = "    \"virtual\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"$ref\": \"#/definitions/Visibility\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionDefinition {
    #[serde(
        rename = "baseFunctions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub base_functions: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "functionSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_selector: Option<String>,
    pub id: i64,
    pub implemented: bool,
    pub kind: FunctionDefinitionKind,
    pub modifiers: Vec<ModifierInvocation>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: FunctionDefinitionNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub parameters: ParameterList,
    #[serde(rename = "returnParameters")]
    pub return_parameters: ParameterList,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
    #[serde(rename = "virtual")]
    pub virtual_: bool,
    pub visibility: Visibility,
}
impl From<&FunctionDefinition> for FunctionDefinition {
    fn from(value: &FunctionDefinition) -> Self {
        value.clone()
    }
}
#[doc = "FunctionDefinitionKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"function\","]
#[doc = "    \"receive\","]
#[doc = "    \"constructor\","]
#[doc = "    \"fallback\","]
#[doc = "    \"freeFunction\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionDefinitionKind {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "receive")]
    Receive,
    #[serde(rename = "constructor")]
    Constructor,
    #[serde(rename = "fallback")]
    Fallback,
    #[serde(rename = "freeFunction")]
    FreeFunction,
}
impl From<&FunctionDefinitionKind> for FunctionDefinitionKind {
    fn from(value: &FunctionDefinitionKind) -> Self {
        value.clone()
    }
}
impl ToString for FunctionDefinitionKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Function => "function".to_string(),
            Self::Receive => "receive".to_string(),
            Self::Constructor => "constructor".to_string(),
            Self::Fallback => "fallback".to_string(),
            Self::FreeFunction => "freeFunction".to_string(),
        }
    }
}
impl std::str::FromStr for FunctionDefinitionKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            "receive" => Ok(Self::Receive),
            "constructor" => Ok(Self::Constructor),
            "fallback" => Ok(Self::Fallback),
            "freeFunction" => Ok(Self::FreeFunction),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionDefinitionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionDefinitionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionDefinitionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "FunctionDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"FunctionDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionDefinitionNodeType {
    FunctionDefinition,
}
impl From<&FunctionDefinitionNodeType> for FunctionDefinitionNodeType {
    fn from(value: &FunctionDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for FunctionDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionDefinition => "FunctionDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for FunctionDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "FunctionDefinition" => Ok(Self::FunctionDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "FunctionTypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameterTypes\","]
#[doc = "    \"returnParameterTypes\","]
#[doc = "    \"src\","]
#[doc = "    \"stateMutability\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"FunctionTypeName\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameterTypes\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"returnParameterTypes\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateMutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/StateMutability\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"$ref\": \"#/definitions/Visibility\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionTypeName {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: FunctionTypeNameNodeType,
    #[serde(rename = "parameterTypes")]
    pub parameter_types: ParameterList,
    #[serde(rename = "returnParameterTypes")]
    pub return_parameter_types: ParameterList,
    pub src: SourceLocation,
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    pub visibility: Visibility,
}
impl From<&FunctionTypeName> for FunctionTypeName {
    fn from(value: &FunctionTypeName) -> Self {
        value.clone()
    }
}
#[doc = "FunctionTypeNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"FunctionTypeName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionTypeNameNodeType {
    FunctionTypeName,
}
impl From<&FunctionTypeNameNodeType> for FunctionTypeNameNodeType {
    fn from(value: &FunctionTypeNameNodeType) -> Self {
        value.clone()
    }
}
impl ToString for FunctionTypeNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::FunctionTypeName => "FunctionTypeName".to_string(),
        }
    }
}
impl std::str::FromStr for FunctionTypeNameNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "FunctionTypeName" => Ok(Self::FunctionTypeName),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FunctionTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FunctionTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FunctionTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"overloadedDeclarations\","]
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
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Identifier\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overloadedDeclarations\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"referencedDeclaration\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
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
pub struct Identifier {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "nodeType")]
    pub node_type: IdentifierNodeType,
    #[serde(rename = "overloadedDeclarations")]
    pub overloaded_declarations: Vec<i64>,
    #[serde(
        rename = "referencedDeclaration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&Identifier> for Identifier {
    fn from(value: &Identifier) -> Self {
        value.clone()
    }
}
#[doc = "IdentifierNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Identifier\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IdentifierNodeType {
    Identifier,
}
impl From<&IdentifierNodeType> for IdentifierNodeType {
    fn from(value: &IdentifierNodeType) -> Self {
        value.clone()
    }
}
impl ToString for IdentifierNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Identifier => "Identifier".to_string(),
        }
    }
}
impl std::str::FromStr for IdentifierNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Identifier" => Ok(Self::Identifier),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IdentifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IdentifierPath"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"referencedDeclaration\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocations\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"IdentifierPath\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"referencedDeclaration\": {"]
#[doc = "      \"type\": \"integer\""]
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
pub struct IdentifierPath {
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name_locations: Vec<String>,
    #[serde(rename = "nodeType")]
    pub node_type: IdentifierPathNodeType,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,
    pub src: SourceLocation,
}
impl From<&IdentifierPath> for IdentifierPath {
    fn from(value: &IdentifierPath) -> Self {
        value.clone()
    }
}
#[doc = "IdentifierPathNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"IdentifierPath\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IdentifierPathNodeType {
    IdentifierPath,
}
impl From<&IdentifierPathNodeType> for IdentifierPathNodeType {
    fn from(value: &IdentifierPathNodeType) -> Self {
        value.clone()
    }
}
impl ToString for IdentifierPathNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::IdentifierPath => "IdentifierPath".to_string(),
        }
    }
}
impl std::str::FromStr for IdentifierPathNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "IdentifierPath" => Ok(Self::IdentifierPath),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IdentifierPathNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentifierPathNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentifierPathNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IfStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"condition\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"trueBody\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"falseBody\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Statement\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Block\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"IfStatement\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"trueBody\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Statement\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Block\""]
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
pub struct IfStatement {
    pub condition: Expression,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "falseBody", default, skip_serializing_if = "Option::is_none")]
    pub false_body: Option<IfStatementFalseBody>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: IfStatementNodeType,
    pub src: SourceLocation,
    #[serde(rename = "trueBody")]
    pub true_body: IfStatementTrueBody,
}
impl From<&IfStatement> for IfStatement {
    fn from(value: &IfStatement) -> Self {
        value.clone()
    }
}
#[doc = "IfStatementFalseBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Statement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IfStatementFalseBody {
    #[serde(flatten, default)]
    pub subtype_0: Box<Option<Statement>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Block>,
}
impl From<&IfStatementFalseBody> for IfStatementFalseBody {
    fn from(value: &IfStatementFalseBody) -> Self {
        value.clone()
    }
}
#[doc = "IfStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"IfStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IfStatementNodeType {
    IfStatement,
}
impl From<&IfStatementNodeType> for IfStatementNodeType {
    fn from(value: &IfStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for IfStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::IfStatement => "IfStatement".to_string(),
        }
    }
}
impl std::str::FromStr for IfStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "IfStatement" => Ok(Self::IfStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IfStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IfStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IfStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IfStatementTrueBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Statement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IfStatementTrueBody {
    #[serde(flatten, default)]
    pub subtype_0: Box<Option<Statement>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Block>,
}
impl From<&IfStatementTrueBody> for IfStatementTrueBody {
    fn from(value: &IfStatementTrueBody) -> Self {
        value.clone()
    }
}
#[doc = "ImportDirective"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"absolutePath\","]
#[doc = "    \"file\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"scope\","]
#[doc = "    \"sourceUnit\","]
#[doc = "    \"src\","]
#[doc = "    \"symbolAliases\","]
#[doc = "    \"unitAlias\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"absolutePath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ImportDirective\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"sourceUnit\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"symbolAliases\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"foreign\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"foreign\": {"]
#[doc = "            \"$ref\": \"#/definitions/Identifier\""]
#[doc = "          },"]
#[doc = "          \"local\": {"]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"null\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"nameLocation\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"unitAlias\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportDirective {
    #[serde(rename = "absolutePath")]
    pub absolute_path: String,
    pub file: String,
    pub id: i64,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: ImportDirectiveNodeType,
    pub scope: i64,
    #[serde(rename = "sourceUnit")]
    pub source_unit: i64,
    pub src: SourceLocation,
    #[serde(rename = "symbolAliases")]
    pub symbol_aliases: Vec<ImportDirectiveSymbolAliasesItem>,
    #[serde(rename = "unitAlias")]
    pub unit_alias: String,
}
impl From<&ImportDirective> for ImportDirective {
    fn from(value: &ImportDirective) -> Self {
        value.clone()
    }
}
#[doc = "ImportDirectiveNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ImportDirective\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ImportDirectiveNodeType {
    ImportDirective,
}
impl From<&ImportDirectiveNodeType> for ImportDirectiveNodeType {
    fn from(value: &ImportDirectiveNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ImportDirectiveNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ImportDirective => "ImportDirective".to_string(),
        }
    }
}
impl std::str::FromStr for ImportDirectiveNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ImportDirective" => Ok(Self::ImportDirective),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ImportDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ImportDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ImportDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ImportDirectiveSymbolAliasesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"foreign\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"foreign\": {"]
#[doc = "      \"$ref\": \"#/definitions/Identifier\""]
#[doc = "    },"]
#[doc = "    \"local\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportDirectiveSymbolAliasesItem {
    pub foreign: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<String>,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
}
impl From<&ImportDirectiveSymbolAliasesItem> for ImportDirectiveSymbolAliasesItem {
    fn from(value: &ImportDirectiveSymbolAliasesItem) -> Self {
        value.clone()
    }
}
#[doc = "IndexAccess"]
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
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"indexExpression\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
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
#[doc = "        \"IndexAccess\""]
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
pub struct IndexAccess {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "baseExpression")]
    pub base_expression: Box<Expression>,
    pub id: i64,
    #[serde(rename = "indexExpression", default)]
    pub index_expression: Box<Option<Expression>>,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "nodeType")]
    pub node_type: IndexAccessNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&IndexAccess> for IndexAccess {
    fn from(value: &IndexAccess) -> Self {
        value.clone()
    }
}
#[doc = "IndexAccessNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"IndexAccess\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IndexAccessNodeType {
    IndexAccess,
}
impl From<&IndexAccessNodeType> for IndexAccessNodeType {
    fn from(value: &IndexAccessNodeType) -> Self {
        value.clone()
    }
}
impl ToString for IndexAccessNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::IndexAccess => "IndexAccess".to_string(),
        }
    }
}
impl std::str::FromStr for IndexAccessNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "IndexAccess" => Ok(Self::IndexAccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IndexAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IndexAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IndexAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
#[serde(deny_unknown_fields)]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "IndexRangeAccess" => Ok(Self::IndexRangeAccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IndexRangeAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IndexRangeAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IndexRangeAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
#[serde(deny_unknown_fields)]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "InheritanceSpecifier" => Ok(Self::InheritanceSpecifier),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InheritanceSpecifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InheritanceSpecifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InheritanceSpecifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InlineAssembly"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"AST\","]
#[doc = "    \"evmVersion\","]
#[doc = "    \"externalReferences\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AST\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"evmVersion\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"homestead\","]
#[doc = "        \"tangerineWhistle\","]
#[doc = "        \"spuriousDragon\","]
#[doc = "        \"byzantium\","]
#[doc = "        \"constantinople\","]
#[doc = "        \"petersburg\","]
#[doc = "        \"istanbul\","]
#[doc = "        \"berlin\","]
#[doc = "        \"london\","]
#[doc = "        \"paris\","]
#[doc = "        \"shanghai\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"externalReferences\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"declaration\","]
#[doc = "          \"isOffset\","]
#[doc = "          \"isSlot\","]
#[doc = "          \"src\","]
#[doc = "          \"valueSize\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"declaration\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"isOffset\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"isSlot\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"src\": {"]
#[doc = "            \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "          },"]
#[doc = "          \"suffix\": {"]
#[doc = "            \"enum\": ["]
#[doc = "              \"slot\","]
#[doc = "              \"offset\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"valueSize\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"enum\": ["]
#[doc = "          \"memory-safe\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"InlineAssembly\""]
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
pub struct InlineAssembly {
    #[serde(rename = "AST")]
    pub ast: YulBlock,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "evmVersion")]
    pub evm_version: InlineAssemblyEvmVersion,
    #[serde(rename = "externalReferences")]
    pub external_references: Vec<InlineAssemblyExternalReferencesItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<InlineAssemblyFlagsItem>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: InlineAssemblyNodeType,
    pub src: SourceLocation,
}
impl From<&InlineAssembly> for InlineAssembly {
    fn from(value: &InlineAssembly) -> Self {
        value.clone()
    }
}
#[doc = "InlineAssemblyEvmVersion"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"homestead\","]
#[doc = "    \"tangerineWhistle\","]
#[doc = "    \"spuriousDragon\","]
#[doc = "    \"byzantium\","]
#[doc = "    \"constantinople\","]
#[doc = "    \"petersburg\","]
#[doc = "    \"istanbul\","]
#[doc = "    \"berlin\","]
#[doc = "    \"london\","]
#[doc = "    \"paris\","]
#[doc = "    \"shanghai\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyEvmVersion {
    #[serde(rename = "homestead")]
    Homestead,
    #[serde(rename = "tangerineWhistle")]
    TangerineWhistle,
    #[serde(rename = "spuriousDragon")]
    SpuriousDragon,
    #[serde(rename = "byzantium")]
    Byzantium,
    #[serde(rename = "constantinople")]
    Constantinople,
    #[serde(rename = "petersburg")]
    Petersburg,
    #[serde(rename = "istanbul")]
    Istanbul,
    #[serde(rename = "berlin")]
    Berlin,
    #[serde(rename = "london")]
    London,
    #[serde(rename = "paris")]
    Paris,
    #[serde(rename = "shanghai")]
    Shanghai,
}
impl From<&InlineAssemblyEvmVersion> for InlineAssemblyEvmVersion {
    fn from(value: &InlineAssemblyEvmVersion) -> Self {
        value.clone()
    }
}
impl ToString for InlineAssemblyEvmVersion {
    fn to_string(&self) -> String {
        match *self {
            Self::Homestead => "homestead".to_string(),
            Self::TangerineWhistle => "tangerineWhistle".to_string(),
            Self::SpuriousDragon => "spuriousDragon".to_string(),
            Self::Byzantium => "byzantium".to_string(),
            Self::Constantinople => "constantinople".to_string(),
            Self::Petersburg => "petersburg".to_string(),
            Self::Istanbul => "istanbul".to_string(),
            Self::Berlin => "berlin".to_string(),
            Self::London => "london".to_string(),
            Self::Paris => "paris".to_string(),
            Self::Shanghai => "shanghai".to_string(),
        }
    }
}
impl std::str::FromStr for InlineAssemblyEvmVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "homestead" => Ok(Self::Homestead),
            "tangerineWhistle" => Ok(Self::TangerineWhistle),
            "spuriousDragon" => Ok(Self::SpuriousDragon),
            "byzantium" => Ok(Self::Byzantium),
            "constantinople" => Ok(Self::Constantinople),
            "petersburg" => Ok(Self::Petersburg),
            "istanbul" => Ok(Self::Istanbul),
            "berlin" => Ok(Self::Berlin),
            "london" => Ok(Self::London),
            "paris" => Ok(Self::Paris),
            "shanghai" => Ok(Self::Shanghai),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineAssemblyEvmVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineAssemblyEvmVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineAssemblyEvmVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InlineAssemblyExternalReferencesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"declaration\","]
#[doc = "    \"isOffset\","]
#[doc = "    \"isSlot\","]
#[doc = "    \"src\","]
#[doc = "    \"valueSize\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"declaration\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isOffset\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSlot\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"suffix\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"slot\","]
#[doc = "        \"offset\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueSize\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InlineAssemblyExternalReferencesItem {
    pub declaration: i64,
    #[serde(rename = "isOffset")]
    pub is_offset: bool,
    #[serde(rename = "isSlot")]
    pub is_slot: bool,
    pub src: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<InlineAssemblyExternalReferencesItemSuffix>,
    #[serde(rename = "valueSize")]
    pub value_size: i64,
}
impl From<&InlineAssemblyExternalReferencesItem> for InlineAssemblyExternalReferencesItem {
    fn from(value: &InlineAssemblyExternalReferencesItem) -> Self {
        value.clone()
    }
}
#[doc = "InlineAssemblyExternalReferencesItemSuffix"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"slot\","]
#[doc = "    \"offset\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyExternalReferencesItemSuffix {
    #[serde(rename = "slot")]
    Slot,
    #[serde(rename = "offset")]
    Offset,
}
impl From<&InlineAssemblyExternalReferencesItemSuffix>
    for InlineAssemblyExternalReferencesItemSuffix
{
    fn from(value: &InlineAssemblyExternalReferencesItemSuffix) -> Self {
        value.clone()
    }
}
impl ToString for InlineAssemblyExternalReferencesItemSuffix {
    fn to_string(&self) -> String {
        match *self {
            Self::Slot => "slot".to_string(),
            Self::Offset => "offset".to_string(),
        }
    }
}
impl std::str::FromStr for InlineAssemblyExternalReferencesItemSuffix {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "slot" => Ok(Self::Slot),
            "offset" => Ok(Self::Offset),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineAssemblyExternalReferencesItemSuffix {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineAssemblyExternalReferencesItemSuffix {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineAssemblyExternalReferencesItemSuffix {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InlineAssemblyFlagsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"memory-safe\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyFlagsItem {
    #[serde(rename = "memory-safe")]
    MemorySafe,
}
impl From<&InlineAssemblyFlagsItem> for InlineAssemblyFlagsItem {
    fn from(value: &InlineAssemblyFlagsItem) -> Self {
        value.clone()
    }
}
impl ToString for InlineAssemblyFlagsItem {
    fn to_string(&self) -> String {
        match *self {
            Self::MemorySafe => "memory-safe".to_string(),
        }
    }
}
impl std::str::FromStr for InlineAssemblyFlagsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "memory-safe" => Ok(Self::MemorySafe),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineAssemblyFlagsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineAssemblyFlagsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineAssemblyFlagsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InlineAssemblyNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"InlineAssembly\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyNodeType {
    InlineAssembly,
}
impl From<&InlineAssemblyNodeType> for InlineAssemblyNodeType {
    fn from(value: &InlineAssemblyNodeType) -> Self {
        value.clone()
    }
}
impl ToString for InlineAssemblyNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::InlineAssembly => "InlineAssembly".to_string(),
        }
    }
}
impl std::str::FromStr for InlineAssemblyNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "InlineAssembly" => Ok(Self::InlineAssembly),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineAssemblyNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineAssemblyNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineAssemblyNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Literal"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"hexValue\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"kind\","]
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
#[doc = "    \"hexValue\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9a-f]*$\""]
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
#[doc = "        \"bool\","]
#[doc = "        \"number\","]
#[doc = "        \"string\","]
#[doc = "        \"hexString\","]
#[doc = "        \"unicodeString\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lValueRequested\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Literal\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"subdenomination\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"seconds\","]
#[doc = "            \"minutes\","]
#[doc = "            \"hours\","]
#[doc = "            \"days\","]
#[doc = "            \"weeks\","]
#[doc = "            \"wei\","]
#[doc = "            \"gwei\","]
#[doc = "            \"ether\","]
#[doc = "            \"finney\","]
#[doc = "            \"szabo\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
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
pub struct Literal {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "hexValue")]
    pub hex_value: LiteralHexValue,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    pub kind: LiteralKind,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "nodeType")]
    pub node_type: LiteralNodeType,
    pub src: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdenomination: Option<LiteralSubdenomination>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&Literal> for Literal {
    fn from(value: &Literal) -> Self {
        value.clone()
    }
}
#[doc = "LiteralHexValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-f]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiteralHexValue(String);
impl std::ops::Deref for LiteralHexValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiteralHexValue> for String {
    fn from(value: LiteralHexValue) -> Self {
        value.0
    }
}
impl From<&LiteralHexValue> for LiteralHexValue {
    fn from(value: &LiteralHexValue) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiteralHexValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9a-f]*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-f]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiteralHexValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiteralHexValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiteralHexValue {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiteralHexValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "LiteralKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"bool\","]
#[doc = "    \"number\","]
#[doc = "    \"string\","]
#[doc = "    \"hexString\","]
#[doc = "    \"unicodeString\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiteralKind {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "hexString")]
    HexString,
    #[serde(rename = "unicodeString")]
    UnicodeString,
}
impl From<&LiteralKind> for LiteralKind {
    fn from(value: &LiteralKind) -> Self {
        value.clone()
    }
}
impl ToString for LiteralKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Bool => "bool".to_string(),
            Self::Number => "number".to_string(),
            Self::String => "string".to_string(),
            Self::HexString => "hexString".to_string(),
            Self::UnicodeString => "unicodeString".to_string(),
        }
    }
}
impl std::str::FromStr for LiteralKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "bool" => Ok(Self::Bool),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "hexString" => Ok(Self::HexString),
            "unicodeString" => Ok(Self::UnicodeString),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LiteralKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiteralKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiteralKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "LiteralNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Literal\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiteralNodeType {
    Literal,
}
impl From<&LiteralNodeType> for LiteralNodeType {
    fn from(value: &LiteralNodeType) -> Self {
        value.clone()
    }
}
impl ToString for LiteralNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Literal => "Literal".to_string(),
        }
    }
}
impl std::str::FromStr for LiteralNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Literal" => Ok(Self::Literal),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LiteralNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiteralNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiteralNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "LiteralSubdenomination"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"seconds\","]
#[doc = "    \"minutes\","]
#[doc = "    \"hours\","]
#[doc = "    \"days\","]
#[doc = "    \"weeks\","]
#[doc = "    \"wei\","]
#[doc = "    \"gwei\","]
#[doc = "    \"ether\","]
#[doc = "    \"finney\","]
#[doc = "    \"szabo\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiteralSubdenomination {
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "wei")]
    Wei,
    #[serde(rename = "gwei")]
    Gwei,
    #[serde(rename = "ether")]
    Ether,
    #[serde(rename = "finney")]
    Finney,
    #[serde(rename = "szabo")]
    Szabo,
}
impl From<&LiteralSubdenomination> for LiteralSubdenomination {
    fn from(value: &LiteralSubdenomination) -> Self {
        value.clone()
    }
}
impl ToString for LiteralSubdenomination {
    fn to_string(&self) -> String {
        match *self {
            Self::Seconds => "seconds".to_string(),
            Self::Minutes => "minutes".to_string(),
            Self::Hours => "hours".to_string(),
            Self::Days => "days".to_string(),
            Self::Weeks => "weeks".to_string(),
            Self::Wei => "wei".to_string(),
            Self::Gwei => "gwei".to_string(),
            Self::Ether => "ether".to_string(),
            Self::Finney => "finney".to_string(),
            Self::Szabo => "szabo".to_string(),
        }
    }
}
impl std::str::FromStr for LiteralSubdenomination {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "seconds" => Ok(Self::Seconds),
            "minutes" => Ok(Self::Minutes),
            "hours" => Ok(Self::Hours),
            "days" => Ok(Self::Days),
            "weeks" => Ok(Self::Weeks),
            "wei" => Ok(Self::Wei),
            "gwei" => Ok(Self::Gwei),
            "ether" => Ok(Self::Ether),
            "finney" => Ok(Self::Finney),
            "szabo" => Ok(Self::Szabo),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LiteralSubdenomination {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiteralSubdenomination {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiteralSubdenomination {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Mapping"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"keyType\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"valueType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"keyName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"keyNameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"keyType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Mapping\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"valueName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"valueNameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"valueType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    pub id: i64,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(
        rename = "keyNameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_name_location: Option<String>,
    #[serde(rename = "keyType")]
    pub key_type: Box<TypeName>,
    #[serde(rename = "nodeType")]
    pub node_type: MappingNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "valueName", default, skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
    #[serde(
        rename = "valueNameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub value_name_location: Option<String>,
    #[serde(rename = "valueType")]
    pub value_type: Box<TypeName>,
}
impl From<&Mapping> for Mapping {
    fn from(value: &Mapping) -> Self {
        value.clone()
    }
}
#[doc = "MappingNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Mapping\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MappingNodeType {
    Mapping,
}
impl From<&MappingNodeType> for MappingNodeType {
    fn from(value: &MappingNodeType) -> Self {
        value.clone()
    }
}
impl ToString for MappingNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Mapping => "Mapping".to_string(),
        }
    }
}
impl std::str::FromStr for MappingNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Mapping" => Ok(Self::Mapping),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MappingNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MappingNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MappingNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "MemberAccess"]
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
#[doc = "    \"isLValue\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"memberName\","]
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
#[doc = "    \"memberLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"memberName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"MemberAccess\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"referencedDeclaration\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
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
pub struct MemberAccess {
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
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(
        rename = "memberLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub member_location: Option<String>,
    #[serde(rename = "memberName")]
    pub member_name: String,
    #[serde(rename = "nodeType")]
    pub node_type: MemberAccessNodeType,
    #[serde(
        rename = "referencedDeclaration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&MemberAccess> for MemberAccess {
    fn from(value: &MemberAccess) -> Self {
        value.clone()
    }
}
#[doc = "MemberAccessNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"MemberAccess\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MemberAccessNodeType {
    MemberAccess,
}
impl From<&MemberAccessNodeType> for MemberAccessNodeType {
    fn from(value: &MemberAccessNodeType) -> Self {
        value.clone()
    }
}
impl ToString for MemberAccessNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::MemberAccess => "MemberAccess".to_string(),
        }
    }
}
impl std::str::FromStr for MemberAccessNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "MemberAccess" => Ok(Self::MemberAccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MemberAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MemberAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MemberAccessNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ModifierDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\","]
#[doc = "    \"virtual\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseModifiers\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ModifierDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overrides\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/OverrideSpecifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"virtual\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"$ref\": \"#/definitions/Visibility\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModifierDefinition {
    #[serde(
        rename = "baseModifiers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_modifiers: Option<Vec<i64>>,
    pub body: Block,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: ModifierDefinitionNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    #[serde(rename = "virtual")]
    pub virtual_: bool,
    pub visibility: Visibility,
}
impl From<&ModifierDefinition> for ModifierDefinition {
    fn from(value: &ModifierDefinition) -> Self {
        value.clone()
    }
}
#[doc = "ModifierDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ModifierDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierDefinitionNodeType {
    ModifierDefinition,
}
impl From<&ModifierDefinitionNodeType> for ModifierDefinitionNodeType {
    fn from(value: &ModifierDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ModifierDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ModifierDefinition => "ModifierDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for ModifierDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ModifierDefinition" => Ok(Self::ModifierDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ModifierDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ModifierDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ModifierDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ModifierInvocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"modifierName\","]
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
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"modifierInvocation\","]
#[doc = "        \"baseConstructorSpecifier\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modifierName\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Identifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ModifierInvocation\""]
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
pub struct ModifierInvocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<Expression>>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModifierInvocationKind>,
    #[serde(rename = "modifierName")]
    pub modifier_name: ModifierInvocationModifierName,
    #[serde(rename = "nodeType")]
    pub node_type: ModifierInvocationNodeType,
    pub src: SourceLocation,
}
impl From<&ModifierInvocation> for ModifierInvocation {
    fn from(value: &ModifierInvocation) -> Self {
        value.clone()
    }
}
#[doc = "ModifierInvocationKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"modifierInvocation\","]
#[doc = "    \"baseConstructorSpecifier\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierInvocationKind {
    #[serde(rename = "modifierInvocation")]
    ModifierInvocation,
    #[serde(rename = "baseConstructorSpecifier")]
    BaseConstructorSpecifier,
}
impl From<&ModifierInvocationKind> for ModifierInvocationKind {
    fn from(value: &ModifierInvocationKind) -> Self {
        value.clone()
    }
}
impl ToString for ModifierInvocationKind {
    fn to_string(&self) -> String {
        match *self {
            Self::ModifierInvocation => "modifierInvocation".to_string(),
            Self::BaseConstructorSpecifier => "baseConstructorSpecifier".to_string(),
        }
    }
}
impl std::str::FromStr for ModifierInvocationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "modifierInvocation" => Ok(Self::ModifierInvocation),
            "baseConstructorSpecifier" => Ok(Self::BaseConstructorSpecifier),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ModifierInvocationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ModifierInvocationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ModifierInvocationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ModifierInvocationModifierName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Identifier\""]
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
pub enum ModifierInvocationModifierName {
    Variant0(Identifier),
    Variant1(IdentifierPath),
}
impl From<&ModifierInvocationModifierName> for ModifierInvocationModifierName {
    fn from(value: &ModifierInvocationModifierName) -> Self {
        value.clone()
    }
}
impl From<Identifier> for ModifierInvocationModifierName {
    fn from(value: Identifier) -> Self {
        Self::Variant0(value)
    }
}
impl From<IdentifierPath> for ModifierInvocationModifierName {
    fn from(value: IdentifierPath) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "ModifierInvocationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ModifierInvocation\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierInvocationNodeType {
    ModifierInvocation,
}
impl From<&ModifierInvocationNodeType> for ModifierInvocationNodeType {
    fn from(value: &ModifierInvocationNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ModifierInvocationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ModifierInvocation => "ModifierInvocation".to_string(),
        }
    }
}
impl std::str::FromStr for ModifierInvocationNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ModifierInvocation" => Ok(Self::ModifierInvocation),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ModifierInvocationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ModifierInvocationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ModifierInvocationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Mutability"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"mutable\","]
#[doc = "    \"immutable\","]
#[doc = "    \"constant\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Mutability {
    #[serde(rename = "mutable")]
    Mutable,
    #[serde(rename = "immutable")]
    Immutable,
    #[serde(rename = "constant")]
    Constant,
}
impl From<&Mutability> for Mutability {
    fn from(value: &Mutability) -> Self {
        value.clone()
    }
}
impl ToString for Mutability {
    fn to_string(&self) -> String {
        match *self {
            Self::Mutable => "mutable".to_string(),
            Self::Immutable => "immutable".to_string(),
            Self::Constant => "constant".to_string(),
        }
    }
}
impl std::str::FromStr for Mutability {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "mutable" => Ok(Self::Mutable),
            "immutable" => Ok(Self::Immutable),
            "constant" => Ok(Self::Constant),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Mutability {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Mutability {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Mutability {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "NewExpression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isPure\","]
#[doc = "    \"lValueRequested\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"typeName\""]
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
#[doc = "        \"NewExpression\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
#[doc = "    },"]
#[doc = "    \"typeName\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NewExpression {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue", default, skip_serializing_if = "Option::is_none")]
    pub is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "nodeType")]
    pub node_type: NewExpressionNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    pub type_name: TypeName,
}
impl From<&NewExpression> for NewExpression {
    fn from(value: &NewExpression) -> Self {
        value.clone()
    }
}
#[doc = "NewExpressionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"NewExpression\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NewExpressionNodeType {
    NewExpression,
}
impl From<&NewExpressionNodeType> for NewExpressionNodeType {
    fn from(value: &NewExpressionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for NewExpressionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::NewExpression => "NewExpression".to_string(),
        }
    }
}
impl std::str::FromStr for NewExpressionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "NewExpression" => Ok(Self::NewExpression),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NewExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NewExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NewExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "OverrideSpecifier" => Ok(Self::OverrideSpecifier),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for OverrideSpecifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OverrideSpecifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OverrideSpecifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
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
pub struct OverrideSpecifierOverrides {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Vec<UserDefinedTypeName>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Vec<IdentifierPath>>,
}
impl From<&OverrideSpecifierOverrides> for OverrideSpecifierOverrides {
    fn from(value: &OverrideSpecifierOverrides) -> Self {
        value.clone()
    }
}
#[doc = "ParameterList"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ParameterList\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "      }"]
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
pub struct ParameterList {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: ParameterListNodeType,
    pub parameters: Vec<VariableDeclaration>,
    pub src: SourceLocation,
}
impl From<&ParameterList> for ParameterList {
    fn from(value: &ParameterList) -> Self {
        value.clone()
    }
}
#[doc = "ParameterListNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ParameterList\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParameterListNodeType {
    ParameterList,
}
impl From<&ParameterListNodeType> for ParameterListNodeType {
    fn from(value: &ParameterListNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ParameterListNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::ParameterList => "ParameterList".to_string(),
        }
    }
}
impl std::str::FromStr for ParameterListNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ParameterList" => Ok(Self::ParameterList),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ParameterListNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParameterListNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParameterListNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "PlaceholderStatement"]
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
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"PlaceholderStatement\""]
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
pub struct PlaceholderStatement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: PlaceholderStatementNodeType,
    pub src: SourceLocation,
}
impl From<&PlaceholderStatement> for PlaceholderStatement {
    fn from(value: &PlaceholderStatement) -> Self {
        value.clone()
    }
}
#[doc = "PlaceholderStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"PlaceholderStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PlaceholderStatementNodeType {
    PlaceholderStatement,
}
impl From<&PlaceholderStatementNodeType> for PlaceholderStatementNodeType {
    fn from(value: &PlaceholderStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for PlaceholderStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::PlaceholderStatement => "PlaceholderStatement".to_string(),
        }
    }
}
impl std::str::FromStr for PlaceholderStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "PlaceholderStatement" => Ok(Self::PlaceholderStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PlaceholderStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PlaceholderStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PlaceholderStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "PragmaDirective"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"literals\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"literals\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"PragmaDirective\""]
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
pub struct PragmaDirective {
    pub id: i64,
    pub literals: Vec<String>,
    #[serde(rename = "nodeType")]
    pub node_type: PragmaDirectiveNodeType,
    pub src: SourceLocation,
}
impl From<&PragmaDirective> for PragmaDirective {
    fn from(value: &PragmaDirective) -> Self {
        value.clone()
    }
}
#[doc = "PragmaDirectiveNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"PragmaDirective\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PragmaDirectiveNodeType {
    PragmaDirective,
}
impl From<&PragmaDirectiveNodeType> for PragmaDirectiveNodeType {
    fn from(value: &PragmaDirectiveNodeType) -> Self {
        value.clone()
    }
}
impl ToString for PragmaDirectiveNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::PragmaDirective => "PragmaDirective".to_string(),
        }
    }
}
impl std::str::FromStr for PragmaDirectiveNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "PragmaDirective" => Ok(Self::PragmaDirective),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PragmaDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PragmaDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PragmaDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Return"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"functionReturnParameters\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"expression\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"functionReturnParameters\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"Return\""]
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
pub struct Return {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<Expression>,
    #[serde(rename = "functionReturnParameters")]
    pub function_return_parameters: i64,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: ReturnNodeType,
    pub src: SourceLocation,
}
impl From<&Return> for Return {
    fn from(value: &Return) -> Self {
        value.clone()
    }
}
#[doc = "ReturnNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"Return\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReturnNodeType {
    Return,
}
impl From<&ReturnNodeType> for ReturnNodeType {
    fn from(value: &ReturnNodeType) -> Self {
        value.clone()
    }
}
impl ToString for ReturnNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::Return => "Return".to_string(),
        }
    }
}
impl std::str::FromStr for ReturnNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Return" => Ok(Self::Return),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ReturnNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReturnNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReturnNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "RevertStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"errorCall\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"errorCall\": {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"RevertStatement\""]
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
pub struct RevertStatement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "errorCall")]
    pub error_call: FunctionCall,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: RevertStatementNodeType,
    pub src: SourceLocation,
}
impl From<&RevertStatement> for RevertStatement {
    fn from(value: &RevertStatement) -> Self {
        value.clone()
    }
}
#[doc = "RevertStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"RevertStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RevertStatementNodeType {
    RevertStatement,
}
impl From<&RevertStatementNodeType> for RevertStatementNodeType {
    fn from(value: &RevertStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for RevertStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::RevertStatement => "RevertStatement".to_string(),
        }
    }
}
impl std::str::FromStr for RevertStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "RevertStatement" => Ok(Self::RevertStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RevertStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RevertStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RevertStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SourceLocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^\\\\d+:\\\\d+:\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourceLocation(String);
impl std::ops::Deref for SourceLocation {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SourceLocation> for String {
    fn from(value: SourceLocation) -> Self {
        value.0
    }
}
impl From<&SourceLocation> for SourceLocation {
    fn from(value: &SourceLocation) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SourceLocation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^\\d+:\\d+:\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^\\d+:\\d+:\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SourceLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SourceLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SourceLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "SourceUnit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SourceUnit\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"absolutePath\","]
#[doc = "    \"exportedSymbols\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"nodes\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"absolutePath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"experimentalSolidity\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"exportedSymbols\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"license\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"SourceUnit\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ContractDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ImportDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/PragmaDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/StructDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/UserDefinedValueTypeDefinition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/UsingForDirective\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
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
pub struct SourceUnit {
    #[serde(rename = "absolutePath")]
    pub absolute_path: String,
    #[serde(
        rename = "experimentalSolidity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_solidity: Option<bool>,
    #[serde(rename = "exportedSymbols")]
    pub exported_symbols: std::collections::HashMap<String, Vec<i64>>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: SourceUnitNodeType,
    pub nodes: Vec<SourceUnitNodesItem>,
    pub src: SourceLocation,
}
impl From<&SourceUnit> for SourceUnit {
    fn from(value: &SourceUnit) -> Self {
        value.clone()
    }
}
#[doc = "SourceUnitNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"SourceUnit\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SourceUnitNodeType {
    SourceUnit,
}
impl From<&SourceUnitNodeType> for SourceUnitNodeType {
    fn from(value: &SourceUnitNodeType) -> Self {
        value.clone()
    }
}
impl ToString for SourceUnitNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::SourceUnit => "SourceUnit".to_string(),
        }
    }
}
impl std::str::FromStr for SourceUnitNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "SourceUnit" => Ok(Self::SourceUnit),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SourceUnitNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SourceUnitNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SourceUnitNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SourceUnitNodesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ContractDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImportDirective\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PragmaDirective\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/StructDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedValueTypeDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UsingForDirective\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SourceUnitNodesItem {
    ContractDefinition(ContractDefinition),
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}
impl From<&SourceUnitNodesItem> for SourceUnitNodesItem {
    fn from(value: &SourceUnitNodesItem) -> Self {
        value.clone()
    }
}
impl From<ContractDefinition> for SourceUnitNodesItem {
    fn from(value: ContractDefinition) -> Self {
        Self::ContractDefinition(value)
    }
}
impl From<EnumDefinition> for SourceUnitNodesItem {
    fn from(value: EnumDefinition) -> Self {
        Self::EnumDefinition(value)
    }
}
impl From<ErrorDefinition> for SourceUnitNodesItem {
    fn from(value: ErrorDefinition) -> Self {
        Self::ErrorDefinition(value)
    }
}
impl From<FunctionDefinition> for SourceUnitNodesItem {
    fn from(value: FunctionDefinition) -> Self {
        Self::FunctionDefinition(value)
    }
}
impl From<ImportDirective> for SourceUnitNodesItem {
    fn from(value: ImportDirective) -> Self {
        Self::ImportDirective(value)
    }
}
impl From<PragmaDirective> for SourceUnitNodesItem {
    fn from(value: PragmaDirective) -> Self {
        Self::PragmaDirective(value)
    }
}
impl From<StructDefinition> for SourceUnitNodesItem {
    fn from(value: StructDefinition) -> Self {
        Self::StructDefinition(value)
    }
}
impl From<UserDefinedValueTypeDefinition> for SourceUnitNodesItem {
    fn from(value: UserDefinedValueTypeDefinition) -> Self {
        Self::UserDefinedValueTypeDefinition(value)
    }
}
impl From<UsingForDirective> for SourceUnitNodesItem {
    fn from(value: UsingForDirective) -> Self {
        Self::UsingForDirective(value)
    }
}
impl From<VariableDeclaration> for SourceUnitNodesItem {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}
#[doc = "StateMutability"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"payable\","]
#[doc = "    \"pure\","]
#[doc = "    \"nonpayable\","]
#[doc = "    \"view\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StateMutability {
    #[serde(rename = "payable")]
    Payable,
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "nonpayable")]
    Nonpayable,
    #[serde(rename = "view")]
    View,
}
impl From<&StateMutability> for StateMutability {
    fn from(value: &StateMutability) -> Self {
        value.clone()
    }
}
impl ToString for StateMutability {
    fn to_string(&self) -> String {
        match *self {
            Self::Payable => "payable".to_string(),
            Self::Pure => "pure".to_string(),
            Self::Nonpayable => "nonpayable".to_string(),
            Self::View => "view".to_string(),
        }
    }
}
impl std::str::FromStr for StateMutability {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "payable" => Ok(Self::Payable),
            "pure" => Ok(Self::Pure),
            "nonpayable" => Ok(Self::Nonpayable),
            "view" => Ok(Self::View),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for StateMutability {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StateMutability {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StateMutability {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/*
#[doc = "Statement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Break\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Continue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DoWhileStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EmitStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExpressionStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ForStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IfStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InlineAssembly\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PlaceholderStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Return\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/RevertStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TryStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UncheckedBlock\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclarationStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/WhileStatement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statement {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Block>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Break>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<Continue>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<Box<DoWhileStatement>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<EmitStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_5: Option<ExpressionStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_6: Option<ForStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_7: Option<IfStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_8: Option<InlineAssembly>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_9: Option<PlaceholderStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_10: Option<Return>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_11: Option<RevertStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_12: Option<TryStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_13: Option<UncheckedBlock>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_14: Option<VariableDeclarationStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_15: Option<WhileStatement>,
}
*/

#[doc = "Statement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Break\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Continue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DoWhileStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EmitStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExpressionStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ForStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IfStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InlineAssembly\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PlaceholderStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Return\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/RevertStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TryStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UncheckedBlock\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclarationStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/WhileStatement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Statement {
    #[serde(rename = "Block")]
    Block(Block),
    #[serde(rename = "Break")]
    Break(Break),
    #[serde(rename = "Continue")]
    Continue(Continue),
    #[serde(rename = "DoWhileStatement")]
    DoWhileStatement(Box<DoWhileStatement>),
    #[serde(rename = "EmitStatement")]
    EmitStatement(EmitStatement),
    #[serde(rename = "ExpressionStatement")]
    ExpressionStatement(ExpressionStatement),
    #[serde(rename = "ForStatement")]
    ForStatement(ForStatement),
    #[serde(rename = "IfStatement")]
    IfStatement(IfStatement),
    #[serde(rename = "InlineAssembly")]
    InlineAssembly(InlineAssembly),
    #[serde(rename = "PlaceholderStatement")]
    PlaceholderStatement(PlaceholderStatement),
    #[serde(rename = "Return")]
    Return(Return),
    #[serde(rename = "RevertStatement")]
    RevertStatement(RevertStatement),
    #[serde(rename = "TryStatement")]
    TryStatement(TryStatement),
    #[serde(rename = "UncheckedBlock")]
    UncheckedBlock(UncheckedBlock),
    #[serde(rename = "VariableDeclarationStatement")]
    VariableDeclarationStatement(VariableDeclarationStatement),
    #[serde(rename = "WhileStatement")]
    WhileStatement(WhileStatement),
}
impl From<&Statement> for Statement {
    fn from(value: &Statement) -> Self {
        value.clone()
    }
}
impl ToString for Statement {
    fn to_string(&self) -> String {
        match *self {
            Self::Block(ref value) => "Block".to_string(),
            Self::Break(ref value) => "Break".to_string(),
            Self::Continue(ref value) => "Continue".to_string(),
            Self::DoWhileStatement(ref value) => "DoWhileStatement".to_string(),
            Self::EmitStatement(ref value) => "EmitStatement".to_string(),
            Self::ExpressionStatement(ref value) => "ExpressionStatement".to_string(),
            Self::ForStatement(ref value) => "ForStatement".to_string(),
            Self::IfStatement(ref value) => "IfStatement".to_string(),
            Self::InlineAssembly(ref value) => "InlineAssembly".to_string(),
            Self::PlaceholderStatement(ref value) => "PlaceholderStatement".to_string(),
            Self::Return(ref value) => "Return".to_string(),
            Self::RevertStatement(ref value) => "RevertStatement".to_string(),
            Self::TryStatement(ref value) => "TryStatement".to_string(),
            Self::UncheckedBlock(ref value) => "UncheckedBlock".to_string(),
            Self::VariableDeclarationStatement(ref value) => "VariableDeclarationStatement".to_string(),
            Self::WhileStatement(ref value) => "WhileStatement".to_string(),
        }
    }
}
impl From<Block> for Statement {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}
impl From<Break> for Statement {
    fn from(value: Break) -> Self {
        Self::Break(value)
    }
}
impl From<Continue> for Statement {
    fn from(value: Continue) -> Self {
        Self::Continue(value)
    }
}
impl From<Box<DoWhileStatement>> for Statement {
    fn from(value: Box<DoWhileStatement>) -> Self {
        Self::DoWhileStatement(value)
    }
}
impl From<EmitStatement> for Statement {
    fn from(value: EmitStatement) -> Self {
        Self::EmitStatement(value)
    }
}
impl From<ExpressionStatement> for Statement {
    fn from(value: ExpressionStatement) -> Self {
        Self::ExpressionStatement(value)
    }
}
impl From<ForStatement> for Statement {
    fn from(value: ForStatement) -> Self {
        Self::ForStatement(value)
    }
}
impl From<IfStatement> for Statement {
    fn from(value: IfStatement) -> Self {
        Self::IfStatement(value)
    }
}
impl From<InlineAssembly> for Statement {
    fn from(value: InlineAssembly) -> Self {
        Self::InlineAssembly(value)
    }
}
impl From<PlaceholderStatement> for Statement {
    fn from(value: PlaceholderStatement) -> Self {
        Self::PlaceholderStatement(value)
    }
}
impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Self::Return(value)
    }
}
impl From<RevertStatement> for Statement {
    fn from(value: RevertStatement) -> Self {
        Self::RevertStatement(value)
    }
}
impl From<TryStatement> for Statement {
    fn from(value: TryStatement) -> Self {
        Self::TryStatement(value)
    }
}
impl From<UncheckedBlock> for Statement {
    fn from(value: UncheckedBlock) -> Self {
        Self::UncheckedBlock(value)
    }
}
impl From<VariableDeclarationStatement> for Statement {
    fn from(value: VariableDeclarationStatement) -> Self {
        Self::VariableDeclarationStatement(value)
    }
}
impl From<WhileStatement> for Statement {
    fn from(value: WhileStatement) -> Self {
        Self::WhileStatement(value)
    }
}

#[doc = "StorageLocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"calldata\","]
#[doc = "    \"default\","]
#[doc = "    \"memory\","]
#[doc = "    \"storage\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StorageLocation {
    #[serde(rename = "calldata")]
    Calldata,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "storage")]
    Storage,
}
impl From<&StorageLocation> for StorageLocation {
    fn from(value: &StorageLocation) -> Self {
        value.clone()
    }
}
impl ToString for StorageLocation {
    fn to_string(&self) -> String {
        match *self {
            Self::Calldata => "calldata".to_string(),
            Self::Default => "default".to_string(),
            Self::Memory => "memory".to_string(),
            Self::Storage => "storage".to_string(),
        }
    }
}
impl std::str::FromStr for StorageLocation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "calldata" => Ok(Self::Calldata),
            "default" => Ok(Self::Default),
            "memory" => Ok(Self::Memory),
            "storage" => Ok(Self::Storage),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for StorageLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StorageLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StorageLocation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "StructDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"canonicalName\","]
#[doc = "    \"id\","]
#[doc = "    \"members\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonicalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"members\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"StructDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"$ref\": \"#/definitions/Visibility\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructDefinition {
    #[serde(rename = "canonicalName")]
    pub canonical_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    pub id: i64,
    pub members: Vec<VariableDeclaration>,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: StructDefinitionNodeType,
    pub scope: i64,
    pub src: SourceLocation,
    pub visibility: Visibility,
}
impl From<&StructDefinition> for StructDefinition {
    fn from(value: &StructDefinition) -> Self {
        value.clone()
    }
}
#[doc = "StructDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"StructDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StructDefinitionNodeType {
    StructDefinition,
}
impl From<&StructDefinitionNodeType> for StructDefinitionNodeType {
    fn from(value: &StructDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for StructDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::StructDefinition => "StructDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for StructDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "StructDefinition" => Ok(Self::StructDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for StructDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StructDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StructDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "StructuredDocumentation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"StructuredDocumentation\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuredDocumentation {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: StructuredDocumentationNodeType,
    pub src: SourceLocation,
    pub text: String,
}
impl From<&StructuredDocumentation> for StructuredDocumentation {
    fn from(value: &StructuredDocumentation) -> Self {
        value.clone()
    }
}
#[doc = "StructuredDocumentationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"StructuredDocumentation\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StructuredDocumentationNodeType {
    StructuredDocumentation,
}
impl From<&StructuredDocumentationNodeType> for StructuredDocumentationNodeType {
    fn from(value: &StructuredDocumentationNodeType) -> Self {
        value.clone()
    }
}
impl ToString for StructuredDocumentationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::StructuredDocumentation => "StructuredDocumentation".to_string(),
        }
    }
}
impl std::str::FromStr for StructuredDocumentationNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "StructuredDocumentation" => Ok(Self::StructuredDocumentation),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for StructuredDocumentationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StructuredDocumentationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StructuredDocumentationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "TryCatchClause"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"block\","]
#[doc = "    \"errorName\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"block\": {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    \"errorName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"TryCatchClause\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ParameterList\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
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
pub struct TryCatchClause {
    pub block: Block,
    #[serde(rename = "errorName")]
    pub error_name: String,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: TryCatchClauseNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterList>,
    pub src: SourceLocation,
}
impl From<&TryCatchClause> for TryCatchClause {
    fn from(value: &TryCatchClause) -> Self {
        value.clone()
    }
}
#[doc = "TryCatchClauseNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"TryCatchClause\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TryCatchClauseNodeType {
    TryCatchClause,
}
impl From<&TryCatchClauseNodeType> for TryCatchClauseNodeType {
    fn from(value: &TryCatchClauseNodeType) -> Self {
        value.clone()
    }
}
impl ToString for TryCatchClauseNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::TryCatchClause => "TryCatchClause".to_string(),
        }
    }
}
impl std::str::FromStr for TryCatchClauseNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "TryCatchClause" => Ok(Self::TryCatchClause),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TryCatchClauseNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TryCatchClauseNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TryCatchClauseNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "TryStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"clauses\","]
#[doc = "    \"externalCall\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"clauses\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/TryCatchClause\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"externalCall\": {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"TryStatement\""]
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
pub struct TryStatement {
    pub clauses: Vec<TryCatchClause>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "externalCall")]
    pub external_call: FunctionCall,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: TryStatementNodeType,
    pub src: SourceLocation,
}
impl From<&TryStatement> for TryStatement {
    fn from(value: &TryStatement) -> Self {
        value.clone()
    }
}
#[doc = "TryStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"TryStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TryStatementNodeType {
    TryStatement,
}
impl From<&TryStatementNodeType> for TryStatementNodeType {
    fn from(value: &TryStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for TryStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::TryStatement => "TryStatement".to_string(),
        }
    }
}
impl std::str::FromStr for TryStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "TryStatement" => Ok(Self::TryStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TryStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TryStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TryStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "TupleExpression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"components\","]
#[doc = "    \"id\","]
#[doc = "    \"isConstant\","]
#[doc = "    \"isInlineArray\","]
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
#[doc = "    \"components\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/Expression\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"null\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isConstant\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isInlineArray\": {"]
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
#[doc = "        \"TupleExpression\""]
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
pub struct TupleExpression {
    #[serde(
        rename = "argumentTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub components: Vec<Option<Expression>>,
    pub id: i64,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isInlineArray")]
    pub is_inline_array: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "nodeType")]
    pub node_type: TupleExpressionNodeType,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
impl From<&TupleExpression> for TupleExpression {
    fn from(value: &TupleExpression) -> Self {
        value.clone()
    }
}
#[doc = "TupleExpressionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"TupleExpression\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TupleExpressionNodeType {
    TupleExpression,
}
impl From<&TupleExpressionNodeType> for TupleExpressionNodeType {
    fn from(value: &TupleExpressionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for TupleExpressionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::TupleExpression => "TupleExpression".to_string(),
        }
    }
}
impl std::str::FromStr for TupleExpressionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "TupleExpression" => Ok(Self::TupleExpression),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TupleExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TupleExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TupleExpressionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "TypeDescriptions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"typeIdentifier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"typeString\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
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
pub struct TypeDescriptions {
    #[serde(
        rename = "typeIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,
    #[serde(
        rename = "typeString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_string: Option<String>,
}
impl From<&TypeDescriptions> for TypeDescriptions {
    fn from(value: &TypeDescriptions) -> Self {
        value.clone()
    }
}
#[doc = "TypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ArrayTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElementaryTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Mapping\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TypeName {
    ArrayTypeName(Box<ArrayTypeName>),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}
impl From<&TypeName> for TypeName {
    fn from(value: &TypeName) -> Self {
        value.clone()
    }
}
impl From<Box<ArrayTypeName>> for TypeName {
    fn from(value: Box<ArrayTypeName>) -> Self {
        Self::ArrayTypeName(value)
    }
}
impl From<ElementaryTypeName> for TypeName {
    fn from(value: ElementaryTypeName) -> Self {
        Self::ElementaryTypeName(value)
    }
}
impl From<FunctionTypeName> for TypeName {
    fn from(value: FunctionTypeName) -> Self {
        Self::FunctionTypeName(value)
    }
}
impl From<Mapping> for TypeName {
    fn from(value: Mapping) -> Self {
        Self::Mapping(value)
    }
}
impl From<UserDefinedTypeName> for TypeName {
    fn from(value: UserDefinedTypeName) -> Self {
        Self::UserDefinedTypeName(value)
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "UnaryOperation" => Ok(Self::UnaryOperation),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UnaryOperationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UnaryOperationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UnaryOperationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
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
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UnaryOperationOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UnaryOperationOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "UncheckedBlock"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"statements\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"UncheckedBlock\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"statements\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Statement\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: UncheckedBlockNodeType,
    pub src: SourceLocation,
    pub statements: Vec<Statement>,
}
impl From<&UncheckedBlock> for UncheckedBlock {
    fn from(value: &UncheckedBlock) -> Self {
        value.clone()
    }
}
#[doc = "UncheckedBlockNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"UncheckedBlock\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UncheckedBlockNodeType {
    UncheckedBlock,
}
impl From<&UncheckedBlockNodeType> for UncheckedBlockNodeType {
    fn from(value: &UncheckedBlockNodeType) -> Self {
        value.clone()
    }
}
impl ToString for UncheckedBlockNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::UncheckedBlock => "UncheckedBlock".to_string(),
        }
    }
}
impl std::str::FromStr for UncheckedBlockNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "UncheckedBlock" => Ok(Self::UncheckedBlock),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UncheckedBlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UncheckedBlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UncheckedBlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "UserDefinedTypeName" => Ok(Self::UserDefinedTypeName),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UserDefinedTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UserDefinedTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UserDefinedTypeNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "UserDefinedValueTypeDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"underlyingType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonicalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"UserDefinedValueTypeDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"underlyingType\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserDefinedValueTypeDefinition {
    #[serde(
        rename = "canonicalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub canonical_name: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: UserDefinedValueTypeDefinitionNodeType,
    pub src: SourceLocation,
    #[serde(rename = "underlyingType")]
    pub underlying_type: TypeName,
}
impl From<&UserDefinedValueTypeDefinition> for UserDefinedValueTypeDefinition {
    fn from(value: &UserDefinedValueTypeDefinition) -> Self {
        value.clone()
    }
}
#[doc = "UserDefinedValueTypeDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserDefinedValueTypeDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UserDefinedValueTypeDefinitionNodeType {
    UserDefinedValueTypeDefinition,
}
impl From<&UserDefinedValueTypeDefinitionNodeType> for UserDefinedValueTypeDefinitionNodeType {
    fn from(value: &UserDefinedValueTypeDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for UserDefinedValueTypeDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::UserDefinedValueTypeDefinition => "UserDefinedValueTypeDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for UserDefinedValueTypeDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "UserDefinedValueTypeDefinition" => Ok(Self::UserDefinedValueTypeDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UserDefinedValueTypeDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UserDefinedValueTypeDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UserDefinedValueTypeDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
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
#[doc = "UsingForDirectiveFunctionListItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"function\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"function\": {"]
#[doc = "          \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"definition\","]
#[doc = "        \"operator\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"definition\": {"]
#[doc = "          \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "        },"]
#[doc = "        \"operator\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"&\","]
#[doc = "            \"|\","]
#[doc = "            \"^\","]
#[doc = "            \"~\","]
#[doc = "            \"+\","]
#[doc = "            \"-\","]
#[doc = "            \"*\","]
#[doc = "            \"/\","]
#[doc = "            \"%\","]
#[doc = "            \"==\","]
#[doc = "            \"!=\","]
#[doc = "            \"<\","]
#[doc = "            \"<=\","]
#[doc = "            \">\","]
#[doc = "            \">=\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum UsingForDirectiveFunctionListItem {
    Variant0 {
        function: IdentifierPath,
    },
    Variant1 {
        definition: IdentifierPath,
        operator: UsingForDirectiveFunctionListItemVariant1Operator,
    },
}
impl From<&UsingForDirectiveFunctionListItem> for UsingForDirectiveFunctionListItem {
    fn from(value: &UsingForDirectiveFunctionListItem) -> Self {
        value.clone()
    }
}
#[doc = "UsingForDirectiveFunctionListItemVariant1Operator"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"&\","]
#[doc = "    \"|\","]
#[doc = "    \"^\","]
#[doc = "    \"~\","]
#[doc = "    \"+\","]
#[doc = "    \"-\","]
#[doc = "    \"*\","]
#[doc = "    \"/\","]
#[doc = "    \"%\","]
#[doc = "    \"==\","]
#[doc = "    \"!=\","]
#[doc = "    \"<\","]
#[doc = "    \"<=\","]
#[doc = "    \">\","]
#[doc = "    \">=\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UsingForDirectiveFunctionListItemVariant1Operator {
    #[serde(rename = "&")]
    BitwiseAnd,
    #[serde(rename = "|")]
    BitwiseOr,
    #[serde(rename = "^")]
    BitwiseXor,
    #[serde(rename = "~")]
    BitwiseNot,
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Subtract,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "/")]
    Divide,
    #[serde(rename = "%")]
    Modulo,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "<=")]
    LessEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = ">=")]
    GreaterEqual,
}
impl From<&UsingForDirectiveFunctionListItemVariant1Operator>
    for UsingForDirectiveFunctionListItemVariant1Operator
{
    fn from(value: &UsingForDirectiveFunctionListItemVariant1Operator) -> Self {
        value.clone()
    }
}
impl ToString for UsingForDirectiveFunctionListItemVariant1Operator {
    fn to_string(&self) -> String {
        match *self {
            Self::BitwiseAnd => "&".to_string(),
            Self::BitwiseOr => "|".to_string(),
            Self::BitwiseXor => "^".to_string(),
            Self::BitwiseNot => "~".to_string(),
            Self::Add => "+".to_string(),
            Self::Subtract => "-".to_string(),
            Self::Multiply => "*".to_string(),
            Self::Divide => "/".to_string(),
            Self::Modulo => "%".to_string(),
            Self::Equal => "==".to_string(),
            Self::NotEqual => "!=".to_string(),
            Self::Less => "<".to_string(),
            Self::LessEqual => "<=".to_string(),
            Self::Greater => ">".to_string(),
            Self::GreaterEqual => ">=".to_string(),
        }
    }
}
impl std::str::FromStr for UsingForDirectiveFunctionListItemVariant1Operator {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "&" => Ok(Self::BitwiseAnd),
            "|" => Ok(Self::BitwiseOr),
            "^" => Ok(Self::BitwiseXor),
            "~" => Ok(Self::BitwiseNot),
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            "%" => Ok(Self::Modulo),
            "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            "<" => Ok(Self::Less),
            "<=" => Ok(Self::LessEqual),
            ">" => Ok(Self::Greater),
            ">=" => Ok(Self::GreaterEqual),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UsingForDirectiveFunctionListItemVariant1Operator {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UsingForDirectiveFunctionListItemVariant1Operator {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UsingForDirectiveFunctionListItemVariant1Operator {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "UsingForDirectiveLibraryName"]
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
pub enum UsingForDirectiveLibraryName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}
impl From<&UsingForDirectiveLibraryName> for UsingForDirectiveLibraryName {
    fn from(value: &UsingForDirectiveLibraryName) -> Self {
        value.clone()
    }
}
impl From<UserDefinedTypeName> for UsingForDirectiveLibraryName {
    fn from(value: UserDefinedTypeName) -> Self {
        Self::UserDefinedTypeName(value)
    }
}
impl From<IdentifierPath> for UsingForDirectiveLibraryName {
    fn from(value: IdentifierPath) -> Self {
        Self::IdentifierPath(value)
    }
}
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "UsingForDirective" => Ok(Self::UsingForDirective),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UsingForDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UsingForDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UsingForDirectiveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "VariableDeclaration"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"constant\","]
#[doc = "    \"id\","]
#[doc = "    \"mutability\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"scope\","]
#[doc = "    \"src\","]
#[doc = "    \"stateVariable\","]
#[doc = "    \"storageLocation\","]
#[doc = "    \"typeDescriptions\","]
#[doc = "    \"visibility\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"baseFunctions\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"constant\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/StructuredDocumentation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"functionSelector\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"indexed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"mutability\": {"]
#[doc = "      \"$ref\": \"#/definitions/Mutability\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"VariableDeclaration\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overrides\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/OverrideSpecifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"stateVariable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"storageLocation\": {"]
#[doc = "      \"$ref\": \"#/definitions/StorageLocation\""]
#[doc = "    },"]
#[doc = "    \"typeDescriptions\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeDescriptions\""]
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
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Expression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"$ref\": \"#/definitions/Visibility\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VariableDeclaration {
    #[serde(
        rename = "baseFunctions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_functions: Option<Vec<i64>>,
    pub constant: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<StructuredDocumentation>,
    #[serde(
        rename = "functionSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_selector: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    pub mutability: Mutability,
    pub name: String,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
    #[serde(rename = "nodeType")]
    pub node_type: VariableDeclarationNodeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(rename = "stateVariable")]
    pub state_variable: bool,
    #[serde(rename = "storageLocation")]
    pub storage_location: StorageLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<TypeName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Expression>,
    pub visibility: Visibility,
}
impl From<&VariableDeclaration> for VariableDeclaration {
    fn from(value: &VariableDeclaration) -> Self {
        value.clone()
    }
}
#[doc = "VariableDeclarationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"VariableDeclaration\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VariableDeclarationNodeType {
    VariableDeclaration,
}
impl From<&VariableDeclarationNodeType> for VariableDeclarationNodeType {
    fn from(value: &VariableDeclarationNodeType) -> Self {
        value.clone()
    }
}
impl ToString for VariableDeclarationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::VariableDeclaration => "VariableDeclaration".to_string(),
        }
    }
}
impl std::str::FromStr for VariableDeclarationNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "VariableDeclaration" => Ok(Self::VariableDeclaration),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for VariableDeclarationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VariableDeclarationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VariableDeclarationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "VariableDeclarationStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"assignments\","]
#[doc = "    \"declarations\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"assignments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"null\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"declarations\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"null\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"initialValue\": {"]
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
#[doc = "        \"VariableDeclarationStatement\""]
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
pub struct VariableDeclarationStatement {
    pub assignments: Vec<Option<i64>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(
        rename = "initialValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_value: Option<Expression>,
    #[serde(rename = "nodeType")]
    pub node_type: VariableDeclarationStatementNodeType,
    pub src: SourceLocation,
}
impl From<&VariableDeclarationStatement> for VariableDeclarationStatement {
    fn from(value: &VariableDeclarationStatement) -> Self {
        value.clone()
    }
}
#[doc = "VariableDeclarationStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"VariableDeclarationStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VariableDeclarationStatementNodeType {
    VariableDeclarationStatement,
}
impl From<&VariableDeclarationStatementNodeType> for VariableDeclarationStatementNodeType {
    fn from(value: &VariableDeclarationStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for VariableDeclarationStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::VariableDeclarationStatement => "VariableDeclarationStatement".to_string(),
        }
    }
}
impl std::str::FromStr for VariableDeclarationStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "VariableDeclarationStatement" => Ok(Self::VariableDeclarationStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for VariableDeclarationStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VariableDeclarationStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VariableDeclarationStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Visibility"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"external\","]
#[doc = "    \"public\","]
#[doc = "    \"internal\","]
#[doc = "    \"private\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Visibility {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
}
impl From<&Visibility> for Visibility {
    fn from(value: &Visibility) -> Self {
        value.clone()
    }
}
impl ToString for Visibility {
    fn to_string(&self) -> String {
        match *self {
            Self::External => "external".to_string(),
            Self::Public => "public".to_string(),
            Self::Internal => "internal".to_string(),
            Self::Private => "private".to_string(),
        }
    }
}
impl std::str::FromStr for Visibility {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "external" => Ok(Self::External),
            "public" => Ok(Self::Public),
            "internal" => Ok(Self::Internal),
            "private" => Ok(Self::Private),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Visibility {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Visibility {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Visibility {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "WhileStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"condition\","]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Block\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Statement\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    \"documentation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"WhileStatement\""]
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
pub struct WhileStatement {
    pub body: WhileStatementBody,
    pub condition: Expression,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: WhileStatementNodeType,
    pub src: SourceLocation,
}
impl From<&WhileStatement> for WhileStatement {
    fn from(value: &WhileStatement) -> Self {
        value.clone()
    }
}
#[doc = "WhileStatementBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Statement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WhileStatementBody {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Block>,
    #[serde(flatten, default)]
    pub subtype_1: Box<Option<Statement>>,
}
impl From<&WhileStatementBody> for WhileStatementBody {
    fn from(value: &WhileStatementBody) -> Self {
        value.clone()
    }
}
#[doc = "WhileStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"WhileStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WhileStatementNodeType {
    WhileStatement,
}
impl From<&WhileStatementNodeType> for WhileStatementNodeType {
    fn from(value: &WhileStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for WhileStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::WhileStatement => "WhileStatement".to_string(),
        }
    }
}
impl std::str::FromStr for WhileStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "WhileStatement" => Ok(Self::WhileStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WhileStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WhileStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WhileStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulAssignment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"value\","]
#[doc = "    \"variableNames\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulAssignment\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"variableNames\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulIdentifier\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulAssignment {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulAssignmentNodeType,
    pub src: SourceLocation,
    pub value: YulExpression,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<YulIdentifier>,
}
impl From<&YulAssignment> for YulAssignment {
    fn from(value: &YulAssignment) -> Self {
        value.clone()
    }
}
#[doc = "YulAssignmentNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulAssignment\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulAssignmentNodeType {
    YulAssignment,
}
impl From<&YulAssignmentNodeType> for YulAssignmentNodeType {
    fn from(value: &YulAssignmentNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulAssignmentNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulAssignment => "YulAssignment".to_string(),
        }
    }
}
impl std::str::FromStr for YulAssignmentNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulAssignment" => Ok(Self::YulAssignment),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulAssignmentNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulAssignmentNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulAssignmentNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulBlock"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"statements\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulBlock\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"statements\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulStatement\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulBlock {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulBlockNodeType,
    pub src: SourceLocation,
    pub statements: Vec<YulStatement>,
}
impl From<&YulBlock> for YulBlock {
    fn from(value: &YulBlock) -> Self {
        value.clone()
    }
}
#[doc = "YulBlockNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulBlock\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulBlockNodeType {
    YulBlock,
}
impl From<&YulBlockNodeType> for YulBlockNodeType {
    fn from(value: &YulBlockNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulBlockNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulBlock => "YulBlock".to_string(),
        }
    }
}
impl std::str::FromStr for YulBlockNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulBlock" => Ok(Self::YulBlock),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulBlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulBlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulBlockNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulBreak"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulBreak\""]
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
pub struct YulBreak {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulBreakNodeType,
    pub src: SourceLocation,
}
impl From<&YulBreak> for YulBreak {
    fn from(value: &YulBreak) -> Self {
        value.clone()
    }
}
#[doc = "YulBreakNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulBreak\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulBreakNodeType {
    YulBreak,
}
impl From<&YulBreakNodeType> for YulBreakNodeType {
    fn from(value: &YulBreakNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulBreakNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulBreak => "YulBreak".to_string(),
        }
    }
}
impl std::str::FromStr for YulBreakNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulBreak" => Ok(Self::YulBreak),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulBreakNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulBreakNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulBreakNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulCase"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulCase\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"default\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/YulLiteral\""]
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
pub struct YulCase {
    pub body: YulBlock,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulCaseNodeType,
    pub src: SourceLocation,
    pub value: YulCaseValue,
}
impl From<&YulCase> for YulCase {
    fn from(value: &YulCase) -> Self {
        value.clone()
    }
}
#[doc = "YulCaseNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulCase\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulCaseNodeType {
    YulCase,
}
impl From<&YulCaseNodeType> for YulCaseNodeType {
    fn from(value: &YulCaseNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulCaseNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulCase => "YulCase".to_string(),
        }
    }
}
impl std::str::FromStr for YulCaseNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulCase" => Ok(Self::YulCase),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulCaseNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulCaseNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulCaseNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulCaseValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteral\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YulCaseValue {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<YulCaseValueSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<YulLiteral>,
}
impl From<&YulCaseValue> for YulCaseValue {
    fn from(value: &YulCaseValue) -> Self {
        value.clone()
    }
}
#[doc = "YulCaseValueSubtype0"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulCaseValueSubtype0 {
    #[serde(rename = "default")]
    Default,
}
impl From<&YulCaseValueSubtype0> for YulCaseValueSubtype0 {
    fn from(value: &YulCaseValueSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for YulCaseValueSubtype0 {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for YulCaseValueSubtype0 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulCaseValueSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulCaseValueSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulCaseValueSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulContinue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulContinue\""]
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
pub struct YulContinue {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulContinueNodeType,
    pub src: SourceLocation,
}
impl From<&YulContinue> for YulContinue {
    fn from(value: &YulContinue) -> Self {
        value.clone()
    }
}
#[doc = "YulContinueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulContinue\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulContinueNodeType {
    YulContinue,
}
impl From<&YulContinueNodeType> for YulContinueNodeType {
    fn from(value: &YulContinueNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulContinueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulContinue => "YulContinue".to_string(),
        }
    }
}
impl std::str::FromStr for YulContinueNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulContinue" => Ok(Self::YulContinue),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulContinueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulContinueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulContinueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulExpression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulFunctionCall\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulIdentifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteral\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YulExpression {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<YulFunctionCall>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<YulIdentifier>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<YulLiteral>,
}
impl From<&YulExpression> for YulExpression {
    fn from(value: &YulExpression) -> Self {
        value.clone()
    }
}
#[doc = "YulExpressionStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expression\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expression\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulExpressionStatement\""]
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
pub struct YulExpressionStatement {
    pub expression: YulExpression,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulExpressionStatementNodeType,
    pub src: SourceLocation,
}
impl From<&YulExpressionStatement> for YulExpressionStatement {
    fn from(value: &YulExpressionStatement) -> Self {
        value.clone()
    }
}
#[doc = "YulExpressionStatementNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulExpressionStatement\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulExpressionStatementNodeType {
    YulExpressionStatement,
}
impl From<&YulExpressionStatementNodeType> for YulExpressionStatementNodeType {
    fn from(value: &YulExpressionStatementNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulExpressionStatementNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulExpressionStatement => "YulExpressionStatement".to_string(),
        }
    }
}
impl std::str::FromStr for YulExpressionStatementNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulExpressionStatement" => Ok(Self::YulExpressionStatement),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulExpressionStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulExpressionStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulExpressionStatementNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulForLoop"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"condition\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"post\","]
#[doc = "    \"pre\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulForLoop\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"post\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"pre\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
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
pub struct YulForLoop {
    pub body: YulBlock,
    pub condition: YulExpression,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulForLoopNodeType,
    pub post: YulBlock,
    pub pre: YulBlock,
    pub src: SourceLocation,
}
impl From<&YulForLoop> for YulForLoop {
    fn from(value: &YulForLoop) -> Self {
        value.clone()
    }
}
#[doc = "YulForLoopNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulForLoop\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulForLoopNodeType {
    YulForLoop,
}
impl From<&YulForLoopNodeType> for YulForLoopNodeType {
    fn from(value: &YulForLoopNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulForLoopNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulForLoop => "YulForLoop".to_string(),
        }
    }
}
impl std::str::FromStr for YulForLoopNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulForLoop" => Ok(Self::YulForLoop),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulForLoopNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulForLoopNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulForLoopNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulFunctionCall"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"arguments\","]
#[doc = "    \"functionName\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"functionName\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulIdentifier\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulFunctionCall\""]
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
pub struct YulFunctionCall {
    pub arguments: Vec<YulExpression>,
    #[serde(rename = "functionName")]
    pub function_name: YulIdentifier,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulFunctionCallNodeType,
    pub src: SourceLocation,
}
impl From<&YulFunctionCall> for YulFunctionCall {
    fn from(value: &YulFunctionCall) -> Self {
        value.clone()
    }
}
#[doc = "YulFunctionCallNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulFunctionCall\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulFunctionCallNodeType {
    YulFunctionCall,
}
impl From<&YulFunctionCallNodeType> for YulFunctionCallNodeType {
    fn from(value: &YulFunctionCallNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulFunctionCallNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulFunctionCall => "YulFunctionCall".to_string(),
        }
    }
}
impl std::str::FromStr for YulFunctionCallNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulFunctionCall" => Ok(Self::YulFunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulFunctionCallNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulFunctionCallNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulFunctionCallNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulFunctionDefinition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"parameters\","]
#[doc = "    \"returnVariables\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulFunctionDefinition\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulTypedName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"returnVariables\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulTypedName\""]
#[doc = "      }"]
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
pub struct YulFunctionDefinition {
    pub body: YulBlock,
    pub name: String,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulFunctionDefinitionNodeType,
    pub parameters: Vec<YulTypedName>,
    #[serde(rename = "returnVariables")]
    pub return_variables: Vec<YulTypedName>,
    pub src: SourceLocation,
}
impl From<&YulFunctionDefinition> for YulFunctionDefinition {
    fn from(value: &YulFunctionDefinition) -> Self {
        value.clone()
    }
}
#[doc = "YulFunctionDefinitionNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulFunctionDefinition\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulFunctionDefinitionNodeType {
    YulFunctionDefinition,
}
impl From<&YulFunctionDefinitionNodeType> for YulFunctionDefinitionNodeType {
    fn from(value: &YulFunctionDefinitionNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulFunctionDefinitionNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulFunctionDefinition => "YulFunctionDefinition".to_string(),
        }
    }
}
impl std::str::FromStr for YulFunctionDefinitionNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulFunctionDefinition" => Ok(Self::YulFunctionDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulFunctionDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulFunctionDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulFunctionDefinitionNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulIdentifier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulIdentifier\""]
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
pub struct YulIdentifier {
    pub name: String,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulIdentifierNodeType,
    pub src: SourceLocation,
}
impl From<&YulIdentifier> for YulIdentifier {
    fn from(value: &YulIdentifier) -> Self {
        value.clone()
    }
}
#[doc = "YulIdentifierNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulIdentifier\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulIdentifierNodeType {
    YulIdentifier,
}
impl From<&YulIdentifierNodeType> for YulIdentifierNodeType {
    fn from(value: &YulIdentifierNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulIdentifierNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulIdentifier => "YulIdentifier".to_string(),
        }
    }
}
impl std::str::FromStr for YulIdentifierNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulIdentifier" => Ok(Self::YulIdentifier),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulIdentifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulIdentifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulIdentifierNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulIf"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"body\","]
#[doc = "    \"condition\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulIf\""]
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
pub struct YulIf {
    pub body: YulBlock,
    pub condition: YulExpression,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulIfNodeType,
    pub src: SourceLocation,
}
impl From<&YulIf> for YulIf {
    fn from(value: &YulIf) -> Self {
        value.clone()
    }
}
#[doc = "YulIfNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulIf\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulIfNodeType {
    YulIf,
}
impl From<&YulIfNodeType> for YulIfNodeType {
    fn from(value: &YulIfNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulIfNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulIf => "YulIf".to_string(),
        }
    }
}
impl std::str::FromStr for YulIfNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulIf" => Ok(Self::YulIf),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulIfNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulIfNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulIfNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulLeave"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulLeave\""]
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
pub struct YulLeave {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulLeaveNodeType,
    pub src: SourceLocation,
}
impl From<&YulLeave> for YulLeave {
    fn from(value: &YulLeave) -> Self {
        value.clone()
    }
}
#[doc = "YulLeaveNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulLeave\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLeaveNodeType {
    YulLeave,
}
impl From<&YulLeaveNodeType> for YulLeaveNodeType {
    fn from(value: &YulLeaveNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulLeaveNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulLeave => "YulLeave".to_string(),
        }
    }
}
impl std::str::FromStr for YulLeaveNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulLeave" => Ok(Self::YulLeave),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulLeaveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulLeaveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulLeaveNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulLiteral"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteralValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteralHexValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum YulLiteral {
    Value(YulLiteralValue),
    HexValue(YulLiteralHexValue),
}
impl From<&YulLiteral> for YulLiteral {
    fn from(value: &YulLiteral) -> Self {
        value.clone()
    }
}
impl From<YulLiteralValue> for YulLiteral {
    fn from(value: YulLiteralValue) -> Self {
        Self::Value(value)
    }
}
impl From<YulLiteralHexValue> for YulLiteral {
    fn from(value: YulLiteralHexValue) -> Self {
        Self::HexValue(value)
    }
}
#[doc = "YulLiteralHexValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"hexValue\","]
#[doc = "    \"kind\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"hexValue\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"number\","]
#[doc = "        \"string\","]
#[doc = "        \"bool\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulLiteral\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulLiteralHexValue {
    #[serde(rename = "hexValue")]
    pub hex_value: String,
    pub kind: YulLiteralHexValueKind,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulLiteralHexValueNodeType,
    pub src: SourceLocation,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&YulLiteralHexValue> for YulLiteralHexValue {
    fn from(value: &YulLiteralHexValue) -> Self {
        value.clone()
    }
}
#[doc = "YulLiteralHexValueKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"number\","]
#[doc = "    \"string\","]
#[doc = "    \"bool\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLiteralHexValueKind {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bool")]
    Bool,
}
impl From<&YulLiteralHexValueKind> for YulLiteralHexValueKind {
    fn from(value: &YulLiteralHexValueKind) -> Self {
        value.clone()
    }
}
impl ToString for YulLiteralHexValueKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Number => "number".to_string(),
            Self::String => "string".to_string(),
            Self::Bool => "bool".to_string(),
        }
    }
}
impl std::str::FromStr for YulLiteralHexValueKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "bool" => Ok(Self::Bool),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulLiteralHexValueKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulLiteralHexValueKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulLiteralHexValueKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulLiteralHexValueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulLiteral\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLiteralHexValueNodeType {
    YulLiteral,
}
impl From<&YulLiteralHexValueNodeType> for YulLiteralHexValueNodeType {
    fn from(value: &YulLiteralHexValueNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulLiteralHexValueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulLiteral => "YulLiteral".to_string(),
        }
    }
}
impl std::str::FromStr for YulLiteralHexValueNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulLiteral" => Ok(Self::YulLiteral),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulLiteralHexValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulLiteralHexValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulLiteralHexValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulLiteralValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"number\","]
#[doc = "        \"string\","]
#[doc = "        \"bool\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulLiteral\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulLiteralValue {
    pub kind: YulLiteralValueKind,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulLiteralValueNodeType,
    pub src: SourceLocation,
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
impl From<&YulLiteralValue> for YulLiteralValue {
    fn from(value: &YulLiteralValue) -> Self {
        value.clone()
    }
}
#[doc = "YulLiteralValueKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"number\","]
#[doc = "    \"string\","]
#[doc = "    \"bool\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLiteralValueKind {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bool")]
    Bool,
}
impl From<&YulLiteralValueKind> for YulLiteralValueKind {
    fn from(value: &YulLiteralValueKind) -> Self {
        value.clone()
    }
}
impl ToString for YulLiteralValueKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Number => "number".to_string(),
            Self::String => "string".to_string(),
            Self::Bool => "bool".to_string(),
        }
    }
}
impl std::str::FromStr for YulLiteralValueKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "bool" => Ok(Self::Bool),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulLiteralValueKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulLiteralValueKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulLiteralValueKind {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulLiteralValueNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulLiteral\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulLiteralValueNodeType {
    YulLiteral,
}
impl From<&YulLiteralValueNodeType> for YulLiteralValueNodeType {
    fn from(value: &YulLiteralValueNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulLiteralValueNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulLiteral => "YulLiteral".to_string(),
        }
    }
}
impl std::str::FromStr for YulLiteralValueNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulLiteral" => Ok(Self::YulLiteral),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulLiteralValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulLiteralValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulLiteralValueNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulAssignment\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulBreak\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulContinue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpressionStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLeave\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulForLoop\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulFunctionDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulIf\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulSwitch\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulVariableDeclaration\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YulStatement {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<YulAssignment>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<YulBlock>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<YulBreak>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<YulContinue>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<YulExpressionStatement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_5: Option<YulLeave>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_6: Option<YulForLoop>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_7: Option<YulFunctionDefinition>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_8: Option<YulIf>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_9: Option<YulSwitch>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_10: Option<YulVariableDeclaration>,
}
impl From<&YulStatement> for YulStatement {
    fn from(value: &YulStatement) -> Self {
        value.clone()
    }
}
#[doc = "YulSwitch"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cases\","]
#[doc = "    \"expression\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cases\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulCase\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"expression\": {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulSwitch\""]
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
pub struct YulSwitch {
    pub cases: Vec<YulCase>,
    pub expression: YulExpression,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulSwitchNodeType,
    pub src: SourceLocation,
}
impl From<&YulSwitch> for YulSwitch {
    fn from(value: &YulSwitch) -> Self {
        value.clone()
    }
}
#[doc = "YulSwitchNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulSwitch\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulSwitchNodeType {
    YulSwitch,
}
impl From<&YulSwitchNodeType> for YulSwitchNodeType {
    fn from(value: &YulSwitchNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulSwitchNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulSwitch => "YulSwitch".to_string(),
        }
    }
}
impl std::str::FromStr for YulSwitchNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulSwitch" => Ok(Self::YulSwitch),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulSwitchNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulSwitchNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulSwitchNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulTypedName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulTypedName\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulTypedName {
    pub name: String,
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulTypedNameNodeType,
    pub src: SourceLocation,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&YulTypedName> for YulTypedName {
    fn from(value: &YulTypedName) -> Self {
        value.clone()
    }
}
#[doc = "YulTypedNameNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulTypedName\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulTypedNameNodeType {
    YulTypedName,
}
impl From<&YulTypedNameNodeType> for YulTypedNameNodeType {
    fn from(value: &YulTypedNameNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulTypedNameNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulTypedName => "YulTypedName".to_string(),
        }
    }
}
impl std::str::FromStr for YulTypedNameNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulTypedName" => Ok(Self::YulTypedName),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulTypedNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulTypedNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulTypedNameNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "YulVariableDeclaration"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"variables\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nativeSrc\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"YulVariableDeclaration\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/YulExpression\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"variables\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/YulTypedName\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YulVariableDeclaration {
    #[serde(rename = "nativeSrc", default, skip_serializing_if = "Option::is_none")]
    pub native_src: Option<SourceLocation>,
    #[serde(rename = "nodeType")]
    pub node_type: YulVariableDeclarationNodeType,
    pub src: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<YulExpression>,
    pub variables: Vec<YulTypedName>,
}
impl From<&YulVariableDeclaration> for YulVariableDeclaration {
    fn from(value: &YulVariableDeclaration) -> Self {
        value.clone()
    }
}
#[doc = "YulVariableDeclarationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"YulVariableDeclaration\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YulVariableDeclarationNodeType {
    YulVariableDeclaration,
}
impl From<&YulVariableDeclarationNodeType> for YulVariableDeclarationNodeType {
    fn from(value: &YulVariableDeclarationNodeType) -> Self {
        value.clone()
    }
}
impl ToString for YulVariableDeclarationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::YulVariableDeclaration => "YulVariableDeclaration".to_string(),
        }
    }
}
impl std::str::FromStr for YulVariableDeclarationNodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "YulVariableDeclaration" => Ok(Self::YulVariableDeclaration),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for YulVariableDeclarationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YulVariableDeclarationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YulVariableDeclarationNodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
