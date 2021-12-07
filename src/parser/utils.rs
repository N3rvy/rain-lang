use crate::{ast::node::ASTBody, tokenizer::tokens::{Token, ParenthesisKind, ParenthesisState}, error::LangError, common::messages::UNEXPECTED_END_OF_FILE};

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
            Some(_) => parse_statement(tokens),
            None => return Err(LangError::new_parser(UNEXPECTED_END_OF_FILE.to_string())),
        };
        
        let node = match result {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        
        body.push(node);
    }
    
    Ok(body)
}