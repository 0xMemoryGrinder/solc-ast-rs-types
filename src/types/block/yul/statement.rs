use serde::{Deserialize, Serialize};

mod assignment;
pub use assignment::*;
mod r#break;
pub use r#break::*;
mod r#continue;
pub use r#continue::*;
mod expression;
pub use expression::*;
mod leave;
pub use leave::*;
mod r#for;
pub use r#for::*;
mod function_definition;
pub use function_definition::*;
mod r#if;
pub use r#if::*;
mod r#switch;
pub use r#switch::*;
mod variable_declaration;
pub use variable_declaration::*;

use crate::types::YulBlock;

#[doc = "YulStatement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulAssignment\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulBlock\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulBreak\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulContinue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulExpressionStatement\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulLeave\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulForLoop\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulFunctionDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulIf\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulSwitch\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/YulVariableDeclaration\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum YulStatement {
    YulAssignment(YulAssignment),
    YulBlock(YulBlock),
    YulBreak(YulBreak),
    YulContinue(YulContinue),
    YulExpressionStatement(YulExpressionStatement),
    YulLeave(YulLeave),
    YulForLoop(YulForLoop),
    YulFunctionDefinition(YulFunctionDefinition),
    YulIf(YulIf),
    YulSwitch(YulSwitch),
    YulVariableDeclaration(YulVariableDeclaration),
}

impl From<&YulStatement> for YulStatement {
    fn from(value: &YulStatement) -> Self {
        value.clone()
    }
}

impl From<YulAssignment> for YulStatement {
    fn from(value: YulAssignment) -> Self {
        YulStatement::YulAssignment(value)
    }
}

impl From<YulBlock> for YulStatement {
    fn from(value: YulBlock) -> Self {
        YulStatement::YulBlock(value)
    }
}

impl From<YulBreak> for YulStatement {
    fn from(value: YulBreak) -> Self {
        YulStatement::YulBreak(value)
    }
}

impl From<YulContinue> for YulStatement {
    fn from(value: YulContinue) -> Self {
        YulStatement::YulContinue(value)
    }
}

impl From<YulExpressionStatement> for YulStatement {
    fn from(value: YulExpressionStatement) -> Self {
        YulStatement::YulExpressionStatement(value)
    }
}

impl From<YulLeave> for YulStatement {
    fn from(value: YulLeave) -> Self {
        YulStatement::YulLeave(value)
    }
}

impl From<YulForLoop> for YulStatement {
    fn from(value: YulForLoop) -> Self {
        YulStatement::YulForLoop(value)
    }
}

impl From<YulFunctionDefinition> for YulStatement {
    fn from(value: YulFunctionDefinition) -> Self {
        YulStatement::YulFunctionDefinition(value)
    }
}

impl From<YulIf> for YulStatement {
    fn from(value: YulIf) -> Self {
        YulStatement::YulIf(value)
    }
}

impl From<YulSwitch> for YulStatement {
    fn from(value: YulSwitch) -> Self {
        YulStatement::YulSwitch(value)
    }
}

impl From<YulVariableDeclaration> for YulStatement {
    fn from(value: YulVariableDeclaration) -> Self {
        YulStatement::YulVariableDeclaration(value)
    }
}
