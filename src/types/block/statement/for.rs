use serde::{Deserialize, Serialize};

use crate::types::{Block, Expression, ExpressionStatement, SourceLocation, VariableDeclarationStatement};


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
#[serde(untagged)]
pub enum ForStatementBody {
    Block(Block),
    Statement(ExpressionStatement),
}

impl From<&ForStatementBody> for ForStatementBody {
    fn from(value: &ForStatementBody) -> Self {
        value.clone()
    }
}

impl From<Block> for ForStatementBody {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}

impl From<ExpressionStatement> for ForStatementBody {
    fn from(value: ExpressionStatement) -> Self {
        Self::Statement(value)
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



// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "ForStatement" => Ok(Self::ForStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ForStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ForStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ForStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}