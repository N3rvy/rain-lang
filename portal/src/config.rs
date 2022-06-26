use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    #[serde(default)]
    pub src_dir: String,
    #[serde(default)]
    pub main: String,
    pub build_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "Project name".to_string(),
            src_dir: "src".to_string(),
            main: "main".to_string(),
            build_path: "output.wasm".to_string(),
        }
    }
}