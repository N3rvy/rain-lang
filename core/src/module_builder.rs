use common::errors::LangError;

use crate::{Engine, ExternalType, module::EngineModule};

pub struct EngineModuleBuilder<'a, Builder: ModuleBuilder<'a>, Eng: Engine<'a>> {
    engine: &'a Eng,
    sources: Vec<String>,
    data: Builder,
}

impl<'a, Builder, Eng> EngineModuleBuilder<'a, Builder, Eng>
where
    Builder: ModuleBuilder<'a, Engine = Eng>,
    Eng: Engine<'a>,
{
    pub fn new(engine: &'a Eng) -> Self {
        Self {
            engine,
            sources: Vec::new(),
            data: Builder::new(),
        }
    }
    
    pub fn with_source(mut self, source: String) -> Self {
        self.sources.push(source);
        self
    }
    
    pub fn build(mut self) -> Result<Builder::Module, LangError> {
        Builder::build(&mut self.data, &self.engine, &self.sources)
    }
}

pub trait ModuleBuilder<'a>
{
    type Module: EngineModule;
    type Engine: Engine<'a>;

    fn new() -> Self;
    fn build(&self, engine: &'a Self::Engine, sources: &Vec<String>) -> Result<Self::Module, LangError>;
}

pub trait ModuleBuilderSetFunction<'a, Eng, Args, R>
where
    Self: ModuleBuilder<'a>,
    R: ExternalType,
{
    fn with_func<F>(&mut self, engine: &'a Self::Engine, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}