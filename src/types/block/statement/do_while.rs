use serde::{Deserialize, Serialize};

use crate::types::{Block, Expression, SourceLocation, Statement};

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
#[serde(untagged)]
pub enum DoWhileStatementBody {
    Block(Block),
    Statement(Statement),
}

impl From<&DoWhileStatementBody> for DoWhileStatementBody {
    fn from(value: &DoWhileStatementBody) -> Self {
        value.clone()
    }
}

impl From<Block> for DoWhileStatementBody {
    fn from(value: Block) -> Self {
        DoWhileStatementBody::Block(value)
    }
}

impl From<Statement> for DoWhileStatementBody {
    fn from(value: Statement) -> Self {
        DoWhileStatementBody::Statement(value)
    }
}

// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "DoWhileStatement" => Ok(Self::DoWhileStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for DoWhileStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for DoWhileStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for DoWhileStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
