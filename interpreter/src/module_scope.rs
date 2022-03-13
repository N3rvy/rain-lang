use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use common::module::ModuleUID;
use crate::LangValue;

pub struct ModuleScope {
    uid: ModuleUID,
    variables: Mutex<HashMap<String, LangValue>>,
    imports: HashMap<ModuleUID, Arc<ModuleScope>>,
}

impl ModuleScope {
    pub fn new(uid: ModuleUID) -> Arc<Self> {
        Arc::new(Self {
            uid,
            variables: Mutex::new(HashMap::new()),
            imports: HashMap::new(),
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
            self.imports
                .get(&module)
                .and_then(|module| module.get_var(name))
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
