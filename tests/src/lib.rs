pub mod errors;
use core::{Engine, LangError, EngineGetFunction, AnyValue, InternalFunction};

use errors::{INIT_NOT_FOUND, WRONG_RESULT};
use interpreter::{InterpreterEngine, InterpreterFunction};
use lazy_static::lazy_static;
pub mod functions;

lazy_static! {
    pub static ref ENGINE: InterpreterEngine<'static> = Engine::new();
}

pub fn test_script(source: &str, value: AnyValue) -> Result<(), LangError> {
    let module = ENGINE
        .build_module()
        .with_source(source.to_string())
        .build()?;

    let func: InterpreterFunction<(), AnyValue> = match ENGINE .get_function(&module, "init") {
        Some(func) => func,
        None => return Err(LangError::new_runtime(INIT_NOT_FOUND.to_string())),
    };

    let result = func.call(())?;

    if !val_equal(&result, &value) {
        return Err(LangError::new_runtime(WRONG_RESULT.to_string()));
    }

    Ok(())
}

fn val_equal(a: &AnyValue, b: &AnyValue) -> bool {
    match (a, b) {
        (AnyValue::Nothing, AnyValue::Nothing) => true,
        (AnyValue::Int(a), AnyValue::Int(b)) => a == b,
        (AnyValue::Float(a), AnyValue::Float(b)) => a == b,
        (AnyValue::Bool(a), AnyValue::Bool(b)) => a == b,
        (AnyValue::String(a), AnyValue::String(b)) => a == b,
        _ => false,
    }
}