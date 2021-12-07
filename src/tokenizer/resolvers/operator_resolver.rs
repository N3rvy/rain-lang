use crate::{tokenizer::tokens::{Token, OperatorKind, MathOperatorKind, BoolOperatorKind}, common::{lang_value::LangValue, messages::INVALID_OPERATOR_TOKEN_ERROR}, error::LangError};
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
        if char.is_whitespace() {
            let token = match self.chars.as_str() {
                // Operators
                "=" => Token::Operator(OperatorKind::Assign),
                ".." => Token::Operator(OperatorKind::Range),
                
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
                _ => return AddResult::Err(LangError::new_tokenizer(0, 0, INVALID_OPERATOR_TOKEN_ERROR.to_string()))
            };
            
            AddResult::End(token)
        } else {
            self.add_char(char);
            AddResult::Ok
        }
    }
}