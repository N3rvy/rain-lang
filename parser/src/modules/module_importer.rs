use common::module::ModuleUID;
use common::module::ModuleIdentifier;

/// User defined module import technique (from file system to http requests)
pub trait ModuleImporter {
    /// Returns an unique identifier of a specified module identifier.
    /// It is used to check if the same module is already been loaded.
    /// Example: "../player/movement.vrs" -> "src/player/movement"
    fn get_unique_identifier(&self, identifier: &ModuleIdentifier) -> Option<ModuleUID>;

    /// Returns the code of a module as a string
    fn load_module(&self, identifier: &ModuleIdentifier, declaration: bool) -> Option<String>;
}