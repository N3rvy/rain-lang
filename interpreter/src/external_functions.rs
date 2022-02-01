use core::{LangError, ExternalType};
use std::sync::Arc;

use common::messages::{INCORRECT_NUMBER_OF_PARAMETERS, EXTERNAL_FUNCTION_PARAMETER_WRONG_TYPE};

use crate::{lang_value::{LangValue, LangExternalFunction}, convert_values::ConvertLangValue};


pub struct ExternalFunctionRunner {
    args_count: usize,
    func: Box<dyn Fn(Vec<LangValue>) -> Option<LangValue> + Send + Sync + 'static>,
}

impl ExternalFunctionRunner {
    pub fn run(&self, args: Vec<LangValue>) -> Result<LangValue, LangError> {
        if args.len() != self.args_count {
            return Err(LangError::new_runtime(INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
        }
        
        match (self.func)(args) {
            Some(val) => Ok(val),
            None => Err(LangError::new_runtime(EXTERNAL_FUNCTION_PARAMETER_WRONG_TYPE.to_string())),
        }
    }
}


pub trait IntoExternalFunctionRunner<A, R: ExternalType> {
    fn external(self) -> LangExternalFunction;
}


// TODO: Maybe make a macro for this

impl<R, F> IntoExternalFunctionRunner<(), R> for F
where
    R: ConvertLangValue,
    F: Fn<(), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 0,
            func: Box::new(move |_| {
                let res = self();
                
                Some(R::from(res))
            }),
        })
    }
}

impl<A0, R, F> IntoExternalFunctionRunner<(A0,), R> for F
where
    A0: ConvertLangValue,
    R: ConvertLangValue,
    F: Fn<(A0,), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 1,
            func: Box::new(move |args| {
                let arg0 = A0::into(&args[0])?;
                
                let res = self(arg0);
                
                Some(R::from(res))
            }),
        })
    }
}

impl<A0, A1, R, F> IntoExternalFunctionRunner<(A0,A1), R> for F
where
    A0: ConvertLangValue,
    A1: ConvertLangValue,
    R: ConvertLangValue,
    F: Fn<(A0,A1), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 2,
            func: Box::new(move |args| {
                let arg0 = A0::into(&args[0])?;
                let arg1 = A1::into(&args[1])?;
                
                let res = self(arg0, arg1);
                
                Some(R::from(res))
            }),
        })
    }
}

impl<A0, A1, A2, R, F> IntoExternalFunctionRunner<(A0,A1,A2), R> for F
where
    A0: ConvertLangValue,
    A1: ConvertLangValue,
    A2: ConvertLangValue,
    R: ConvertLangValue,
    F: Fn<(A0,A1,A2), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 2,
            func: Box::new(move |args| {
                let arg0 = A0::into(&args[0])?;
                let arg1 = A1::into(&args[1])?;
                let arg2 = A2::into(&args[2])?;
                
                let res = self(arg0, arg1, arg2);
                
                Some(R::from(res))
            }),
        })
    }
}

impl<A0, A1, A2, A3, R, F> IntoExternalFunctionRunner<(A0,A1,A2,A3), R> for F
where
    A0: ConvertLangValue,
    A1: ConvertLangValue,
    A2: ConvertLangValue,
    A3: ConvertLangValue,
    R: ConvertLangValue,
    F: Fn<(A0,A1,A2,A3), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 2,
            func: Box::new(move |args| {
                let arg0 = A0::into(&args[0])?;
                let arg1 = A1::into(&args[1])?;
                let arg2 = A2::into(&args[2])?;
                let arg3 = A3::into(&args[3])?;
                
                let res = self(arg0, arg1, arg2, arg3);
                
                Some(R::from(res))
            }),
        })
    }
}