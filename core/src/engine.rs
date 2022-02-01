use common::errors::LangError;
use parser::{parser::parse, type_check::check_types};
use tokenizer::tokenizer::tokenize;

use crate::{import::Importer, execution_engine::ExecutionEngine, externals::ExternalType};


pub struct Engine<Imp: Importer, Exec: ExecutionEngine> {
    importer: Imp,
    pub execution_engine: Exec,
}

impl<Imp: Importer, Exec: ExecutionEngine> Engine<Imp, Exec> {
    pub fn new(importer: Imp, execution_engine: Exec) -> Self {
        Self {
            importer,
            execution_engine,
        }
    }

    pub fn execute(&self, script: String) -> Result<(), LangError> {
        let tokens = tokenize(script)?;
        let ast = parse(tokens)?;
        
        check_types(&ast)?;

        self.execution_engine.execute(ast)
    }

    pub fn execute_import(&self, identifier: &String) -> Result<Option<()>, LangError> {
        let script = match self.importer.import(identifier) {
            crate::ImportResult::Imported(script) => script,
            crate::ImportResult::ImportError(err) => return Err(err),
            crate::ImportResult::AlreadyImported => return Ok(Some(())),
            crate::ImportResult::NotFound => return Ok(None),
        };

        match self.execute(script) {
            Ok(_) => Ok(Some(())),
            Err(err) => Err(err),
        }
    }

    pub fn get_function<Ret: ExternalType>(&self, name: &str) -> Option<Box<dyn Fn(&Exec) -> Result<Ret, LangError>>>
    {
        self.execution_engine.get_function(name)
    }
}