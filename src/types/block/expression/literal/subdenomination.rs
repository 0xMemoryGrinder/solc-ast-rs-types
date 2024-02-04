use serde::{Deserialize, Serialize};


#[doc = "LiteralSubdenomination"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"seconds\","]
#[doc = "    \"minutes\","]
#[doc = "    \"hours\","]
#[doc = "    \"days\","]
#[doc = "    \"weeks\","]
#[doc = "    \"wei\","]
#[doc = "    \"gwei\","]
#[doc = "    \"ether\","]
#[doc = "    \"finney\","]
#[doc = "    \"szabo\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiteralSubdenomination {
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "wei")]
    Wei,
    #[serde(rename = "gwei")]
    Gwei,
    #[serde(rename = "ether")]
    Ether,
    #[serde(rename = "finney")]
    Finney,
    #[serde(rename = "szabo")]
    Szabo,
}

impl From<&LiteralSubdenomination> for LiteralSubdenomination {
    fn from(value: &LiteralSubdenomination) -> Self {
        value.clone()
    }
}

impl ToString for LiteralSubdenomination {
    fn to_string(&self) -> String {
        match *self {
            Self::Seconds => "seconds".to_string(),
            Self::Minutes => "minutes".to_string(),
            Self::Hours => "hours".to_string(),
            Self::Days => "days".to_string(),
            Self::Weeks => "weeks".to_string(),
            Self::Wei => "wei".to_string(),
            Self::Gwei => "gwei".to_string(),
            Self::Ether => "ether".to_string(),
            Self::Finney => "finney".to_string(),
            Self::Szabo => "szabo".to_string(),
        }
    }
}

impl std::str::FromStr for LiteralSubdenomination {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "seconds" => Ok(Self::Seconds),
            "minutes" => Ok(Self::Minutes),
            "hours" => Ok(Self::Hours),
            "days" => Ok(Self::Days),
            "weeks" => Ok(Self::Weeks),
            "wei" => Ok(Self::Wei),
            "gwei" => Ok(Self::Gwei),
            "ether" => Ok(Self::Ether),
            "finney" => Ok(Self::Finney),
            "szabo" => Ok(Self::Szabo),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for LiteralSubdenomination {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for LiteralSubdenomination {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for LiteralSubdenomination {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}