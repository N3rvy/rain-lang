#![feature(explicit_generic_args_with_impl_trait)]

use core::{AnyValue, Engine, EngineGetFunction, InternalFunction, EngineExternalModule};
use std::{env, env::args, ops::Index};
use common::module::{ModuleIdentifier, ModuleUID};
use interpreter::{InterpreterEngine, InterpreterFunction};
use core::parser::ModuleImporter;
use core::external_module::{ExternalModule, ExternalModuleSetFunction, ExternalModuleSetValue};
use interpreter::external_module::InterpreterExternalModule;

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

    // Creating the identifier of the definitions module
    let std_identifier = ModuleIdentifier("std".to_string());

    // Creating the external definitions module
    let mut std_module = InterpreterExternalModule::new(
        &mut engine,
        &std_identifier,
        &ReplImporter).unwrap();

    std_module.set_function("times4", |x: i32| x * 4);
    std_module.set_function("print", |value: AnyValue| println!("{}", value.to_string()));
    std_module.set_value("extvalue", 10);

    engine.insert_external_module(std_module);

    // Creating the module from the source file
    let module = engine
        .load_module(source_path.to_string(), &ReplImporter)?;

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
    fn get_unique_identifier(&self, identifier: &ModuleIdentifier) -> Option<ModuleUID> {
        Some(ModuleUID::from_string(identifier.0.clone()))
    }

    fn load_module(&self, identifier: &ModuleIdentifier) -> Option<String> {
        let mod_path = match env::current_dir() {
            Ok(path) => path,
            Err(_) => return None,
        };
        let mod_path = mod_path.join(&identifier.0);

        let source = std::fs::read_to_string(mod_path);
        let source = match source {
            Ok(source) => source,
            Err(_) => return None,
        };
        Some(source)
    }
}