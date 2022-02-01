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

#[derive(Default)]
pub struct Interpreter;

pub struct Module<'a> {
    scope: Scope<'a>,
}

impl<'a> Module<'a> {
    fn new(scope: Scope<'a>) -> Self {
        Self {
            scope
        }
    }
}


impl ExecutionEngine for Interpreter {
    type Module = Module<'static>;

    fn create_module(&self, ast: ASTNode) -> Result<Self::Module, core::LangError> {
        let scope = Scope::new();

        match self.evaluate_ast(&scope, &ast) {
            EvalResult::Ok(_) | EvalResult::Ret(_, _) => Ok(Module::new(scope)),
            EvalResult::Err(err) => Err(err),
        }
    }

    fn get_function<Ret: ExternalType>(&self, module: &Self::Module, name: &str) -> Option<Box<dyn Fn(&Self, &Self::Module) -> Result<Ret, LangError>>>
    {
        let value = module.scope.get_var(&name.to_string());
        let func = match value {
            None => return None,
            Some(value) => match value {
                LangValue::Function(func) => func,
                _ => return None
            },
        };

        // TODO: Missing parameters
        Some(Box::new(move |exec_engine, module| {
            let result = exec_engine.invoke_function(
                &Scope::new_child(&module.scope),
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