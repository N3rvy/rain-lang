use common::{errors::LangError, ast::ASTNode};

use crate::externals::ExternalType;


pub trait ExecutionEngine {
    fn execute(&self, ast: ASTNode) -> Result<(), LangError>;
    fn get_function<Ret: ExternalType>(&self, name: &str) -> Option<Box<dyn Fn(&Self) -> Result<Ret, LangError>>>;
}