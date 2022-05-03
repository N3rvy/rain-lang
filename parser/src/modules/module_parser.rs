use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::{Class, Function, FunctionType, LiteralKind, ClassType, TypeKind};
use common::constants::CLASS_SELF_REFERENCE;
use common::errors::{BuildErrorKind, LangError, LoadErrorKind, ParserErrorKind};
use common::module::{ClassDefinition, FunctionDefinition, Module, ModuleFeature, ModuleUID, VariableDefinition};
use common::tokens::{Token, TokenKind};
use tokenizer::iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_loader::ModuleLoader;
use crate::modules::parsable_types::ParsableModule;
use crate::parser_scope::ParserScope;
use crate::parser_module_scope::{GlobalKind, ModuleParserScope};
use crate::utils::TokensExtensions;

pub struct ParsingModule {
    pub types: HashMap<String, Arc<ClassType>>,
    pub module: Arc<ParsableModule>,

    // Indicates whether the module is loaded or not (this means that all the types are already parsed)
    pub loaded: Cell<bool>,
}

/// This struct finalizes parsing for the `ParsableModule`s.
/// It's job is to go through every declaration inside a `ParsableModule`
/// and through parsing it converts it to a definition
pub struct ModuleParser<'a> {
    pub module_loader: &'a ModuleLoader,
    pub modules: HashMap<ModuleUID, ParsingModule>,
    pub parsable_modules: Vec<Arc<ParsableModule>>,
}

impl<'a> ModuleParser<'a> {
    pub fn new(module_loader: &'a ModuleLoader, modules: Vec<Arc<ParsableModule>>) -> Self {
        let mut parser = Self {
            module_loader,
            modules: HashMap::new(),
            parsable_modules: modules,
        };
        // Loading all the types (classes)
        // This only adds the types but they will not contain neither methods nor fields
        for module in &parser.parsable_modules {
            let mut types = HashMap::new();

            for (name, class) in &module.classes {
                let class_type = Arc::new(ClassType {
                    name: class.name.clone(),
                    module: class.module,
                    kind: class.kind.clone(),
                    fields: Default::default(),
                    methods: Default::default(),
                });
                types.insert(name.clone(), class_type);
            }

            parser.modules.insert(module.uid, ParsingModule {
                types,
                module: module.clone(),
                loaded: Cell::new(false),
            });
        }

        parser
    }

    pub fn parse_module(&self, uid: ModuleUID, importer: &impl ModuleImporter) -> Result<Module, LangError> {
        let module = match self.modules.get(&uid) {
            Some(module) => module,
            None => return Err(LangError::parser(
                &Token::new(TokenKind::NewLine, 0, 0),
                ParserErrorKind::UnexpectedError("parse_module: Module not found".to_string()))),
        };

        let module_scope = self.create_scope(module, uid, importer)?;

        let mut features = HashMap::new();

        for (name, var) in &module.module.variables {
            let metadata = module_scope.convert_parsable_type(&var.type_kind)?;

            let data = match var.body {
                Some(body) => {
                    let mut tokens = module.module.tokens.new_clone(body);
                    let token = &tokens.peek().unwrap();
                    let data = Self::parse_variable_value(&mut tokens)?;

                    let data_type = TypeKind::from(&data);

                    if !data_type.is_compatible(&metadata) {
                        return Err(LangError::wrong_type(&token, &metadata, &data_type));
                    }

                    Some(data)
                },
                None => None,
            };

            features.insert(
                name.clone(),
                ModuleFeature::Variable(VariableDefinition {
                    data,
                    metadata,
                })
            );
        }

        for (name, func) in &module.module.functions {

            let metadata = module_scope.convert_parsable_func_type(&func.func_type)?;

            let data = match func.body {
                Some(body) => {
                    let mut tokens = module.module.tokens.new_clone(body);

                    let scope = module_scope.new_child();

                    let token = &tokens.peek().unwrap();

                    let data = Self::parse_function_value(
                        &mut tokens,
                        &scope,
                        &func.params,
                        metadata.clone(),
                        None)?;

                    if !scope.eval_type.borrow().is_compatible(&metadata.1) {
                        return Err(LangError::wrong_type(&token, &metadata.1, &scope.eval_type.into_inner()));
                    }

                    Some(data)
                },
                None => None,
            };

            features.insert(
                name.clone(),
                ModuleFeature::Function(FunctionDefinition {
                    data,
                    metadata,
                })
            );
        }

        for (name, class) in &module.module.classes {
            let class_type = match module_scope.globals.get(name) {
                Some(GlobalKind::Class(_, metadata)) => metadata.clone(),
                _ => return Err(LangError::build(BuildErrorKind::UnexpectedError("parse_module: variable is not a class".to_string()))),
            };

            let mut methods = Vec::new();

            for (name, method) in &class.methods {
                let metadata = module_scope.convert_parsable_func_type(&method.func_type)?;

                let data = match method.body {
                    Some(body) => {
                        let mut tokens = module.module.tokens.new_clone(body);

                        let scope = module_scope.new_child();

                        let token = &tokens.peek().unwrap();

                        let data = Self::parse_function_value(
                            &mut tokens,
                            &scope,
                            &method.params,
                            metadata.clone(),
                            Some(class_type.clone()))?;

                        if !scope.eval_type.borrow().is_compatible(&metadata.1) {
                            return Err(LangError::wrong_type(&token, &metadata.1, &scope.eval_type.into_inner()));
                        }

                        Some(data)
                    },
                    None => None,
                };

                methods.push((
                    name.clone(),
                    FunctionDefinition {
                        data,
                        metadata: metadata.clone(),
                    }
                ));

                class_type.methods
                    .borrow_mut()
                    .push((name.clone(), metadata));
            }

            for (name, field) in &class.fields {
                class_type.fields
                    .borrow_mut()
                    .push((
                        name.clone(),
                        module_scope.convert_parsable_type(&field)?));
            }

            features.insert(
                name.clone(),
                ModuleFeature::Class(ClassDefinition {
                    data: Class::new(methods),
                    metadata: class_type,
                })
            );
        }

        let mut imports = Vec::new();
        for import in &module.module.imports {
            let uid = match importer.get_unique_identifier(import) {
                Some(uid) => uid,
                None => return Err(LangError::load(LoadErrorKind::LoadModuleError(import.0.clone()))),
            };

            imports.push(uid);
        }

        let module = Module {
            id: module.module.id.clone(),
            uid,

            imports,
            features,
        };

        Ok(module)
    }

