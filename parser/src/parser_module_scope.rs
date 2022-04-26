use std::collections::HashMap;
use std::sync::Arc;
use common::ast::types::{ClassType, FunctionType, TypeKind};
use common::module::ModuleUID;
use crate::parser::ParserScope;

pub enum ScopeGetResult {
    Class(ModuleUID, Arc<ClassType>),
    Ref(ModuleUID, TypeKind),
    None,
}

enum GlobalKind {
    Var(ModuleUID, TypeKind),
    Func(ModuleUID, FunctionType),
    Class(ModuleUID, Arc<ClassType>),
}

pub struct ParserModuleScope {
    pub uid: ModuleUID,
    globals: HashMap<String, GlobalKind>,
}

impl ParserModuleScope {
    pub fn new(module_uid: ModuleUID) -> Self {
        Self {
            uid: module_uid,
            globals: HashMap::new(),
        }
    }

    pub fn new_child(&self) -> ParserScope {
        ParserScope::new_module_child(self)
    }

    pub fn get(&self, name: &String) -> ScopeGetResult {
        match self.globals.get(name) {
            Some(GlobalKind::Var(uid, type_)) => ScopeGetResult::Ref(*uid, type_.clone()),
            Some(GlobalKind::Func(uid, type_))
                => ScopeGetResult::Ref(*uid, TypeKind::Function(type_.clone())),
            Some(GlobalKind::Class(uid, type_)) => ScopeGetResult::Class(*uid, type_.clone()),
            None => ScopeGetResult::None,
        }
    }

    pub fn declare_var(&mut self, name: String, type_kind: TypeKind) {
        self.globals
            .insert(name, GlobalKind::Var(self.uid, type_kind));
    }

    pub fn declare_func(&mut self, name: String, func_type: FunctionType) {
        self.globals
            .insert(name, GlobalKind::Func(self.uid, func_type));
    }

    pub fn declare_class(&mut self, name: String, class_type: Arc<ClassType>) {
        self.globals
            .insert(name, GlobalKind::Class(self.uid, class_type));
    }

    pub fn declare_external_func(&mut self, name: String, module: ModuleUID, func_type: FunctionType) {
        self.globals
            .insert(name, GlobalKind::Func(module, func_type));
    }

    pub fn declare_external_var(&mut self, name: String, module: ModuleUID, type_: TypeKind) {
        self.globals
            .insert(name, GlobalKind::Var(module, type_));
    }

    pub fn declare_external_class(&mut self, name: String, module: ModuleUID, class_type: Arc<ClassType>) {
        self.globals
            .insert(name, GlobalKind::Class(module, class_type));
    }
}