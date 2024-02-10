use serde::{Deserialize, Serialize};

#[doc = "ContractDefinitionContractKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"contract\","]
#[doc = "    \"interface\","]
#[doc = "    \"library\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ContractDefinitionContractKind {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library,
}

impl From<&ContractDefinitionContractKind> for ContractDefinitionContractKind {
    fn from(value: &ContractDefinitionContractKind) -> Self {
        value.clone()
    }
}

impl ToString for ContractDefinitionContractKind {
    fn to_string(&self) -> String {
        match *self {
            Self::Contract => "contract".to_string(),
            Self::Interface => "interface".to_string(),
            Self::Library => "library".to_string(),
        }
    }
}

impl std::str::FromStr for ContractDefinitionContractKind {
    type Err = crate::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::error::ConversionError> {
        match value {
            "contract" => Ok(Self::Contract),
            "interface" => Ok(Self::Interface),
            "library" => Ok(Self::Library),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for ContractDefinitionContractKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for ContractDefinitionContractKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for ContractDefinitionContractKind {
    type Error = crate::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::error::ConversionError> {
        value.parse()
    }
}