    fn create_scope(&self, parsing_module: &ParsingModule, uid: ModuleUID, importer: &impl ModuleImporter) -> Result<ModuleParserScope, LangError> {
        let mut scope = ModuleParserScope::new(uid);

        let module = &parsing_module.module;

        // Adds all the types to the scope (they still don't contain anything)
        for (name, _) in &module.classes {
            let class_type = match parsing_module.types.get(name)
            {
                Some(class) => class.clone(),
                _ => return Err(LangError::build(BuildErrorKind::UnexpectedError("create_scope: variable is not a class".to_string()))),
            };

            scope.declare_class(name.clone(), class_type);
        }

        for import in &module.imports {
            let uid = match importer.get_unique_identifier(import) {
                Some(uid) => uid,
                None => return Err(LangError::load(LoadErrorKind::LoadModuleError(import.0.clone()))),
            };

            let parsing_module = match self.modules.get(&uid) {
                Some(module) => module,
                None => return Err(LangError::build(BuildErrorKind::UnexpectedError("create_scope: could not found parsing module".to_string()))),
            };

            for (name, _) in &parsing_module.module.classes {
                let class = parsing_module.types.get(name).unwrap();

                scope.declare_external_class(name.clone(), uid, class.clone());
            }
        }

        // If the module is not loaded the load all the classes in it
        Self::load_parsing_module(&mut scope, parsing_module)?;

        // Declaring every type into the scope
        for (name, var) in &module.variables {
            scope.declare_var(name.clone(), scope.convert_parsable_type(&var.type_kind)?);
        }

        for (name, func) in &module.functions {
            scope.declare_func(name.clone(), scope.convert_parsable_func_type(&func.func_type)?);
        }

        for import in &module.imports {
            let uid = match importer.get_unique_identifier(import) {
                Some(uid) => uid,
                None => return Err(LangError::load(LoadErrorKind::LoadModuleError(import.0.clone()))),
            };

            let parsing_module = match self.modules.get(&uid) {
                Some(module) => module,
                None => return Err(LangError::build(BuildErrorKind::UnexpectedError("create_scope: could not found parsing module".to_string()))),
            };

            // If the module is not loaded the load all the classes in it
            Self::load_parsing_module(&mut scope, parsing_module)?;

            for (name, var) in &parsing_module.module.variables {
                scope.declare_external_var(name.clone(), uid, scope.convert_parsable_type(&var.type_kind)?);
            }

            for (name, func) in &parsing_module.module.functions {
                scope.declare_external_func(name.clone(), uid, scope.convert_parsable_func_type(&func.func_type)?);
            }
        }

        Ok(scope)
    }

    fn load_parsing_module(scope: &mut ModuleParserScope, parsing_module: &ParsingModule) -> Result<(), LangError> {
        if !parsing_module.loaded.get() {
            // This adds all the values
            for (name, parsable_class) in &parsing_module.module.classes {
                let class = scope.get_class(name)?;

                let mut methods = class.methods.borrow_mut();
                let mut fields = class.fields.borrow_mut();

                for (name, method) in &parsable_class.methods {
                    let func = scope.convert_parsable_func_type(&method.func_type)?;
                    methods.push((name.clone(), func));
                }

                for (name, field) in &parsable_class.fields {
                    let field = scope.convert_parsable_type(&field)?;
                    fields.push((name.clone(), field));
                }
            }

            parsing_module.loaded.set(true);
        }
        Ok(())
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
        method: Option<Arc<ClassType>>,
    ) -> Result<Arc<Function>, LangError> {
        if params.len() != func_type.0.len() {
            return Err(
                LangError::parser(
                    &tokens.peek().unwrap(),
                    ParserErrorKind::UnexpectedError(
                        "parse_function_value: different params lenghts".to_string())));
        }

        if let Some(ref method) = method {
            scope.declare(CLASS_SELF_REFERENCE.to_string(), TypeKind::Class(method.clone()));
        }

        for i in 0..params.len() {
            scope.declare(params[i].clone(), func_type.0[i].clone());
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body, params.clone(), method))
    }
}
