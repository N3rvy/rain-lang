use common::ast::types::TypeKind;
use common::errors::LangError;
use common::module::{ModuleIdentifier, ModuleUID};
use parser::modules::module_importer::ModuleImporter;
use parser::modules::module_loader::{LoadModuleResult, ModuleLoader};

use crate::{externals::ExternalType, module::EngineModule};
use crate::errors::MODULE_NOT_FOUND;
use crate::engine_module_loader::EngineModuleLoader;


pub trait Engine
where
    Self: Sized,
{
    type Module: EngineModule<Engine = Self>;

    #[inline]
    fn load_module<Importer: ModuleImporter>(&mut self, identifier: &ModuleIdentifier) -> Result<ModuleUID, LangError> {
        self
            .module_loader()
            .load_module::<Importer>(identifier)
    }

    fn global_types(&self) -> &Vec<(String, TypeKind)>;
    fn module_loader(&mut self) -> &mut EngineModuleLoader<Self>;

    fn new() -> Self;
}

pub trait EngineGetFunction<'a, Args, R, Ret: InternalFunction<Args, R>> : Engine {
    fn get_function(&'a self, uid: ModuleUID, name: &str)
                    -> Option<Ret>;
}

pub trait InternalFunction<Args, R> {
    fn call(&self, args: Args) -> R;
}

pub trait EngineSetFunction<'a, Args, R: ExternalType> : Engine {
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}
