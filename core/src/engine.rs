use std::sync::Arc;
use common::errors::LangError;
use common::module::{Module, ModuleUID};
use parser::modules::module_importer::ModuleImporter;
use parser::modules::module_loader::ModuleLoader;
use anyhow::Result;

use crate::module::EngineModule;
use crate::external_module::ExternalModule;


pub trait Engine
where
    Self: Sized,
{
    type Module: EngineModule<Engine = Self>;

    fn load_module(&mut self, identifier: impl Into<String>, importer: &impl ModuleImporter) -> Result<ModuleUID>;
    fn load_def_module(&mut self, import_identifier: impl Into<String>, module_id: impl Into<String>, importer: &impl ModuleImporter) -> Result<ModuleUID>;
    fn insert_module(&mut self, module: Arc<Module>) -> Result<()>;

    fn module_loader(&mut self) -> &mut ModuleLoader;

    fn new() -> Self;
}

pub trait EngineBuildSource : Engine {
    fn build_module_source(&self, uid: ModuleUID) -> Result<Vec<u8>, LangError>;
}

pub trait EngineExternalModule : Engine {
    type ExternalModule: ExternalModule<Engine = Self>;

    fn insert_external_module(&mut self, module: Self::ExternalModule);
}

pub trait EngineGetFunction<Args, R, Ret: InternalFunction<Args, R>> : Engine {
    fn get_function(&self, uid: ModuleUID, name: &str)
                    -> Option<Ret>;
}

pub trait InternalFunction<Args, R> {
    fn call(&self, args: Args) -> R;
}
