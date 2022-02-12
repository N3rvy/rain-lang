use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{lang_value::LangValue};


#[derive(Clone)]
pub struct LangObject {
    fields: Arc<Mutex<HashMap<String, LangValue>>>,
}

#[allow(dead_code)]
impl LangObject {
    pub fn new() -> Self {
        Self {
            fields: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn from_map(map: HashMap<String, LangValue>) -> Self {
        Self {
            fields: Arc::new(Mutex::new(map))
        }
    }

    pub(crate) fn get(&self, name: &String) -> LangValue  {
        match self.fields.lock() {
            Ok(map) => 
                match map.get(name) {
                    Some(value) => value.clone(),
                    None => LangValue::Nothing,
                },
            Err(_) => LangValue::Nothing,
        }
    }
    
    pub fn len(&self) -> usize {
        match self.fields.lock() {
            Ok(value) => value.len(),
            Err(_) => 0,
        }
    }
}