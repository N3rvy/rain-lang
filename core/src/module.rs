use common::ast::module::ASTModule;
use common::errors::LangError;
use crate::{Engine, ExternalType, InternalFunction};
use crate::engine_module_builder::EngineModuleBuilder;


pub trait EngineModule : Sized {
    type Engine: Engine;

    fn new(builder: &EngineModuleBuilder<Self::Engine>, module: ASTModule) -> Result<Self, LangError>;
}

pub trait EngineModuleSetFunction<Args, R: ExternalType> : EngineModule {
    type Function: InternalFunction<Args, R>;

    fn set_function<F>(&self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}