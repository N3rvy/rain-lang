#![feature(unboxed_closures)]
#![feature(try_trait_v2)]

use core::{ExecutionEngine, ExternalType};
use common::ast::ASTNode;
use common::errors::LangError;
use errors::CANT_CONVERT_VALUE;
use evaluate::EvalResult;
use lang_value::LangValue;
use scope::Scope;

mod scope;
mod evaluate;
mod lang_value;
mod convert_values;
mod external_functions;
mod object;
mod errors;

pub struct Interpreter<'a> {
    global_scope: Scope<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            global_scope: Scope::new(),
        }
    }
}

impl<'a> ExecutionEngine for Interpreter<'a> {
    fn execute(&self, ast: ASTNode) -> Result<(), core::LangError> {
        match self.evaluate_ast(&self.global_scope, &ast) {
            EvalResult::Ok(_) | EvalResult::Ret(_, _) => Ok(()),
            EvalResult::Err(err) => Err(err),
        }
    }

    fn get_function<Ret: ExternalType>(&self, name: &str) -> Option<Box<dyn Fn(&Self) -> Result<Ret, LangError>>>
    {
        let value = self.global_scope.get_var(&name.to_string());
        let func = match value {
            None => return None,
            Some(value) => match value {
                LangValue::Function(func) => func,
                _ => return None
            },
        };

        // TODO: Missing parameters
        Some(Box::new(move |exec_engine| {
            let result = exec_engine.invoke_function(
                &Scope::new_child(&exec_engine.global_scope),
                &LangValue::Function(func.clone()),
                vec![],
            );

            let value = match result {
                EvalResult::Ok(value) => value,
                EvalResult::Ret(value, _) => value,
                EvalResult::Err(err) => return Err(err),
            };

            match value.into() {
                None => Err(LangError::new_runtime(CANT_CONVERT_VALUE.to_string())),
                Some(value) => match Ret::concretize(value) {
                    None => Err(LangError::new_runtime(CANT_CONVERT_VALUE.to_string())),
                    Some(value) => Ok(value),
                },
            }
        }))
    }
}