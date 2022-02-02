use std::{collections::HashMap, cell::RefCell, sync::Mutex};

use crate::lang_value::LangValue;

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: Mutex<RefCell<HashMap<String, LangValue>>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: Mutex::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn new_child(parent: &'a Scope) -> Self {
        Self {
            parent: Some(&parent),
            variables: Mutex::new(RefCell::new(HashMap::new())),
        }
    }
    
    pub fn declare_var(&self, name: String, value: LangValue) {
        self.variables.lock().unwrap().borrow_mut().insert(name, value); 
    }
    
    pub(super) fn get_var(&self, name: &String) -> Option<LangValue> {
        match (*self.variables.lock().unwrap()).borrow().get(name) {
            Some(value) => Some(value.clone()),
            None => {
                match &self.parent {
                    Some(scope) => scope.get_var(name),
                    None => None,
                }
            },
        }
    }
    
    pub(super) fn set_var(&self, name: &String, value: LangValue) -> bool {
        match self.variables.lock().unwrap().borrow_mut().get_mut(name) {
            Some(val) => {
                *val = value;
                true
            },
            None => {
                match &self.parent {
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