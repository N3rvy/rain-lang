use std::sync::Arc;

use crate::{lang_value::{LangValue, Function}, errors::LangError, messages::{INCORRECT_NUMBER_OF_PARAMETERS, EXTERNAL_FUNCTION_PARAMETER_WRONG_TYPE}};


pub struct ExternalFunctionRunner {
    args_count: usize, 
    func: Box<dyn Fn(Vec<LangValue>) -> Option<LangValue>>,
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


pub trait IntoExternalFunctionRunner<A, R: ExtFuncParam> {
    fn external(self) -> ExternalFunctionRunner;
}


impl<A0, R, F> IntoExternalFunctionRunner<(A0,), R> for F
where
    A0: ExtFuncParam,
    R: ExtFuncParam,
    F: Fn<(A0,), Output = R> + 'static
{
    fn external(self) -> ExternalFunctionRunner {
        ExternalFunctionRunner {
            args_count: 1,
            func: Box::new(move |args| {
                let arg0 = A0::into(&args[0])?;
                
                let res = self(arg0);
                
                Some(R::from(res))
            }),
        }
    }
}


pub trait ExtFuncParam
    where Self: Sized + 'static
{
    fn from(val: Self) -> LangValue;
    fn into(val: &LangValue) -> Option<Self>;
}


impl ExtFuncParam for i32 {
    fn from(val: Self) -> LangValue {
        LangValue::Int(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_i32()
    }
}

impl ExtFuncParam for f32 {
    fn from(val: Self) -> LangValue {
        LangValue::Float(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_f32()
    }
}

impl ExtFuncParam for bool {
    fn from(val: Self) -> LangValue {
        LangValue::Bool(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_bool()
    }
}

impl ExtFuncParam for String {
    fn from(val: Self) -> LangValue {
        LangValue::String(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_string()
    }
}

impl ExtFuncParam for Arc<Function> {
    fn from(val: Self) -> LangValue {
        LangValue::Function(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_function()
    }
}

// impl ExtFuncParam for Arc<> {
//     fn from(val: Self) -> LangValue {
//         LangValue::ExtFunction(val)
//     }

//     fn into(val: &LangValue) -> Option<Self> {
//         val.as_ext_function()
//     }
// }