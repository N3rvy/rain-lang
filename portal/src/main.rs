mod args;
mod config;
mod build;
mod init;

use std::fs;
use std::path::PathBuf;
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

struct ReplImporter {
    src_dir: PathBuf,
}

impl ReplImporter {
    fn get_path(&self, identifier: &ModuleIdentifier) -> PathBuf {
        let mut path = identifier.0.clone();
        if path.starts_with("/") {
            path.remove(0);
        }
        path += ".rn";

        self.src_dir.join(path)
    }
}

impl ModuleImporter for ReplImporter {
    fn get_unique_identifier(&self, identifier: &ModuleIdentifier) -> Option<ModuleUID> {
        let uid = if identifier.0.starts_with("/") {
            let path = self.get_path(identifier);

            let path = fs::canonicalize(&path).ok()?;
            path.to_str().unwrap().to_string()
        } else {
            identifier.0.clone()
        };

        Some(ModuleUID::from_string(uid))
    }

    fn load_module(&self, identifier: &ModuleIdentifier) -> Option<String> {
        let path = self.get_path(identifier);

        fs::read_to_string(path.to_str()?).ok()
    }
}