use crate::{ast::node::ASTNode, common::{lang_value::LangValue, messages::{NOVALUE_ASSIGN, VARIABLE_NOT_DECLARED, NOVALUE_RIGHT_OPERATOR, NOVALUE_LEFT_OPERATOR}}, error::LangError, tokenizer::tokens::{MathOperatorKind, BoolOperatorKind}};

use super::scope::Scope;


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
        ASTNode::FunctionInvok { variable: _ } => todo!(),
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
                        MathOperatorKind::Multiply => todo!(),
                        MathOperatorKind::Divide => todo!(),
                        MathOperatorKind::Modulus => todo!(),
                        MathOperatorKind::Power => todo!(),
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
                        BoolOperatorKind::Equal => todo!(),
                        BoolOperatorKind::Different => todo!(),
                        BoolOperatorKind::Bigger => todo!(),
                        BoolOperatorKind::Smaller => todo!(),
                        BoolOperatorKind::BiggerEq => todo!(),
                        BoolOperatorKind::SmallerEq => todo!(),
                    };
                    
                    EvalResult::Some(value)
                },
                (EvalResult::Err(err), _) => EvalResult::Err(err),
                (_, EvalResult::Err(err)) => EvalResult::Err(err),
                (EvalResult::None, _) => EvalResult::Err(LangError::new_runtime(NOVALUE_LEFT_OPERATOR.to_string())),
                (_, EvalResult::None) => EvalResult::Err(LangError::new_runtime(NOVALUE_RIGHT_OPERATOR.to_string())),
            }
        },
    }
}