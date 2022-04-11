use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

fn default_main() -> String { "main.vrs".to_string() }