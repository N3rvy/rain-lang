use core::{Engine, Importer, ImportResult};
use std::{io::{BufRead, stdin}, fs};

use interpreter::Interpreter;

fn main() {
    let engine = Engine::new(ReplImporter::default(), Interpreter::new());
    
    // TODO: Reimplement engine.register("print", print.external());
    
    for script in stdin().lock().lines() {
        if let Ok(script) = script {
            match engine.execute(script) {
                Ok(result) => println!("{:?}", result),
                Err(err) => println!("{}", err),
            }
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
