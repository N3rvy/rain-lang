use std::{collections::HashMap, cell::RefCell, sync::{Mutex, Arc}};
use std::borrow::Borrow;
use std::sync::MutexGuard;
use crate::lang_value::LangValue;
use crate::module_scope::ModuleScope;

pub enum Parent<'a> {
    Module(Arc<ModuleScope>),
    Scope(&'a Scope<'a>),
    None,
}

pub struct Scope<'a> {
    parent: Parent<'a>,
    variables: Mutex<HashMap<String, LangValue>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            parent: Parent::None,
            variables: Mutex::new(HashMap::new()),
        })
    }

    pub fn new_module_child(module: Arc<ModuleScope>) -> Self {
        Self {
            parent: Parent::Module(module),
            variables: Mutex::new(HashMap::new())
        }
    }

    pub fn new_child(&'a self) -> Self {
        Self {
            parent: Parent::Scope(self),
            variables: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn declare_var(&self, name: String, value: LangValue) {
        self.variables.lock().unwrap().insert(name, value);
    }
    
    pub(super) fn get_var(&self, name: &String) -> Option<LangValue> {
        match (*self.variables.lock().unwrap()).borrow().get(name) {
            Some(value) => Some(value.clone()),
            None => {
                match &self.parent {
                    Parent::Module(module) => module.get_var(name),
                    Parent::Scope(scope) => scope.get_var(name),
                    Parent::None => None,
                }
            },
        }
    }
    
    pub(super) fn set_var(&self, name: &String, value: LangValue) -> bool {
        match self.variables.lock().unwrap().get_mut(name) {
            Some(val) => {
                *val = value;
                true
            },
            None => {
                match &self.parent {
                    Parent::Module(_) => false,
                    Parent::Scope(scope) => {
                        scope.set_var(name, value);  
                        true
                    },
                    Parent::None => false,
                }
            },
        }
    }
}