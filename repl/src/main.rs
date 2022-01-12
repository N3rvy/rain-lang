use std::io::{BufRead, stdin};
use reverse::{Vm, IntoScript, LangError};


fn main() -> Result<(), LangError> {
    let vm = Vm::new();
    
    for script in stdin().lock().lines() {
        if let Ok(script) = script {
            println!("{:?}", vm.evaluate(&script.script()?));
        }
    }
    
    Ok(())
}
