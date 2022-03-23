use core::{Engine, EngineBuildSource, parser::{ModuleLoader, ModuleImporter}, LangError};
use std::sync::Arc;
use common::module::{Module, ModuleUID, ModuleIdentifier, ModuleMetadata};
use crate::{module::WasmModule, external_module::WasmExternalModule, errors::COULD_NOT_FIND_MODULE};

pub struct WasmEngine {
    module_loader: ModuleLoader,
}

impl Engine for WasmEngine {
    type Module = WasmModule;
    type ExternalModule = WasmExternalModule;

    fn load_module(&mut self, identifier: impl Into<String>, importer: &impl ModuleImporter) -> Result<ModuleUID, LangError> {
        let uid = match importer.get_unique_identifier(&ModuleIdentifier(identifier.into())) {
            Some(uid) => uid,
            None => return Err(LangError::new_runtime(COULD_NOT_FIND_MODULE.to_string()))
        };

        Ok(uid)
    }

    fn insert_module(&mut self, _module: Arc<Module>) -> Result<(), LangError> {
        Ok(())
    }

    fn module_loader(&mut self) -> &mut ModuleLoader {
        &mut self.module_loader
    }

    fn insert_external_module(&mut self, module: Self::ExternalModule) {
        self.module_loader
            .insert_module(module.uid, Module {
                uid: module.uid,
                metadata: ModuleMetadata {
                    definitions: module.definitions,
                },
                imports: Vec::new(),
                functions: Vec::new(),
                variables: Vec::new(),
            });
    }

    fn new() -> Self {
        Self {
            module_loader: ModuleLoader::new(),
        }
    }
}

impl EngineBuildSource for WasmEngine {
    fn build_source(&self) -> Result<Vec<u8>, LangError> {
        todo!();
    }
}