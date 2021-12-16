#![feature(try_trait_v2)]

use common::{script::Script, lang_value::LangValue, errors::LangError, external_functions::ConvertLangValue};
use scope::Scope;

pub mod scope;
pub mod evaluate;


pub struct Vm<'a> {
    scope: Scope<'a>,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(None),
        }
    }
    
    pub fn register(&self, name: &str, val: impl ConvertLangValue) {
        self.scope.declare_var(name.to_string(), ConvertLangValue::from(val))
    }

    #[inline]
    pub fn evaluate(&self, script: Script) -> Result<LangValue, LangError> {
        self.evaluate_in_scope(script, &self.scope)
    }
    
    #[inline]
    pub fn evaluate_in_separate_scope(&self, script: Script) -> Result<LangValue, LangError> {
        let scope = Scope::new(None);
        self.evaluate_in_scope(script, &scope)
    }
    
    #[inline]
    pub fn evaluate_in_upper_scope(&self, script: Script) -> Result<LangValue, LangError> {
        let scope = Scope::new(Some(&self.scope));
        self.evaluate_in_scope(script, &scope)
    }
    
    pub fn evaluate_in_scope(&self, script: Script, scope: &Scope) -> Result<LangValue, LangError> {
        match evaluate::evaluate(&script.ast, &scope) {
            evaluate::EvalResult::Ok(val) => Ok(val),
            evaluate::EvalResult::Ret(val, _) => Ok(val),
            evaluate::EvalResult::Err(err) => Err(err),
        }
    }
}