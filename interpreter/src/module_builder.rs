use core::{module_builder::ModuleBuilder, LangError, Engine};

use common::ast::ASTNode;

use crate::{InterpreterModule, InterpreterEngine, scope::Scope};


pub struct InterpreterModuleBuilder;

impl<'a> ModuleBuilder<'a> for InterpreterModuleBuilder {
    type Engine = InterpreterEngine<'a>;

    fn build(engine: &'a Self::Engine, asts: Vec<ASTNode>) -> Result<<Self::Engine as Engine<'a>>::Module, LangError> {
        let scope = Scope::new_child(
            &engine.global_module.scope);
        
        for ast in asts {
            scope.evaluate_ast(&ast);
        }

        Ok(InterpreterModule::new(scope))
    }
}