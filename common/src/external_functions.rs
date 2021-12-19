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


pub trait AsMethod {
    fn as_method(self) -> Self;
}


pub trait IntoExternalFunctionRunner<A, R: ConvertLangValue> {
    fn external(self) -> Arc<ExternalFunctionRunner>;
}


// TODO: Maybe make a macro for this

impl<R, F> IntoExternalFunctionRunner<(), R> for F
where
    R: ConvertLangValue,
    F: Fn<(), Output = R> + 'static
{
    fn external(self) -> Arc<ExternalFunctionRunner> {
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
    F: Fn<(A0,), Output = R> + 'static
{
    fn external(self) -> Arc<ExternalFunctionRunner> {
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
    F: Fn<(A0,A1), Output = R> + 'static
{
    fn external(self) -> Arc<ExternalFunctionRunner> {
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
    F: Fn<(A0,A1,A2), Output = R> + 'static
{
    fn external(self) -> Arc<ExternalFunctionRunner> {
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
    F: Fn<(A0,A1,A2,A3), Output = R> + 'static
{
    fn external(self) -> Arc<ExternalFunctionRunner> {
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


pub trait ConvertLangValue
    where Self: Sized + 'static
{
    fn from(val: Self) -> LangValue;
    // TODO: Make this passed by ownership and not by reference
    fn into(val: &LangValue) -> Option<Self>;
}


impl ConvertLangValue for LangValue {
    fn from(val: Self) -> LangValue {
        val
    }

    fn into(val: &LangValue) -> Option<Self> {
        Some(val.clone())
    }
}

impl ConvertLangValue for () {
    fn from(_: Self) -> LangValue {
        LangValue::Nothing
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_unit()
    }
}

impl ConvertLangValue for i32 {
    fn from(val: Self) -> LangValue {
        LangValue::Int(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_i32()
    }
}

impl ConvertLangValue for f32 {
    fn from(val: Self) -> LangValue {
        LangValue::Float(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_f32()
    }
}

impl ConvertLangValue for bool {
    fn from(val: Self) -> LangValue {
        LangValue::Bool(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_bool()
    }
}

impl ConvertLangValue for String {
    fn from(val: Self) -> LangValue {
        LangValue::String(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_string()
    }
}

impl ConvertLangValue for Arc<Function> {
    fn from(val: Self) -> LangValue {
        LangValue::Function(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_function()
    }
}

impl ConvertLangValue for Arc<ExternalFunctionRunner> {
    fn from(val: Self) -> LangValue {
        LangValue::ExtFunction(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_ext_function()
    }
}

impl ConvertLangValue for Arc<Vec<LangValue>> {
    fn from(val: Self) -> LangValue {
        LangValue::Vector(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_vec()
    }
}