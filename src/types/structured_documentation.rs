use serde::{Deserialize, Serialize};

use crate::SourceLocation;


#[doc = "StructuredDocumentation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"nodeType\","]
#[doc = "    \"src\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"nodeType\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"StructuredDocumentation\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuredDocumentation {
    pub id: i64,
    #[serde(rename = "nodeType")]
    pub node_type: StructuredDocumentationNodeType,
    pub src: SourceLocation,
    pub text: String,
}

impl From<&StructuredDocumentation> for StructuredDocumentation {
    fn from(value: &StructuredDocumentation) -> Self {
        value.clone()
    }
}


// Node type
#[doc = "StructuredDocumentationNodeType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"StructuredDocumentation\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StructuredDocumentationNodeType {
    StructuredDocumentation,
}

impl From<&StructuredDocumentationNodeType> for StructuredDocumentationNodeType {
    fn from(value: &StructuredDocumentationNodeType) -> Self {
        value.clone()
    }
}

impl ToString for StructuredDocumentationNodeType {
    fn to_string(&self) -> String {
        match *self {
            Self::StructuredDocumentation => "StructuredDocumentation".to_string(),
        }
    }
}

impl std::str::FromStr for StructuredDocumentationNodeType {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "StructuredDocumentation" => Ok(Self::StructuredDocumentation),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for StructuredDocumentationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for StructuredDocumentationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for StructuredDocumentationNodeType {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}