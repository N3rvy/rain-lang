use std::sync::Arc;
use common::ast::ASTNode;
use common::ast::types::{Function, FunctionType, TypeKind};
use common::errors::LangError;
use common::module::{FunctionDefinition, Module, ModuleUID, VariableDefinition};
use tokenizer::iterator::Tokens;
use crate::errors::WRONG_TYPE;
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_initializer::{DeclarationKind, ParsableModule};
use crate::modules::module_loader::ModuleLoaderContext;
use crate::parser::ParserScope;
use crate::parser_module_scope::ParserModuleScope;

pub struct ModuleParser<'a> {
    loader_context: &'a ModuleLoaderContext<'a>,
}

impl<'a> ModuleParser<'a> {
    pub fn new(loader_context: &'a ModuleLoaderContext) -> Self {
        Self {
            loader_context
        }
    }

    pub fn parse_module(&self, module: &ParsableModule, uid: ModuleUID, importer: &impl ModuleImporter) -> Result<Module, LangError> {
        let scope = self.create_scope(&module, uid, importer);

        let mut functions = Vec::new();
        let mut variables = Vec::new();

        // Parsing every definition
        for (name, decl) in &module.declarations {
            let mut tokens = module.tokens.new_clone(decl.body);

            match &decl.kind {
                DeclarationKind::Variable(type_) => {
                    let value = Self::parse_variable_value(&mut tokens, &scope.new_child())?;

                    variables.push((
                        name.clone(),
                        VariableDefinition {
                            data: value,
                            metadata: type_.clone(),
                        },
                    ));
                },
                DeclarationKind::Function(func_type) => {
                    let scope = scope.new_child();

                    let value = Self::parse_function_value(
                        &mut tokens,
                        &scope,
                        func_type.clone())?;

                    if !scope.eval_type.borrow().is_compatible(func_type.1.as_ref()) {
                        return Err(LangError::new_parser(WRONG_TYPE.to_string()));
                    }

                    functions.push((
                        name.clone(),
                        FunctionDefinition {
                            data: value,
                            metadata: func_type.clone(),
                        },
                    ));
                },
            };
        }

        let module = Module {
            uid,

            imports: Vec::new(),
            functions,
            variables,
        };

        Ok(module)
    }

    fn create_scope(&self, module: &ParsableModule, uid: ModuleUID, importer: &impl ModuleImporter) -> ParserModuleScope {
        let mut scope = ParserModuleScope::new(uid);

        // Declaring every type into the scope
        for (name, def) in &module.declarations {
            let type_kind = match &def.kind {
                DeclarationKind::Variable(t) => t.clone(),
                DeclarationKind::Function(ft) => TypeKind::Function(ft.clone()),
            };

            scope.declare(name.clone(), type_kind);
        }

        for import in &module.imports {
            let uid = match importer.get_unique_identifier(import) {
                Some(uid) => uid,
                None => continue,
            };

            let definitions = self.loader_context.get_definitions(uid);

            for (name, decl) in definitions {
                scope.declare_external(name, uid, decl);
            }
        }

        scope
    }

    fn parse_variable_value(tokens: &mut Tokens, scope: &ParserScope) -> Result<ASTNode, LangError> {
        scope.parse_statement(tokens)
    }

    fn parse_function_value(tokens: &mut Tokens, scope: &ParserScope, func_type: FunctionType) -> Result<Arc<Function>, LangError> {
        for (name, type_) in func_type.0 {
            scope.declare(name, type_);
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body))
    }

}