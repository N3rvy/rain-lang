use common::{errors::LangError, ast::ASTNode};


pub trait ExecutionEngine {
    fn execute(&self, ast: ASTNode) -> Result<(), LangError>;
    fn get_function<Args, Ret, F: Fn<Args, Output = Ret>>(&self, name: &str) -> Option<F>;
}