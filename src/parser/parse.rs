use crate::{tokenizer::tokens::{Token, ParenthesisKind, ParenthesisState}, ast::node::{ASTNode, ASTChild}, error::LangError, common::{messages::{UNEXPECTED_END_OF_FILE, UNEXPECTED_TOKEN, TOKEN_NOT_HANDLED_FORMAT}, lang_value::{LangValue, Function}}, vm::vm::EvalResult};
use crate::tokenizer::tokens::OperatorKind;

use super::utils::{parse_body, parse_parameter_values, parse_parameter_names};

macro_rules! expect_token {
    ($token: expr, $pattern: pat_param) => {
        let tok = $token;

        match tok {
            Some($pattern) => (),
            Some(token) => return Err(LangError::new_parser_unexpected_token(token)),
            None => return Err(LangError::new_parser_end_of_file()),
        }
    };
}

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
        return Err(LangError::new_parser_end_of_file());
    }
    
    let token = token.unwrap();
    
    let result = match &token {
        Token::Function => {
            let next= tokens.pop();
            
            // "name" | (
            match next {
                Some(Token::Symbol(name)) => {
                    // (
                    expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));
                    
                    // ...)
                    let parameters = parse_parameter_names(tokens)?;
                    
                    // {
                    expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));

                    // ...}
                    let body = parse_body(tokens)?;

                    ASTNode::new_variable_decl(
                        name,
                        ASTNode::new_literal(
                            LangValue::Function(Function::new(body, parameters))))
                },
                Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open)) => {
                    // ...)
                    let parameters = parse_parameter_names(tokens)?;

                    // {
                    expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));

                    // ...}
                    let body = parse_body(tokens)?;
                    
                    ASTNode::new_literal(
                        LangValue::Function(Function::new(body, parameters)))
                },
                Some(token) => return Err(LangError::new_parser_unexpected_token(token.clone())),
                None => return Err(LangError::new_parser_end_of_file()),
            }
        },
        Token::Variable => {
            let name = tokens.pop();
            let assign = tokens.pop();
            
            let name = match name {
                Some(Token::Symbol(name)) => name,
                Some(token) => return Err(LangError::new_parser_unexpected_token(token)),
                None => return Err(LangError::new_parser_end_of_file()),
            };

            match assign {
                Some(Token::Operator(OperatorKind::Assign)) => (),
                Some(token) => return Err(LangError::new_parser_unexpected_token(token.clone())),
                None => return Err(LangError::new_parser_end_of_file()),
            }

            let value = parse_statement(tokens);

            match value {
                Ok(node) => ASTNode::new_variable_decl(name, node),
                Err(err) => return Err(err),
            }
        },
        Token::Operator(_) | Token::BoolOperator(_) | Token::MathOperator(_) => return Err(LangError::new_parser_unexpected_token(token.clone())),
        Token::Symbol(name) => ASTNode::new_variable_ref(name.clone()),
        Token::Literal(value) => ASTNode::new_literal(value.clone()),
        Token::Parenthesis(kind, state) => {
            match (kind, state) {
                (ParenthesisKind::Round, ParenthesisState::Open) => {
                    let result = parse_statement(tokens);
                    
                    match tokens.pop() {
                        Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close)) => (),
                        Some(token) => return Err(LangError::new_parser_unexpected_token(token)),
                        None => return Err(LangError::new_parser_end_of_file()),
                    }
                    
                    result?
                },
                _ => return Err(LangError::new_parser_unexpected_token(token.clone()))
            }
        },
        Token::Return => {
            let value = parse_statement(tokens)?;
            
            ASTNode::new_return_statement(value)
        },
        Token::If => {
            // condition
            let condition = parse_statement(tokens)?;
            // {
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
            // ...}
            let body = parse_body(tokens)?;
            
            ASTNode::new_if_statement(condition, body)
        },
    };
    
    
    // Getting the infix and returning if it's None
    let infix = tokens.last().cloned();
    if matches!(infix, None) { return Ok(result) }
    
    let infix = infix.unwrap();

    match infix {
        Token::MathOperator(operator) => {
            tokens.pop();
            let right = parse_statement(tokens);
            
            match right {
                Ok(right) => Ok(ASTNode::new_math_operation(operator.clone(), result, right)),
                Err(err) => Err(err),
            }
        },
        Token::BoolOperator(operator) => {
            tokens.pop();
            let right = parse_statement(tokens);
            
            match right {
                Ok(right) => Ok(ASTNode::new_bool_operation(operator.clone(), result, right)),
                Err(err) => Err(err),
            }

        },
        Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open) => {
            tokens.pop();

            let parameters = parse_parameter_values(tokens)?;

            Ok(ASTNode::new_function_invok(result, parameters))
        },
        
        _ => Ok(result),
    }
}