#![feature(unboxed_closures)]

pub use common::errors::LangError;
pub use engine::{Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
pub use externals::{ExternalType, AnyValue};

mod engine;
mod externals;
pub mod module;
pub mod module_builder;