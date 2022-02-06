use core::{AnyValue, Engine, EngineSetFunction};
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

            let func = match engine.get_function::<AnyValue>(&module, "main") {
                Some(func) => func,
                None => continue,
            };
            println!("{:?}", func(&engine, &module));
        }
    }
}

fn print(val: AnyValue) {
    println!("{}", val.to_string());
}