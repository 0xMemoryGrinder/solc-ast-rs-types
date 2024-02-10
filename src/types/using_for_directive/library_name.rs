use serde::{Deserialize, Serialize};

use crate::types::{IdentifierPath, UserDefinedTypeName};

#[doc = "UsingForDirectiveLibraryName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IdentifierPath\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UsingForDirectiveLibraryName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

impl From<&UsingForDirectiveLibraryName> for UsingForDirectiveLibraryName {
    fn from(value: &UsingForDirectiveLibraryName) -> Self {
        value.clone()
    }
}

impl From<UserDefinedTypeName> for UsingForDirectiveLibraryName {
    fn from(value: UserDefinedTypeName) -> Self {
        Self::UserDefinedTypeName(value)
    }
}

impl From<IdentifierPath> for UsingForDirectiveLibraryName {
    fn from(value: IdentifierPath) -> Self {
        Self::IdentifierPath(value)
    }
}
