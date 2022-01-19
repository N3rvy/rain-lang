#![feature(unboxed_closures)]

pub use common::lang_value::LangValue;
pub use common::errors::LangError;
pub use common::external_functions::IntoExternalFunctionRunner;
pub use common::external_functions::ExternalFunctionRunner;
pub use common::external_functions::AsMethod;
pub use common::lang_value::Function;
pub use common::object::LangObject;
pub use engine::Engine;
pub use import::{ImportResult, Importer};
pub use execution_engine::ExecutionEngine;

mod engine;
mod import;
mod execution_engine;