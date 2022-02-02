use common::{errors::LangError, ast::ASTNode};
use parser::{parser::parse, type_check::check_types};
use tokenizer::tokenizer::tokenize;
use crate::externals::ExternalType;


pub trait Engine {
    type Module;

    fn create_module(&self, script: String) -> Result<Self::Module, LangError> {
        let tokens = tokenize(script)?;
        let ast = parse(tokens)?;
        
        check_types(&ast)?;

        self.create_module_from_ast(ast)
    }

    fn new() -> Self;
    
    fn create_module_from_ast(&self, ast: ASTNode) -> Result<Self::Module, LangError>;
    fn get_function<Ret: ExternalType>(&self, module: &Self::Module, name: &str) -> Option<Box<dyn Fn(&Self, &Self::Module) -> Result<Ret, LangError>>>;
}

pub trait EngineSetFunction<Args, R: ExternalType> : Engine {
    fn set_function<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}