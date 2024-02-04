use serde::{Deserialize, Serialize};


#[doc = "Mutability"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"mutable\","]
#[doc = "    \"immutable\","]
#[doc = "    \"constant\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Mutability {
    #[serde(rename = "mutable")]
    Mutable,
    #[serde(rename = "immutable")]
    Immutable,
    #[serde(rename = "constant")]
    Constant,
}

impl From<&Mutability> for Mutability {
    fn from(value: &Mutability) -> Self {
        value.clone()
    }
}

impl ToString for Mutability {
    fn to_string(&self) -> String {
        match *self {
            Self::Mutable => "mutable".to_string(),
            Self::Immutable => "immutable".to_string(),
            Self::Constant => "constant".to_string(),
        }
    }
}

impl std::str::FromStr for Mutability {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "mutable" => Ok(Self::Mutable),
            "immutable" => Ok(Self::Immutable),
            "constant" => Ok(Self::Constant),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for Mutability {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for Mutability {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for Mutability {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}