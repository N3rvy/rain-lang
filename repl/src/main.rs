use std::io::{BufRead, stdin};
use reverse::{Vm, IntoScript};


fn main() {
    let vm = Vm::new();
    
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
