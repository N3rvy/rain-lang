use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use common::module::ModuleUID;
use crate::{InterpreterEngine, InterpreterModule, LangValue, ModuleStore};

pub struct ModuleScope {
    uid: ModuleUID,
    variables: Mutex<HashMap<String, LangValue>>,
    modules: Arc<RefCell<ModuleStore<InterpreterModule>>>,
}

impl ModuleScope {
    pub fn new(uid: ModuleUID, engine: &InterpreterEngine) -> Arc<Self> {
        Arc::new(Self {
            uid,
            variables: Mutex::new(HashMap::new()),
            modules: engine.module_store.clone(),
        })
    }

    pub fn set_var(&self, name: String, value: LangValue) {
        self.variables
            .lock()
            .unwrap()
            .insert(name, value);
    }

    /// Returns the variable and if it does not have it, it searches imported modules
    pub fn search_var(&self, module: ModuleUID, name: &String) -> Option<LangValue> {
        if module == self.uid {
            self.get_var(name)
        } else {
            self.modules
                .borrow()
                .get(module)
                .and_then(|m| m.scope.search_var(module, name))
        }
    }

    pub fn get_var(&self, name: &String) -> Option<LangValue> {
        self.variables
            .lock()
            .unwrap()
            .get(name)
            .cloned()
    }
}
