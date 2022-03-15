use std::sync::Arc;
use common::ast::ASTNode;
use common::ast::types::{Function, FunctionType, TypeKind};
use common::errors::LangError;
use common::module::{Module, ModuleMetadata, ModuleUID};
use tokenizer::iterator::Tokens;
use crate::errors::{UNEXPECTED_ERROR, WRONG_TYPE};
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_initializer::{DeclarationKind, ParsableModule};
use crate::modules::module_loader::ModuleLoaderContext;
use crate::parser::ParserScope;
use crate::parser_module_scope::ParserModuleScope;

pub struct ModuleParser<'a> {
    loader_context: &'a ModuleLoaderContext,
}

impl<'a> ModuleParser<'a> {
    pub fn new(loader_context: &'a ModuleLoaderContext) -> Self {
        Self {
            loader_context
        }
    }

    pub fn parse_module<Importer: ModuleImporter>(&self, module: &ParsableModule, uid: ModuleUID) -> Result<Module, LangError> {
        let scope = self.create_scope::<Importer>(&module, uid);

        let mut functions = Vec::new();
        let mut variables = Vec::new();

        // Parsing every definition
        for (name, decl) in &module.declarations {
            let mut tokens = module.tokens.new_clone(decl.body);

            match &decl.kind {
                DeclarationKind::Variable(_) => {
                    let value = Self::parse_variable_value(&mut tokens, &scope.new_child())?;

                    variables.push((name.clone(), value));
                },
                DeclarationKind::Function(params, func_type) => {
                    let scope = scope.new_child();

                    let value = Self::parse_function_value(
                        &mut tokens,
                        &scope,
                        params,
                        func_type.clone())?;

                    if !scope.eval_type.borrow().is_compatible(func_type.1.as_ref()) {
                        return Err(LangError::new_parser(WRONG_TYPE.to_string()));
                    }

                    functions.push((name.clone(), value));
                },
            };
        }

        let module = Module {
            uid,
            imports: vec![],
            metadata: ModuleMetadata { definitions: vec![] },

            functions,
            variables,
        };

        Ok(module)
    }

    fn create_scope<Importer: ModuleImporter>(&self, module: &ParsableModule, uid: ModuleUID) -> ParserModuleScope {
        let mut scope = ParserModuleScope::new(uid);

        // Declaring every type into the scope
        for (name, def) in &module.declarations {
            let type_kind = match &def.kind {
                DeclarationKind::Variable(t) => t.clone(),
                DeclarationKind::Function(_, ft) => TypeKind::Function(ft.clone()),
            };

            scope.declare(name.clone(), type_kind);
        }

        for import in &module.imports {
            let uid = match Importer::get_unique_identifier(import) {
                Some(uid) => uid,
                None => continue,
            };

            let metadata = match self.loader_context.get_metadata(uid) {
                Some(uid) => uid,
                None => continue,
            };

            for (name, decl) in &metadata.definitions {
                scope.declare_external(name.clone(), uid, decl.clone());
            }
        }

        scope
    }

    fn parse_variable_value(tokens: &mut Tokens, scope: &ParserScope) -> Result<ASTNode, LangError> {
        scope.parse_statement(tokens)
    }

    fn parse_function_value(tokens: &mut Tokens, scope: &ParserScope, params: &Vec<String>, func_type: FunctionType) -> Result<Arc<Function>, LangError> {
        if params.len() != func_type.0.len() {
            return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string()));
        }

        for i in 0..params.len() {
            scope.declare(params[i].clone(), func_type.0[i].clone());
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body, params.clone()))
    }

}