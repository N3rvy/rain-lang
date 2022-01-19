use common::errors::LangError;
use parser::{parser::parse, type_check::check_types};
use tokenizer::tokenizer::tokenize;

use crate::{import::Importer, execution_engine::ExecutionEngine};


pub struct Engine<Imp: Importer, Exec: ExecutionEngine> {
    importer: Imp,
    execution_engine: Exec,
}

impl<Imp: Importer + Default, Exec: ExecutionEngine + Default> Default for Engine<Imp, Exec> {
    fn default() -> Self {
        Self {
            importer: Default::default(),
            execution_engine: Default::default()
        }
    }
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

    pub fn get_function<Args, Ret, F: Fn<Args, Output = Ret>>(&self, name: &str) -> Option<F> {
        self.execution_engine.get_function(name)
    }
}