use common::{errors::LangError, ast::ASTNode};

use crate::externals::ExternalType;


pub trait ExecutionEngine {
    type Module;

    fn create_module(&self, ast: ASTNode) -> Result<Self::Module, LangError>;
    fn get_function<Ret: ExternalType>(&self, module: &Self::Module, name: &str) -> Option<Box<dyn Fn(&Self, &Self::Module) -> Result<Ret, LangError>>>;
}