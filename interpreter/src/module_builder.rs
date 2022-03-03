use core::{module_builder::ModuleBuilder, LangError, Engine};

use common::ast::ASTNode;

use crate::{InterpreterModule, InterpreterEngine, scope::Scope, evaluate::EvaluateAST};


pub struct InterpreterModuleBuilder;

impl<'a> ModuleBuilder<'a> for InterpreterModuleBuilder {
    type Engine = InterpreterEngine;

    fn build(engine: &'a Self::Engine, asts: Vec<ASTNode>) -> Result<<Self::Engine as Engine<'a>>::Module, LangError> {
        let scope = Scope::new_child(
            engine
                .global_module
                .scope
                .clone());
        
        for ast in asts {
            scope.clone().evaluate_ast(&ast);
        }

        Ok(InterpreterModule::new(scope))
    }
}