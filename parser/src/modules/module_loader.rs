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

    pub fn insert_module(&mut self, uid: ModuleUID, module: Module) {
        self.modules
            .borrow_mut()
            .insert(uid, Arc::new(module));
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
                .insert(*uid, module);
        }

        // Loading the main module
        let module = Arc::new(
            parser.parse_module(&parsable_module, uid, importer)?);

        modules.push(module.clone());

        self.modules
            .borrow_mut()
            .insert(uid, module);

        Ok((uid, modules))
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

    pub fn get_module(&self, uid: ModuleUID) -> Option<Arc<Module>> {
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
            None => self.module_loader
                .get_module(module)
                .and_then(|module|
                    Some(module.metadata.clone())),
        }
    }
}