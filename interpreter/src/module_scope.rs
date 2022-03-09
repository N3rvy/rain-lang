use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::LangValue;

pub struct ModuleScope {
    variables: Mutex<HashMap<String, LangValue>>,
}

impl ModuleScope {
    pub fn new() -> Self {
        Self {
            variables: Mutex::new(HashMap::new()),
        }
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

    pub fn get_var(&self, name: &String) -> Option<LangValue> {
        self.variables
            .lock()
            .unwrap()
            .get(name)
            .cloned()
    }
}
