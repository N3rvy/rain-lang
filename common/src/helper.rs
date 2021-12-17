use std::collections::HashMap;
use super::{lang_value::{LangValue, LangValueDiscriminant}, external_functions::ConvertLangValue};


pub struct Helper {
    fields: HashMap<String, LangValue>
}

impl Helper {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }
    
    #[inline]
    pub fn register(&mut self, name: &str, value: impl ConvertLangValue) {
        self.fields.insert(name.to_string(), ConvertLangValue::from(value));
    }
    
    #[inline]
    pub fn get(&self, name: &String) -> Option<&LangValue> {
        self.fields.get(name)
    }
}

pub struct HelperRegistry {
    map: HashMap<LangValueDiscriminant, Helper>,
}

impl HelperRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    
    pub fn register_helper(&mut self, discriminant: LangValueDiscriminant, helper: Helper) {
        self.map.insert(discriminant, helper);
    }
    
    pub fn get_helper_from_discriminant(&self, discriminant: LangValueDiscriminant) -> Option<&Helper> {
        self.map.get(&discriminant)
    }
    
    pub fn get_helper(&self, value: &LangValue) -> Option<&Helper> {
        self.map.get(&value.into())
    }
}