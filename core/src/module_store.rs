use std::collections::HashMap;
use std::sync::Arc;
use common::module::ModuleUID;
use crate::module::EngineModule;

pub struct ModuleStore<EngModule: EngineModule> {
    modules: HashMap<ModuleUID, Arc<EngModule>>,
}

impl<EngModule: EngineModule> ModuleStore<EngModule> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn insert(&mut self, uid: ModuleUID, module: EngModule) {
        self.modules.insert(uid, Arc::new(module));
    }

    pub fn get(&self, uid: ModuleUID) -> Option<Arc<EngModule>> {
        self.modules
            .get(&uid)
            .cloned()
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }
}