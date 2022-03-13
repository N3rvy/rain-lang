use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use common::ast::types::TypeKind;
use common::module::{Module, ModuleMetadata, ModuleUID};
use common::errors::LangError;
use common::module::ModuleIdentifier;
use tokenizer::tokenizer::Tokenizer;
use crate::errors::{LOAD_MODULE_ERROR, MODULE_NOT_FOUND, UNEXPECTED_ERROR};
use crate::modules::module_initializer::{ParsableModule, ModuleInitializer, Declaration, DeclarationKind};
use crate::modules::module_importer::ModuleImporter;
use crate::modules::module_parser::ModuleParser;

pub struct ModuleLoader {
    modules: RefCell<HashMap<ModuleUID, Arc<Module>>>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            modules: RefCell::new(HashMap::new()),
        }
    }

    pub fn load_module<Importer: ModuleImporter>(&mut self, id: &ModuleIdentifier) -> Result<ModuleUID, LangError> {
        let uid = match Importer::get_unique_identifier(id) {
            Some(uid) => uid,
            None => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string()))
        };

        // If cached then simply return
        if self.modules.borrow().contains_key(&uid) {
            return Ok(uid)
        }

        let source = match Importer::load_module(id) {
            Some(source) => source,
            None => return Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string()))
        };
        let tokens = Tokenizer::tokenize(&source)?;
        let parsable_module = ModuleInitializer::create(tokens)?;
        let context = self.create_context::<Importer>(&parsable_module);
        let parser = ModuleParser::new(&context);

        // Loading all the dependencies
        for (uid, parsable_module) in &context.modules {
            let module = parser.parse_module::<Importer>(parsable_module)?;
            self.modules.borrow_mut().insert(*uid, Arc::new(module));
        }

        // Loading the main module
        let module = parser.parse_module::<Importer>(&parsable_module)?;

        self.modules.borrow_mut().insert(uid, Arc::new(module));

        Ok(uid)
    }

    fn create_context<Importer: ModuleImporter>(&self, module: &ParsableModule) -> ModuleLoaderContext {
        let mut modules = Vec::new();

        self.add_imports::<Importer>(&mut modules, &module);

        ModuleLoaderContext {
            modules,
            module_loader: self,
        }
    }

    fn add_imports<Importer: ModuleImporter>(
        &self,
        vec: &mut Vec<(ModuleUID, ParsableModule)>,
        module: &ParsableModule
    ) -> Result<(), LangError> {

        for import in &module.imports {
            let uid = match Importer::get_unique_identifier(import) {
                Some(uid) => uid,
                None => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string())),
            };

            if self.modules.borrow().contains_key(&uid) {
                break
            }

            let source = match Importer::load_module(&import) {
                Some(source) => source,
                None => return Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string()))
            };
            let tokens = Tokenizer::tokenize(&source)?;

            let parsable_module = ModuleInitializer::create(tokens)?;

            self.add_imports::<Importer>(vec, &parsable_module);

            vec.push((uid, parsable_module));
        }

        Ok(())
    }

    pub fn get_module(&self, uid: ModuleUID) -> Option<Arc<Module>> {
        self.modules
            .borrow()
            .get(&uid)
            .cloned()
    }

    pub fn get_or_load_module<Importer: ModuleImporter>(&mut self, id: &ModuleIdentifier) -> Result<(ModuleUID, Arc<Module>), LangError> {
        let uid = self.load_module::<Importer>(id)?;

        match self.get_module(uid) {
            Some(module) => Ok((uid, module)),
            None => Err(LangError::new_parser(UNEXPECTED_ERROR.to_string()))
        }
    }
}

pub struct ModuleLoaderContext<'a> {
    module_loader: &'a ModuleLoader,
    modules: Vec<(ModuleUID, ParsableModule)>,
}

impl<'a> ModuleLoaderContext<'a> {
    pub fn get_metadata(&self, module: ModuleUID) -> Option<ModuleMetadata> {
        match self.modules
            .iter()
            .find(|(uid, _)| *uid == module)
        {
            Some((_, module)) => {
                Some(ModuleMetadata {
                    definitions: module.declarations
                        .iter()
                        .map(|(name, decl)| {
                            (
                                name.clone(),
                                match &decl.kind {
                                    DeclarationKind::Variable(tk) => tk.clone(),
                                    DeclarationKind::Function(_, ft) => TypeKind::Function(ft.clone()),
                                }
                            )
                        })
                        .collect(),
                })
            },
            None => todo!(),
        }
    }
}