mod args;

use core::{Engine, parser::ModuleImporter, EngineBuildSource};
use std::{env, ops::Index, fs::File, io::Write};
use std::env::Args;
use common::module::{ModuleIdentifier, ModuleUID};
use wasm::engine::WasmEngine;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Creating the engine
    let mut engine = WasmEngine::new();

    if let Some(def) = args.def {
        let split = def.split(':').collect::<Vec<&str>>();

        engine.load_def_module(*split.index(0), *split.index(1), &ReplImporter)?;
    }

    // Creating the module from the source file
    let module = engine
        .load_module(args.main, &ReplImporter)?;

    let wasm = engine.build_module_source(module)?;

    let path = env::current_dir()?.join(args.out);
    let mut file = File::create(&path)?;
    file.write_all(wasm.as_slice())?;

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