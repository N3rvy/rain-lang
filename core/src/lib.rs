#![feature(unboxed_closures)]
#![feature(generic_associated_types)]

pub use common::errors::LangError;
pub use engine::{Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
pub use externals::{ExternalType, AnyValue};

mod engine;
mod externals;
pub mod module;
mod errors;
pub mod engine_module_loader;

pub mod parser {
    pub use parser::modules::module_importer::ModuleImporter;
    pub use parser::modules::module_parser::ParseModule;
}