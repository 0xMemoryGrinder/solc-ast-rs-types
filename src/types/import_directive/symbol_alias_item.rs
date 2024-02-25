use serde::{Deserialize, Serialize};

use crate::types::Identifier;

#[doc = "ImportDirectiveSymbolAliasesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"foreign\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"foreign\": {"]
#[doc = "      \"$ref\": \"#/definitions/Identifier\""]
#[doc = "    },"]
#[doc = "    \"local\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nameLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportDirectiveSymbolAliasesItem {
    pub foreign: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<String>,
    #[serde(
        rename = "nameLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_location: Option<String>,
}

impl From<&ImportDirectiveSymbolAliasesItem> for ImportDirectiveSymbolAliasesItem {
    fn from(value: &ImportDirectiveSymbolAliasesItem) -> Self {
        value.clone()
    }
}
