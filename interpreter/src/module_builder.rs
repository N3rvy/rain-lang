use core::{module_builder::ModuleBuilder, LangError, Engine};

use crate::{InterpreterModule, InterpreterEngine, scope::Scope};


pub struct InterpreterModuleBuilder;

impl<'a> ModuleBuilder<'a> for InterpreterModuleBuilder {
    type Engine = InterpreterEngine<'a>;

    fn build(engine: &'a Self::Engine, sources: &Vec<String>) -> Result<<Self::Engine as Engine<'a>>::Module, LangError> {
        let scope = Scope::new_child(
            &engine.global_module.scope);
        
        for source in sources {
            let ast = InterpreterEngine::source_to_ast(source)?;
            
            scope.evaluate_ast(&ast);
        }

        Ok(InterpreterModule::new(scope))
    }
}