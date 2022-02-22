use common::errors::LangError;
use crate::{Engine, ExternalType};

pub struct EngineModuleBuilder<'a, Eng: Engine<'a>> {
    engine: &'a Eng,
    sources: Vec<String>,
}

impl<'a, Eng> EngineModuleBuilder<'a, Eng>
where
    Eng: Engine<'a>,
{
    pub fn new(engine: &'a Eng) -> Self {
        Self {
            engine,
            sources: Vec::new(),
        }
    }
    
    pub fn with_source(mut self, source: String) -> Self {
        self.sources.push(source);
        self
    }
    
    pub fn build(self) -> Result<Eng::Module, LangError> {
        Eng::ModuleBuilder::build(&self.engine, &self.sources)
    }
}

pub trait ModuleBuilder<'a>
{
    type Engine: Engine<'a>;

    fn build(engine: &'a Self::Engine, sources: &Vec<String>) -> Result<<Self::Engine as Engine<'a>>::Module, LangError>;
}

pub trait ModuleBuilderSetFunction<'a, Eng, Args, R>
where
    Self: ModuleBuilder<'a>,
    R: ExternalType,
{
    fn with_func<F>(&mut self, engine: &'a Self::Engine, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}