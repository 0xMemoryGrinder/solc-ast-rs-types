use serde::{Deserialize, Serialize};

mod hex_value;
pub use hex_value::*;
mod value;
pub use value::*;


#[doc = "YulLiteral"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteralValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteralHexValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum YulLiteral {
    Value(YulLiteralValue),
    HexValue(YulLiteralHexValue),
}

impl From<&YulLiteral> for YulLiteral {
    fn from(value: &YulLiteral) -> Self {
        value.clone()
    }
}

impl From<YulLiteralValue> for YulLiteral {
    fn from(value: YulLiteralValue) -> Self {
        Self::Value(value)
    }
}

impl From<YulLiteralHexValue> for YulLiteral {
    fn from(value: YulLiteralHexValue) -> Self {
        Self::HexValue(value)
    }
}