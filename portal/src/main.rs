mod args;
mod config;
mod build;
mod init;

use std::env;
use build::build;
use init::init;
use core::{Engine, parser::ModuleImporter, EngineBuildSource};
use clap::Parser;
use common::module::{ModuleIdentifier, ModuleUID};
use crate::args::{Args, Task};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.task {
        Task::Init => {
            init(args)
        },
        Task::Build => {
            build(args)
        },
    }
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