use serde::{Deserialize, Serialize};

use crate::types::{Expression, SourceLocation, TypeDescriptions};

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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
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
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AssignmentOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for AssignmentOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "Assignment" => Ok(Self::Assignment),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for AssignmentNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AssignmentNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for AssignmentNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
