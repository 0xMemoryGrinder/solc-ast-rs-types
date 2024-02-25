use serde::{Deserialize, Serialize};

#[doc = "TypeDescriptions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"typeIdentifier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"typeString\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeDescriptions {
    #[serde(
        rename = "typeIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,
    #[serde(
        rename = "typeString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_string: Option<String>,
}

impl From<&TypeDescriptions> for TypeDescriptions {
    fn from(value: &TypeDescriptions) -> Self {
        value.clone()
    }
}
