use serde::{Deserialize, Serialize};

use crate::types::{Identifier, IdentifierPath};

#[doc = "ModifierInvocationModifierName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Identifier\""]
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
pub enum ModifierInvocationModifierName {
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
}

impl From<&ModifierInvocationModifierName> for ModifierInvocationModifierName {
    fn from(value: &ModifierInvocationModifierName) -> Self {
        value.clone()
    }
}

impl From<Identifier> for ModifierInvocationModifierName {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

impl From<IdentifierPath> for ModifierInvocationModifierName {
    fn from(value: IdentifierPath) -> Self {
        Self::IdentifierPath(value)
    }
}
