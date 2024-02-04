use serde::{Deserialize, Serialize};

mod array;
pub use array::*;
mod elementary;
pub use elementary::*;
mod function;
pub use function::*;
mod mapping;
pub use mapping::*;
mod user_defined;
pub use user_defined::*;


#[doc = "TypeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ArrayTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElementaryTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionTypeName\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Mapping\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedTypeName\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TypeName {
    ArrayTypeName(Box<ArrayTypeName>),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}

impl From<&TypeName> for TypeName {
    fn from(value: &TypeName) -> Self {
        value.clone()
    }
}

impl From<Box<ArrayTypeName>> for TypeName {
    fn from(value: Box<ArrayTypeName>) -> Self {
        Self::ArrayTypeName(value)
    }
}

impl From<ElementaryTypeName> for TypeName {
    fn from(value: ElementaryTypeName) -> Self {
        Self::ElementaryTypeName(value)
    }
}

impl From<FunctionTypeName> for TypeName {
    fn from(value: FunctionTypeName) -> Self {
        Self::FunctionTypeName(value)
    }
}

impl From<Mapping> for TypeName {
    fn from(value: Mapping) -> Self {
        Self::Mapping(value)
    }
}

impl From<UserDefinedTypeName> for TypeName {
    fn from(value: UserDefinedTypeName) -> Self {
        Self::UserDefinedTypeName(value)
    }
}