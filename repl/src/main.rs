use core::{AnyValue, Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
use std::{io::Error, env::args, ops::Index, fs};
use interpreter::{InterpreterEngine, InterpreterFunction};

fn main() -> Result<(), Error> {
    // *** ATTENTION ***
    // This is a temporary solution and this is not a real REPL

    // Obtaining the args
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("Invalid argument count");
    }

    // Obtaining the source file
    let source_path = args.index(1);
    let source = fs::read_to_string(source_path)?;

    // Creating the engine
    let mut engine = InterpreterEngine::new();
    engine.set_function("print", print);
    engine.set_function("sum", |a: i32, b: i32| a + b);

    // Creating the module from the source file
    let module = engine
        .build_module()
        .with_source(source)
        .build();

    let module = match module {
        Ok(module) => module,
        Err(err) => {
            panic!("{}", err);
        },
    };

    // Obtaning the main function inside the module
    let func: InterpreterFunction<(), AnyValue> = match engine.get_function(&module, "main") {
        Some(func) => func,
        None => panic!("Main function not found"),
    };

    // Printing the return value of the function
    println!("{:?}", func.call(()));
    
    Ok(())
}

fn print(val: AnyValue) {
    println!("{}", val.to_string());
}