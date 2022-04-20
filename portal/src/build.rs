use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use common::module::ModuleUID;
use wasm::engine::WasmEngine;
use crate::{Args, Engine, EngineBuildSource, ReplImporter};
use crate::config::Config;

pub fn build(args: Args) -> anyhow::Result<()> {
    let config_str = read_to_string(args.module)?;
    let config = serde_json::from_str::<Config>(config_str.as_str())?;

    // Creating the engine
    let mut engine = WasmEngine::new();

    // Loading core lib
    engine.module_loader()
        .load_module_with_source(
            ModuleUID::from_string("core".to_string()),
            &include_str!("../core_lib/lib.vrs").to_string(),
            &ReplImporter,
        )?;

    let def_path = PathBuf::from_str(config.definition_dir.as_str())?;
    for (def_name, file_name) in config.definitions.iter() {
        engine.load_def_module(
            def_path
                .join(file_name)
                .to_str()
                .unwrap(),
            def_name,
            &ReplImporter)?;
    }

    // Creating the module from the source file
    let main_path = Path::new(config.src_dir.as_str()).join(config.main);

    let module = engine
        .load_module(main_path.to_str().unwrap(), &ReplImporter)?;

    let wasm = engine.build_module_source(module)?;

    let path = env::current_dir()?.join(config.build_path);
    let mut file = File::create(&path)?;
    file.write_all(wasm.as_slice())?;

    println!("Build successfull! Output file at {}", path.to_str().unwrap());

    Ok(())
}