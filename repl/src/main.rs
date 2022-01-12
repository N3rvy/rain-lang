use std::io::{BufRead, stdin};
use reverse::{Vm, IntoScript};


fn main() {
    let vm = Vm::new();
    
    for script in stdin().lock().lines() {
        if let Ok(script) = script {
            match script.script() {
                Ok(script) => println!("{:?}", vm.evaluate(&script)),
                Err(err) => println!("{}", err),
            }
        }
    }
}
