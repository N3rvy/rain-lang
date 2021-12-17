#![feature(try_trait_v2)]

use std::sync::Arc;

use common::{script::Script, lang_value::LangValue, errors::LangError, external_functions::ConvertLangValue, helper::{HelperRegistry, Helper}};
use helpers::DefaultHelperRegistry;
use scope::Scope;

pub mod scope;
pub mod evaluate;
pub mod helpers;


pub struct Vm<'a> {
    registry: Arc<HelperRegistry>,
    scope: Scope<'a>,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        let registry = Arc::new(HelperRegistry::default());
        Self {
            scope: Scope::new(registry.clone()),
            registry,
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
        let scope = Scope::new(self.registry.clone());
        self.evaluate_in_scope(script, &scope)
    }
    
    #[inline]
    pub fn evaluate_in_upper_scope(&self, script: Script) -> Result<LangValue, LangError> {
        let scope = Scope::new_child(&self.scope);
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