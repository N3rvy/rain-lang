use std::ops::{Try, FromResidual, ControlFlow};

use crate::{ast::node::ASTNode, common::{lang_value::LangValue, messages::{NOVALUE_ASSIGN, VARIABLE_NOT_DECLARED, NOVALUE_RIGHT_OPERATOR, NOVALUE_LEFT_OPERATOR, VARIABLE_IS_NOT_A_FUNCTION, INCORRECT_NUMBER_OF_PARAMETERS}}, error::LangError, tokenizer::tokens::{MathOperatorKind, BoolOperatorKind}};

use super::scope::Scope;


pub enum EvalResult {
    Ok(LangValue),
    Ret(LangValue),
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
            EvalResult::Ret(value) => ControlFlow::Break(EvalResult::Ret(value)),
            EvalResult::Err(err) => ControlFlow::Break(EvalResult::Err(err)),
        }
    }
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
        ASTNode::FunctionInvok { variable, parameters } => {
            let func_node = match evaluate(variable, scope)? {
                LangValue::Function(node) => node,
                _ => return EvalResult::Err(LangError::new_runtime(VARIABLE_IS_NOT_A_FUNCTION.to_string())),
            };
            let func_node = func_node.as_ref();
            
            let mut func_scope = Scope::new(Some(scope));
            
            // Parameters
            if parameters.len() != func_node.parameters.len() {
                return EvalResult::Err(LangError::new_runtime(INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
            }
            for i in 0..parameters.len() {
                let value = evaluate(&parameters[i], &mut func_scope)?;
                func_scope.declare_var(func_node.parameters[i].to_string(), value);
            }

            for child in &func_node.body {
                // Matching to make the return statement stop
                match evaluate(child, &mut func_scope) {
                    EvalResult::Ok(_) => (),
                    EvalResult::Ret(value) => return EvalResult::Ok(value),
                    EvalResult::Err(err) => return EvalResult::Err(err),
                }
            }
            
            EvalResult::Ok(LangValue::Nothing)
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
        ASTNode::ReturnStatement { value } => EvalResult::Ret(evaluate(value, scope)?),
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
    }
}