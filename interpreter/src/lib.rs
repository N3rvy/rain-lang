#![feature(unboxed_closures)]
#![feature(try_trait_v2)]

use core::ExecutionEngine;
use common::ast::ASTNode;
use evaluate::EvalResult;
use scope::Scope;

mod scope;
mod evaluate;

pub struct Interpreter<'a> {
    global_scope: Scope<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            global_scope: Scope::new(),
        }
    }
}

impl<'a> ExecutionEngine for Interpreter<'a> {
    fn execute(&self, ast: ASTNode) -> Result<(), core::LangError> {
        match self.evaluate_ast(&self.global_scope, &ast) {
            EvalResult::Ok(_) | EvalResult::Ret(_, _) => Ok(()),
            EvalResult::Err(err) => Err(err),
        }
    }

    fn get_function<Args, Ret, F: Fn<Args, Output = Ret>>(&self, _name: &str) -> Option<F> {
        todo!()
    }
}