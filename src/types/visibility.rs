use serde::{Deserialize, Serialize};

#[doc = "Visibility"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"external\","]
#[doc = "    \"public\","]
#[doc = "    \"internal\","]
#[doc = "    \"private\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Visibility {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
}

impl From<&Visibility> for Visibility {
    fn from(value: &Visibility) -> Self {
        value.clone()
    }
}

impl ToString for Visibility {
    fn to_string(&self) -> String {
        match *self {
            Self::External => "external".to_string(),
            Self::Public => "public".to_string(),
            Self::Internal => "internal".to_string(),
            Self::Private => "private".to_string(),
        }
    }
}

impl std::str::FromStr for Visibility {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "external" => Ok(Self::External),
            "public" => Ok(Self::Public),
            "internal" => Ok(Self::Internal),
            "private" => Ok(Self::Private),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for Visibility {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for Visibility {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for Visibility {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
