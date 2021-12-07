use std::sync::Arc;

use crate::{tokenizer::tokens::{Token, ParenthesisKind, ParenthesisState}, ast::node::{ASTNode, ASTChild}, error::LangError, common::{messages::{UNEXPECTED_END_OF_FILE, UNEXPECTED_TOKEN}, lang_value::LangValue}};
use crate::common::messages::{UNEXPECTED_ERROR, UNEXPECTED_SYMBOL};
use crate::tokenizer::tokens::OperatorKind;

use super::utils::parse_body;


pub fn parse(mut tokens: Vec<Token>) -> Result<Box<ASTNode>, LangError> {
    // Reversing the vector for using it as a stack
    tokens.reverse();
    
    let mut body = Vec::new(); 
    
    loop {
        if tokens.is_empty() { break }

        match parse_statement(&mut tokens) {
            Ok(node) => body.push(node),
            Err(err) => return Err(err),
        }
    }
    
    Ok(ASTNode::new_root(body))
}

pub(super) fn parse_statement(tokens: &mut Vec<Token>) -> Result<ASTChild, LangError> {
    let token = tokens.pop();
    if let None = token {
        return Err(LangError::new_parser(UNEXPECTED_END_OF_FILE.to_string()));
    }
    
    let result = match token.unwrap() {
        Token::Function => {
            let next= tokens.pop();
            
            // "name" | {
            match next {
                Some(Token::Symbol(name)) => {
                    // {
                    match tokens.pop() {
                        Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open)) => {
                            // ...}
                            match parse_body(tokens) {
                                Ok(body) => Ok(
                                    ASTNode::new_variable_decl(
                                        name,
                                        ASTNode::new_literal(
                                            LangValue::Function(Arc::new(body))))),
                                Err(err) => Err(err),
                            }
                        }
                        _ => Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string())),
                    }
                    
                },
                Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open)) => {
                    // ...}
                    match parse_body(tokens) {
                        Ok(body) => Ok(
                            ASTNode::new_literal(
                                LangValue::Function(Arc::new(body)))),
                        Err(err) => Err(err),
                    }
                },
                None => Err(LangError::new_parser(UNEXPECTED_END_OF_FILE.to_string())),
                _ => Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string())),
            }
        },
        Token::Variable => {
            let name = tokens.pop();
            let assign = tokens.pop();

            if !matches!(assign, Some(Token::Operator(OperatorKind::Assign))) {
                return Err(LangError::new_parser(UNEXPECTED_SYMBOL.to_string()));
            }

            let value = parse_statement(tokens);

            match (name, value) {
                (Some(Token::Symbol(name)), Ok(node)) => Ok(ASTNode::new_variable_decl(name, node)),
                _ => Err(LangError::new_parser(UNEXPECTED_ERROR.to_string()))
            }
        },
        Token::Operator(_) | Token::BoolOperator(_) | Token::MathOperator(_) => Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string())),
        Token::Symbol(name) => Ok(ASTNode::new_variable_ref(name)),
        Token::Literal(value) => Ok(ASTNode::new_literal(value)),
        Token::Parenthesis(kind, state) => {
            match (kind, state) {
                (ParenthesisKind::Round, ParenthesisState::Open) => {
                    let result = parse_statement(tokens);
                    if matches!(tokens.pop(), Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close))) {
                        result
                    } else {
                        Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string()))
                    }
                },
                _ => Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string()))
            }
        },
    };
    

    match result {
        Ok(left) => {
            let next = tokens.pop(); 

            match next {
                Some(Token::MathOperator(operator)) => {
                    let right = parse_statement(tokens);
                    
                    match right {
                        Ok(right) => Ok(ASTNode::new_math_operation(operator.clone(), left, right)),
                        Err(err) => Err(err),
                    }
                },
                Some(Token::BoolOperator(operator)) => {
                    let right = parse_statement(tokens);
                    
                    match right {
                        Ok(right) => Ok(ASTNode::new_bool_operation(operator.clone(), left, right)),
                        Err(err) => Err(err),
                    }

                }

                Some(next) => {
                    tokens.push(next);
                    Ok(left)
                },
                
                _ => Ok(left),
            }
        },
        Err(err ) => Err(err),
    }
}