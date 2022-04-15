#![feature(unboxed_closures)]
#![feature(generic_associated_types)]

pub use common::errors::LangError;
pub use engine::{Engine, EngineGetFunction, InternalFunction, EngineBuildSource, EngineExternalModule};
pub use externals::{ExternalType, AnyValue};

mod engine;
mod externals;
pub mod module;
pub mod module_store;
pub mod external_module;

pub mod parser {
    pub use parser::modules::module_importer::ModuleImporter;
    pub use parser::modules::module_initializer::ParsableModule;
    pub use parser::modules::module_loader::{ModuleLoader, ModuleKind};
}

pub mod reexport {
    pub use anyhow;
}