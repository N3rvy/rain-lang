use common::ast::module::ASTModule;
use common::errors::LangError;
use crate::{Engine, ExternalType, InternalFunction};
use crate::module_builder::ModuleBuilder;


pub trait EngineModule : Sized {
    type Engine: Engine;

    fn new(builder: &ModuleBuilder<Self::Engine>, module: ASTModule) -> Result<Self, LangError>;
}

pub trait EngineModuleSetFunction<Args, R: ExternalType> : EngineModule {
    type Function: InternalFunction<Args, R>;

    fn set_function<F>(&self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}