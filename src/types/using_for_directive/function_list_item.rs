use serde::{Deserialize, Serialize};

use crate::types::IdentifierPath;

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
    Function {
        function: IdentifierPath,
    },
    FunctionList {
        definition: IdentifierPath,
        operator: UsingForDirectiveFunctionListItemOperator,
    },
}
impl From<&UsingForDirectiveFunctionListItem> for UsingForDirectiveFunctionListItem {
    fn from(value: &UsingForDirectiveFunctionListItem) -> Self {
        value.clone()
    }
}

#[doc = "UsingForDirectiveFunctionListItemOperator"]
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
pub enum UsingForDirectiveFunctionListItemOperator {
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
impl From<&UsingForDirectiveFunctionListItemOperator>
    for UsingForDirectiveFunctionListItemOperator
{
    fn from(value: &UsingForDirectiveFunctionListItemOperator) -> Self {
        value.clone()
    }
}
impl ToString for UsingForDirectiveFunctionListItemOperator {
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
impl std::str::FromStr for UsingForDirectiveFunctionListItemOperator {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
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
impl std::convert::TryFrom<&str> for UsingForDirectiveFunctionListItemOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UsingForDirectiveFunctionListItemOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UsingForDirectiveFunctionListItemOperator {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}