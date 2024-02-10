use serde::{Deserialize, Serialize};

#[doc = "FunctionDefinitionKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"function\","]
#[doc = "    \"receive\","]
#[doc = "    \"constructor\","]
#[doc = "    \"fallback\","]
#[doc = "    \"freeFunction\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FunctionDefinitionKind {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "receive")]
    Receive,
    #[serde(rename = "constructor")]
    Constructor,
    #[serde(rename = "fallback")]
    Fallback,
    #[serde(rename = "freeFunction")]
    FreeFunction,
}

impl From<&FunctionDefinitionKind> for FunctionDefinitionKind {
    fn from(value: &FunctionDefinitionKind) -> Self {
        value.clone()
    }
}

impl ToString for FunctionDefinitionKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Function => "function".to_string(),
            Self::Receive => "receive".to_string(),
            Self::Constructor => "constructor".to_string(),
            Self::Fallback => "fallback".to_string(),
            Self::FreeFunction => "freeFunction".to_string(),
        }
    }
}

impl std::str::FromStr for FunctionDefinitionKind {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            "receive" => Ok(Self::Receive),
            "constructor" => Ok(Self::Constructor),
            "fallback" => Ok(Self::Fallback),
            "freeFunction" => Ok(Self::FreeFunction),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for FunctionDefinitionKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for FunctionDefinitionKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for FunctionDefinitionKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
