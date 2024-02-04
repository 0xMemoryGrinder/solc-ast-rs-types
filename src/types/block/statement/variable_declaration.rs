use serde::{Deserialize, Serialize};

use crate::{Expression, SourceLocation, VariableDeclaration};


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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "VariableDeclarationStatement" => Ok(Self::VariableDeclarationStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for VariableDeclarationStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for VariableDeclarationStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for VariableDeclarationStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}