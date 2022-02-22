use common::{ast::ASTNode, errors::LangError};
use parser::parser::parse;
use tokenizer::tokenizer::Tokenizer;

use crate::{externals::ExternalType, module_builder::{ModuleBuilder, EngineModuleBuilder}};


pub trait Engine<'a>
where
    Self: Sized
{
    type Module;
    type ModuleBuilder: ModuleBuilder<'a, Engine = Self>;

    fn build_module(&'a self) -> EngineModuleBuilder<'a, Self::ModuleBuilder, Self> {
        EngineModuleBuilder::<'a, Self::ModuleBuilder, Self>::new(&self)
    }
    
    fn source_to_ast(source: &String) -> Result<ASTNode, LangError> {
        let tokens = Tokenizer::tokenize(source)?;
        let ast = parse(tokens)?;

        Ok(ast)
    }

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
    fn set_function<F>(&'a self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}