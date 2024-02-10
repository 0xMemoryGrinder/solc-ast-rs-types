use serde::{Deserialize, Serialize};

#[doc = "InlineAssemblyEvmVersion"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"homestead\","]
#[doc = "    \"tangerineWhistle\","]
#[doc = "    \"spuriousDragon\","]
#[doc = "    \"byzantium\","]
#[doc = "    \"constantinople\","]
#[doc = "    \"petersburg\","]
#[doc = "    \"istanbul\","]
#[doc = "    \"berlin\","]
#[doc = "    \"london\","]
#[doc = "    \"paris\","]
#[doc = "    \"shanghai\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineAssemblyEvmVersion {
    #[serde(rename = "homestead")]
    Homestead,
    #[serde(rename = "tangerineWhistle")]
    TangerineWhistle,
    #[serde(rename = "spuriousDragon")]
    SpuriousDragon,
    #[serde(rename = "byzantium")]
    Byzantium,
    #[serde(rename = "constantinople")]
    Constantinople,
    #[serde(rename = "petersburg")]
    Petersburg,
    #[serde(rename = "istanbul")]
    Istanbul,
    #[serde(rename = "berlin")]
    Berlin,
    #[serde(rename = "london")]
    London,
    #[serde(rename = "paris")]
    Paris,
    #[serde(rename = "shanghai")]
    Shanghai,
}

impl From<&InlineAssemblyEvmVersion> for InlineAssemblyEvmVersion {
    fn from(value: &InlineAssemblyEvmVersion) -> Self {
        value.clone()
    }
}

impl ToString for InlineAssemblyEvmVersion {
    fn to_string(&self) -> String {
        match *self {
            Self::Homestead => "homestead".to_string(),
            Self::TangerineWhistle => "tangerineWhistle".to_string(),
            Self::SpuriousDragon => "spuriousDragon".to_string(),
            Self::Byzantium => "byzantium".to_string(),
            Self::Constantinople => "constantinople".to_string(),
            Self::Petersburg => "petersburg".to_string(),
            Self::Istanbul => "istanbul".to_string(),
            Self::Berlin => "berlin".to_string(),
            Self::London => "london".to_string(),
            Self::Paris => "paris".to_string(),
            Self::Shanghai => "shanghai".to_string(),
        }
    }
}

impl std::str::FromStr for InlineAssemblyEvmVersion {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "homestead" => Ok(Self::Homestead),
            "tangerineWhistle" => Ok(Self::TangerineWhistle),
            "spuriousDragon" => Ok(Self::SpuriousDragon),
            "byzantium" => Ok(Self::Byzantium),
            "constantinople" => Ok(Self::Constantinople),
            "petersburg" => Ok(Self::Petersburg),
            "istanbul" => Ok(Self::Istanbul),
            "berlin" => Ok(Self::Berlin),
            "london" => Ok(Self::London),
            "paris" => Ok(Self::Paris),
            "shanghai" => Ok(Self::Shanghai),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for InlineAssemblyEvmVersion {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for InlineAssemblyEvmVersion {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for InlineAssemblyEvmVersion {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
