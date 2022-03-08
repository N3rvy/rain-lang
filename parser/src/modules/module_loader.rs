use std::collections::HashMap;
use std::marker::PhantomData;
use common::errors::LangError;
use tokenizer::tokenizer::Tokenizer;
use crate::errors::{LOAD_MODULE_ERROR, UNIQUE_ID_ERROR};
use crate::modules::parser_module::{ParseModule, ParseModuleParser};
use crate::modules::module_importer::{ModuleIdentifier, ModuleImporter, ModuleUID};

pub struct ModuleLoader<Importer: ModuleImporter> {
    modules: HashMap<ModuleUID, ParseModule>,
    _marker: PhantomData<Importer>,
}

impl<Importer: ModuleImporter> ModuleLoader<Importer> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            _marker: PhantomData::default(),
        }
    }

    pub fn load_module(&mut self, id: &ModuleIdentifier) -> LoadModuleResult {
        let uid = match Importer::get_unique_identifier(id) {
            Some(uid) => uid,
            None => return LoadModuleResult::Err(LangError::new_parser(UNIQUE_ID_ERROR.to_string())),
        };

        if self.modules.contains_key(&uid) {
            return LoadModuleResult::AlreadyLoaded(uid)
        }

        let source = match Importer::load_module(id) {
            Some(source) => source,
            None => return LoadModuleResult::Err(LangError::new_parser(LOAD_MODULE_ERROR.to_string())),
        };

        let tokens = match Tokenizer::tokenize(&source) {
            Ok(tok) => tok,
            Err(err) => return LoadModuleResult::Err(err),
        };

        let module = ParseModuleParser::new(self)
            .parse(tokens);

        let module = match module {
            Ok(m) => m,
            Err(err) => return LoadModuleResult::Err(err),
        };

        self.modules.insert(uid.clone(), module);

        LoadModuleResult::Ok(uid)
    }

    pub fn modules(&self) -> &HashMap<ModuleUID, ParseModule> {
        &self.modules
    }

    pub fn modules_owned(self) -> HashMap<ModuleUID, ParseModule> {
        self.modules
    }
}

pub enum LoadModuleResult {
    Ok(ModuleUID),
    AlreadyLoaded(ModuleUID),
    NotFound,
    Err(LangError),
}