use serde::{Deserialize, Serialize};

use crate::types::{ContractDefinition, EnumDefinition, ErrorDefinition, FunctionDefinition, ImportDirective, PragmaDirective, StructDefinition, UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration};

#[doc = "SourceUnitNodesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ContractDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EnumDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ErrorDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FunctionDefinition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImportDirective\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PragmaDirective\""]
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
pub enum SourceUnitNodesItem {
    ContractDefinition(ContractDefinition),
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}

impl From<&SourceUnitNodesItem> for SourceUnitNodesItem {
    fn from(value: &SourceUnitNodesItem) -> Self {
        value.clone()
    }
}

impl From<ContractDefinition> for SourceUnitNodesItem {
    fn from(value: ContractDefinition) -> Self {
        Self::ContractDefinition(value)
    }
}

impl From<EnumDefinition> for SourceUnitNodesItem {
    fn from(value: EnumDefinition) -> Self {
        Self::EnumDefinition(value)
    }
}

impl From<ErrorDefinition> for SourceUnitNodesItem {
    fn from(value: ErrorDefinition) -> Self {
        Self::ErrorDefinition(value)
    }
}

impl From<FunctionDefinition> for SourceUnitNodesItem {
    fn from(value: FunctionDefinition) -> Self {
        Self::FunctionDefinition(value)
    }
}

impl From<ImportDirective> for SourceUnitNodesItem {
    fn from(value: ImportDirective) -> Self {
        Self::ImportDirective(value)
    }
}

impl From<PragmaDirective> for SourceUnitNodesItem {
    fn from(value: PragmaDirective) -> Self {
        Self::PragmaDirective(value)
    }
}

impl From<StructDefinition> for SourceUnitNodesItem {
    fn from(value: StructDefinition) -> Self {
        Self::StructDefinition(value)
    }
}

impl From<UserDefinedValueTypeDefinition> for SourceUnitNodesItem {
    fn from(value: UserDefinedValueTypeDefinition) -> Self {
        Self::UserDefinedValueTypeDefinition(value)
    }
}

impl From<UsingForDirective> for SourceUnitNodesItem {
    fn from(value: UsingForDirective) -> Self {
        Self::UsingForDirective(value)
    }
}

impl From<VariableDeclaration> for SourceUnitNodesItem {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}