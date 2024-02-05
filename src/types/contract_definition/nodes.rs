use serde::{Deserialize, Serialize};

use crate::types::{EnumDefinition, ErrorDefinition, EventDefinition, FunctionDefinition, ModifierDefinition, StructDefinition, UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration};


#[doc = "ContractDefinitionNodesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EventDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ModifierDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/StructDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UserDefinedValueTypeDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UsingForDirective\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableDeclaration\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContractDefinitionNodesItem {
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}

impl From<&ContractDefinitionNodesItem> for ContractDefinitionNodesItem {
    fn from(value: &ContractDefinitionNodesItem) -> Self {
        value.clone()
    }
}

impl From<EnumDefinition> for ContractDefinitionNodesItem {
    fn from(value: EnumDefinition) -> Self {
        Self::EnumDefinition(value)
    }
}

impl From<ErrorDefinition> for ContractDefinitionNodesItem {
    fn from(value: ErrorDefinition) -> Self {
        Self::ErrorDefinition(value)
    }
}

impl From<EventDefinition> for ContractDefinitionNodesItem {
    fn from(value: EventDefinition) -> Self {
        Self::EventDefinition(value)
    }
}

impl From<FunctionDefinition> for ContractDefinitionNodesItem {
    fn from(value: FunctionDefinition) -> Self {
        Self::FunctionDefinition(value)
    }
}

impl From<ModifierDefinition> for ContractDefinitionNodesItem {
    fn from(value: ModifierDefinition) -> Self {
        Self::ModifierDefinition(value)
    }
}

impl From<StructDefinition> for ContractDefinitionNodesItem {
    fn from(value: StructDefinition) -> Self {
        Self::StructDefinition(value)
    }
}

impl From<UserDefinedValueTypeDefinition> for ContractDefinitionNodesItem {
    fn from(value: UserDefinedValueTypeDefinition) -> Self {
        Self::UserDefinedValueTypeDefinition(value)
    }
}

impl From<UsingForDirective> for ContractDefinitionNodesItem {
    fn from(value: UsingForDirective) -> Self {
        Self::UsingForDirective(value)
    }
}

impl From<VariableDeclaration> for ContractDefinitionNodesItem {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}