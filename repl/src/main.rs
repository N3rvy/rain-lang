use core::Engine;
use std::{io::{BufRead, stdin}, fs};

fn main() {
    let vm = Engine::<ReplImporter>::default();
    
    vm.register("print", print.external());
    
    for script in stdin().lock().lines() {
        if let Ok(script) = script {
            match script.script() {
                Ok(script) => match vm.evaluate(&script) {
                    Ok(result) => println!("{:?}", result),
                    Err(err) => println!("{}", err),
                },
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn print(value: LangValue) {
    println!("{}", value.to_string());
}

#[derive(Default)]
struct ReplImporter;

impl Importer for ReplImporter {
    fn import(&self, identifier: &String) -> ImportResult {
        match fs::read_to_string(identifier) {
            Ok(script) => match script.script() {
                Ok(script) => ImportResult::Imported(script),
                Err(err) => ImportResult::ImportError(err),
            },
            Err(_) => ImportResult::NotFound,
        }
    }
}
