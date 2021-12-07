use crate::{tokenizer::tokens::{Token, OperatorKind, MathOperatorKind, BoolOperatorKind}, common::messages::{INVALID_OPERATOR_TOKEN_ERROR, UNEXPECTED_ERROR}, error::LangError};
use super::resolver::{Resolver, ResolverKind, AddResult};
impl Resolver {
    pub(crate) fn new_operator() -> Self {
        Self {
            kind: ResolverKind::StringLiteral,
            add_fn: Self::add_operator,
            chars: Default::default(),
        }
    }
    
    fn add_operator(&mut self, char: char) -> AddResult {
        match char {
            '=' | '.' | ',' | '!' | '>' | '<' | '+' | '-' | '*' | '/' | '%' | '^' => {
                self.add_char(char);
                AddResult::Ok
            },
            c if c.is_whitespace() => {
                match self.end_operator() {
                    Ok(token) => AddResult::End(token),
                    Err(err) => AddResult::Err(err),
                }
            }
            _ => {
                let result = match self.end_operator() {
                    Ok(token) => {
                        match Resolver::from_char_and_add(char) {
                            Ok(res) => AddResult::Changed(token, res),
                            Err(err) => AddResult::Err(err),
                        }
                        
                    },
                    Err(err) => AddResult::Err(err),
                };
                
                result
            },
        }
    }
    
    fn end_operator(&self) -> Result<Token, LangError> {
        Ok(match self.chars.as_str() {
            // Operators
            "=" => Token::Operator(OperatorKind::Assign),
            ".." => Token::Operator(OperatorKind::Range),
            "," => Token::Operator(OperatorKind::Comma),
            
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