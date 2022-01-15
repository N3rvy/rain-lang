#![feature(try_trait_v2)]

use std::{sync::Arc, borrow::{Borrow, BorrowMut}, ops::Add};

use common::{script::Script, lang_value::{LangValue, LangValueDiscriminant}, errors::LangError, external_functions::ConvertLangValue, helper::HelperRegistry, ast::ASTNode};
use helpers::DefaultHelperRegistry;
use scope::Scope;

pub mod scope;
pub mod evaluate;
pub mod helpers;


pub struct Vm {
    registry: Arc<HelperRegistry>,
    scope: Arc<Scope>,
}

impl Vm {
    pub fn new() -> Self {
        let registry = Arc::new(HelperRegistry::default());
        Self {
            scope: Scope::new(registry.clone()),
            registry,
        }
    }
    
    pub fn new_scope(&self) -> Arc<Scope> {
        Scope::new_child(self.scope.clone())
    }
    
    pub fn register(&self, name: &str, val: impl ConvertLangValue) {
        self.scope.declare_var(name.to_string(), ConvertLangValue::from(val))
    }
    
    #[inline]
    pub fn invoke(&self, name: &str) -> Result<LangValue, LangError> {
        Self::invoke_in_scope(name, self.scope.clone())
    }
    
    // TODO: Arguments, and abstract return value
    pub fn invoke_in_scope(name: &str, scope: Arc<Scope>) -> Result<LangValue, LangError> {
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
        Self::get_var_in_scope(name, self.scope.clone())
    }
    
    pub fn get_var_in_scope<T: ConvertLangValue>(name: &str, scope: Arc<Scope>) -> Option<T> {
        T::into(&scope.get_var(&name.to_string())?)
    }

    #[inline]
    pub fn evaluate(&self, script: &Script) -> Result<LangValue, LangError> {
        self.evaluate_in_scope(script, self.scope.clone())
    }
    
    #[inline]
    pub fn evaluate_in_separate_scope(&self, script: &Script) -> Result<LangValue, LangError> {
        let scope = Scope::new(self.registry.clone());
        self.evaluate_in_scope(script, scope)
    }
    
    #[inline]
    pub fn evaluate_in_upper_scope(&self, script: &Script) -> Result<LangValue, LangError> {
        let scope = Scope::new_child(self.scope.clone());
        self.evaluate_in_scope(script, scope)
    }
    
    pub fn evaluate_in_scope(&self, script: &Script, scope: Arc<Scope>) -> Result<LangValue, LangError> {
        match evaluate::evaluate(&script.ast, scope) {
            evaluate::EvalResult::Ok(val) => Ok(val),
            evaluate::EvalResult::Ret(val, _) => Ok(val),
            evaluate::EvalResult::Err(err) => Err(err),
        }
    }
}