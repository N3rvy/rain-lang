use common::{errors::LangError, ast::ASTNode};
use parser::{parser::parse, type_check::check_types};
use tokenizer::tokenizer::tokenize;
use crate::externals::ExternalType;


pub trait Engine<'a> {
    type Module;

    fn create_module(&'a self, script: String) -> Result<Self::Module, LangError> {
        let tokens = tokenize(script)?;
        let ast = parse(tokens)?;
        
        check_types(&ast)?;

        self.create_module_from_ast(ast)
    }

    fn new() -> Self;
    
    fn create_module_from_ast(&'a self, ast: ASTNode) -> Result<Self::Module, LangError>;
    fn global_module(&'a self) -> &Self::Module;
}

pub trait EngineGetFunction<'a, Args, R: ExternalType> : Engine<'a> {
    fn get_function(&self, module: &Self::Module, name: &str)
        -> Option<Box<dyn Fn(Args) -> Result<R, LangError>>>;
}

pub trait EngineSetFunction<'a, Args, R: ExternalType> : Engine<'a> {
    #[inline]
    fn set_function<F>(&'a self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static
    {
        self.set_function_in_module(self.global_module(), name, func);
    }

    fn set_function_in_module<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}