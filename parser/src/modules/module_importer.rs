use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use common::errors::LangError;
use tokenizer::iterator::Tokens;

pub enum ImportResult {
    Ok(String, Tokens),
    NotFound,
}

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

pub trait ModuleImporter {
    /// Returns an unique identifier of a specified module identifier.
    /// It is used to check if the same module is already been loaded.
    /// Example: "../player/movement.vrs" -> "src/player/movement"
    fn get_unique_identifier(identifier: &ModuleIdentifier) -> Option<ModuleUID>;

    /// Returns the code of a module as a string
    fn load_module(identifier: &ModuleIdentifier) -> Option<String>;
}