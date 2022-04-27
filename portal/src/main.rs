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
    declaration_dir: PathBuf,
}

impl ReplImporter {
    fn get_path(&self, identifier: &ModuleIdentifier, declaration: bool) -> PathBuf {
        if declaration {
            self.declaration_dir.join(identifier.0.clone() + ".d.vrs")
        } else {
            self.src_dir.join(identifier.0.clone() + ".vrs")
        }
    }
}

impl ModuleImporter for ReplImporter {
    fn get_unique_identifier(&self, identifier: &ModuleIdentifier) -> Option<ModuleUID> {
        let path = self.get_path(identifier, false);

        let path = fs::canonicalize(&path).ok()?;
        Some(ModuleUID::from_string(path.to_str().unwrap().to_string()))
    }

    fn load_module(&self, identifier: &ModuleIdentifier, declaration: bool) -> Option<String> {
        let path = self.get_path(identifier, declaration);

        std::fs::read_to_string(path.to_str()?).ok()
    }
}