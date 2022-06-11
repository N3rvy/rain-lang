use core::parser::ModuleImporter;
use core::{Engine, EngineBuildSource};
use common::module::{ModuleIdentifier, ModuleUID};
use wasm::engine::WasmEngine;
use wasm_bindgen::prelude::*;
use common::constants::CORE_MODULE_ID;

static mut MODULES: Vec<(String, String)> = Vec::new();
static mut ENGINE: Option<WasmEngine> = None;

#[wasm_bindgen]
pub fn add_module(id: &str, module: &str) {
    unsafe {
        MODULES.push((id.to_string(), module.to_string()));
    }
}

#[wasm_bindgen]
pub fn init_engine() -> Result<(), JsValue> {
    unsafe {
        let mut engine = WasmEngine::new();

        match engine.module_loader()
            .load_module_with_source(
                ModuleIdentifier(CORE_MODULE_ID.to_string()),
                ModuleUID::from_string(CORE_MODULE_ID.to_string()),
                &include_str!("../../core_lib/lib.rn").to_string(),
                &PanicImporter,
            ) {
            Err(err) => return Err(JsValue::from(err.to_string())),
            _ => ()
        }

        for (id, module) in &MODULES {
            match engine.module_loader()
                .load_module_with_source(
                    ModuleIdentifier(id.to_string()),
                    ModuleUID::from_string(id.to_string()),
                    module,
                    &PanicImporter,
                ) {
                Err(err) => return Err(JsValue::from(err.to_string())),
                _ => ()
            }
        }

        ENGINE = Some(engine);

        Ok(())
    }
}

#[wasm_bindgen]
pub fn build_from_code(code: &str) -> Result<Vec<u8>, JsValue> {
    unsafe {
        let engine = match &mut ENGINE {
            Some(engine) => engine,
            None => panic!("Engine not initialized"),
        };

        let (module, _) = match engine
            .module_loader()
            .load_module_with_source(
                ModuleIdentifier("main".to_string()),
                ModuleUID::from_string("main".to_string()),
                &code.to_string(),
                &PanicImporter
            ) {
            Ok(val) => val,
            Err(err) => return Err(JsValue::from(err.to_string()))
        };

        match engine.build_module_source(module.uid) {
            Ok(data) => Ok(data),
            Err(err) => Err(JsValue::from(err.to_string()))
        }
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