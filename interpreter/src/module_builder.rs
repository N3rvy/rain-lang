use core::{module_builder::ModuleBuilder, LangError, Engine};

use crate::{InterpreterModule, InterpreterEngine, scope::Scope};


pub struct InterpreterModuleBuilder;

impl<'a> ModuleBuilder<'a> for InterpreterModuleBuilder {
    type Module = InterpreterModule<'a>;
    type Engine = InterpreterEngine<'a>;

    fn new() -> Self { Self }
    fn build(&self, engine: &'a Self::Engine, sources: &Vec<String>) -> Result<Self::Module, LangError> {
        let scope = Scope::new_child(
            &engine.global_module.scope);
        
        for source in sources {
            let ast = InterpreterEngine::source_to_ast(source)?;
            
            scope.evaluate_ast(&ast);
        }

        Ok(InterpreterModule::new(scope))
    }
}