use common::{ast::{ASTNode, ASTChild}, errors::LangError, types::{ParenthesisKind, ParenthesisState, OperatorKind, ReturnKind}, lang_value::{LangValue, Function}, messages::UNEXPECTED_TOKEN};
use tokenizer::tokens::Token;

use super::utils::{parse_body, parse_parameter_values, parse_parameter_names};

macro_rules! expect_token {
    ($token: expr, $pattern: pat_param) => {
        let tok = $token;

        match tok {
            Some($pattern) => (),
            Some(_) => return Err(LangError::new_parser_unexpected_token()),
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
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            }
        },
        Token::Variable => {
            let name = tokens.pop();
            let assign = tokens.pop();
            
            let name = match name {
                Some(Token::Symbol(name)) => name,
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            };

            match assign {
                Some(Token::Operator(OperatorKind::Assign)) => (),
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            }

            let value = parse_statement(tokens);

            match value {
                Ok(node) => ASTNode::new_variable_decl(name, node),
                Err(err) => return Err(err),
            }
        },
        Token::Operator(_) | Token::BoolOperator(_) | Token::MathOperator(_) => return Err(LangError::new_parser_unexpected_token()),
        Token::Symbol(name) => ASTNode::new_variable_ref(name.clone()),
        Token::Literal(value) => ASTNode::new_literal(value.clone()),
        Token::Parenthesis(kind, state) => {
            match (kind, state) {
                (ParenthesisKind::Round, ParenthesisState::Open) => {
                    let result = parse_statement(tokens);
                    
                    match tokens.pop() {
                        Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close)) => (),
                        Some(_) => return Err(LangError::new_parser_unexpected_token()),
                        None => return Err(LangError::new_parser_end_of_file()),
                    }
                    
                    result?
                },
                (ParenthesisKind::Square, ParenthesisState::Open) => {
                    let values = parse_parameter_values(tokens, ParenthesisKind::Square)?;
                    
                    ASTNode::new_vector_literal(values)
                },
                _ => return Err(LangError::new_parser_unexpected_token())
            }
        },
        Token::Return | Token::Break => {
            let value = match tokens.last() {
                Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close)) => {
                    None
                },
                Some(_) => {
                    Some(parse_statement(tokens)?)
                },
                None => return Err(LangError::new_parser_end_of_file()),
            };
            
            let kind = match &token {
                Token::Return => ReturnKind::Return,
                Token::Break => ReturnKind::Break,
                _ => panic!("Like WTF"),
            };

            ASTNode::new_return_statement(value, kind)
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
        Token::For => {
            // iter name
            let iter_name = match tokens.pop() {
                Some(Token::Symbol(name)) => name,
                _ => return Err(LangError::new_parser(UNEXPECTED_TOKEN.to_string())),
            };
            
            // in
            expect_token!(tokens.pop(), Token::Operator(OperatorKind::In));
            
            // min value
            let min = parse_statement(tokens)?;
            
            // ..
            expect_token!(tokens.pop(), Token::Operator(OperatorKind::Range));
            
            // max value
            let max = parse_statement(tokens)?;
            
            // {
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
            
            // ...}
            let body = parse_body(tokens)?;
            
            ASTNode::new_for_statement(min, max, body, iter_name)
        },
        Token::While => {
            // condition 
            let condition = parse_statement(tokens)?;
            // {
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
            // ...}
            let body = parse_body(tokens)?;
            
            ASTNode::new_while_statement(condition, body)
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
        Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open) => {
            tokens.pop();
            
            let value = parse_statement(tokens)?;
            
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close));
            
            Ok(ASTNode::new_value_field_access(result, value))
        },
        Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open) => {
            tokens.pop();

            let parameters = parse_parameter_values(tokens, ParenthesisKind::Round)?;

            Ok(ASTNode::new_function_invok(result, parameters))
        },
        Token::Operator(OperatorKind::Dot) => {
            tokens.pop();

            let field_name = match tokens.pop() {
                Some(Token::Symbol(field_name)) => field_name,
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            };
            
            Ok(ASTNode::new_field_access(result, field_name))
        },
        Token::Operator(OperatorKind::Assign) => {
            let name = match result.as_ref()  {
                ASTNode::VaraibleRef { name } => name.to_string(),
                _ => return Ok(result),
            };

            tokens.pop();

            let value = parse_statement(tokens)?;

            Ok(ASTNode::new_variable_asgn(name, value))
        },
        
        _ => Ok(result),
    }
}