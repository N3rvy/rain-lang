pub use common::lang_value::LangValue;
pub use common::external_functions::IntoExtFunc;
pub use vm::scope::Scope;

use common::errors::LangError;
use parser::parser::parse;
use tokenizer::tokenizer::tokenize;
use vm::vm::EvalResult;
use vm;


pub fn evaluate(script: String) -> Result<LangValue, LangError> {
    let mut scope = Scope::new(None);
    evaluate_scope(script, &mut scope)
}

pub fn evaluate_scope(script: String, scope: &mut Scope) -> Result<LangValue, LangError> {
    let tokens = tokenize(script)?;
    let ast = parse(tokens)?;
    match vm::vm::evaluate(&ast, scope) {
        EvalResult::Ok(value) => Ok(value),
        EvalResult::Ret(value, _) => Ok(value),
        EvalResult::Err(err) => Err(err),
    }
}