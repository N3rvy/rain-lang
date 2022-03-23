use std::sync::Arc;
use common::errors::LangError;
use common::module::{Module, ModuleIdentifier};
use parser::modules::module_importer::ModuleImporter;
use crate::Engine;


pub trait EngineModule : Sized {
    type Engine: Engine;

    fn new(engine: &mut Self::Engine, id: &ModuleIdentifier, importer: &impl ModuleImporter) -> Result<Self, LangError>;
    fn from_module(engine: &mut Self::Engine, module: Arc<Module>) -> Result<Self, LangError>;
}