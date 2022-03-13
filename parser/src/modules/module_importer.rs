use common::module::ModuleUID;
use common::module::ModuleIdentifier;

pub trait ModuleImporter {
    /// Returns an unique identifier of a specified module identifier.
    /// It is used to check if the same module is already been loaded.
    /// Example: "../player/movement.vrs" -> "src/player/movement"
    fn get_unique_identifier(identifier: &ModuleIdentifier) -> Option<ModuleUID>;

    /// Returns the code of a module as a string
    fn load_module(identifier: &ModuleIdentifier) -> Option<String>;
}