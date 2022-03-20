use std::sync::Arc;
use common::errors::LangError;
use common::module::{Module, ModuleUID};
use parser::modules::module_importer::ModuleImporter;
use parser::modules::module_loader::ModuleLoader;

use crate::module::EngineModule;
use crate::external_module::ExternalModule;


pub trait Engine
where
    Self: Sized,
{
    type Module: EngineModule<Engine = Self>;
    type ExternalModule: ExternalModule<Engine = Self>;

    fn load_module(&mut self, identifier: impl Into<String>, importer: &impl ModuleImporter) -> Result<ModuleUID, LangError>;

    fn insert_module(&mut self, module: Arc<Module>) -> Result<(), LangError>;

    fn module_loader(&mut self) -> &mut ModuleLoader;
    fn insert_external_module(&mut self, module: Self::ExternalModule);

    fn new() -> Self;
}

pub trait EngineGetFunction<Args, R, Ret: InternalFunction<Args, R>> : Engine {
    fn get_function(&self, uid: ModuleUID, name: &str)
                    -> Option<Ret>;
}

pub trait InternalFunction<Args, R> {
    fn call(&self, args: Args) -> R;
}