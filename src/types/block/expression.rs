use serde::{Deserialize, Serialize};

mod assignment;
pub use assignment::*;
mod binary_operation;
pub use binary_operation::*;
mod conditional;
pub use conditional::*;
mod elementary_typename;
pub use elementary_typename::*;
mod function_call;
pub use function_call::*;
mod function_call_options;
pub use function_call_options::*;
mod identifier;
pub use identifier::*;
mod index_access;
pub use index_access::*;
mod index_range_access;
pub use index_range_access::*;
mod literal;
pub use literal::*;
mod member_access;
pub use member_access::*;
mod new;
pub use new::*;
mod tuple;
pub use tuple::*;
mod unary_operation;
pub use unary_operation::*;


#[doc = "Expression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Assignment\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BinaryOperation\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Conditional\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElementaryTypeNameExpression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCall\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionCallOptions\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IndexAccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IndexRangeAccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Literal\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/MemberAccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/NewExpression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TupleExpression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UnaryOperation\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Expression {
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    Identifier(Identifier),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
}

impl From<&Expression> for Expression {
    fn from(value: &Expression) -> Self {
        value.clone()
    }
}

impl From<Assignment> for Expression {
    fn from(value: Assignment) -> Self {
        Expression::Assignment(value)
    }
}

impl From<BinaryOperation> for Expression {
    fn from(value: BinaryOperation) -> Self {
        Expression::BinaryOperation(value)
    }
}

impl From<Conditional> for Expression {
    fn from(value: Conditional) -> Self {
        Expression::Conditional(value)
    }
}

impl From<ElementaryTypeNameExpression> for Expression {
    fn from(value: ElementaryTypeNameExpression) -> Self {
        Expression::ElementaryTypeNameExpression(value)
    }
}

impl From<FunctionCall> for Expression {
    fn from(value: FunctionCall) -> Self {
        Expression::FunctionCall(value)
    }
}

impl From<FunctionCallOptions> for Expression {
    fn from(value: FunctionCallOptions) -> Self {
        Expression::FunctionCallOptions(value)
    }
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Expression::Identifier(value)
    }
}

impl From<IndexAccess> for Expression {
    fn from(value: IndexAccess) -> Self {
        Expression::IndexAccess(value)
    }
}

impl From<IndexRangeAccess> for Expression {
    fn from(value: IndexRangeAccess) -> Self {
        Expression::IndexRangeAccess(value)
    }
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Literal(value)
    }
}

impl From<MemberAccess> for Expression {
    fn from(value: MemberAccess) -> Self {
        Expression::MemberAccess(value)
    }
}

impl From<NewExpression> for Expression {
    fn from(value: NewExpression) -> Self {
        Expression::NewExpression(value)
    }
}

impl From<TupleExpression> for Expression {
    fn from(value: TupleExpression) -> Self {
        Expression::TupleExpression(value)
    }
}

impl From<UnaryOperation> for Expression {
    fn from(value: UnaryOperation) -> Self {
        Expression::UnaryOperation(value)
    }
}