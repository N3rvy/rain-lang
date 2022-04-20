use std::sync::Arc;
use common::ast::types::{Function, FunctionType, LiteralKind, TypeKind};
use common::errors::{LangError, ParserErrorKind};
use common::module::{FunctionDefinition, Module, ModuleUID, VariableDefinition};
use common::tokens::TokenKind;
use tokenizer::iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_initializer::{DeclarationKind, ParsableModule};
use crate::modules::module_loader::ModuleLoaderContext;
use crate::parser::ParserScope;
use crate::parser_module_scope::ParserModuleScope;
use crate::utils::TokensExtensions;

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
                    let token = &tokens.peek().unwrap();
                    let value = Self::parse_variable_value(&mut tokens)?;
                    let value_type = TypeKind::from(value.clone());

                    if !value_type.is_compatible(type_) {
                        return Err(LangError::wrong_type(&token, type_, &value_type));
                    }

                    variables.push((
                        name.clone(),
                        VariableDefinition {
                            data: value,
                            metadata: type_.clone(),
                        },
                    ));
                },
                DeclarationKind::Function(params, func_type) => {
                    let scope = scope.new_child();

                    let token = &tokens.peek().unwrap();

                    let value = Self::parse_function_value(
                        &mut tokens,
                        &scope,
                        params,
                        func_type.clone())?;

                    if !scope.eval_type.borrow().is_compatible(func_type.1.as_ref()) {
                        return Err(LangError::wrong_type(&token, &scope.eval_type.borrow(), &func_type.1));
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
                DeclarationKind::Function(_, ft) => TypeKind::Function(ft.clone()),
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

    fn parse_variable_value(tokens: &mut Tokens) -> Result<LiteralKind, LangError> {
        let token = tokens.pop_err()?;

        match token.kind {
            TokenKind::Literal(kind) => {
                Ok(kind.clone())
            },
            _ => Err(LangError::new_parser_unexpected_token(&token)),
        }
    }

    fn parse_function_value(tokens: &mut Tokens, scope: &ParserScope, params: &Vec<String>, func_type: FunctionType) -> Result<Arc<Function>, LangError> {
        if params.len() != func_type.0.len() {
            return Err(
                LangError::parser(
                    &tokens.peek().unwrap(),
                    ParserErrorKind::UnexpectedError(
                        "parse_function_value: different params lenghts".to_string())));
        }

        for i in 0..params.len() {
            scope.declare(params[i].clone(), func_type.0[i].clone());
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body, params.clone()))
    }

}