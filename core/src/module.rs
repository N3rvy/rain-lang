use std::sync::Arc;
use common::errors::LangError;
use common::module::Module;
use crate::{Engine, ExternalType, InternalFunction};


pub trait EngineModule : Sized {
    type Engine: Engine;

    fn new(engine: &mut Self::Engine, module: Arc<Module>) -> Result<Self, LangError>;
}

pub trait EngineModuleSetFunction<Args, R: ExternalType> : EngineModule {
    type Function: InternalFunction<Args, R>;

    fn set_function<F>(&self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}