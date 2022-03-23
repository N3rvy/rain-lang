use common::module::ModuleIdentifier;
use parser::modules::module_importer::ModuleImporter;
use crate::{Engine, ExternalType};

pub trait ExternalModule {
    type Engine: Engine;

    fn new(engine: &mut Self::Engine, id: &ModuleIdentifier, importer: &impl ModuleImporter)
        -> Option<Self>
    where Self: Sized;
}

pub trait ExternalModuleSetFunctionMetadata<Args, R: ExternalType> : ExternalModule {
    fn set_function(&mut self, name: &str);
}

pub trait ExternalModuleSetFunction<Args, R: ExternalType> : ExternalModule {
    fn set_function<F>(&mut self, name: &str, func: F)
        where F: Fn<Args, Output = R> + Send + Sync + 'static;
}

pub trait ExternalModuleSetValue<R: ExternalType> : ExternalModule {
    fn set_value(&mut self, name: &str, value: R);
}