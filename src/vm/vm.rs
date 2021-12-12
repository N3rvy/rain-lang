use std::ops::{Try, FromResidual, ControlFlow};

use crate::{ast::node::ASTNode, common::{lang_value::{LangValue, Function}, messages::{VARIABLE_IS_NOT_A_NUMBER, VARIABLE_NOT_DECLARED, VARIABLE_IS_NOT_A_FUNCTION, INCORRECT_NUMBER_OF_PARAMETERS}, types::ReturnKind}, error::LangError, tokenizer::tokens::{MathOperatorKind, BoolOperatorKind}};

use super::scope::Scope;


pub enum EvalResult {
    Ok(LangValue),
    Ret(LangValue, ReturnKind),
    Err(LangError),
}

impl FromResidual for EvalResult {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl Try for EvalResult {
    type Output = LangValue;
    type Residual = EvalResult;

    fn from_output(output: Self::Output) -> Self {
        EvalResult::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            EvalResult::Ok(value) => ControlFlow::Continue(value),
            EvalResult::Ret(value, kind) => ControlFlow::Break(EvalResult::Ret(value, kind)),
            EvalResult::Err(err) => ControlFlow::Break(EvalResult::Err(err)),
        }
    }
}

macro_rules! expect_some {
    ($value:expr, $err:expr) => {
        match $value {
            Some(val) => val,
            None => return EvalResult::Err(LangError::new_runtime($err)),
        }
    };
}


