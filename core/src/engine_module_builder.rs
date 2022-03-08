use std::collections::HashMap;
use std::sync::Arc;
use common::ast::ASTNode;
use common::ast::module::ASTModule;
use common::ast::types::{Function, FunctionType, TypeKind};
use common::errors::LangError;
use parser::modules::module_parser::{DeclarationKind, ParseModule};
use parser::modules::module_importer::ModuleUID;
use parser::parser::ParserScope;
use tokenizer::iterator::Tokens;
use crate::Engine;
use crate::errors::{UNEXPECTED_ERROR, WRONG_TYPE};
use crate::module::EngineModule;

pub struct EngineModuleBuilder<Eng: Engine> {
    modules: HashMap<ModuleUID, Eng::Module>,
}

impl<Eng: Engine> EngineModuleBuilder<Eng> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn load_module(&mut self, uid: ModuleUID, mut module: ParseModule) -> Result<(), LangError> {
        let scope = self.create_scope(&module);

        let mut functions = Vec::new();
        let mut variables = Vec::new();

        // Parsing every definition
        for (name, decl) in module.declarations {
            module.tokens.rollback(decl.body);

            match decl.kind {
                DeclarationKind::Variable(_) => {
                    let value = Self::parse_variable_value(&mut module.tokens, &scope.new_child())?;

                    variables.push((name, value));
                },
                DeclarationKind::Function(params, func_type) => {
                    let scope = scope.new_child();

                    let value = Self::parse_function_value(
                        &mut module.tokens,
                        &scope,
                        params,
                        func_type.clone())?;

                    if !scope.eval_type.borrow().is_compatible(func_type.1.as_ref()) {
                        return Err(LangError::new_parser(WRONG_TYPE.to_string()));
                    }

                    functions.push((name, value));
                },
            };
        }

        let ast_module = ASTModule::new(
            functions,
            variables,
        );

        let module = Eng::Module::new(self, ast_module)?;

        self.insert_module(uid, module);

        Ok(())
    }

    fn create_scope(&self, module: &ParseModule) -> ParserScope {
        let scope = ParserScope::new_root();

        // Declaring every type into the scope
        for (name, def) in &module.declarations {
            let type_kind = match &def.kind {
                DeclarationKind::Variable(t) => t.clone(),
                DeclarationKind::Function(_, ft) => TypeKind::Function(ft.clone()),
            };

            scope.declare(name.clone(), type_kind);
        }

        scope
    }

    fn parse_variable_value(tokens: &mut Tokens, scope: &ParserScope) -> Result<ASTNode, LangError> {
        scope.parse_statement(tokens)
    }

    fn parse_function_value(tokens: &mut Tokens, scope: &ParserScope, params: Vec<String>, func_type: FunctionType) -> Result<Arc<Function>, LangError> {
        if params.len() != func_type.0.len() {
            return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string()));
        }

        for i in 0..params.len() {
            scope.declare(params[i].clone(), func_type.0[i].clone());
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body, params))
    }

    pub fn insert_module(&mut self, uid: ModuleUID, module: Eng::Module) {
        self.modules.insert(uid, module);
    }

    pub fn get_module(&self, uid: ModuleUID) -> Option<&Eng::Module> {
        self.modules.get(&uid)
    }
}