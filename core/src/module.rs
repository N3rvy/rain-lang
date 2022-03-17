use std::sync::Arc;
use common::errors::LangError;
use common::module::{Module, ModuleIdentifier};
use parser::modules::module_importer::ModuleImporter;
use crate::{Engine, ExternalType, InternalFunction};


pub trait EngineModule : Sized {
    type Engine: Engine;

    fn new<Importer: ModuleImporter>(engine: &mut Self::Engine, id: &ModuleIdentifier) -> Result<Self, LangError>;
    fn from_module(engine: &mut Self::Engine, module: Arc<Module>) -> Result<Self, LangError>;
}