pub fn evaluate(ast: &Box<ASTNode>, scope: &mut Scope) -> EvalResult {
    match ast.as_ref() {
        ASTNode::Root { body } => {
            for child in body {
                evaluate(child, scope)?;
            }
            
            EvalResult::Ok(LangValue::Nothing)
        },
        ASTNode::VariableDecl { name, value } => {
            let value = evaluate(value, scope)?;
            scope.declare_var(name.clone(), value);

            EvalResult::Ok(LangValue::Nothing)
        },
        ASTNode::VaraibleRef { name } => {
            match scope.get_var(name) {
                Some(value) => EvalResult::Ok(value.clone()),
                None => EvalResult::Err(LangError::new_runtime(VARIABLE_NOT_DECLARED.to_string())),
            }
        },
        ASTNode::VariableAsgn { name, value } => {
            let value = evaluate(value, scope)?;
            scope.set_var(name, value);
            
            EvalResult::Ok(LangValue::Nothing)
        },
        ASTNode::FunctionInvok { variable, parameters } => {
            let func = evaluate(variable, scope)?;

            let args_count = match &func {
                LangValue::Function(func) => func.parameters.len(),
                LangValue::ExtFunction(func) => func.args_count(),
                _ => return EvalResult::Err(LangError::new_runtime(VARIABLE_IS_NOT_A_FUNCTION.to_string())),
            };
            
            // Parameters
            if parameters.len() != args_count {
                return EvalResult::Err(LangError::new_runtime(INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
            }
            
            let mut param_values = Vec::new();
            
            for param in parameters {
                let value = evaluate(param, scope)?;
                param_values.push(value);
            }
            
            match &func {
                LangValue::Function(func) => {
                    let mut func_scope = Scope::new(Some(scope));
                    for i in 0..parameters.len() {
                        // TODO: PLS BETTER PERFORMANCE! THANKS ME OF THE FUTURE
                        func_scope.declare_var(func.parameters[i].to_string(), param_values[i].clone());
                    }

                    for child in &func.body {
                        // Matching to make the return statement stop
                        match evaluate(child, &mut func_scope) {
                            EvalResult::Ok(_) => (),
                            EvalResult::Ret(value, ReturnKind::Return) => return EvalResult::Ok(value),
                            EvalResult::Ret(value, kind) => return EvalResult::Ret(value, kind),
                            EvalResult::Err(err) => return EvalResult::Err(err),
                        }
                    }
                    
                    EvalResult::Ok(LangValue::Nothing)
                },
                LangValue::ExtFunction(func) => {
                    match func.run(param_values) {
                        Ok(value ) => EvalResult::Ok(value),
                        Err(err) => EvalResult::Err(err),
                    }
                },
                _ => return EvalResult::Err(LangError::new_runtime(VARIABLE_IS_NOT_A_FUNCTION.to_string())),
            }
        },
        ASTNode::Literal { value } => {
            EvalResult::Ok(value.clone())
        },
        ASTNode::MathOperation { operation, left, right } => {
            let left = evaluate(left, scope)?;
            let right = evaluate(right, scope)?;
            
            let value = match operation {
                MathOperatorKind::Plus => left.sum(right),
                MathOperatorKind::Minus => left.minus(right),
                MathOperatorKind::Multiply => left.multiply(right),
                MathOperatorKind::Divide => left.divide(right),
                MathOperatorKind::Modulus => left.modulus(right),
                MathOperatorKind::Power => left.power(right),
            };
            
            EvalResult::Ok(value)
        },
        ASTNode::BoolOperation { operation, left, right } => {
            let left = evaluate(left, scope)?;
            let right = evaluate(right, scope)?;
            
            let value = match operation {
                BoolOperatorKind::Equal => left.equals(&right),
                BoolOperatorKind::Different => left.not_equals(&right),
                BoolOperatorKind::Bigger => left.bigger(&right),
                BoolOperatorKind::Smaller => left.smaller(&right),
                BoolOperatorKind::BiggerEq => left.bigger_eq(&right),
                BoolOperatorKind::SmallerEq => left.smaller_eq(&right),
            };
            
            EvalResult::Ok(LangValue::Bool(value))
        },
        ASTNode::ReturnStatement { value: Some(value ), kind } => EvalResult::Ret(evaluate(value, scope)?, kind.clone()),
        ASTNode::ReturnStatement { value: None, kind } => EvalResult::Ret(LangValue::Nothing, kind.clone()),
        ASTNode::IfStatement { condition, body } => {
            let condition = evaluate(condition, scope)?;
            
            if condition.truthy() {
                let mut if_scope = Scope::new(Some(scope));

                for child in body {
                    evaluate(child, &mut if_scope)?;
                }
            }
            
            EvalResult::Ok(LangValue::Nothing)
        },
        ASTNode::ForStatement { left, right, body, iter_name } => {
            let left = evaluate(left, scope)?.as_i32();
            let right = evaluate(right, scope)?.as_i32();
            
            let min = expect_some!(left, VARIABLE_IS_NOT_A_NUMBER.to_string());
            let max = expect_some!(right, VARIABLE_IS_NOT_A_NUMBER.to_string());
            
            for i in min..max {
                let mut for_scope = Scope::new(Some(scope));
                for_scope.declare_var(iter_name.clone(), LangValue::Int(i));
                
                for child in body {
                    match evaluate(child, &mut for_scope) {
                        EvalResult::Ok(_) => (),
                        EvalResult::Ret(value, ReturnKind::Break) => return EvalResult::Ok(value),
                        EvalResult::Ret(value, kind) => return EvalResult::Ret(value, kind),
                        EvalResult::Err(err) => return EvalResult::Err(err),
                    }
                }
            }
            
            EvalResult::Ok(LangValue::Nothing)
        },
        ASTNode::WhileStatement { condition, body } => {
            while evaluate(condition, scope)?.truthy() {
                let mut while_scope = Scope::new(Some(scope));
                
                for child in body {
                    match evaluate(child, &mut while_scope) {
                        EvalResult::Ok(_) => (),
                        EvalResult::Ret(value, ReturnKind::Break) => return EvalResult::Ok(value),
                        EvalResult::Ret(value, kind) => return EvalResult::Ret(value, kind),
                        EvalResult::Err(err) => return EvalResult::Err(err),
                    }
                }
            }

            EvalResult::Ok(LangValue::Nothing)
        },
    }
}