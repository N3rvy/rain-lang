use common::errors::LangError;
use parser::{parser::parse, type_check::check_types};
use tokenizer::tokenizer::tokenize;

use crate::{import::Importer, execution_engine::ExecutionEngine, externals::ExternalType};


pub struct Engine<Imp: Importer, Exec: ExecutionEngine> {
    _importer: Imp,
    pub execution_engine: Exec,
}

impl<Imp: Importer, Exec: ExecutionEngine> Engine<Imp, Exec> {
    pub fn new(importer: Imp, execution_engine: Exec) -> Self {
        Self {
            _importer: importer,
            execution_engine,
        }
    }

    pub fn create_module(&self, script: String) -> Result<Exec::Module, LangError> {
        let tokens = tokenize(script)?;
        let ast = parse(tokens)?;
        
        check_types(&ast)?;

        self.execution_engine.create_module(ast)
    }

    pub fn get_function<Ret: ExternalType>(&self, module: &Exec::Module, name: &str) -> Option<Box<dyn Fn(&Exec, &Exec::Module) -> Result<Ret, LangError>>> {
        self.execution_engine.get_function(module, name)
    }
}