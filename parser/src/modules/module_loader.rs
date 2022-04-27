use anyhow::anyhow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::{ClassType, FunctionType, TypeKind};
use common::module::{DeclarationModule, Module, ModuleUID};
use common::errors::{LangError, LoadErrorKind, format_load, format_error};
use common::module::ModuleIdentifier;
use tokenizer::tokenizer::Tokenizer;
use crate::modules::module_initializer::{ParsableModule, ModuleInitializer, DeclarationKind};
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_parser::ModuleParser;

// TODO: Move this to the core crate

#[derive(Clone)]
pub enum ModuleKind {
    Definition(Arc<Module>),
    Declaration(Arc<DeclarationModule>),
}

/// This handles the loading and dependency loading of modules
pub struct ModuleLoader {
    modules: RefCell<HashMap<ModuleUID, ModuleKind>>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            modules: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert_module(&mut self, uid: ModuleUID, module: ModuleKind) {
        self.modules
            .borrow_mut()
            .insert(uid, module);
    }

    pub fn load_module_with_source(&mut self, uid: ModuleUID, source: &String, importer: &impl ModuleImporter)
        -> anyhow::Result<(ModuleUID, Vec<Arc<Module>>)>
    {
        let tokens = match Tokenizer::tokenize(&source) {
            Ok(tokens) => tokens,
            Err(err) => return Err(anyhow!(format_error(source, err))),
        };
        let parsable_module = match ModuleInitializer::initialize_module(tokens) {
            Ok(module) => module,
            Err(err) => return Err(anyhow!(format_error(source, err)))
        };
        let context = self.create_context(&parsable_module, importer)?;
        let parser = ModuleParser::new(&context);

        // Return result vector
        let mut modules = Vec::new();

        // Loading all the dependencies
        for (uid, parsable_module) in &context.modules {
            let module = match parser.parse_module(parsable_module, *uid, importer) {
                Ok(module) => module,
                Err(err) => return Err(anyhow!(format_error(source, err))),
            };
            let module = Arc::new(module);

            modules.push(module.clone());

            self.modules
                .borrow_mut()
                .insert(*uid, ModuleKind::Definition(module));
        }

        // Loading the main module
        let module = match parser.parse_module(&parsable_module, uid, importer) {
            Ok(module) => module,
            Err(err) => return Err(anyhow!(format_error(source, err))),
        };
        let module = Arc::new(module);

        modules.push(module.clone());

        self.modules
            .borrow_mut()
            .insert(uid, ModuleKind::Definition(module));

        Ok((uid, modules))
    }

    pub fn load_declaration_module_with_source(
        &mut self,
        id: ModuleIdentifier,
        uid: ModuleUID,
        source: &String,
        _importer: &impl ModuleImporter
    ) -> Result<Arc<DeclarationModule>, LangError> {
        let tokens = Tokenizer::tokenize(&source)?;
        let decl_module = Arc::new(ModuleInitializer::parse_declaration_module(tokens, id)?);

        self.modules
            .borrow_mut()
            .insert(uid, ModuleKind::Declaration(decl_module.clone()));

        Ok(decl_module)
    }

    pub fn load_module(&mut self, id: &ModuleIdentifier, importer: &impl ModuleImporter) -> anyhow::Result<(ModuleUID, Vec<Arc<Module>>)> {
        let uid = match importer.get_unique_identifier(id) {
            Some(uid) => uid,
            None => return Err(anyhow!(format_load(LoadErrorKind::ModuleNotFound(id.0.clone()))))
        };

        // If cached then simply return
        if self.modules.borrow().contains_key(&uid) {
            return Ok((uid, Vec::new()))
        }

        let source = match importer.load_module(id) {
            Some(source) => source,
            None => return Err(anyhow!(format_load(LoadErrorKind::LoadModuleError(id.0.clone()))))
        };

        self.load_module_with_source(uid, &source, importer)
    }

    pub fn load_declaration_module(
        &mut self,
        id: &ModuleIdentifier,
        module_id: &ModuleIdentifier,
        importer: &impl ModuleImporter
    ) -> anyhow::Result<(ModuleUID, Option<Arc<DeclarationModule>>)> {
        let module_uid = ModuleUID::from_string(module_id.0.clone());

        // If cached then simply return
        if self.modules.borrow().contains_key(&module_uid) {
            return Ok((module_uid, None))
        }

        let source = match importer.load_module(id) {
            Some(source) => source,
            None => return Err(anyhow!(format_load(LoadErrorKind::LoadModuleError(id.0.clone()))))
        };

        let res = self.load_declaration_module_with_source(module_id.clone(), module_uid, &source, importer);

        match res {
            Ok(res) => Ok((module_uid, Some(res))),
            Err(err) => Err(anyhow!(format_error(&source, err))),
        }
    }

