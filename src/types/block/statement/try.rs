use serde::{Deserialize, Serialize};

use crate::{Block, FunctionCall, ParameterList, SourceLocation};


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



// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "TryStatement" => Ok(Self::TryStatement),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for TryStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for TryStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for TryStatementNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "TryCatchClause" => Ok(Self::TryCatchClause),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for TryCatchClauseNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for TryCatchClauseNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for TryCatchClauseNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}