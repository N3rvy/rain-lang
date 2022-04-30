use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::{Class, Function, FunctionType, LiteralKind, ClassType, TypeKind};
use common::constants::DECLARATION_IMPORT_PREFIX;
use common::errors::{LangError, ParserErrorKind};
use common::module::{ClassDefinition, FunctionDefinition, Module, ModuleFeature, ModuleUID, VariableDefinition};
use common::tokens::{Token, TokenKind};
use tokenizer::iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_loader::{GlobalDeclarationKind, ModuleLoaderContext};
use common::ast::parsing_types::{ParsableFunctionType, ParsableType};
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
        let module_scope = self.create_scope(&module, uid, importer);

        let mut features = HashMap::new();

        for (name, var) in &module.variables {

            let metadata = Self::convert_parsable_type(&module_scope, &var.type_kind)?;

            let data = match var.body {
                Some(body) => {
                    let mut tokens = module.tokens.new_clone(body);
                    let token = &tokens.peek().unwrap();
                    let data = Self::parse_variable_value(&mut tokens)?;

                    let data_type = TypeKind::from(&data);

                    if data_type.is_compatible(&metadata) {
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

            let metadata = Self::convert_parsable_func_type(&module_scope, &func.func_type)?;

            let data = match func.body {
                Some(body) => {
                    let mut tokens = module.tokens.new_clone(body);

                    let scope = module_scope.new_child();

                    let token = &tokens.peek().unwrap();

                    let data = Self::parse_function_value(
                        &mut tokens,
                        &scope,
                        &func.params,
                        Self::convert_parsable_func_type(&module_scope, &func.func_type)?,
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

            for (name, method) in class.methods {
                let metadata = Self::convert_parsable_func_type(&module_scope, &method.func_type)?;

                let data = match method.body {
                    Some(body) => {
                        let mut tokens = module.tokens.new_clone(body);

                        let scope = module_scope.new_child();

                        let token = &tokens.peek().unwrap();

                        let data = Self::parse_function_value(
                            &mut tokens,
                            &scope,
                            &method.params,
                            Self::convert_parsable_func_type(&module_scope, &method.func_type)?,
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
                    Self::convert_parsable_type(&module_scope, &field)?,
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
                kind: class.class_type.kind.clone(),
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

    fn convert_parsable_func_type(scope: &ParserModuleScope, func_type: &ParsableFunctionType) -> Result<FunctionType, LangError> {
        let mut params = Vec::new();

        for param in &func_type.0 {
            params.push(Self::convert_type(scope, param.clone())?);
        }

        Ok(FunctionType(params, Box::new(Self::convert_type(scope, func_type.1.clone())?)))
    }

    fn convert_parsable_type(scope: &ParserModuleScope, type_: &ParsableType) -> Result<TypeKind, LangError> {
        Ok(match type_ {
            ParsableType::Unknown => TypeKind::Unknown,
            ParsableType::Nothing => TypeKind::Nothing,
            ParsableType::Int => TypeKind::Int,
            ParsableType::Float => TypeKind::Float,
            ParsableType::Bool => TypeKind::Bool,
            ParsableType::String => TypeKind::String,
            ParsableType::Vector(type_) => TypeKind::Vector(Box::new(Self::convert_parsable_type(type_.as_ref())?)),
            ParsableType::Function((params, return_type)) => {
                let mut params_types = Vec::new();

                for parm in params {
                    params_types.push(Self::convert_parsable_type(scope, parm.as_ref())?);
                }

                TypeKind::Func(
                    Box::new(Self::convert_type(scope, return_type)?)
                )
            },
            ParsableType::Custom(name) => {
                // TODO: This need a token position in case of error

                let uid = ModuleUID::from_string(name.clone());

                match scope.get_declaration(uid, name) {
                    Some(declaration) => match declaration {
                        GlobalDeclarationKind::Var(type_) => type_.clone(),
                        _ => return Err(LangError::parser(
                            &Token::new(TokenKind::Symbol(name.clone()), 0, 0),
                            ParserErrorKind::UnexpectedError(
                                "convert_parsable_type: custom type is not a variable".to_string()))),
                    },
                    None => return Err(
                        LangError::parser(
                            &Token::new(TokenKind::Symbol(name.clone()), 0, 0),
                            ParserErrorKind::VarNotFound)),
                }
            },
        })
    }

}
