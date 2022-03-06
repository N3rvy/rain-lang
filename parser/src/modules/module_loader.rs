use std::collections::HashMap;
use common::errors::LangError;
use tokenizer::iterator::Tokens;
use tokenizer::tokenizer::Tokenizer;
use crate::modules::loading_module::LoadingModule;
use crate::modules::module_importer::{Identifier, ModuleImporter, UniqueIdentifier};

pub struct ModuleLoader<Importer: ModuleImporter> {
    importer: Importer,
    modules: HashMap<UniqueIdentifier, LoadingModule>,
}

impl<Importer: ModuleImporter + Default> ModuleLoader<Importer> {
    pub fn new() -> Self {
        Self {
            importer: Importer::default(),
            modules: HashMap::new(),
        }
    }
}

impl<Importer: ModuleImporter> ModuleLoader<Importer> {
    pub fn with_importer(importer: Importer) -> Self {
        Self {
            importer,
            modules: HashMap::new(),
        }
    }

    pub fn load_module(&mut self, identifier: String) -> LoadModuleResult {
        let identifier = Identifier(identifier);
        let unique_identifier = match self.importer.get_unique_identifier(identifier) {
            Ok(id) => id,
            Err(err) => return LoadModuleResult::Err(err),
        };

        if !self.modules.contains_key(&unique_identifier) {
            return LoadModuleResult::AlreadyLoaded
        }

        let source = match self.importer.load_module(&unique_identifier) {
            Ok(source) => source,
            Err(err) => return LoadModuleResult::Err(err),
        };

        let tokens = match Tokenizer::tokenize(&source) {
            Ok(tok) => tok,
            Err(err) => return LoadModuleResult::Err(err),
        };
        let module = match LoadingModule::from_tokens(tokens) {
            Ok(m) => m,
            Err(err) => return LoadModuleResult::Err(err),
        };

        for dep in module.imports() {
            let result = self.load_module(dep.clone());
            match result {
                LoadModuleResult::Ok | LoadModuleResult::AlreadyLoaded => (),
                LoadModuleResult::NotFound | LoadModuleResult::Err(_) => return result
            }
        }

        self.modules.insert(unique_identifier, module);

        LoadModuleResult::Ok
    }
}

pub enum LoadModuleResult {
    Ok,
    AlreadyLoaded,
    NotFound,
    Err(LangError),
}