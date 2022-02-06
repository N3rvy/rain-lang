#![feature(unboxed_closures)]
#![feature(try_trait_v2)]

use core::{ExternalType, Engine, EngineSetFunction};
use common::ast::ASTNode;
use common::errors::LangError;
use errors::CANT_CONVERT_VALUE;
use evaluate::EvalResult;
use external_functions::IntoExternalFunctionRunner;
use lang_value::LangValue;
use scope::Scope;

mod scope;
mod evaluate;
mod lang_value;
mod external_functions;
mod object;
mod errors;

pub struct InterpreterEngine<'a> {
    global_module: Module<'a>,
}

impl<'a> Default for InterpreterEngine<'a> {
    fn default() -> Self {
        Self {
            global_module: Module::new(Scope::new())
        }
    }
}


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


impl<'a> Engine<'a> for InterpreterEngine<'a> {
    type Module = Module<'a>;

    fn new() -> Self {
        Self::default()
    }

    fn global_module(&'a self) -> &Self::Module { &self.global_module }

    fn create_module_from_ast(&'a self, ast: ASTNode) -> Result<Self::Module, core::LangError> {
        let scope = Scope::new_child(&self.global_module.scope);

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

impl<'a, R> EngineSetFunction<'a, (), R> for InterpreterEngine<'a>
where
    R: ExternalType
{
    fn set_function_in_module<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<(), Output = R> + Send + Sync + 'static
    {
        let ext_func = IntoExternalFunctionRunner::<(), R>::external(func);

        module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0> EngineSetFunction<'a, (A0,), R> for InterpreterEngine<'a>
where
    A0: ExternalType,
    R: ExternalType
{
    fn set_function_in_module<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<(A0,), Output = R> + Send + Sync + 'static
    {
        let ext_func = IntoExternalFunctionRunner::<(A0,), R>::external(func);

        module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0, A1> EngineSetFunction<'a, (A0, A1), R> for InterpreterEngine<'a>
where
    A0: ExternalType,
    A1: ExternalType,
    R: ExternalType
{
    fn set_function_in_module<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<(A0, A1), Output = R> + Send + Sync + 'static
    {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1), R>::external(func);

        module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0, A1, A2> EngineSetFunction<'a, (A0, A1, A2), R> for InterpreterEngine<'a>
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    R: ExternalType
{
    fn set_function_in_module<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<(A0, A1, A2), Output = R> + Send + Sync + 'static
    {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1, A2), R>::external(func);

        module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0, A1, A2, A3> EngineSetFunction<'a, (A0, A1, A2, A3), R> for InterpreterEngine<'a>
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    A3: ExternalType,
    R: ExternalType
{
    fn set_function_in_module<F>(&self, module: &Self::Module, name: &str, func: F)
    where F: Fn<(A0, A1, A2, A3), Output = R> + Send + Sync + 'static
    {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1, A2, A3), R>::external(func);

        module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}