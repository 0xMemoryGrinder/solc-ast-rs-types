use serde::{Deserialize, Serialize};

use crate::types::{SourceLocation, YulBlock, YulLiteral};

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
#[serde(untagged)]
pub enum YulCaseValue {
    Default,
    YulLiteral(YulLiteral),
}

impl From<&YulCaseValue> for YulCaseValue {
    fn from(value: &YulCaseValue) -> Self {
        value.clone()
    }
}

impl From<YulLiteral> for YulCaseValue {
    fn from(value: YulLiteral) -> Self {
        YulCaseValue::YulLiteral(value)
    }
}

// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "YulCase" => Ok(Self::YulCase),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for YulCaseNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for YulCaseNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for YulCaseNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
