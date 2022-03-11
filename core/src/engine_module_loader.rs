use std::collections::HashMap;
use std::sync::Arc;
use common::ast::ASTNode;
use common::ast::module::ASTModule;
use common::ast::types::{Function, FunctionType, TypeKind};
use common::errors::LangError;
use common::module::{ModuleIdentifier, ModuleUID};
use parser::modules::module_parser::{DeclarationKind, ParseModule};
use parser::modules::module_importer::ModuleImporter;
use parser::modules::module_loader::{LoadModuleResult, ModuleLoader};
use parser::parser::ParserScope;
use tokenizer::iterator::Tokens;
use crate::Engine;
use crate::errors::{MODULE_NOT_FOUND, UNEXPECTED_ERROR, WRONG_TYPE};
use crate::module::EngineModule;

pub struct ModuleMetadata {
    pub declarations: Vec<(String, TypeKind)>,
}

pub struct EngineModuleLoader<Eng: Engine> {
    modules: HashMap<ModuleUID, Eng::Module>,
    metadata: HashMap<ModuleUID, ModuleMetadata>,
}

impl<Eng: Engine> EngineModuleLoader<Eng> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn load_module<Importer: ModuleImporter>(&mut self, identifier: &ModuleIdentifier) -> Result<ModuleUID, LangError> {
        let mut loader = ModuleLoader::<Importer>::new();

        let main_uid = loader.load_module(identifier);

        let main_uid = match main_uid {
            LoadModuleResult::Ok(uid) |
            LoadModuleResult::AlreadyLoaded(uid) => uid,
            LoadModuleResult::NotFound => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string())),
            LoadModuleResult::Err(err) => return Err(err),
        };

        self.metadata
            .extend(
                loader
                    .modules()
                    .iter()
                    .map(|(uid, module)| {
                        let declarations = module.declarations
                            .iter()
                            .map(|(name, decl)| {
                                let decl_kind = match & decl.kind {
                                    DeclarationKind::Variable(var_type) => var_type.clone(),
                                    DeclarationKind::Function(_, func_type) => TypeKind::Function(func_type.clone()),
                                };

                                (name.clone(), decl_kind)
                            })
                            .collect::<Vec<(String, TypeKind)>>();

                        (
                            *uid,
                            ModuleMetadata {
                                declarations,
                            }
                        )
                    })
            );

        for (uid, module) in loader.modules_owned() {
            self.build_module(uid, module)?;
        }

        Ok(main_uid)
    }

    pub fn build_module(&mut self, uid: ModuleUID, mut module: ParseModule) -> Result<(), LangError> {
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

        let ast_module = ASTModule {
            imports: module.imports,
            functions,
            variables,
        };

        let eng_module = Eng::Module::new(self, uid, ast_module)?;

        self.insert_module(uid, eng_module);

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

        for import in &module.imports {
            let metadata = self.metadata.get(import);
            let metadata = match metadata {
                Some(md) => md,
                None => return scope,
            };

            for (name, decl) in &metadata.declarations {
                scope.declare(name.clone(), decl.clone());
            }
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

    fn insert_module(&mut self, uid: ModuleUID, module: Eng::Module) {
        self.modules.insert(uid, module);
    }

    pub fn get_metadata(&self, uid: ModuleUID) -> Option<&ModuleMetadata> {
        self.metadata.get(&uid)
    }

    pub fn get_module(&self, uid: ModuleUID) -> Option<&Eng::Module> {
        self.modules.get(&uid)
    }
}