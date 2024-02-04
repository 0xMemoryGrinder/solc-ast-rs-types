use serde::{Deserialize, Serialize};

use crate::SourceLocation;


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



// Node type
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
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "PragmaDirective" => Ok(Self::PragmaDirective),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for PragmaDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for PragmaDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for PragmaDirectiveNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}