use crate::{ast::node::{ASTBody, ASTChild}, tokenizer::tokens::{Token, ParenthesisKind, ParenthesisState, OperatorKind}, error::LangError, common::messages::{PARAMETERS_EXPECTING_COMMA, PARAMETERS_EXPECTING_PARAMETER}};

use super::parse::parse_statement;

/** Parses a block of tokens (something like "{ var x = 10; var y = false }").
 * It consumes only the last parenthesis and expectes the first token to be the first statement,
   in this case it will be "var"
 */ 
pub(super) fn parse_body(tokens: &mut Vec<Token>) -> Result<ASTBody, LangError> {
    let mut body = Vec::new();
    
    loop {
        let token = tokens.last();
            
        let result = match token {
            Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close)) => break,
            Some(_) => parse_statement(tokens)?,
            None => return Err(LangError::new_parser_end_of_file()),
        };
        
        body.push(result);
    }
    
    // Popping the last }
    tokens.pop();
    
    Ok(body)
}

/** Parses a list of parameter names (something like "(arg0, arg1, arg2)").
 * It consumes only the last parenthesis and expectes the first token to be the first argument,
   in this case it will be "arg0"
 */ 
pub(super) fn parse_parameter_names(tokens: &mut Vec<Token>) -> Result<Vec<String>, LangError> {
    let mut names = Vec::new();
    let mut next_is_argument = true;
    
    loop {
        let token = tokens.pop();
        
        match &token {
            Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close)) => break,
            Some(Token::Symbol(name)) => {
                if next_is_argument {
                    next_is_argument = false;
                    names.push(name.clone());
                } else {
                    return Err(LangError::new_parser(token.unwrap().clone(), PARAMETERS_EXPECTING_COMMA.to_string()));
                }
            },
            Some(Token::Operator(OperatorKind::Comma)) => {
                if next_is_argument {
                    return Err(LangError::new_parser(token.unwrap().clone(), PARAMETERS_EXPECTING_PARAMETER.to_string()));
                } else {
                    next_is_argument = true;
                }
            },
            Some(token) => return Err(LangError::new_parser_unexpected_token(token.clone())),
            None => return Err(LangError::new_parser_end_of_file()),
        };
    }

    Ok(names)
}

pub(super) fn parse_parameter_values(tokens: &mut Vec<Token>) -> Result<ASTBody, LangError> {
    let mut body = Vec::new();
    let mut next_is_argument = true;
    
    loop {
        let token = tokens.last();
            
        match token {
            Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close)) => break,
            Some(Token::Operator(OperatorKind::Comma)) => {
                if next_is_argument {
                    return Err(LangError::new_parser(token.unwrap().clone(), PARAMETERS_EXPECTING_PARAMETER.to_string()));
                } else {
                    tokens.pop();

                    next_is_argument = true;
                    continue;
                }
            }
            Some(_) => {
                if next_is_argument {
                    next_is_argument = false;
                    body.push(parse_statement(tokens)?);
                } else {
                    return Err(LangError::new_parser(token.unwrap().clone(), PARAMETERS_EXPECTING_COMMA.to_string()));
                }
            },
            None => return Err(LangError::new_parser_end_of_file()),
        };
    }
    
    // Popping the last )
    tokens.pop();
    
    Ok(body)
}