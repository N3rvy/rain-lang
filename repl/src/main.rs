use std::io::{BufRead, stdin};
use reverse::{Vm, IntoScript, Importer, ImportResult, LangValue, IntoExternalFunctionRunner};


fn main() {
    let vm = Vm::<ReplImporter>::new();
    
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
    fn import(&self, _identifier: &String) -> ImportResult {
        ImportResult::NotFound
    }
}