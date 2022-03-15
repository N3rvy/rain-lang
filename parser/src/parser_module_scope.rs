use std::collections::HashMap;
use common::ast::types::TypeKind;
use common::module::ModuleUID;
use crate::parser::ParserScope;

pub struct ParserModuleScope {
    pub uid: ModuleUID,
    declarations: HashMap<String, (ModuleUID, TypeKind)>,
}

impl ParserModuleScope {
    pub fn new(module_uid: ModuleUID) -> Self {
        Self {
            uid: module_uid,
            declarations: HashMap::new(),
        }
    }

    pub fn new_child(&self) -> ParserScope {
        ParserScope::new_module_child(self)
    }

    pub fn get(&self, name: &String) -> Option<(ModuleUID, TypeKind)> {
        self.declarations
            .get(name)
            .cloned()
    }

    pub fn declare(&mut self, name: String, type_kind: TypeKind) {
        self.declarations
            .insert(name, (self.uid, type_kind));
    }

    pub fn declare_external(&mut self, name: String, module: ModuleUID, type_kind: TypeKind) {
        self.declarations
            .insert(name, (module, type_kind));
    }
}