use core::{AnyValue, Engine};
use std::io::{stdin, BufRead};
use interpreter::InterpreterEngine;

fn main() {
    let engine: InterpreterEngine = Engine::new();
    
    // TODO: Reimplement engine.register("print", print.external());
    
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

// fn print(value: LangValue) {
//     println!("{}", value.to_string());
// }