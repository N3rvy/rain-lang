use std::{collections::HashMap, cell::RefCell};

use crate::common::lang_value::LangValue;


pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: RefCell<HashMap<String, LangValue>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Self {
        Self {
            parent,
            variables: RefCell::new(HashMap::new()),
        }
    }
    
    pub fn declare_var(&self, name: String, value: LangValue) {
        self.variables.borrow_mut().insert(name, value); 
    }
    
    pub(super) fn get_var(&'a self, name: &String) -> Option<LangValue> {
        match self.variables.borrow().get(name) {
            Some(value) => Some(value.clone()),
            None => {
                match self.parent {
                    Some(scope) => scope.get_var(name),
                    None => None,
                }
            },
        }
    }
    
    pub(super) fn set_var(&self, name: &String, value: LangValue) -> bool {
        match self.variables.borrow_mut().get_mut(name) {
            Some(val) => {
                *val = value;
                true
            },
            None => {
                match self.parent {
                    Some(scope) => {
                        scope.set_var(name, value);  
                        true
                    },
                    None => false,
                }
            },
        }
    }
}