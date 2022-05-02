use core::{Engine, EngineBuildSource, parser::{ModuleLoader, ModuleImporter}, LangError};
use std::sync::Arc;
use common::{module::{Module, ModuleUID, ModuleIdentifier}, errors::BuildErrorKind};
use core::reexport::anyhow::Result;
use crate::module::WasmModule;
use crate::build::WasmBuilder;

pub struct WasmEngine {
    module_loader: ModuleLoader,
}

impl Engine for WasmEngine {
    type Module = WasmModule;

    fn load_module(&mut self, identifier: impl Into<String>, importer: &impl ModuleImporter) -> Result<ModuleUID> {
        let (module, _) = self
            .module_loader()
            .load_module(&ModuleIdentifier(identifier.into()), importer)?;

        Ok(module.uid)
    }

    fn insert_module(&mut self, _module: Arc<Module>) -> Result<()> {
        Ok(())
    }

    fn module_loader(&mut self) -> &mut ModuleLoader {
        &mut self.module_loader
    }

    fn new() -> Self {
        Self {
            module_loader: ModuleLoader::new(),
        }
    }
}

impl EngineBuildSource for WasmEngine {
    fn build_module_source(&self, uid: ModuleUID) -> Result<Vec<u8>, LangError> {
        let module = match self.module_loader.get_module(uid) {
            Some(module) => module,
            None => return Err(LangError::build(BuildErrorKind::UnexpectedError("build_module_souce: Module not found".to_string()))),
        };

        let core_module = match self.module_loader.get_module(ModuleUID::from_string("core".to_string())) {
            Some(module) => module,
            None => return Err(LangError::build(BuildErrorKind::UnexpectedError("build_module_souce: Core module not found".to_string()))),
        };

        let builder = WasmBuilder::new(&self.module_loader, module, core_module);
        builder.build()
    }
}