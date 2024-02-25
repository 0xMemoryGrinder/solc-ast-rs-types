use serde::{Deserialize, Serialize};

use crate::types::{Block, Expression, SourceLocation, Statement};

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
#[serde(untagged)]
pub enum IfStatementTrueBody {
    Block(Block),
    Statement(Box<Statement>),
}

impl From<&IfStatementTrueBody> for IfStatementTrueBody {
    fn from(value: &IfStatementTrueBody) -> Self {
        value.clone()
    }
}

impl From<Block> for IfStatementTrueBody {
    fn from(value: Block) -> Self {
        IfStatementTrueBody::Block(value)
    }
}

impl From<Statement> for IfStatementTrueBody {
    fn from(value: Statement) -> Self {
        IfStatementTrueBody::Statement(Box::new(value))
    }
}

impl From<Box<Statement>> for IfStatementTrueBody {
    fn from(value: Box<Statement>) -> Self {
        IfStatementTrueBody::Statement(value)
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
#[serde(untagged)]
pub enum IfStatementFalseBody {
    Block(Block),
    Statement(Box<Statement>),
}

impl From<&IfStatementFalseBody> for IfStatementFalseBody {
    fn from(value: &IfStatementFalseBody) -> Self {
        value.clone()
    }
}

impl From<Block> for IfStatementFalseBody {
    fn from(value: Block) -> Self {
        IfStatementFalseBody::Block(value)
    }
}

impl From<Statement> for IfStatementFalseBody {
    fn from(value: Statement) -> Self {
        IfStatementFalseBody::Statement(Box::new(value))
    }
}

impl From<Box<Statement>> for IfStatementFalseBody {
    fn from(value: Box<Statement>) -> Self {
        IfStatementFalseBody::Statement(value)
    }
}

// Node Type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "IfStatement" => Ok(Self::IfStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for IfStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for IfStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for IfStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
