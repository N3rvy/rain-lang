use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use anyhow::anyhow;
use binaryen::CodegenConfig;
use common::constants::CORE_MODULE_ID;
use common::module::{ModuleIdentifier, ModuleUID};
use wasm::engine::WasmEngine;
use crate::{Args, Engine, EngineBuildSource, ReplImporter};
use crate::config::Config;

pub fn build(args: Args) -> anyhow::Result<()> {
    let config_str = read_to_string(args.module)?;
    let config = serde_json::from_str::<Config>(config_str.as_str())?;

    // Creating the engine
    let mut engine = WasmEngine::new();

    let importer = ReplImporter {
        src_dir: PathBuf::from(&config.src_dir),
    };

    // Loading core lib
    engine.module_loader()
        .load_module_with_source(
            ModuleIdentifier(CORE_MODULE_ID.to_string()),
            ModuleUID::from_string(CORE_MODULE_ID.to_string()),
            &include_str!("../../core_lib/lib.vrs").to_string(),
            &importer,
        )?;

    // Creating the module from the source file
    let module = engine
        .load_module(config.main, &importer)?;

    let pre_opt_wasm = engine.build_module_source(module)?;

    let wasm = match args.release {
        false => pre_opt_wasm,
        true => {
            let mut module = binaryen::Module::read(pre_opt_wasm.as_slice())
                .map_err(|_| anyhow!("Error while reading the built wasm"))?;
            module.optimize(&CodegenConfig {
                shrink_level: 1,
                optimization_level: 2,
                debug_info: false,
            });
            module.write()
        }
    };


    let path = env::current_dir()?.join(config.build_path);
    let mut file = File::create(&path)?;
    file.write_all(wasm.as_slice())?;

    println!("Build successful! Output file at {}", path.to_str().unwrap());

    Ok(())
}