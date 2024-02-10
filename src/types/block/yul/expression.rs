use serde::{Deserialize, Serialize};

mod function_call;
pub use function_call::*;
mod identifier;
pub use identifier::*;
mod literal;
pub use literal::*;

#[doc = "YulExpression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulFunctionCall\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulIdentifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLiteral\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum YulExpression {
    YulFunctionCall(YulFunctionCall),
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
}

impl From<&YulExpression> for YulExpression {
    fn from(value: &YulExpression) -> Self {
        value.clone()
    }
}

impl From<YulFunctionCall> for YulExpression {
    fn from(value: YulFunctionCall) -> Self {
        YulExpression::YulFunctionCall(value)
    }
}

impl From<YulIdentifier> for YulExpression {
    fn from(value: YulIdentifier) -> Self {
        YulExpression::YulIdentifier(value)
    }
}

impl From<YulLiteral> for YulExpression {
    fn from(value: YulLiteral) -> Self {
        YulExpression::YulLiteral(value)
    }
}
