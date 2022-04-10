use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::TypeKind;
use common::module::{DefinitionModule, Module, ModuleUID};
use common::errors::LangError;
use common::module::ModuleIdentifier;
use tokenizer::tokenizer::Tokenizer;
use crate::errors::{LOAD_MODULE_ERROR, MODULE_NOT_FOUND};
use crate::modules::module_initializer::{ParsableModule, ModuleInitializer, DeclarationKind};
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_parser::ModuleParser;

#[derive(Clone)]
pub enum ModuleKind {
    Data(Arc<Module>),
    Definition(Arc<DefinitionModule>),
}

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
        -> Result<(ModuleUID, Vec<Arc<Module>>), LangError>
    {
        let tokens = Tokenizer::tokenize(&source)?;
        let parsable_module = ModuleInitializer::create(tokens)?;
        let context = self.create_context(&parsable_module, importer)?;
        let parser = ModuleParser::new(&context);

        // Return result vector
        let mut modules = Vec::new();

        // Loading all the dependencies
        for (uid, parsable_module) in &context.modules {
            let module = Arc::new(
                parser.parse_module(parsable_module, *uid, importer)?);

            modules.push(module.clone());

            self.modules
                .borrow_mut()
                .insert(*uid, ModuleKind::Data(module));
        }

        // Loading the main module
        let module = Arc::new(
            parser.parse_module(&parsable_module, uid, importer)?);

        modules.push(module.clone());

        self.modules
            .borrow_mut()
            .insert(uid, ModuleKind::Data(module));

        Ok((uid, modules))
    }

    pub fn load_def_module_with_source(&mut self, id: ModuleIdentifier, uid: ModuleUID, source: &String, _importer: &impl ModuleImporter)
        -> Result<Arc<DefinitionModule>, LangError>
    {
        let tokens = Tokenizer::tokenize(&source)?;
        let def_module = Arc::new(ModuleInitializer::create_definition(tokens, id)?);

        self.modules
            .borrow_mut()
            .insert(uid, ModuleKind::Definition(def_module.clone()));

        Ok(def_module)
    }

    pub fn load_module(&mut self, id: &ModuleIdentifier, importer: &impl ModuleImporter) -> Result<(ModuleUID, Vec<Arc<Module>>), LangError> {
        let uid = match importer.get_unique_identifier(id) {
            Some(uid) => uid,
            None => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string()))
        };

        // If cached then simply return
        if self.modules.borrow().contains_key(&uid) {
            return Ok((uid, Vec::new()))
        }

        let source = match importer.load_module(id) {
            Some(source) => source,
            None => return Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string()))
        };

        self.load_module_with_source(uid, &source, importer)
    }

    pub fn load_def_module(&mut self, id: &ModuleIdentifier, module_id: &ModuleIdentifier, importer: &impl ModuleImporter) -> Result<(ModuleUID, Option<Arc<DefinitionModule>>), LangError> {
        let module_uid = ModuleUID::from_string(module_id.0.clone());

        // If cached then simply return
        if self.modules.borrow().contains_key(&module_uid) {
            return Ok((module_uid, None))
        }

        let source = match importer.load_module(id) {
            Some(source) => source,
            None => return Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string()))
        };

        Ok((module_uid, Some(self.load_def_module_with_source(module_id.clone(), module_uid, &source, importer)?)))
    }

    fn create_context(&self, module: &ParsableModule, importer: &impl ModuleImporter) -> Result<ModuleLoaderContext, LangError> {
        let mut modules = Vec::new();

        self.add_imports(&mut modules, &module, importer)?;

        Ok(ModuleLoaderContext {
            module_loader: self,
            modules,
        })
    }

    fn add_imports(
        &self,
        vec: &mut Vec<(ModuleUID, ParsableModule)>,
        module: &ParsableModule,
        importer: &impl ModuleImporter,
    ) -> Result<(), LangError> {

        for import in &module.imports {
            let uid = match importer.get_unique_identifier(import) {
                Some(uid) => uid,
                None => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string())),
            };

            if self.modules.borrow().contains_key(&uid) {
                continue
            }

            let source = match importer.load_module(&import) {
                Some(source) => source,
                None => return Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string()))
            };
            let tokens = Tokenizer::tokenize(&source)?;

            let parsable_module = ModuleInitializer::create(tokens)?;

            self.add_imports(vec, &parsable_module, importer)?;

            vec.push((uid, parsable_module));
        }

        Ok(())
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

pub struct ModuleLoaderContext<'a> {
    module_loader: &'a ModuleLoader,
    modules: Vec<(ModuleUID, ParsableModule)>,
}

impl<'a> ModuleLoaderContext<'a> {
    pub fn get_definitions(&self, module_uid: ModuleUID) -> Vec<(String, TypeKind)> {
        let module = self.modules
            .iter()
            .find(|(uid, _)| *uid == module_uid);

        match module {
            Some((_, module)) => {
                module.declarations
                    .iter()
                    .map(|(name, decl)| {
                        let kind = match &decl.kind {
                            DeclarationKind::Variable(type_) => type_.clone(),
                            DeclarationKind::Function(_, type_) => TypeKind::Function(type_.clone()),
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
                            ModuleKind::Data(module) => {
                                Some(module.functions
                                    .iter()
                                    .map(|(name, func)| (name.clone(), TypeKind::Function(func.metadata.clone())))
                                    .chain(module.variables
                                        .iter()
                                        .map(|(name, var)| (name.clone(), var.metadata.clone())))
                                    .collect())
                            },
                            ModuleKind::Definition(module) => {
                                Some(module.functions
                                    .iter()
                                    .map(|(name, func_type)| (name.clone(), TypeKind::Function(func_type.clone())))
                                    .collect())
                            }
                        }
                    })
                    .unwrap_or_else(|| Vec::new())
            }
        }
    }
}