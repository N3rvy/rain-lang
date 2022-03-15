use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::TypeKind;
use common::module::{Module, ModuleMetadata, ModuleUID};
use common::errors::LangError;
use common::module::ModuleIdentifier;
use tokenizer::tokenizer::Tokenizer;
use crate::errors::{LOAD_MODULE_ERROR, MODULE_NOT_FOUND};
use crate::modules::module_initializer::{ParsableModule, ModuleInitializer, DeclarationKind};
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

    pub fn load_module<Importer: ModuleImporter>(&mut self, id: &ModuleIdentifier) -> Result<(ModuleUID, Vec<Arc<Module>>), LangError> {
        let uid = match Importer::get_unique_identifier(id) {
            Some(uid) => uid,
            None => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string()))
        };

        // If cached then simply return
        if self.modules.borrow().contains_key(&uid) {
            return Ok((uid, Vec::new()))
        }

        let source = match Importer::load_module(id) {
            Some(source) => source,
            None => return Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string()))
        };
        let tokens = Tokenizer::tokenize(&source)?;
        let parsable_module = ModuleInitializer::create(tokens)?;
        let context = self.create_context::<Importer>(&parsable_module)?;
        let parser = ModuleParser::new(&context);

        // Return result vector
        let mut modules = Vec::new();

        // Loading all the dependencies
        for (uid, parsable_module) in &context.modules {
            let module = Arc::new(
                parser.parse_module::<Importer>(parsable_module, *uid)?);

            modules.push(module.clone());

            self.modules
                .borrow_mut()
                .insert(*uid, module);
        }

        // Loading the main module
        let module = Arc::new(
            parser.parse_module::<Importer>(&parsable_module, uid)?);

        modules.push(module.clone());

        self.modules
            .borrow_mut()
            .insert(uid, module);

        Ok((uid, modules))
    }

    fn create_context<Importer: ModuleImporter>(&self, module: &ParsableModule) -> Result<ModuleLoaderContext, LangError> {
        let mut modules = Vec::new();

        self.add_imports::<Importer>(&mut modules, &module)?;

        Ok(ModuleLoaderContext {
            modules,
        })
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

            self.add_imports::<Importer>(vec, &parsable_module)?;

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
}

pub struct ModuleLoaderContext {
    modules: Vec<(ModuleUID, ParsableModule)>,
}

impl ModuleLoaderContext {
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