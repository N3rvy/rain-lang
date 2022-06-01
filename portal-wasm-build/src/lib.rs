use core::parser::ModuleImporter;
use core::{Engine, EngineBuildSource};
use common::module::{ModuleIdentifier, ModuleUID};
use wasm::engine::WasmEngine;
use wasm_bindgen::prelude::*;

static mut MODULES: Vec<(String, String)> = Vec::new();
static mut ENGINE: Option<WasmEngine> = None;

#[wasm_bindgen]
pub fn add_module(id: &str, module: &str) {
    unsafe {
        MODULES.push((id.to_string(), module.to_string()));
    }
}

#[wasm_bindgen]
pub fn init_engine() {
    unsafe {
        let mut engine = WasmEngine::new();

        engine.module_loader()
            .load_module_with_source(
                ModuleIdentifier("core".to_string()),
                ModuleUID::from_string("core".to_string()),
                &include_str!("../../core_lib/lib.vrs").to_string(),
                &PanicImporter,
            ).expect("Error while loading core module :-|");

        for (id, module) in &MODULES {
            engine.module_loader()
                .load_module_with_source(
                    ModuleIdentifier(id.to_string()),
                    ModuleUID::from_string(id.to_string()),
                    module,
                    &PanicImporter,
                ).expect("Error while loading imported module :-|");
        }

        ENGINE = Some(engine);
    }
}

#[wasm_bindgen]
pub fn build_from_code(code: &str) -> Vec<u8> {
    unsafe {
        let engine = match &mut ENGINE {
            Some(engine) => engine,
            None => panic!("Engine not initialized"),
        };

        let (module, _) = engine
            .module_loader()
            .load_module_with_source(
                ModuleIdentifier("main".to_string()),
                ModuleUID::from_string("main".to_string()),
                &code.to_string(),
                &PanicImporter
            ).unwrap();

        engine.build_module_source(module.uid).unwrap()
    }
}

struct PanicImporter;
impl ModuleImporter for PanicImporter {
    fn get_unique_identifier(&self, id: &ModuleIdentifier) -> Option<ModuleUID> {
        Some(ModuleUID::from_string(id.0.clone()))
    }

    fn load_module(&self, _: &ModuleIdentifier) -> Option<String> {
        panic!("PanicImporter::load_module");
    }
}