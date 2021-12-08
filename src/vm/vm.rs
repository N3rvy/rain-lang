use crate::{ast::node::ASTNode, common::{lang_value::LangValue, messages::{NOVALUE_ASSIGN, VARIABLE_NOT_DECLARED, NOVALUE_RIGHT_OPERATOR, NOVALUE_LEFT_OPERATOR, VARIABLE_IS_NOT_A_FUNCTION}}, error::LangError, tokenizer::tokens::{MathOperatorKind, BoolOperatorKind}};

use super::scope::Scope;


// TODO: Remove none because lang value already has nothing
pub enum EvalResult {
    None,
    Some(LangValue),
    Err(LangError),
}


pub fn evaluate(ast: &Box<ASTNode>, scope: &mut Scope) -> EvalResult {
    match ast.as_ref() {
        ASTNode::Root { body } => {
            let mut result = EvalResult::None;

            for child in body {
                result = evaluate(child, scope);
            }
            
            result
        },
        ASTNode::VariableDecl { name, value } => {
            let value = match evaluate(value, scope) {
                EvalResult::None => return EvalResult::Err(LangError::new_runtime(NOVALUE_ASSIGN.to_string())),
                EvalResult::Some(value) => value,
                EvalResult::Err(err) => return EvalResult::Err(err),
            };

            scope.declare_var(name.clone(), value);

            EvalResult::None
        },
        ASTNode::VaraibleRef { name } => {
            match scope.get_var(name) {
                Some(value) => EvalResult::Some(value.clone()),
                None => EvalResult::Err(LangError::new_runtime(VARIABLE_NOT_DECLARED.to_string())),
            }
        },
        ASTNode::FunctionInvok { variable } => {
            let func_node = match evaluate(variable, scope) {
                EvalResult::None => return EvalResult::Err(LangError::new_runtime(VARIABLE_NOT_DECLARED.to_string())),
                EvalResult::Err(err) => return EvalResult::Err(err),

                EvalResult::Some(LangValue::Function(node)) => node,
                EvalResult::Some(_) => return EvalResult::Err(LangError::new_runtime(VARIABLE_IS_NOT_A_FUNCTION.to_string())),
            };


            let mut func_scope = Scope::new(Some(scope));
            let mut result = EvalResult::None;

            for child in func_node.as_ref() {
                result = match evaluate(child, &mut func_scope) {
                    EvalResult::None => EvalResult::None,
                    EvalResult::Some(value) => EvalResult::Some(value),
                    EvalResult::Err(err) => EvalResult::Err(err),
                }
            }
            
            result
        },
        ASTNode::Literal { value } => {
            EvalResult::Some(value.clone())
        },
        ASTNode::MathOperation { operation, left, right } => {
            let left = evaluate(left, scope);
            let right = evaluate(right, scope);
            
            match (left, right) {
                (EvalResult::Some(left), EvalResult::Some(right)) => {
                    let value = match operation {
                        MathOperatorKind::Plus => left.sum(right),
                        MathOperatorKind::Minus => left.minus(right),
                        MathOperatorKind::Multiply => left.multiply(right),
                        MathOperatorKind::Divide => left.divide(right),
                        MathOperatorKind::Modulus => left.modulus(right),
                        MathOperatorKind::Power => left.power(right),
                    };
                    
                    EvalResult::Some(value)
                },
                (EvalResult::Err(err), _) => EvalResult::Err(err),
                (_, EvalResult::Err(err)) => EvalResult::Err(err),
                (EvalResult::None, _) => EvalResult::Err(LangError::new_runtime(NOVALUE_LEFT_OPERATOR.to_string())),
                (_, EvalResult::None) => EvalResult::Err(LangError::new_runtime(NOVALUE_RIGHT_OPERATOR.to_string())),
            }
        },
        ASTNode::BoolOperation { operation, left, right } => {
            let left = evaluate(left, scope);
            let right = evaluate(right, scope);
            
            match (left, right) {
                (EvalResult::Some(left), EvalResult::Some(right)) => {
                    let value = match operation {
                        BoolOperatorKind::Equal => left.equals(&right),
                        BoolOperatorKind::Different => left.not_equals(&right),
                        BoolOperatorKind::Bigger => left.bigger(&right),
                        BoolOperatorKind::Smaller => left.smaller(&right),
                        BoolOperatorKind::BiggerEq => left.bigger_eq(&right),
                        BoolOperatorKind::SmallerEq => left.smaller_eq(&right),
                    };
                    
                    EvalResult::Some(LangValue::Bool(value))
                },
                (EvalResult::Err(err), _) => EvalResult::Err(err),
                (_, EvalResult::Err(err)) => EvalResult::Err(err),
                (EvalResult::None, _) => EvalResult::Err(LangError::new_runtime(NOVALUE_LEFT_OPERATOR.to_string())),
                (_, EvalResult::None) => EvalResult::Err(LangError::new_runtime(NOVALUE_RIGHT_OPERATOR.to_string())),
            }
        },
    }
}