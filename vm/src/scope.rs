use std::{collections::HashMap, cell::RefCell, sync::{Arc, Mutex}};

use common::{lang_value::LangValue, external_functions::ExternalFunctionRunner, helper::HelperRegistry};

pub struct Scope {
    parent: Option<Arc<Scope>>,
    variables: Mutex<RefCell<HashMap<String, LangValue>>>,
    pub registry: Arc<HelperRegistry>,
}

impl Scope {
    pub fn new(registry: Arc<HelperRegistry>) -> Arc<Self> {
        Arc::new(Self {
            parent: None,
            variables: Mutex::new(RefCell::new(HashMap::new())),
            registry,
        })
    }

    pub fn new_child(parent: Arc<Scope>) -> Arc<Self> {
        Arc::new(Self {
            parent: Some(parent.clone()),
            variables: Mutex::new(RefCell::new(HashMap::new())),
            registry: parent.registry.clone(),
        })
    }
    
    pub fn declare_var(&self, name: String, value: LangValue) {
        self.variables.lock().unwrap().borrow_mut().insert(name, value); 
    }
    
    pub fn declare_ext_func(&self, name: &str, runner: ExternalFunctionRunner)  {
        self.variables.lock().unwrap().borrow_mut().insert(name.to_string(), LangValue::ExtFunction(Arc::new(runner)));
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