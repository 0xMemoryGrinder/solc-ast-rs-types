use serde::Serialize;

#[doc = "LiteralHexValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-f]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiteralHexValue(String);
impl std::ops::Deref for LiteralHexValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<LiteralHexValue> for String {
    fn from(value: LiteralHexValue) -> Self {
        value.0
    }
}

impl From<&LiteralHexValue> for LiteralHexValue {
    fn from(value: &LiteralHexValue) -> Self {
        value.clone()
    }
}

impl std::str::FromStr for LiteralHexValue {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        if regress::Regex::new("^[0-9a-f]*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-f]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl std::convert::TryFrom<&str> for LiteralHexValue {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for LiteralHexValue {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for LiteralHexValue {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for LiteralHexValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
