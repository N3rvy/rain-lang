use core::{Engine, parser::ModuleImporter, EngineBuildSource};
use std::{env::{self, args}, ops::Index, fs::File, io::Write};
use common::module::{ModuleIdentifier, ModuleUID};
use wasm::engine::WasmEngine;


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
    let mut engine = WasmEngine::new();

    // Creating the module from the source file
    let module = engine
        .load_module(source_path.to_string(), &ReplImporter)?;

    let wasm = engine.build_module_source(module)?;

    let path = env::current_dir()?.join("output.wasm");
    let mut file = File::create(&path)?;
    file.write_all(wasm.as_slice())?;

    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_binary(&engine, wasm.as_slice())?;
    let linker = wasmtime::Linker::new(&engine);
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module)?;

    let memory = instance.get_memory(&mut store, "mem").unwrap();

    let run = instance.get_typed_func::<(), i32, _>(&mut store, "run")?;
    let result = run.call(&mut store, ())? as usize;

    let data = memory.data(&mut store);

    let str_len = u32::from_be_bytes([data[result], data[result+1], data[result+2], data[result+3]]) as usize;
    let str = String::from_utf8(data[result+4..result+4+str_len].to_vec()).unwrap();

    println!("result: {}", str);
    
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