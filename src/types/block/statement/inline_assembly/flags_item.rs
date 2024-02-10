use serde::{Deserialize, Serialize};

#[doc = "InlineAssemblyFlagsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"memory-safe\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyFlagsItem {
    #[serde(rename = "memory-safe")]
    MemorySafe,
}

impl From<&InlineAssemblyFlagsItem> for InlineAssemblyFlagsItem {
    fn from(value: &InlineAssemblyFlagsItem) -> Self {
        value.clone()
    }
}

impl ToString for InlineAssemblyFlagsItem {
    fn to_string(&self) -> String {
        match *self {
            Self::MemorySafe => "memory-safe".to_string(),
        }
    }
}

impl std::str::FromStr for InlineAssemblyFlagsItem {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "memory-safe" => Ok(Self::MemorySafe),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for InlineAssemblyFlagsItem {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for InlineAssemblyFlagsItem {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for InlineAssemblyFlagsItem {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
