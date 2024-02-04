use serde::{Deserialize, Serialize};

#[doc = "StateMutability"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"payable\","]
#[doc = "    \"pure\","]
#[doc = "    \"nonpayable\","]
#[doc = "    \"view\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StateMutability {
    #[serde(rename = "payable")]
    Payable,
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "nonpayable")]
    Nonpayable,
    #[serde(rename = "view")]
    View,
}

impl From<&StateMutability> for StateMutability {
    fn from(value: &StateMutability) -> Self {
        value.clone()
    }
}

impl ToString for StateMutability {
    fn to_string(&self) -> String {
        match *self {
            Self::Payable => "payable".to_string(),
            Self::Pure => "pure".to_string(),
            Self::Nonpayable => "nonpayable".to_string(),
            Self::View => "view".to_string(),
        }
    }
}

impl std::str::FromStr for StateMutability {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "payable" => Ok(Self::Payable),
            "pure" => Ok(Self::Pure),
            "nonpayable" => Ok(Self::Nonpayable),
            "view" => Ok(Self::View),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for StateMutability {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for StateMutability {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for StateMutability {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}