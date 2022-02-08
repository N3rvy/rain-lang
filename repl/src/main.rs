use core::{AnyValue, Engine, EngineSetFunction, EngineGetFunction, InternalFunction, LangError};
use std::io::{stdin, BufRead};
use interpreter::InterpreterEngine;

fn main() {
    let engine = InterpreterEngine::new();

    engine.set_function("print", print);
    
    for script in stdin().lock().lines() {
        if let Ok(script) = script {
            let module = match engine.create_module(script) {
                Ok(module) => module,
                Err(err) => {
                    println!("{}", err);
                    continue
                },
            };

            let func = match EngineGetFunction::<'_, (), Result<i32, LangError>, _>::get_function(&engine, &module, "main") {
                Some(func) => func,
                None => continue,
            };
            println!("{:?}", InternalFunction::<(), Result<i32, LangError>>::call(&func, ()));
        }
    }
}

fn print(val: AnyValue) {
    println!("{}", val.to_string());
}