use core::{Engine, parser::ModuleImporter, EngineBuildSource};
use std::{env::{self, args}, ops::Index, fs::File, io::Write};
use wasmtime::{Caller, Extern, FuncType, Trap, Val, ValType};
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

    engine.load_def_module("repl.d.vrs", "repl", &ReplImporter)?;

    // Creating the module from the source file
    let module = engine
        .load_module(source_path.to_string(), &ReplImporter)?;

    let wasm = engine.build_module_source(module)?;

    let path = env::current_dir()?.join("output.wasm");
    let mut file = File::create(&path)?;
    file.write_all(wasm.as_slice())?;

    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_binary(&engine, wasm.as_slice())?;
    let mut linker = wasmtime::Linker::new(&engine);
    let mut store = wasmtime::Store::new(&engine, ());

    linker.func_new(
        "repl",
        "printI",
        FuncType::new([ValType::I32], []),
        |_, params, _| {
            match params {
                [Val::I32(i)] => println!("LOG: {}", i),
                _ => (),
            };
            Ok(())
        },
    )?;

    linker.func_new(
        "repl",
        "print",
        FuncType::new([ValType::I32], []),
        print_str)?;

    let instance = linker.instantiate(&mut store, &module)?;
    let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;
    run.call(&mut store, ())?;

    Ok(())
}

fn print_str(mut caller: Caller<()>, params: &[Val], _ret: &mut [Val]) -> Result<(), Trap> {
    match params {
        [Val::I32(i)] => {
            let i = *i as usize;

            let mem = match caller.get_export("mem") {
                Some(Extern::Memory(mem)) => mem,
                _ => return Ok(()),
            };

            let data = mem.data(&mut caller);

            let str_len = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]) as usize;
            let str = String::from_utf8(data[i+4..i+4+str_len].to_vec()).unwrap();

            println!("LOG: {}", str);
        },
        _ => (),
    }

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