use serde::Serialize;

#[doc = "SourceLocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^\\\\d+:\\\\d+:\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourceLocation(String);
impl std::ops::Deref for SourceLocation {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SourceLocation> for String {
    fn from(value: SourceLocation) -> Self {
        value.0
    }
}
impl From<&SourceLocation> for SourceLocation {
    fn from(value: &SourceLocation) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SourceLocation {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        if regress::Regex::new("^\\d+:\\d+:\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^\\d+:\\d+:\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SourceLocation {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SourceLocation {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SourceLocation {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SourceLocation {
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