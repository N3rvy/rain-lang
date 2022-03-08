#![feature(unboxed_closures)]

pub use common::errors::LangError;
pub use engine::{Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
pub use externals::{ExternalType, AnyValue};

mod engine;
mod externals;
pub mod module;
mod errors;

pub mod parser {
    pub use parser::modules::module_importer::{ModuleIdentifier, ModuleUID, ModuleImporter};
    pub use parser::modules::loading_module::LoadingModule;
}