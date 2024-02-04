use serde::{Deserialize, Serialize};

use crate::SourceLocation;


#[doc = "InlineAssemblyExternalReferencesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"declaration\","]
#[doc = "    \"isOffset\","]
#[doc = "    \"isSlot\","]
#[doc = "    \"src\","]
#[doc = "    \"valueSize\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"declaration\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isOffset\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSlot\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"src\": {"]
#[doc = "      \"$ref\": \"#/definitions/SourceLocation\""]
#[doc = "    },"]
#[doc = "    \"suffix\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"slot\","]
#[doc = "        \"offset\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueSize\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InlineAssemblyExternalReferencesItem {
    pub declaration: i64,
    #[serde(rename = "isOffset")]
    pub is_offset: bool,
    #[serde(rename = "isSlot")]
    pub is_slot: bool,
    pub src: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<InlineAssemblyExternalReferencesItemSuffix>,
    #[serde(rename = "valueSize")]
    pub value_size: i64,
}

impl From<&InlineAssemblyExternalReferencesItem> for InlineAssemblyExternalReferencesItem {
    fn from(value: &InlineAssemblyExternalReferencesItem) -> Self {
        value.clone()
    }
}


#[doc = "InlineAssemblyExternalReferencesItemSuffix"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"slot\","]
#[doc = "    \"offset\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyExternalReferencesItemSuffix {
    #[serde(rename = "slot")]
    Slot,
    #[serde(rename = "offset")]
    Offset,
}

impl From<&InlineAssemblyExternalReferencesItemSuffix>
    for InlineAssemblyExternalReferencesItemSuffix
{
    fn from(value: &InlineAssemblyExternalReferencesItemSuffix) -> Self {
        value.clone()
    }
}

impl ToString for InlineAssemblyExternalReferencesItemSuffix {
    fn to_string(&self) -> String {
        match *self {
            Self::Slot => "slot".to_string(),
            Self::Offset => "offset".to_string(),
        }
    }
}

impl std::str::FromStr for InlineAssemblyExternalReferencesItemSuffix {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "slot" => Ok(Self::Slot),
            "offset" => Ok(Self::Offset),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for InlineAssemblyExternalReferencesItemSuffix {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for InlineAssemblyExternalReferencesItemSuffix {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for InlineAssemblyExternalReferencesItemSuffix {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}