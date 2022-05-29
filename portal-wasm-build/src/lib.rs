use core::parser::ModuleImporter;
use core::{Engine, EngineBuildSource};
use common::module::{ModuleIdentifier, ModuleUID};
use wasm::engine::WasmEngine;

#[wasm_bindgen]
pub fn build_from_code(code: &str) -> Vec<u8> {
    let mut engine = WasmEngine::new();

    engine.module_loader()
        .load_module_with_source(
            ModuleIdentifier("core".to_string()),
            ModuleUID::from_string("core".to_string()),
            &include_str!("../../core_lib/lib.vrs").to_string(),
            &PanicImporter,
        ).expect("Error while loading core module :-|");

    let (module, _) = engine.module_loader()
        .load_module_with_source(
            ModuleIdentifier("main".to_string()),
            ModuleUID::from_string("main".to_string()),
            &code.to_string(),
            &PanicImporter
        ).unwrap();

    engine.build_module_source(module.uid).unwrap()
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