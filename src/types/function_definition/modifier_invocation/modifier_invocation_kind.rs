use serde::{Deserialize, Serialize};

#[doc = "ModifierInvocationKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"modifierInvocation\","]
#[doc = "    \"baseConstructorSpecifier\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierInvocationKind {
    #[serde(rename = "modifierInvocation")]
    ModifierInvocation,
    #[serde(rename = "baseConstructorSpecifier")]
    BaseConstructorSpecifier,
}

impl From<&ModifierInvocationKind> for ModifierInvocationKind {
    fn from(value: &ModifierInvocationKind) -> Self {
        value.clone()
    }
}

impl ToString for ModifierInvocationKind {
    fn to_string(&self) -> String {
        match *self {
            Self::ModifierInvocation => "modifierInvocation".to_string(),
            Self::BaseConstructorSpecifier => "baseConstructorSpecifier".to_string(),
        }
    }
}

impl std::str::FromStr for ModifierInvocationKind {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "modifierInvocation" => Ok(Self::ModifierInvocation),
            "baseConstructorSpecifier" => Ok(Self::BaseConstructorSpecifier),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ModifierInvocationKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ModifierInvocationKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ModifierInvocationKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
