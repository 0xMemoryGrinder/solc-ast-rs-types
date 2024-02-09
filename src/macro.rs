#[cfg(any(feature = "visit", feature = "visit-mut"))]
macro_rules! make_visitor {
    (
        $(#[$attr:meta])*
        trait $trait_name:ident $(is $mut:ident)?;
    ) => {
        make_visitor! {
            @impl
            $(#[$attr])*
            pub trait $trait_name<'ast> {
                
                fn visit_source_unit(&mut v, source_unit: &'ast $($mut)? SourceUnit) {
                    for node in & $($mut)? source_unit.nodes {
                        v.visit_source_unit_nodes_item(node);
                    }
                }

                fn visit_source_unit_nodes_item(&mut v, node: &'ast $($mut)? SourceUnitNodesItem) {
                    match node {
                        SourceUnitNodesItem::ContractDefinition(contract) => v.visit_contract_definition(contract),
                        SourceUnitNodesItem::EnumDefinition(enumm) => v.visit_enum_definition(enumm),
                        SourceUnitNodesItem::ErrorDefinition(error) => v.visit_error_definition(error),
                        SourceUnitNodesItem::FunctionDefinition(function) => v.visit_function_definition(function),
                        SourceUnitNodesItem::ImportDirective(import) => v.visit_import_directive(import),
                        SourceUnitNodesItem::PragmaDirective(pragma) => v.visit_pragma_directive(pragma),
                        SourceUnitNodesItem::StructDefinition(strukt) => v.visit_struct_definition(strukt),
                        SourceUnitNodesItem::UserDefinedValueTypeDefinition(udt) => v.visit_udt(udt),
                        SourceUnitNodesItem::UsingForDirective(using) => v.visit_using_for_directive(using),
                        SourceUnitNodesItem::VariableDeclaration(variable) => v.visit_variable_declaration(variable),
                    }
                }



                // Contract definitions

                fn visit_contract_definition(&mut v, contract: &'ast $($mut)? ContractDefinition) {
                    if let Some(doc) = & $($mut)? contract.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    for base in & $($mut)? contract.base_contracts {
                        v.visit_inheritance_specifier(base);
                    }
                    for node in & $($mut)? contract.nodes {
                        v.visit_contract_definition_node_item(node);
                    }
                }

                fn visit_inheritance_specifier(&mut v, specifier: &'ast $($mut)? InheritanceSpecifier) {
                    if let Some(args) = & $($mut)? specifier.arguments {
                        for arg in args {
                            v.visit_expression(arg);
                        }
                    }
                    match & $($mut)? specifier.base_name {
                        InheritanceSpecifierBaseName::UserDefinedTypeName(udt) => v.visit_user_defined_type_name(udt),
                        InheritanceSpecifierBaseName::IdentifierPath(path) => v.visit_identifier_path(path),
                    }
                }

                fn visit_contract_definition_node_item(&mut v, node: &'ast $($mut)? ContractDefinitionNodesItem) {
                    match node {
                        ContractDefinitionNodesItem::EnumDefinition(enumm) => v.visit_enum_definition(enumm),
                        ContractDefinitionNodesItem::ErrorDefinition(error) => v.visit_error_definition(error),
                        ContractDefinitionNodesItem::EventDefinition(event) => v.visit_event_definition(event),
                        ContractDefinitionNodesItem::FunctionDefinition(function) => v.visit_function_definition(function),
                        ContractDefinitionNodesItem::ModifierDefinition(modifier) => v.visit_modifier_definition(modifier),
                        ContractDefinitionNodesItem::StructDefinition(strukt) => v.visit_struct_definition(strukt),
                        ContractDefinitionNodesItem::UserDefinedValueTypeDefinition(udt) => v.visit_udt(udt),
                        ContractDefinitionNodesItem::UsingForDirective(using) => v.visit_using_for_directive(using),
                        ContractDefinitionNodesItem::VariableDeclaration(variable) => v.visit_variable_declaration(variable),
                    }
                }

                fn visit_enum_definition(&mut v, enumm: &'ast $($mut)? EnumDefinition) {
                    if let Some(doc) = & $($mut)? enumm.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    for variant in & $($mut)? enumm.members {
                        v.visit_enum_value(variant);
                    }
                }

                fn visit_enum_value(&mut _v, _variant: &'ast $($mut)? EnumValue) {
                    // nothing to do
                }

                fn visit_error_definition(&mut v, error: &'ast $($mut)? ErrorDefinition) {
                    if let Some(doc) = & $($mut)? error.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    v.visit_parameter_list(& $($mut)? error.parameters);
                }

                fn visit_event_definition(&mut v, event: &'ast $($mut)? EventDefinition) {
                    if let Some(doc) = & $($mut)? event.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    v.visit_parameter_list(& $($mut)? event.parameters);
                }

                fn visit_function_definition(&mut v, function: &'ast $($mut)? FunctionDefinition) {
                    if let Some(doc) = & $($mut)? function.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    for modifier in & $($mut)? function.modifiers {
                        v.visit_modifier_invocation(modifier);
                    }
                    v.visit_parameter_list(& $($mut)? function.parameters);
                    v.visit_parameter_list(& $($mut)? function.return_parameters);
                    if let Some(body) = & $($mut)? function.body {
                        v.visit_block(body);
                    }
                }

                fn visit_modifier_definition(&mut v, modifier: &'ast $($mut)? ModifierDefinition) {
                    if let Some(doc) = & $($mut)? modifier.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    v.visit_parameter_list(& $($mut)? modifier.parameters);
                    v.visit_block(& $($mut)? modifier.body);
                }

                fn visit_struct_definition(&mut v, strukt: &'ast $($mut)? StructDefinition) {
                    if let Some(doc) = & $($mut)? strukt.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    for member in & $($mut)? strukt.members {
                        v.visit_variable_declaration(member);
                    }
                }

                fn visit_udt(&mut _v, _udt: &'ast $($mut)? UserDefinedValueTypeDefinition) {
                    // nothing to do
                }

                fn visit_using_for_directive(&mut v, using: &'ast $($mut)? UsingForDirective) {
                    for item in & $($mut)? using.function_list {
                        v.visit_using_for_directive_function_list_item(item);
                    }
                    if let Some(name) = & $($mut)? using.library_name {
                        match name {
                            UsingForDirectiveLibraryName::UserDefinedTypeName(udt) => v.visit_user_defined_type_name(udt),
                            UsingForDirectiveLibraryName::IdentifierPath(path) => v.visit_identifier_path(path),
                        }
                    }
                    if let Some(typen_name) = & $($mut)? using.type_name {
                        v.visit_type_name(typen_name);
                    }
                }

                fn visit_using_for_directive_function_list_item(&mut v, item: &'ast $($mut)? UsingForDirectiveFunctionListItem) {
                    match & $($mut)? item {
                        UsingForDirectiveFunctionListItem::Function { function } => v.visit_identifier_path(function),
                        UsingForDirectiveFunctionListItem::FunctionList { definition, operator: _} => v.visit_identifier_path(definition),
                    }
                }

                fn visit_variable_declaration(&mut v, variable: &'ast $($mut)? VariableDeclaration) {
                    if let Some(doc) = & $($mut)? variable.documentation {
                        v.visit_structured_documentation(doc);
                    }
                    v.visit_type_descriptions(& $($mut)? variable.type_descriptions);
                    if let Some(type_name) = & $($mut)? variable.type_name {
                        v.visit_type_name(type_name);
                    }
                    if let Some(value) = & $($mut)? variable.value {
                        v.visit_expression(value);
                    }
                }





                // Import directives
                fn visit_import_directive(&mut v, import: &'ast $($mut)? ImportDirective) {
                    for alias in & $($mut)? import.symbol_aliases {
                        v.visit_import_directive_symbol_aliases_item(alias);
                    }
                }

                fn visit_import_directive_symbol_aliases_item(&mut v, alias: &'ast $($mut)? ImportDirectiveSymbolAliasesItem) {
                    v.visit_identifier(& $($mut)? alias.foreign);
                }

                // Pragma directives
                fn visit_pragma_directive(&mut _v, _pragma: &'ast $($mut)? PragmaDirective) {
                    // nothing to do
                }






















                fn visit_parameter_list(&mut v, params: &'ast $($mut)? ParameterList) {
                    for param in & $($mut)? params.parameters {
                        v.visit_variable_declaration(param);
                    }
                }

                fn visit_modifier_invocation(&mut v, modifier: &'ast $($mut)? ModifierInvocation) {
                    if let Some(args) = & $($mut)? modifier.arguments {
                        for arg in args {
                            v.visit_expression(arg);
                        }
                    }
                    match & $($mut)? modifier.modifier_name {
                        ModifierInvocationModifierName::Identifier(ident) => v.visit_identifier(ident),
                        ModifierInvocationModifierName::IdentifierPath(path) => v.visit_identifier_path(path),
                    }
                }
                

                fn visit_type_descriptions(&mut _v, _type_descriptions: &'ast $($mut)? TypeDescriptions) {
                    // nothing to do
                }

                fn visit_type_name(&mut v, type_name: &'ast $($mut)? TypeName) {
                    match type_name {
                        TypeName::ArrayTypeName(array) => v.visit_array_type_name(array),
                        TypeName::FunctionTypeName(function) => v.visit_function_type_name(function),
                        TypeName::Mapping(mapping) => v.visit_mapping(mapping),
                        TypeName::UserDefinedTypeName(udt) => v.visit_user_defined_type_name(udt),
                        TypeName::ElementaryTypeName(elementary) => v.visit_elementary_type_name(elementary),
                    }
                }

                fn visit_array_type_name(&mut v, array: &'ast $($mut)? ArrayTypeName) {
                    v.visit_type_name(& $($mut)? array.base_type);
                    if let Some(length) = & $($mut)? array.length {
                        v.visit_expression(length);
                    }
                    v.visit_type_descriptions(& $($mut)? array.type_descriptions);
                }

                fn visit_function_type_name(&mut v, function: &'ast $($mut)? FunctionTypeName) {
                    v.visit_parameter_list(& $($mut)? function.parameter_types);
                    v.visit_parameter_list(& $($mut)? function.return_parameter_types);
                }

                fn visit_mapping(&mut v, mapping: &'ast $($mut)? Mapping) {
                    v.visit_type_name(& $($mut)? mapping.key_type);
                    v.visit_type_name(& $($mut)? mapping.value_type);
                }

                fn visit_user_defined_type_name(&mut _v, _udt: &'ast $($mut)? UserDefinedTypeName) {
                    // nothing to do
                }

                fn visit_elementary_type_name(&mut _v, _elementary: &'ast $($mut)? ElementaryTypeName) {
                    // nothing to do
                }

                fn visit_structured_documentation(&mut _v, _doc: &'ast $($mut)? StructuredDocumentation) {
                    // nothing to do
                }

                fn visit_identifier_path(&mut _v, _path: &'ast $($mut)? IdentifierPath) {
                    // nothing to do
                }
                
                
                
                









                // Statements
                fn visit_block(&mut v, block: &'ast $($mut)? Block) {
                    if let Some(statements) = & $($mut)? block.statements {
                        for stmt in statements {
                            v.visit_statement(stmt);
                        }
                    }
                }

                fn visit_statement(&mut v, stmt: &'ast $($mut)? Statement) {
                    match stmt {
                        Statement::Block(block) => v.visit_block(block),
                        Statement::Break(break_) => v.visit_break(break_),
                        Statement::Continue(continue_) => v.visit_continue(continue_),
                        Statement::DoWhileStatement(do_while) => v.visit_do_while(do_while),
                        Statement::EmitStatement(emit) => v.visit_emit(emit),
                        Statement::ExpressionStatement(expr) => v.visit_expression_statement(expr),
                        Statement::ForStatement(r#for) => v.visit_for(r#for),
                        Statement::IfStatement(r#if) => v.visit_if(r#if),
                        Statement::InlineAssembly(inline) => v.visit_inline_assembly(inline),
                        Statement::PlaceholderStatement(placeholder) => v.visit_placeholder(placeholder),
                        Statement::Return(r#return) => v.visit_return(r#return),
                        Statement::RevertStatement(revert) => v.visit_revert(revert),
                        Statement::TryStatement(r#try) => v.visit_try(r#try),
                        Statement::UncheckedBlock(unchecked) => v.visit_unchecked(unchecked),
                        Statement::VariableDeclarationStatement(variable) => v.visit_variable_declaration_statement(variable),
                        Statement::WhileStatement(r#while) => v.visit_while(r#while),
                    }
                }

                fn visit_break(&mut _v, _break_: &'ast $($mut)? Break) {
                    // nothing to do
                }

                fn visit_continue(&mut _v, _continue: &'ast $($mut)? Continue) {
                    // nothing to do
                }

                fn visit_do_while(&mut v, do_while: &'ast $($mut)? DoWhileStatement) {
                    match & $($mut)? do_while.body {
                        DoWhileStatementBody::Block(block) => v.visit_block(block),
                        DoWhileStatementBody::Statement(stmt) => v.visit_statement(stmt),
                    }
                    v.visit_expression(& $($mut)? do_while.condition);
                }

                fn visit_emit(&mut v, emit: &'ast $($mut)? EmitStatement) {
                    v.visit_function_call(& $($mut)? emit.event_call);
                }

                fn visit_expression_statement(&mut v, expr: &'ast $($mut)? ExpressionStatement) {
                    v.visit_expression(& $($mut)? expr.expression);
                }

                fn visit_for(&mut v, forr: &'ast $($mut)? ForStatement) {
                    match & $($mut)? forr.body {
                        ForStatementBody::Block(block) => v.visit_block(block),
                        ForStatementBody::Statement(stmt) => v.visit_expression_statement(stmt),
                    }
                    if let Some(expr) = & $($mut)? forr.condition {
                        v.visit_expression(expr);
                    }
                    if let Some(expr) = & $($mut)? forr.initialization_expression {
                        match expr {
                            ForStatementInitializationExpression::ExpressionStatement(expr) => v.visit_expression_statement(expr),
                            ForStatementInitializationExpression::VariableDeclarationStatement(decl) => v.visit_variable_declaration_statement(decl),
                        }
                    }
                    if let Some(expr) = & $($mut)? forr.loop_expression {
                        v.visit_expression_statement(expr);
                    }
                }

                fn visit_if(&mut v, iff: &'ast $($mut)? IfStatement) {
                    v.visit_expression(& $($mut)? iff.condition);
                    if let Some(body) = & $($mut)? iff.false_body {
                        match & $($mut)? body {
                            IfStatementFalseBody::Block(block) => v.visit_block(block),
                            IfStatementFalseBody::Statement(stmt) => v.visit_statement(stmt),
                        }
                    }
                    match & $($mut)? iff.true_body {
                        IfStatementTrueBody::Block(block) => v.visit_block(block),
                        IfStatementTrueBody::Statement(stmt) => v.visit_statement(stmt),
                    }
                }

                fn visit_inline_assembly(&mut v, inline: &'ast $($mut)? InlineAssembly) {
                    v.visit_yul_block(& $($mut)? inline.ast);
                }

                fn visit_placeholder(&mut _v, _placeholder: &'ast $($mut)? PlaceholderStatement) {
                    // nothing to do
                }

                fn visit_return(&mut v, returnn: &'ast $($mut)? Return) {
                    if let Some(expr) = & $($mut)? returnn.expression {
                        v.visit_expression(expr);
                    }
                }

                fn visit_revert(&mut v, revert: &'ast $($mut)? RevertStatement) {
                    v.visit_function_call(& $($mut)? revert.error_call);
                }

                fn visit_try(&mut v, tryy: &'ast $($mut)? TryStatement) {
                    for clause in & $($mut)? tryy.clauses {
                        v.visit_try_catch_clause(clause);
                    }
                    v.visit_function_call(& $($mut)? tryy.external_call);
                }

                fn visit_try_catch_clause(&mut v, clause: &'ast $($mut)? TryCatchClause) {
                    v.visit_block(& $($mut)? clause.block);
                    if let Some(params) = & $($mut)? clause.parameters {
                        v.visit_parameter_list(params);
                    }
                }

                fn visit_unchecked(&mut v, unchecked: &'ast $($mut)? UncheckedBlock) {
                    for stmt in & $($mut)? unchecked.statements {
                        v.visit_statement(stmt);
                    }
                }

                fn visit_variable_declaration_statement(&mut v, variable: &'ast $($mut)? VariableDeclarationStatement) {
                    for decl in & $($mut)? variable.declarations {
                        if let Some(var) = & $($mut)? decl {
                            v.visit_variable_declaration(var);
                        }
                    }
                    if let Some(expr) = & $($mut)? variable.initial_value {
                        v.visit_expression(expr);
                    }
                }

                fn visit_while(&mut v, r#while: &'ast $($mut)? WhileStatement) {
                    match & $($mut)? r#while.body {
                        WhileStatementBody::Block(block) => v.visit_block(block),
                        WhileStatementBody::Statement(stmt) => v.visit_statement(stmt),
                    }
                    v.visit_expression(& $($mut)? r#while.condition);
                }






                // Yul
                fn visit_yul_block(&mut v, block: &'ast $($mut)? YulBlock) {
                    for stmt in & $($mut)? block.statements {
                        v.visit_yul_statement(stmt);
                    }
                }

                fn visit_yul_statement(&mut v, stmt: &'ast $($mut)? YulStatement) {
                    match stmt {
                        YulStatement::YulAssignment(assignment) => v.visit_yul_assignment(assignment),
                        YulStatement::YulBlock(block) => v.visit_yul_block(block),
                        YulStatement::YulBreak(break_) => v.visit_yul_break(break_),
                        YulStatement::YulContinue(continue_) => v.visit_yul_continue(continue_),
                        YulStatement::YulExpressionStatement(expr) => v.visit_yul_expression_statement(expr),
                        YulStatement::YulLeave(leave) => v.visit_yul_leave(leave),
                        YulStatement::YulForLoop(r#for) => v.visit_yul_for(r#for),
                        YulStatement::YulFunctionDefinition(function) => v.visit_yul_function_definition(function),
                        YulStatement::YulIf(r#if) => v.visit_yul_if(r#if),
                        YulStatement::YulSwitch(r#switch) => v.visit_yul_switch(r#switch),
                        YulStatement::YulVariableDeclaration(variable) => v.visit_yul_variable_declaration(variable),
                    }
                }

                fn visit_yul_assignment(&mut v, assignment: &'ast $($mut)? YulAssignment) {
                    v.visit_yul_expression(& $($mut)? assignment.value);
                    for ident in & $($mut)? assignment.variable_names {
                        v.visit_yul_identifier(ident);
                    }
                }

                fn visit_yul_break(&mut _v, _break_: &'ast $($mut)? YulBreak) {
                    // nothing to do
                }

                fn visit_yul_continue(&mut _v, _continue: &'ast $($mut)? YulContinue) {
                    // nothing to do
                }

                fn visit_yul_expression_statement(&mut v, expr: &'ast $($mut)? YulExpressionStatement) {
                    v.visit_yul_expression(& $($mut)? expr.expression);
                }

                fn visit_yul_leave(&mut _v, _leave: &'ast $($mut)? YulLeave) {
                    // nothing to do
                }

                fn visit_yul_for(&mut v, forr: &'ast $($mut)? YulForLoop) {
                    v.visit_yul_block(& $($mut)? forr.body);
                    v.visit_yul_expression(& $($mut)? forr.condition);
                    v.visit_yul_block(& $($mut)? forr.post);
                    v.visit_yul_block(& $($mut)? forr.pre);
                }

                fn visit_yul_function_definition(&mut v, function: &'ast $($mut)? YulFunctionDefinition) {
                    v.visit_yul_block(& $($mut)? function.body);
                    for param in & $($mut)? function.parameters {
                        v.visit_yul_typed_name(param);
                    }
                    for param in & $($mut)? function.return_variables {
                        v.visit_yul_typed_name(param);
                    }
                }

                fn visit_yul_if(&mut v, iff: &'ast $($mut)? YulIf) {
                    v.visit_yul_block(& $($mut)? iff.body);
                    v.visit_yul_expression(& $($mut)? iff.condition);
                }

                fn visit_yul_switch(&mut v, r#switch: &'ast $($mut)? YulSwitch) {
                    for case in & $($mut)? r#switch.cases {
                        v.visit_yul_case(case);
                    }
                    v.visit_yul_expression(& $($mut)? r#switch.expression);
                }

                fn visit_yul_variable_declaration(&mut v, variable: &'ast $($mut)? YulVariableDeclaration) {
                    if let Some(value) = & $($mut)? variable.value {
                        v.visit_yul_expression(value);
                    }
                    for var in & $($mut)? variable.variables {
                        v.visit_yul_typed_name(var);
                    }
                }

                fn visit_yul_case(&mut v, case: &'ast $($mut)? YulCase) {
                    v.visit_yul_block(& $($mut)? case.body);
                    match & $($mut)? case.value {
                        YulCaseValue::Default => (),
                        YulCaseValue::YulLiteral(literal) => v.visit_yul_literal(literal),
                    }
                }

                fn visit_yul_identifier(&mut _v, _ident: &'ast $($mut)? YulIdentifier) {
                    // nothing to do
                }

                fn visit_yul_literal(&mut _v, _literal: &'ast $($mut)? YulLiteral) {
                    // nothing to do
                }

                fn visit_yul_function_call(&mut v, call: &'ast $($mut)? YulFunctionCall) {
                    for arg in & $($mut)? call.arguments {
                        v.visit_yul_expression(arg);
                    }
                    v.visit_yul_identifier(& $($mut)? call.function_name);
                }

                fn visit_yul_typed_name(&mut _v, _name: &'ast $($mut)? YulTypedName) {
                    // nothing to do
                }

                fn visit_yul_expression(&mut v, expr: &'ast $($mut)? YulExpression) {
                    match expr {
                        YulExpression::YulFunctionCall(call) => v.visit_yul_function_call(call),
                        YulExpression::YulIdentifier(ident) => v.visit_yul_identifier(ident),
                        YulExpression::YulLiteral(literal) => v.visit_yul_literal(literal),
                    }
                }




                
                // Expressions

                fn visit_expression(&mut v, expr: &'ast $($mut)? Expression) {
                    match expr {
                        Expression::Assignment(assignment) => v.visit_assignment(assignment),
                        Expression::BinaryOperation(binary) => v.visit_binary_operation(binary),
                        Expression::Conditional(conditional) => v.visit_conditional(conditional),
                        Expression::ElementaryTypeNameExpression(elementary) => v.visit_elementary_type_name_expression(elementary),
                        Expression::FunctionCall(call) => v.visit_function_call(call),
                        Expression::FunctionCallOptions(options) => v.visit_function_call_options(options),
                        Expression::Identifier(ident) => v.visit_identifier(ident),
                        Expression::IndexAccess(index) => v.visit_index_access(index),
                        Expression::IndexRangeAccess(range) => v.visit_index_range_access(range),
                        Expression::Literal(literal) => v.visit_literal(literal),
                        Expression::MemberAccess(member) => v.visit_member_access(member),
                        Expression::NewExpression(new) => v.visit_new(new),
                        Expression::TupleExpression(tuple) => v.visit_tuple(tuple),
                        Expression::UnaryOperation(unary) => v.visit_unary_operation(unary),
                    }
                }

                fn visit_assignment(&mut v, assignment: &'ast $($mut)? Assignment) {
                    if let Some(types) = & $($mut)? assignment.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? assignment.left_hand_side);
                    v.visit_expression(& $($mut)? assignment.right_hand_side);
                    v.visit_type_descriptions(& $($mut)? assignment.type_descriptions);
                }

                fn visit_binary_operation(&mut v, binary: &'ast $($mut)? BinaryOperation) {
                    if let Some(types) = & $($mut)? binary.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_type_descriptions(& $($mut)? binary.common_type);
                    v.visit_expression(& $($mut)? binary.left_expression);
                    v.visit_expression(& $($mut)? binary.right_expression);
                    v.visit_type_descriptions(& $($mut)? binary.type_descriptions);
                }

                fn visit_conditional(&mut v, conditional: &'ast $($mut)? Conditional) {
                    if let Some(types) = & $($mut)? conditional.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? conditional.condition);
                    v.visit_expression(& $($mut)? conditional.true_expression);
                    v.visit_expression(& $($mut)? conditional.false_expression);
                    v.visit_type_descriptions(& $($mut)? conditional.type_descriptions);
                }

                fn visit_elementary_type_name_expression(&mut v, elementary: &'ast $($mut)? ElementaryTypeNameExpression) {
                    if let Some(types) = & $($mut)? elementary.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_type_descriptions(& $($mut)? elementary.type_descriptions);
                }

                fn visit_function_call(&mut v, call: &'ast $($mut)? FunctionCall) {
                    if let Some(types) = & $($mut)? call.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    for arg in & $($mut)? call.arguments {
                        v.visit_expression(arg);
                    }
                    v.visit_expression(& $($mut)? call.expression);
                    v.visit_type_descriptions(& $($mut)? call.type_descriptions);
                }

                fn visit_function_call_options(&mut v, options: &'ast $($mut)? FunctionCallOptions) {
                    if let Some(types) = & $($mut)? options.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? options.expression);
                    for option in & $($mut)? options.options {
                        v.visit_expression(option);
                    }
                    v.visit_type_descriptions(& $($mut)? options.type_descriptions);
                }

                fn visit_identifier(&mut v, ident: &'ast $($mut)? Identifier) {
                    if let Some(types) = & $($mut)? ident.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_type_descriptions(& $($mut)? ident.type_descriptions);
                }

                fn visit_index_access(&mut v, index: &'ast $($mut)? IndexAccess) {
                    if let Some(types) = & $($mut)? index.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? index.base_expression);
                    if let Some(expr) = & $($mut)? *index.index_expression {
                        v.visit_expression(expr);
                    }
                    v.visit_type_descriptions(& $($mut)? index.type_descriptions);
                }

                fn visit_index_range_access(&mut v, range: &'ast $($mut)? IndexRangeAccess) {
                    if let Some(types) = & $($mut)? range.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? range.base_expression);
                    if let Some(expr) = & $($mut)? *range.start_expression {
                        v.visit_expression(expr);
                    }
                    if let Some(expr) = & $($mut)? *range.end_expression {
                        v.visit_expression(expr);
                    }
                    v.visit_type_descriptions(& $($mut)? range.type_descriptions);
                }

                fn visit_literal(&mut v, literal: &'ast $($mut)? Literal) {
                    if let Some(types) = & $($mut)? literal.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_type_descriptions(& $($mut)? literal.type_descriptions);
                }

                fn visit_member_access(&mut v, member: &'ast $($mut)? MemberAccess) {
                    if let Some(types) = & $($mut)? member.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? member.expression);
                    v.visit_type_descriptions(& $($mut)? member.type_descriptions);
                }

                fn visit_new(&mut v, new: &'ast $($mut)? NewExpression) {
                    if let Some(types) = & $($mut)? new.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_type_descriptions(& $($mut)? new.type_descriptions);
                    v.visit_type_name(& $($mut)? new.type_name);
                }

                fn visit_tuple(&mut v, tuple: &'ast $($mut)? TupleExpression) {
                    if let Some(types) = & $($mut)? tuple.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    for component in & $($mut)? tuple.components {
                        if let Some(expr) = & $($mut)? *component {
                            v.visit_expression(expr);
                        }
                    }
                    v.visit_type_descriptions(& $($mut)? tuple.type_descriptions);
                }

                fn visit_unary_operation(&mut v, unary: &'ast $($mut)? UnaryOperation) {
                    if let Some(types) = & $($mut)? unary.argument_types {
                        for ty in types {
                            v.visit_type_descriptions(ty);
                        }
                    }
                    v.visit_expression(& $($mut)? unary.sub_expression);
                    v.visit_type_descriptions(& $($mut)? unary.type_descriptions);
                }
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
            }
        }
    };

    (
        @impl
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident<'ast> {$(
            $(#[$fn_attr:meta])*
            fn $fn_name:ident(&mut $v:ident $(, $arg_name:ident : $arg_ty:ty)*) { $($impl:tt)* }
        )*}
    ) => {
        $(#[$attr])*
        $vis trait $trait_name<'ast> {$(
            $(#[$fn_attr])*
            fn $fn_name(&mut self $(, $arg_name: $arg_ty)*) { $fn_name(self $(, $arg_name)*) }
        )*}

        $(
            $(#[$fn_attr])*
            pub fn $fn_name<'ast, V: ?Sized + $trait_name<'ast>>($v: &mut V $(, $arg_name: $arg_ty)*) {
                $($impl)*
            }
        )*
    };
}