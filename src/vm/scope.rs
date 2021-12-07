use std::collections::HashMap;

use crate::common::lang_value::LangValue;


pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<String, LangValue>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
        }
    }
    
    pub(super) fn declare_var(&mut self, name: String, value: LangValue) {
        self.variables.insert(name, value); 
    }
    
    pub(super) fn get_var(&'a self, name: &String) -> Option<&'a LangValue> {
        match self.variables.get(name) {
            Some(value) => Some(value),
            None => {
                match self.parent {
                    Some(scope) => scope.get_var(name),
                    None => None,
                }
            },
        }
    }
}