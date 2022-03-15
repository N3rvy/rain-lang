use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use crate::ast::ASTNode;
use crate::ast::types::{Function, TypeKind};

pub struct ModuleIdentifier(pub String);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct ModuleUID(u64);

impl ModuleUID {
    pub fn from_string(string: String) -> Self {
        let mut hasher = DefaultHasher::new();
        string.hash(&mut hasher);

        ModuleUID(hasher.finish())
    }
}

pub struct ModuleMetadata {
    pub definitions: Vec<(String, TypeKind)>,
}

pub struct Module {
    pub uid: ModuleUID,
    pub imports: Vec<ModuleUID>,
    pub metadata: ModuleMetadata,

    pub functions: Vec<(String, Arc<Function>)>,
    pub variables: Vec<(String, ASTNode)>,
}
