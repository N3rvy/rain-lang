use common::{errors::LangError, ast::types::{OperatorKind, BoolOperatorKind, MathOperatorKind}};
use crate::{tokens::Token, errors::INVALID_OPERATOR_TOKEN_ERROR};
use super::resolver::{Resolver, AddResult};

pub struct OperatorResolver {
    chars: String,
}

impl OperatorResolver {
    pub fn new() -> Self {
        Self {
            chars: String::new(),
        }
    }
    
    fn end_operator(&self) -> Result<Token, LangError> {
        Ok(match self.chars.as_str() {
            // Operators
            "=" => Token::Operator(OperatorKind::Assign),
            ".." => Token::Operator(OperatorKind::Range),
            "," => Token::Operator(OperatorKind::Comma),
            "." => Token::Operator(OperatorKind::Dot),
            ":" => Token::Operator(OperatorKind::Colon),
            
            // Math operator
            "+" => Token::MathOperator(MathOperatorKind::Plus),
            "-" => Token::MathOperator(MathOperatorKind::Minus),
            "*" => Token::MathOperator(MathOperatorKind::Multiply),
            "/" => Token::MathOperator(MathOperatorKind::Divide),
            "%" => Token::MathOperator(MathOperatorKind::Modulus),
            "^" => Token::MathOperator(MathOperatorKind::Power),
            
            // Bool opreator
            "==" => Token::BoolOperator(BoolOperatorKind::Equal),
            "!=" => Token::BoolOperator(BoolOperatorKind::Different),
            ">" => Token::BoolOperator(BoolOperatorKind::Bigger),
            "<" => Token::BoolOperator(BoolOperatorKind::Smaller),
            ">=" => Token::BoolOperator(BoolOperatorKind::BiggerEq),
            "<=" => Token::BoolOperator(BoolOperatorKind::SmallerEq),
            
            // Fallback
            _ => return Err(LangError::new_tokenizer(INVALID_OPERATOR_TOKEN_ERROR.to_string()))
        })
    }
}

impl Resolver for OperatorResolver {
    fn add(&mut self, char: char) -> AddResult {
        match char {
            '=' | '.' | ',' | '!' | '>' | '<' | '+' | '-' | '*' | '/' | '%' | '^' | ':' => {
                self.chars.push(char);
                AddResult::Ok
            },

            _ => {
                match self.end_operator() {
                    Ok(token) => AddResult::Change(token, char),
                    Err(err) => AddResult::Err(err),
                }
            },
        }
    }
}