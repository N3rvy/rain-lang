use crate::{ExternalType, InternalFunction};


pub trait EngineModule {
}

pub trait EngineModuleSetFunction<Args, R: ExternalType> : EngineModule {
    type Function: InternalFunction<Args, R>;

    fn set_function<F>(&self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}