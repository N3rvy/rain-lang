use std::sync::Arc;

use crate::{lang_value::{LangValue, Function}, errors::LangError, messages::{INCORRECT_NUMBER_OF_PARAMETERS, EXTERNAL_FUNCTION_PARAMETER_WRONG_TYPE}};


pub trait ExtFunc {
    fn args_count(&self) -> usize;
    fn run(&self, args: Vec<LangValue>) -> Result<LangValue, LangError> {
        if args.len() != self.args_count() {
            return Err(LangError::new_runtime(INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
        }

        match self.run_internal(args) {
            Some(val) => Ok(val),
            None => Err(LangError::new_runtime(EXTERNAL_FUNCTION_PARAMETER_WRONG_TYPE.to_string())),
        }
    }
    
    fn run_internal(&self, args: Vec<LangValue>) -> Option<LangValue>;
}


pub trait IntoExtFunc {
    fn into(self) -> Arc<dyn ExtFunc>;
}


pub struct ExtFunc1<A: ExtFuncParam, R: ExtFuncParam>(fn(A) -> R);
impl<'a, A: ExtFuncParam, R: ExtFuncParam> ExtFunc for ExtFunc1<A, R> {
    fn args_count(&self) -> usize { 1 }

    fn run_internal(&self, args: Vec<LangValue>) -> Option<LangValue> {
        let arg0 = A::into(args.get(0)?)?;
        
        let res = (self.0)(arg0);
        Some(R::from(res))
    }
}

impl<A: ExtFuncParam, R: ExtFuncParam> IntoExtFunc for fn(A) -> R
{
    fn into(self) -> Arc<dyn ExtFunc> {
        Arc::new(ExtFunc1(self))
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

impl ExtFuncParam for Arc<dyn ExtFunc> {
    fn from(val: Self) -> LangValue {
        LangValue::ExtFunction(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_ext_function()
    }
}