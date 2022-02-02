use core::{LangError, ExternalType};
use std::sync::Arc;

use crate::{lang_value::{LangValue, LangExternalFunction}, errors::{EXTERNAL_FUNCTION_PARAMETER_WRONG_TYPE, EXTERNAL_FUNCTION_INCORRECT_NUMBER_OF_PARAMETERS}};


pub struct ExternalFunctionRunner {
    args_count: usize,
    func: Box<dyn Fn(Vec<LangValue>) -> Option<LangValue> + Send + Sync + 'static>,
}

impl ExternalFunctionRunner {
    pub fn run(&self, args: Vec<LangValue>) -> Result<LangValue, LangError> {
        if args.len() != self.args_count {
            return Err(LangError::new_runtime(EXTERNAL_FUNCTION_INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
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


impl<R, F> IntoExternalFunctionRunner<(), R> for F
where
    R: ExternalType,
    F: Fn<(), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 0,
            func: Box::new(move |_| {
                let res = self();
                
                Some(R::generilize(res).into())
            }),
        })
    }
}

impl<A0, R, F> IntoExternalFunctionRunner<(A0,), R> for F
where
    A0: ExternalType,
    R: ExternalType,
    F: Fn<(A0,), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 1,
            func: Box::new(move |args| {
                let arg0 = A0::concretize(args[0].clone().into())?;
                
                let res = self(arg0);
                
                Some(R::generilize(res).into())
            }),
        })
    }
}

impl<A0, A1, R, F> IntoExternalFunctionRunner<(A0,A1), R> for F
where
    A0: ExternalType,
    A1: ExternalType,
    R: ExternalType,
    F: Fn<(A0,A1), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 2,
            func: Box::new(move |args| {
                let arg0 = A0::concretize(args[0].clone().into())?;
                let arg1 = A1::concretize(args[1].clone().into())?;
                
                let res = self(arg0, arg1);
                
                Some(R::generilize(res).into())
            }),
        })
    }
}

impl<A0, A1, A2, R, F> IntoExternalFunctionRunner<(A0,A1,A2), R> for F
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    R: ExternalType,
    F: Fn<(A0,A1,A2), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 2,
            func: Box::new(move |args| {
                let arg0 = A0::concretize(args[0].clone().into())?;
                let arg1 = A1::concretize(args[1].clone().into())?;
                let arg2 = A2::concretize(args[2].clone().into())?;
                
                let res = self(arg0, arg1, arg2);
                
                Some(R::generilize(res).into())
            }),
        })
    }
}

impl<A0, A1, A2, A3, R, F> IntoExternalFunctionRunner<(A0,A1,A2,A3), R> for F
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    A3: ExternalType,
    R: ExternalType,
    F: Fn<(A0,A1,A2,A3), Output = R> + Send + Sync + 'static
{
    fn external(self) -> LangExternalFunction {
        Arc::new(ExternalFunctionRunner {
            args_count: 2,
            func: Box::new(move |args| {
                let arg0 = A0::concretize(args[0].clone().into())?;
                let arg1 = A1::concretize(args[1].clone().into())?;
                let arg2 = A2::concretize(args[2].clone().into())?;
                let arg3 = A3::concretize(args[3].clone().into())?;
                
                let res = self(arg0, arg1, arg2, arg3);
                
                Some(R::generilize(res).into())
            }),
        })
    }
}