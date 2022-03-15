use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use common::module::ModuleUID;
use crate::{InterpreterEngine, InterpreterModule, LangValue};

pub struct ModuleScope {
    uid: ModuleUID,
    variables: Mutex<HashMap<String, LangValue>>,
    modules: Arc<RefCell<HashMap<ModuleUID, InterpreterModule>>>,
}

impl ModuleScope {
    pub fn new(uid: ModuleUID, engine: &InterpreterEngine) -> Arc<Self> {
        Arc::new(Self {
            uid,
            variables: Mutex::new(HashMap::new()),
            modules: engine.modules.clone(),
        })
    }

    pub fn force_set_var(&self, name: String, value: LangValue) {
        self.variables
            .lock()
            .unwrap()
            .insert(name, value);
    }

    pub fn define_var(&self, name: String) {
        self.variables
            .lock()
            .unwrap()
            .insert(name, LangValue::Nothing);
    }

    pub fn declare_var(&self, name: &String, value: LangValue) -> Option<()> {
        let mut guard = self.variables
            .lock()
            .unwrap();

        let variable = guard
            .get_mut(name);

        match variable {
            Some(var) => {
                *var = value;
                Some(())
            },
            None => None,
        }
    }

    /// Returns the variable and if it does not have it, it searches imported modules
    pub fn search_var(&self, module: ModuleUID, name: &String) -> Option<LangValue> {
        if module == self.uid {
            self.get_var(name)
        } else {
            (*self.modules).borrow()
                .get(&module)
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

    pub fn variables(&self) -> MutexGuard<HashMap<String, LangValue>> {
        self.variables
            .lock()
            .unwrap()
    }
}
