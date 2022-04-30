use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::{Class, Function, FunctionType, LiteralKind, ClassType, TypeKind};
use common::constants::DECLARATION_IMPORT_PREFIX;
use common::errors::{LangError, ParserErrorKind};
use common::module::{ClassDefinition, FunctionDefinition, Module, ModuleFeature, ModuleUID, VariableDefinition};
use common::tokens::TokenKind;
use tokenizer::iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_loader::{GlobalDeclarationKind, ModuleLoaderContext};
use crate::modules::parsable_types::ParsableModule;
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
        let module_scope = self.create_scope(&module, uid, importer)?;

        let mut features = HashMap::new();

        for (name, var) in &module.variables {
            let metadata = module_scope.convert_parsable_type(&var.type_kind)?;

            let data = match var.body {
                Some(body) => {
                    let mut tokens = module.tokens.new_clone(body);
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

        for (name, func) in &module.functions {

            let metadata = module_scope.convert_parsable_func_type(&func.func_type)?;

            let data = match func.body {
                Some(body) => {
                    let mut tokens = module.tokens.new_clone(body);

                    let scope = module_scope.new_child();

                    let token = &tokens.peek().unwrap();

                    let data = Self::parse_function_value(
                        &mut tokens,
                        &scope,
                        &func.params,
                        module_scope.convert_parsable_func_type(&func.func_type)?,
                        false)?;

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

        for (name, class) in &module.classes {
            let mut methods = Vec::new();
            let mut method_types = Vec::new();

            for (name, method) in &class.methods {
                let metadata = module_scope.convert_parsable_func_type(&method.func_type)?;

                let data = match method.body {
                    Some(body) => {
                        let mut tokens = module.tokens.new_clone(body);

                        let scope = module_scope.new_child();

                        let token = &tokens.peek().unwrap();

                        let data = Self::parse_function_value(
                            &mut tokens,
                            &scope,
                            &method.params,
                            module_scope.convert_parsable_func_type(&method.func_type)?,
                            false)?;

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

                method_types.push((
                    name.clone(),
                    metadata,
                ));
            }

            let mut fields = Vec::new();

            for (name, field) in &class.fields {
                fields.push((
                    name.clone(),
                    module_scope.convert_parsable_type(&field)?,
                ));
            }

            let class_type = Arc::new(ClassType {
                name: name.clone(),
                module: uid,
                kind: class.kind.clone(),
                fields,
                methods: method_types,
            });

            features.insert(
                name.clone(),
                ModuleFeature::Class(ClassDefinition {
                    data: Class::new(methods),
                    metadata: class_type,
                })
            );
        }

        let module = Module {
            id: module.id.clone(),
            uid,

            imports: Vec::new(),
            features,
        };

        Ok(module)
    }

    fn create_scope(&self, module: &ParsableModule, uid: ModuleUID, importer: &impl ModuleImporter) -> Result<ParserModuleScope, LangError> {
        let mut scope = ParserModuleScope::new(uid);

        for (name, class) in &module.classes {
            let mut methods = Vec::new();
            for (name, func) in &class.methods {
                methods.push((
                    name.clone(),
                    scope.convert_parsable_func_type(&func.func_type)?,
                ));
            }

            let mut fields = Vec::new();
            for (name, field) in &class.fields {
                fields.push((
                    name.clone(),
                    scope.convert_parsable_type(&field)?,
                ));
            }

            let class_type = Arc::new(ClassType {
                name: name.clone(),
                kind: class.kind.clone(),
                module: uid,
                fields,
                methods,
            });

            scope.declare_class(name.clone(), class_type);
        }

        // Declaring every type into the scope
        for (name, var) in &module.variables {
            scope.declare_var(name.clone(), scope.convert_parsable_type(&var.type_kind)?);
        }

        for (name, func) in &module.functions {
            scope.declare_func(name.clone(), scope.convert_parsable_func_type(&func.func_type)?);
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
                    GlobalDeclarationKind::Var(type_)=> scope.declare_external_var(
                        name.clone(),
                        uid,
                        scope.convert_parsable_type(&type_)?),
                    GlobalDeclarationKind::Func(type_) => scope.declare_external_func(
                        name.clone(),

                        uid,
                        scope.convert_parsable_func_type(&type_)?),
                    GlobalDeclarationKind::Class(type_) => scope.declare_external_class(name.clone(), uid, type_),
                }
            }
        }

        Ok(scope)
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