    fn create_context(&self, module: &ParsableModule, importer: &impl ModuleImporter) -> anyhow::Result<ModuleLoaderContext> {
        let mut modules = Vec::new();

        self.load_imports(&mut modules, &module, importer)?;

        Ok(ModuleLoaderContext {
            module_loader: self,
            modules,
        })
    }

    fn load_imports(
        &self,
        vec: &mut Vec<(ModuleUID, ParsableModule)>,
        module: &ParsableModule,
        importer: &impl ModuleImporter,
    ) -> anyhow::Result<()> {

        for import in &module.imports {
            let uid = match importer.get_unique_identifier(import) {
                Some(uid) => uid,
                None => return Err(anyhow!(format_load(LoadErrorKind::ModuleNotFound(import.0.clone())))),
            };

            if self.modules.borrow().contains_key(&uid) {
                continue
            }

            let source = match importer.load_module(&import) {
                Some(source) => source,
                None => return Err(anyhow!(format_load(LoadErrorKind::LoadModuleError(import.0.clone()))))
            };
            let tokens = Tokenizer::tokenize(&source)?;

            let parsable_module = match ModuleInitializer::initialize_module(tokens) {
                Ok(module) => module,
                Err(err) => return Err(anyhow!(format_error(&source, err)))
            };

            self.load_imports(vec, &parsable_module, importer)?;

            vec.push((uid, parsable_module));
        }

        Ok(())
    }

    pub fn declaration_modules(&self) -> Vec<Arc<DeclarationModule>> {
        self.modules
            .borrow()
            .iter()
            .filter_map(|(_, module)| {
                match module {
                    ModuleKind::Declaration(decl_module) => Some(decl_module.clone()),
                    ModuleKind::Definition(_) => None,
                }
            })
            .collect()
    }

    pub fn modules(&self) -> Vec<ModuleKind> {
        self.modules
            .borrow()
            .iter()
            .map(|(_, module)| module.clone())
            .collect()
    }

    pub fn get_module(&self, uid: ModuleUID) -> Option<ModuleKind> {
        self.modules
            .borrow()
            .get(&uid)
            .cloned()
    }
}

pub enum GlobalDeclarationKind {
    Var(TypeKind),
    Func(FunctionType),
    Class(Arc<ClassType>),
}

/// This contains the `ParsableModule`s used for loading a module's dependencies
pub struct ModuleLoaderContext<'a> {
    module_loader: &'a ModuleLoader,
    modules: Vec<(ModuleUID, ParsableModule)>,
}

impl<'a> ModuleLoaderContext<'a> {
    pub fn get_declarations(&self, module_uid: ModuleUID) -> Vec<(String, GlobalDeclarationKind)> {
        let module = self.modules
            .iter()
            .find(|(uid, _)| *uid == module_uid);

        match module {
            Some((_, module)) => {
                module.declarations
                    .iter()
                    .map(|(name, decl)| {
                        let kind = match &decl.kind {
                            DeclarationKind::Variable(type_) => GlobalDeclarationKind::Var(type_.clone()),
                            DeclarationKind::Function(_, type_) => GlobalDeclarationKind::Func(type_.clone()),
                        };

                        (name.clone(), kind)
                    })
                    .collect()
            },
            None => {
                self.module_loader
                    .get_module(module_uid)
                    .and_then(|module| {
                        match module {
                            ModuleKind::Definition(module) => {
                                Some(module.functions
                                    .iter()
                                    .map(|(name, func)| (name.clone(), GlobalDeclarationKind::Func(func.metadata.clone())))
                                    .chain(module.variables
                                        .iter()
                                        .map(|(name, var)| (name.clone(), GlobalDeclarationKind::Var(var.metadata.clone()))))
                                    .collect())
                            },
                            ModuleKind::Declaration(module) => {
                                Some(module.functions
                                    .iter()
                                    .map(|(name, func_type)| (name.clone(), GlobalDeclarationKind::Func(func_type.clone())))
                                    .collect())
                            }
                        }
                    })
                    .unwrap_or_else(|| Vec::new())
            }
        }
    }
}