use std::{collections::HashMap, cell::RefCell, sync::Arc};

use common::{lang_value::LangValue, external_functions::ExternalFunctionRunner, helper::HelperRegistry};

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: RefCell<HashMap<String, LangValue>>,
    pub registry: Arc<HelperRegistry>,
}

impl<'a> Scope<'a> {
    pub fn new(registry: Arc<HelperRegistry>) -> Self {
        Self {
            parent: None,
            variables: RefCell::new(HashMap::new()),
            registry,
        }
    }

    pub fn new_child(parent: &'a Scope<'a>) -> Self {
        Self {
            parent: Some(parent),
            variables: RefCell::new(HashMap::new()),
            registry: parent.registry.clone(),
        }
    }
    
    pub fn declare_var(&self, name: String, value: LangValue) {
        self.variables.borrow_mut().insert(name, value); 
    }
    
    pub fn declare_ext_func(&self, name: &str, runner: ExternalFunctionRunner)  {
        self.variables.borrow_mut().insert(name.to_string(), LangValue::ExtFunction(Arc::new(runner)));
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