use serde::{Deserialize, Serialize};

use crate::{SourceLocation, YulExpression, YulTypedName};


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



// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulVariableDeclaration" => Ok(Self::YulVariableDeclaration),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulVariableDeclarationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulVariableDeclarationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulVariableDeclarationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
