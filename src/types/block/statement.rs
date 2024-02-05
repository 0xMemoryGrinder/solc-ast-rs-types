use serde::{Deserialize, Serialize};

mod r#break;
pub use r#break::*;
mod r#continue;
pub use r#continue::*;
mod do_while;
pub use do_while::*;
mod emit;
pub use emit::*;
mod expression;
pub use expression::*;
mod r#for;
pub use r#for::*;
mod r#if;
pub use r#if::*;
mod inline_assembly;
pub use inline_assembly::*;
mod placeholder;
pub use placeholder::*;
mod r#return;
pub use r#return::*;
mod revert;
pub use revert::*;
mod r#try;
pub use r#try::*;
mod unchecked;
pub use unchecked::*;
mod variable_declaration;
pub use variable_declaration::*;
mod r#while;
pub use r#while::*;

use crate::types::Block;


#[doc = "Statement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Block\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Break\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Continue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DoWhileStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EmitStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExpressionStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ForStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/IfStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InlineAssembly\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PlaceholderStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Return\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/RevertStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TryStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UncheckedBlock\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclarationStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/WhileStatement\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Statement {
    #[serde(rename = "Block")]
    Block(Block),
    #[serde(rename = "Break")]
    Break(Break),
    #[serde(rename = "Continue")]
    Continue(Continue),
    #[serde(rename = "DoWhileStatement")]
    DoWhileStatement(Box<DoWhileStatement>),
    #[serde(rename = "EmitStatement")]
    EmitStatement(EmitStatement),
    #[serde(rename = "ExpressionStatement")]
    ExpressionStatement(ExpressionStatement),
    #[serde(rename = "ForStatement")]
    ForStatement(ForStatement),
    #[serde(rename = "IfStatement")]
    IfStatement(IfStatement),
    #[serde(rename = "InlineAssembly")]
    InlineAssembly(InlineAssembly),
    #[serde(rename = "PlaceholderStatement")]
    PlaceholderStatement(PlaceholderStatement),
    #[serde(rename = "Return")]
    Return(Return),
    #[serde(rename = "RevertStatement")]
    RevertStatement(RevertStatement),
    #[serde(rename = "TryStatement")]
    TryStatement(TryStatement),
    #[serde(rename = "UncheckedBlock")]
    UncheckedBlock(UncheckedBlock),
    #[serde(rename = "VariableDeclarationStatement")]
    VariableDeclarationStatement(VariableDeclarationStatement),
    #[serde(rename = "WhileStatement")]
    WhileStatement(WhileStatement),
}

impl From<&Statement> for Statement {
    fn from(value: &Statement) -> Self {
        value.clone()
    }
}

impl From<Block> for Statement {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}

impl From<Break> for Statement {
    fn from(value: Break) -> Self {
        Self::Break(value)
    }
}

impl From<Continue> for Statement {
    fn from(value: Continue) -> Self {
        Self::Continue(value)
    }
}

impl From<Box<DoWhileStatement>> for Statement {
    fn from(value: Box<DoWhileStatement>) -> Self {
        Self::DoWhileStatement(value)
    }
}

impl From<EmitStatement> for Statement {
    fn from(value: EmitStatement) -> Self {
        Self::EmitStatement(value)
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(value: ExpressionStatement) -> Self {
        Self::ExpressionStatement(value)
    }
}

impl From<ForStatement> for Statement {
    fn from(value: ForStatement) -> Self {
        Self::ForStatement(value)
    }
}

impl From<IfStatement> for Statement {
    fn from(value: IfStatement) -> Self {
        Self::IfStatement(value)
    }
}

impl From<InlineAssembly> for Statement {
    fn from(value: InlineAssembly) -> Self {
        Self::InlineAssembly(value)
    }
}

impl From<PlaceholderStatement> for Statement {
    fn from(value: PlaceholderStatement) -> Self {
        Self::PlaceholderStatement(value)
    }
}

impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Self::Return(value)
    }
}

impl From<RevertStatement> for Statement {
    fn from(value: RevertStatement) -> Self {
        Self::RevertStatement(value)
    }
}

impl From<TryStatement> for Statement {
    fn from(value: TryStatement) -> Self {
        Self::TryStatement(value)
    }
}

impl From<UncheckedBlock> for Statement {
    fn from(value: UncheckedBlock) -> Self {
        Self::UncheckedBlock(value)
    }
}

impl From<VariableDeclarationStatement> for Statement {
    fn from(value: VariableDeclarationStatement) -> Self {
        Self::VariableDeclarationStatement(value)
    }
}

impl From<WhileStatement> for Statement {
    fn from(value: WhileStatement) -> Self {
        Self::WhileStatement(value)
    }
}