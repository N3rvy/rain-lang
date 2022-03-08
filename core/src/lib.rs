#![feature(unboxed_closures)]
#![feature(generic_associated_types)]

pub use common::errors::LangError;
pub use engine::{Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
pub use externals::{ExternalType, AnyValue};

mod engine;
mod externals;
pub mod module;
mod errors;
pub mod module_builder;

pub mod parser {
    pub use parser::modules::module_importer::{ModuleIdentifier, ModuleUID, ModuleImporter};
    pub use parser::modules::loading_module::LoadingModule;
}