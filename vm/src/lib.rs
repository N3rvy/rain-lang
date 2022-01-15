#![feature(try_trait_v2)]

use std::sync::Arc;

use common::{script::Script, lang_value::{LangValue}, errors::LangError, convert_values::ConvertLangValue, helper::HelperRegistry, ast::ASTNode};
use helpers::DefaultHelperRegistry;
use scope::Scope;

pub mod scope;
pub mod evaluate;
pub mod helpers;


pub struct Vm<'a> {
    registry: Arc<HelperRegistry>,
    global_scope: Scope<'a>,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        let registry = Arc::new(HelperRegistry::default());
        Self {
            global_scope: Scope::new(registry.clone()),
            registry,
        }
    }
    
    pub fn new_scope(&self) -> Scope {
        Scope::new_child(&self.global_scope)
    }
    
    pub fn register(&self, name: &str, val: impl ConvertLangValue) {
        self.global_scope.declare_var(name.to_string(), ConvertLangValue::from(val))
    }
    
    #[inline]
    pub fn invoke(&self, name: &str) -> Result<LangValue, LangError> {
        Self::invoke_in_scope(name, &self.global_scope)
    }
    
    // TODO: Arguments, and abstract return value
    pub fn invoke_in_scope(name: &str, scope: &Scope) -> Result<LangValue, LangError> {
        let runner = ASTNode::new_function_invok(
            ASTNode::new_variable_ref(name.to_string()),
            Vec::with_capacity(0),
        );

        match evaluate::evaluate(&runner, scope) {
            evaluate::EvalResult::Ok(val) => Ok(val),
            evaluate::EvalResult::Ret(val, _) => Ok(val),
            evaluate::EvalResult::Err(err) => Err(err),
        }
    }
    
    #[inline]
    pub fn get_var<T: ConvertLangValue>(&self, name: &str) -> Option<T> {
        Self::get_var_in_scope(name, &self.global_scope)
    }
    
    pub fn get_var_in_scope<T: ConvertLangValue>(name: &str, scope: &Scope) -> Option<T> {
        T::into(&scope.get_var(&name.to_string())?)
    }

    #[inline]
    pub fn evaluate(&self, script: &Script) -> Result<LangValue, LangError> {
        self.evaluate_in_scope(script, &self.global_scope)
    }
    
    #[inline]
    pub fn evaluate_in_separate_scope(&self, script: &Script) -> Result<LangValue, LangError> {
        let scope = Scope::new(self.registry.clone());
        self.evaluate_in_scope(script, &scope)
    }
    
    #[inline]
    pub fn evaluate_in_upper_scope(&self, script: &Script) -> Result<LangValue, LangError> {
        let scope = Scope::new_child(&self.global_scope);
        self.evaluate_in_scope(script, &scope)
    }
    
    pub fn evaluate_in_scope(&self, script: &Script, scope: &Scope) -> Result<LangValue, LangError> {
        match evaluate::evaluate(&script.ast, scope) {
            evaluate::EvalResult::Ok(val) => Ok(val),
            evaluate::EvalResult::Ret(val, _) => Ok(val),
            evaluate::EvalResult::Err(err) => Err(err),
        }
    }
}