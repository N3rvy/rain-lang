use crate::{tokenizer::tokens::Token, ast::node::{ASTNode, ASTChild}, error::LangError, common::{messages::{UNEXPECTED_END_OF_FILE, UNEXPECTED_TOKEN}, lang_value::LangValue}};


fn parse(mut tokens: Vec<Token>) -> ASTNode {
    // Reversing the vector for using it as a stack
    tokens.reverse();
    
    let root = ASTNode::Root{
        body: Vec::new()
    };
    
    let mut node = Box::new(root);
    
    loop {
    }
    
    root
}

fn parse_statement(tokens: &mut Vec<Token>) -> Result<ASTChild, LangError> {
    let token = tokens.pop();
    if let None = token {
        return Err(LangError::new_parser(UNEXPECTED_END_OF_FILE.to_string()));
    }
    
    match token.unwrap() {
        Token::Function => {
            let name = tokens.pop();
            let body = parse_statement(tokens);
            
            match (name, body) {
                (Some(Token::Symbol(name)), Ok(node)) => Ok(
                    ASTNode::new_variable_decl(
                        name,
                        ASTNode::new_literal(LangValue::Function(node)),
                    )),
                
                (None, Ok(node)) => Ok(ASTNode::new_literal(LangValue::Function(node))),
                
                (_, Err(err)) => Err(err),
                _ => Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string())),
            }
        },
        Token::Variable => todo!(),
        Token::Operator(_) => todo!(),
        Token::BoolOperator(_) => todo!(),
        Token::MathOperator(_) => todo!(),
        Token::Symbol(_) => todo!(),
        Token::Literal(_) => todo!(),
        Token::Parenthesis(_, _) => todo!(),
    }
}