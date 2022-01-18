use common::{ast::{ASTNode, NodeKind, TypeKind}, errors::LangError, types::{ParenthesisKind, ParenthesisState, OperatorKind, ReturnKind}, lang_value::{LangValue, Function}, messages::UNEXPECTED_TOKEN};
use tokenizer::tokens::Token;

use crate::{utils::parse_object_values, expect_token};

use super::utils::{parse_body, parse_parameter_values, parse_parameter_names};

pub fn parse(mut tokens: Vec<Token>) -> Result<ASTNode, LangError> {
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
    
    Ok(ASTNode::new(NodeKind::new_root(body), TypeKind::Nothing))
}

pub(super) fn parse_statement(tokens: &mut Vec<Token>) -> Result<ASTNode, LangError> {
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

                    ASTNode::new(
                        NodeKind::new_variable_decl(
                            name,
                            ASTNode::new(
                                NodeKind::new_literal(
                                    LangValue::Function(
                                        Function::new(body, parameters)
                                    )
                                ),
                                TypeKind::Unknown
                            )
                        ),
                        TypeKind::Unknown
                    )
                },
                Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open)) => {
                    // ...)
                    let parameters = parse_parameter_names(tokens)?;

                    // {
                    expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));

                    // ...}
                    let body = parse_body(tokens)?;
                    
                    ASTNode::new(
                        NodeKind::new_literal(
                            LangValue::Function(Function::new(body, parameters))
                        ),
                        TypeKind::Unknown
                    )
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
                Ok(node) => ASTNode::new(NodeKind::new_variable_decl(name, node), TypeKind::Unknown),
                Err(err) => return Err(err),
            }
        },
        Token::Operator(_) | Token::BoolOperator(_) | Token::MathOperator(_) => return Err(LangError::new_parser_unexpected_token()),
        Token::Symbol(name) => ASTNode::new(NodeKind::new_variable_ref(name.clone()), TypeKind::Unknown),
        Token::Literal(value) => ASTNode::new(NodeKind::new_literal(value.clone()), TypeKind::Unknown),
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
                    
                    ASTNode::new(NodeKind::new_vector_literal(values), TypeKind::Unknown)
                },
                (ParenthesisKind::Curly, ParenthesisState::Open) => {
                    let values = parse_object_values(tokens)?;
                    
                    ASTNode::new(NodeKind::new_object_literal(values), TypeKind::Unknown)
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

            ASTNode::new(NodeKind::new_return_statement(value, kind), TypeKind::Unknown)
        },
        Token::If => {
            // condition
            let condition = parse_statement(tokens)?;
            // {
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
            // ...}
            let body = parse_body(tokens)?;
            
            ASTNode::new(NodeKind::new_if_statement(condition, body), TypeKind::Unknown)
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
            
            ASTNode::new(NodeKind::new_for_statement(min, max, body, iter_name), TypeKind::Unknown)
        },
        Token::While => {
            // condition 
            let condition = parse_statement(tokens)?;
            // {
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
            // ...}
            let body = parse_body(tokens)?;
            
            ASTNode::new(NodeKind::new_while_statement(condition, body), TypeKind::Unknown)
        },
        Token::Import => {
            // identifier
            let identifier = match tokens.pop() {
                Some(Token::Literal(LangValue::String(ident))) => ident,
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            };
            
            ASTNode::new(NodeKind::new_import(identifier), TypeKind::Unknown)
        },
        Token::Type(_) => return Err(LangError::new_parser_unexpected_token()),
    };
    

    let mut node = result;
    
    Ok(loop {
        let res = parse_infix(node, tokens)?; 
        if res.1 {
            node = res.0;
        } else {
            break res.0;
        }
    })
}

/// The bool in the tuple is a bool representing whether the infix was valid or not
fn parse_infix(node: ASTNode, tokens: &mut Vec<Token>) -> Result<(ASTNode, bool), LangError> {

    // Getting the infix and returning if it's None
    let infix = tokens.last().cloned();
    if matches!(infix, None) { return Ok((node, false)) }
    
    let infix = infix.unwrap();

    match infix {
        Token::MathOperator(operator) => {
            tokens.pop();
            let right = parse_statement(tokens);
            
            match right {
                Ok(right) => Ok((
                        ASTNode::new(
                            NodeKind::new_math_operation(operator.clone(), node, right),
                            TypeKind::Unknown),
                        true)),
                Err(err) => Err(err),
            }
        },
        Token::BoolOperator(operator) => {
            tokens.pop();
            let right = parse_statement(tokens);
            
            match right {
                Ok(right) => Ok((
                    ASTNode::new(
                        NodeKind::new_bool_operation(operator.clone(), node, right),
                        TypeKind::Unknown),
                    true)),
                Err(err) => Err(err),
            }

        },
        Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open) => {
            tokens.pop();
            
            let value = parse_statement(tokens)?;
            
            expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close));
            
            Ok((
                ASTNode::new(
                    NodeKind::new_value_field_access(node, value),
                    TypeKind::Unknown),
                true)) 
        },
        Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open) => {
            tokens.pop();

            let parameters = parse_parameter_values(tokens, ParenthesisKind::Round)?;
            
            if let NodeKind::FieldAccess { variable: obj, field_name } = *node.kind {
                Ok((
                    ASTNode::new(
                        NodeKind::new_method_invok(obj, field_name , parameters),
                        TypeKind::Unknown),
                    true))
            } else {
                Ok((
                    ASTNode::new(
                        NodeKind::new_function_invok(node, parameters),
                        TypeKind::Unknown),
                    true))
            }
        },
        Token::Operator(OperatorKind::Dot) => {
            tokens.pop();

            let field_name = match tokens.pop() {
                Some(Token::Symbol(field_name)) => field_name,
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            };
            
            Ok((
                ASTNode::new(
                    NodeKind::new_field_access(node, field_name),
                    TypeKind::Unknown),
                true))
        },
        Token::Operator(OperatorKind::Assign) => {
            let name = match node.kind.as_ref() {
                NodeKind::VaraibleRef { name } => name.to_string(),
                _ => return Ok((node, false)),
            };

            tokens.pop();

            let value = parse_statement(tokens)?;

            Ok((
                ASTNode::new(
                    NodeKind::new_variable_asgn(name, value),
                    TypeKind::Unknown),
                true))
        },
        
        _ => Ok((node, false)),
    }
}