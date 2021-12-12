use crate::{common::{lang_value::LangValue, messages::INCORRECT_NUMBER_OF_PARAMETERS}, error::LangError};


pub struct ExtFunc<F> {
    func: F,
}


pub trait RunExtFunc {
    fn args_count(&self) -> usize;
    fn run(&self, args: Vec<LangValue>) -> Result<LangValue, LangError>;
}

pub trait IntoExtFunc<F> {
    fn external_func(self) -> ExtFunc<F>;
}

impl<F> IntoExtFunc<F> for F
    where F: Fn() -> LangValue
{
    fn external_func(self) -> ExtFunc<F> {
        ExtFunc {
            func: self,
        }
    }
}

impl<F> RunExtFunc for ExtFunc<F>
    where F: Fn() -> LangValue
{
    fn args_count(&self) -> usize {
        0
    }

    fn run(&self, params: Vec<LangValue>) -> Result<LangValue, LangError> {
        if params.len() != 0 { return Err(LangError::new_runtime(INCORRECT_NUMBER_OF_PARAMETERS.to_string())) }
        
        Ok((self.func)())
    }
}