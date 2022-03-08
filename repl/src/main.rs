use core::{AnyValue, Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
use std::{env::args, ops::Index};
use std::process::id;
use common::errors::LangError;
use interpreter::{InterpreterEngine, InterpreterFunction};
use core::parser::{ModuleIdentifier, ModuleImporter, ModuleUID};

fn main() -> anyhow::Result<()> {
    // *** ATTENTION ***
    // This is a temporary solution and this is not a real REPL

    // Obtaining the args
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("Invalid argument count");
    }

    // Obtaining the source file
    let source_path = args.index(1);

    // Creating the engine
    let mut engine = InterpreterEngine::new();
    engine.set_function("print", print);
    engine.set_function("sum", |a: i32, b: i32| a + b);

    // Creating the module from the source file
    let module = engine
        .load_module::<ReplImporter>(&ModuleIdentifier(source_path.to_string()))?;

    // Obtaning the main function inside the module
    let func: InterpreterFunction<(), AnyValue> = match engine.get_function(module, "main") {
        Some(func) => func,
        None => panic!("Main function not found"),
    };

    // Printing the return value of the function
    println!("{:?}", func.call(()));
    
    Ok(())
}

struct ReplImporter;

impl ModuleImporter for ReplImporter {
    fn get_unique_identifier(identifier: &ModuleIdentifier) -> Option<ModuleUID> {
        Some(ModuleUID::from_string(identifier.0.clone()))
    }

    fn load_module(identifier: &ModuleIdentifier) -> Option<String> {
        let args = args().collect::<Vec<String>>();
        let exec_path = args.get(0).unwrap();

        dbg!(&identifier.0);
        let source = std::fs::read_to_string(&identifier.0);
        let source = match source {
            Ok(source) => source,
            Err(_) => return None,
        };
        Some(source)
    }
}

fn print(val: AnyValue) {
    println!("{}", val.to_string());
}