use core::module::EngineModule;

use crate::engine::WasmEngine;

pub struct WasmModule;

impl EngineModule for WasmModule {
    type Engine = WasmEngine;

    fn new(_engine: &mut Self::Engine, _id: &common::module::ModuleIdentifier, _importer: &impl core::parser::ModuleImporter) -> Result<Self, core::LangError> {
        todo!()
    }

    fn from_module(_engine: &mut Self::Engine, _module: std::sync::Arc<common::module::Module>) -> Result<Self, core::LangError> {
        todo!()
    }
}