use common::ast::types::TypeKind;

use crate::{externals::ExternalType, module_builder::{ModuleBuilder, EngineModuleBuilder}, module::EngineModule};


pub trait Engine<'a>
where
    Self: Sized
{
    type Module: EngineModule;
    type ModuleBuilder: ModuleBuilder<'a, Engine = Self>;

    fn build_module(&'a self) -> EngineModuleBuilder<'a, Self> {
        EngineModuleBuilder::new(&self)
    }

    fn global_types(&'a self) -> &'a Vec<(String, TypeKind)>;

    fn new() -> Self;
}

pub trait EngineGetFunction<'a, Args, R, Ret: InternalFunction<Args, R>> : Engine<'a> {
    fn get_function(&'a self, module: &'a Self::Module, name: &str)
        -> Option<Ret>;
}

pub trait InternalFunction<Args, R> {
    fn call(&self, args: Args) -> R;
}

pub trait EngineSetFunction<'a, Args, R: ExternalType> : Engine<'a> {
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}