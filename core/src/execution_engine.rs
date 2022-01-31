use common::{errors::LangError, ast::ASTNode};
use common::convert_values::ConvertLangValue;


pub trait ExecutionEngine {
    fn execute(&self, ast: ASTNode) -> Result<(), LangError>;
    fn get_function<Ret: ConvertLangValue>(&self, name: &str) -> Option<Box<dyn Fn(&Self) -> Result<Ret, LangError>>>;
}