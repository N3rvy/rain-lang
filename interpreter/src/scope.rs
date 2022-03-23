use std::{collections::HashMap, sync::{Mutex, Arc}};
use std::borrow::Borrow;
use common::module::ModuleUID;
use crate::lang_value::LangValue;
use crate::module_scope::ModuleScope;

pub enum Parent<'a> {
    Module(Arc<ModuleScope>),
    Scope(&'a Scope<'a>),
}

pub struct Scope<'a> {
    parent: Parent<'a>,
    variables: Mutex<HashMap<String, LangValue>>,
}

impl<'a> Scope<'a> {
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
    
    pub(super) fn get_var(&self, module_uid: ModuleUID, name: &String) -> Option<LangValue> {
        match (*self.variables.lock().unwrap()).borrow().get(name) {
            Some(value) => Some(value.clone()),
            None => {
                match &self.parent {
                    Parent::Module(module) => module.search_var(module_uid, name),
                    Parent::Scope(scope) => scope.get_var(module_uid, name),
                }
            },
        }
    }
    
    pub(super) fn set_var(&self, name: &String, value: LangValue) {
        match self.variables.lock().unwrap().get_mut(name) {
            Some(val) => {
                *val = value;
            },
            None => {
                match &self.parent {
                    Parent::Module(module) => module.set_var(name.to_string(), value),
                    Parent::Scope(scope) => {
                        scope.set_var(name, value);  
                    },
                }
            },
        }
    }
}