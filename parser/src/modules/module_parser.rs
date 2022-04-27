use std::sync::Arc;
use common::ast::types::{Class, Function, FunctionType, LiteralKind, ClassType, TypeKind};
use common::constants::DECLARATION_IMPORT_PREFIX;
use common::errors::{LangError, ParserErrorKind};
use common::module::{ClassDefinition, FunctionDefinition, Module, ModuleUID, VariableDefinition};
use common::tokens::TokenKind;
use tokenizer::iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_initializer::ParsableModule;
use crate::modules::module_loader::{GlobalDeclarationKind, ModuleLoaderContext};
use crate::parser_scope::ParserScope;
use crate::parser_module_scope::ParserModuleScope;
use crate::utils::TokensExtensions;

/// This struct finalizes parsing for the `ParsableModule`s.
/// It's job is to go through every declaration inside a `ParsableModule`
/// and through parsing it converts it to a definition
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

        let mut variables = Vec::new();
        for (name, var) in &module.variables {
            let mut tokens = module.tokens.new_clone(var.body);

            let token = &tokens.peek().unwrap();
            let value = Self::parse_variable_value(&mut tokens)?;
            let value_type = TypeKind::from(value.clone());

            if !value_type.is_compatible(&var.type_kind) {
                return Err(LangError::wrong_type(&token, &var.type_kind, &value_type));
            }

            variables.push((
                name.clone(),
                VariableDefinition {
                    data: value,
                    metadata: var.type_kind.clone(),
                },
            ));
        }

        let mut functions = Vec::new();
        for (name, func) in &module.functions {
            let mut tokens = module.tokens.new_clone(func.body);

            let scope = scope.new_child();

            let token = &tokens.peek().unwrap();

            let value = Self::parse_function_value(
                &mut tokens,
                &scope,
                &func.params,
                func.func_type.clone(),
                false)?;

            if !scope.eval_type.borrow().is_compatible(func.func_type.1.as_ref()) {
                return Err(LangError::wrong_type(&token, &scope.eval_type.borrow(), &func.func_type.1));
            }

            functions.push((
                name.clone(),
                FunctionDefinition {
                    data: value,
                    metadata: func.func_type.clone(),
                }));
        }

        let mut classes = Vec::new();

        for (name, class) in &module.classes {
        let class_type = Arc::new(ClassType {
        name: name.clone(),
        module: uid,
        fields: class.class_type.fields.clone(),
        methods: vec![],
        });

        let scope = scope.new_child();

            let mut functions = Vec::new();
            let mut function_types = Vec::new();

            for (name, func) in &class.functions {
                let mut tokens = module.tokens.new_clone(func.body);

                let mut params = func.params.clone();
                let mut type_ = func.func_type.clone();

                params.insert(0, "self".to_string());
                type_.0.insert(0, TypeKind::Object(class_type.clone()));

                let func = Self::parse_function_value(&mut tokens, &scope, &params, type_.clone(), true)?;

                functions.push((
                    name.clone(),
                    func,
                ));

                function_types.push((
                    name.clone(),
                    type_.clone(),
                ));
            }

            unsafe {
                // This should not be done but... IDC, it is safe (the last words before the disaster...)
                // TODO: This also causes recursion so maybe change in the future (just maybe...)
                let method_ptr = &class_type.methods as *const Vec<(String, FunctionType)> as *mut Vec<(String, FunctionType)>;
                *method_ptr = function_types;
            }

            classes.push((
                name.clone(),
                ClassDefinition {
                    data: Class::new(functions),
                    metadata: class_type,
                }
            ))
        }

        let module = Module {
            uid,

            imports: Vec::new(),
            functions,
            variables,
            classes,
        };

        Ok(module)
    }

    fn create_scope(&self, module: &ParsableModule, uid: ModuleUID, importer: &impl ModuleImporter) -> ParserModuleScope {
        let mut scope = ParserModuleScope::new(uid);

        // Declaring every type into the scope
        for (name, var) in &module.variables {
            scope.declare_var(name.clone(), var.type_kind.clone());
        }

        for (name, func) in &module.functions {
            scope.declare_func(name.clone(), func.func_type.clone());
        }

        for (name, class) in &module.classes {
            let methods = class.functions
                .iter()
                .map(|(name, func)| (name.clone(), func.func_type.clone()))
                .collect();

            let class_type = Arc::new(ClassType {
                methods,
                name: name.clone(),
                module: uid,
                fields: class.class_type.fields.clone(),
            });

            scope.declare_class(name.clone(), class_type);
        }

        for import in &module.imports {
            // TODO: This is horrible please fix
            let uid = if import.0.starts_with(DECLARATION_IMPORT_PREFIX) {
                ModuleUID::from_string(import.0.clone())
            } else {
                match importer.get_unique_identifier(import) {
                    Some(uid) => uid,
                    None => continue,
                }
            };

            let definitions = self.loader_context.get_declarations(uid);

            for (name, def) in definitions {
                match def {
                    GlobalDeclarationKind::Var(type_) => scope.declare_external_var(name.clone(), uid, type_),
                    GlobalDeclarationKind::Func(type_) => scope.declare_external_func(name.clone(), uid, type_),
                    GlobalDeclarationKind::Class(type_) => scope.declare_external_class(name.clone(), uid, type_),
                }
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

    fn parse_function_value(
        tokens: &mut Tokens,
        scope: &ParserScope,
        params: &Vec<String>,
        func_type: FunctionType,
        is_method: bool
    ) -> Result<Arc<Function>, LangError> {
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

        Ok(Function::new(body, params.clone(), is_method))
    }

}
