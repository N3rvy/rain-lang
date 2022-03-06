use common::errors::LangError;
use tokenizer::iterator::Tokens;
use crate::modules::module_loader::ModuleLoader;

pub enum ImportResult {
    Ok(String, Tokens),
    NotFound,
}

pub struct Identifier(pub String);
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct UniqueIdentifier(pub String);

pub trait ModuleImporter {
    /// Returns an unique identifier of a specified module identifier.
    /// It is used to check if the same module is already been loaded.
    /// Example: "../player/movement.vrs" -> "src/player/movement"
    fn get_unique_identifier(&self, identifier: Identifier) -> Result<UniqueIdentifier, LangError>;

    /// Returns the code of a module as a string
    fn load_module(&self, unique_identifier: &UniqueIdentifier) -> Result<String, LangError>;
}