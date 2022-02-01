use core::{Engine, Importer, ImportResult, AnyValue};
use std::{io::{BufRead, stdin}, fs};

use interpreter::Interpreter;

fn main() {
    let engine = Engine::new(ReplImporter::default(), Interpreter::default());
    
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
            println!("{:?}", func(&engine.execution_engine, &module));
        }
    }
}

// fn print(value: LangValue) {
//     println!("{}", value.to_string());
// }

#[derive(Default)]
struct ReplImporter;

impl Importer for ReplImporter {
    fn import(&self, identifier: &String) -> ImportResult {
        match fs::read_to_string(identifier) {
            Ok(script) => ImportResult::Imported(script),
            Err(_) => ImportResult::NotFound,
        }
    }
}
