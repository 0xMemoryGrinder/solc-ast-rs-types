use serde::{Deserialize, Serialize};


#[doc = "StorageLocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"calldata\","]
#[doc = "    \"default\","]
#[doc = "    \"memory\","]
#[doc = "    \"storage\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StorageLocation {
    #[serde(rename = "calldata")]
    Calldata,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "storage")]
    Storage,
}

impl From<&StorageLocation> for StorageLocation {
    fn from(value: &StorageLocation) -> Self {
        value.clone()
    }
}

impl ToString for StorageLocation {
    fn to_string(&self) -> String {
        match *self {
            Self::Calldata => "calldata".to_string(),
            Self::Default => "default".to_string(),
            Self::Memory => "memory".to_string(),
            Self::Storage => "storage".to_string(),
        }
    }
}

impl std::str::FromStr for StorageLocation {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "calldata" => Ok(Self::Calldata),
            "default" => Ok(Self::Default),
            "memory" => Ok(Self::Memory),
            "storage" => Ok(Self::Storage),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for StorageLocation {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for StorageLocation {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for StorageLocation {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}