use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct ModuleIdentifier(pub String);

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct ModuleUID(u64);

impl ModuleUID {
    pub fn from_string(string: String) -> Self {
        let mut hasher = DefaultHasher::new();
        string.hash(&mut hasher);

        ModuleUID(hasher.finish())
    }
}
