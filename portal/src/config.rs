use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    #[serde(default)]
    pub src_dir: String,
    #[serde(default = "default_main")]
    pub main: String,
    #[serde(default)]
    pub definition_dir: String,
    #[serde(default)]
    pub definitions: HashMap<String, String>,
    pub build_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "Project name".to_string(),
            src_dir: "src".to_string(),
            main: "main.vrs".to_string(),
            definition_dir: "".to_string(),
            definitions: HashMap::new(),
            build_path: "output.wasm".to_string(),
        }
    }
}

fn default_main() -> String { "main.vrs".to_string() }