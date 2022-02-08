#![feature(unboxed_closures)]

pub use common::errors::LangError;
pub use engine::{Engine, EngineSetFunction, EngineGetFunction};
pub use externals::{ExternalType, AnyValue};

mod engine;
mod externals;