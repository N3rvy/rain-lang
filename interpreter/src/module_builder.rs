use core::{module_builder::ModuleBuilder, LangError, Engine};
use common::ast::module::ASTModule;
use crate::{InterpreterModule, InterpreterEngine, scope::Scope, lang_value::LangValue, evaluate::EvalResult};


pub struct InterpreterModuleBuilder;

impl<'a> ModuleBuilder<'a> for InterpreterModuleBuilder {
    type Engine = InterpreterEngine<'a>;

    fn build(engine: &'a Self::Engine, modules: Vec<ASTModule>) -> Result<<Self::Engine as Engine<'a>>::Module, LangError> {
        let scope = Scope::new_child(
            &engine.global_module.scope);
        
        for module in modules {
            for (func_name, func) in module.functions {
                scope.declare_var(func_name.clone(), LangValue::Function(func.clone()));
            }

            for (var_name, var) in module.variables {
                let value = match scope.evaluate_ast(&var) {
                    EvalResult::Ok(value) => value,
                    EvalResult::Ret(value, _) => value,
                    EvalResult::Err(err) => return Err(err),
                };
                scope.declare_var(var_name.clone(), value);
            }
        }

        Ok(InterpreterModule::new(scope))
    }
}