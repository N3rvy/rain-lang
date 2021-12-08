use crate::{tokenizer::tokens::Token, common::{lang_value::LangValue, messages::{FLOAT_PARSE_ERROR, INT_PARSE_ERROR, INVALID_OPERATOR_TOKEN_ERROR}}, error::LangError};
use super::resolver::{Resolver, ResolverKind, AddResult};

impl Resolver {
    pub(crate) fn new_number() -> Self {
        Self {
            kind: ResolverKind::NumberLiteral,
            add_fn: Self::add_number,
            chars: Default::default(),
        }
    }
    
    fn add_number(&mut self, char: char) -> AddResult {
        match char {
            '0'..='9' => {
                self.add_char(char);
                AddResult::Ok
            },
            '.' => {
                // If there is a second point then switch this resolver from a number resolver to an operator resolver
                if self.chars.contains('.') {
                    match self.end_number() {
                        Ok(token) => AddResult::Change(token, char),
                        Err(err) => AddResult::Err(err),
                    }
                } else {
                    self.chars.push(char);
                    AddResult::Ok
                }
            },

            _ => {
                match self.end_number() {
                    Ok(token) => AddResult::Change(token, char),
                    Err(err) => AddResult::Err(err),
                }
            }
        }
    }
    
    fn end_number(&self) -> Result<Token, LangError>  {
        if self.chars.contains('.') {
            match self.chars.parse::<f32>() {
                Ok(value) => Ok(Token::Literal(LangValue::Float(value))),
                Err(error) => Err(LangError::new_tokenizer(FLOAT_PARSE_ERROR.to_string())),
            }
        } else {
            match self.chars.parse::<i32>() {
                Ok(value) => Ok(Token::Literal(LangValue::Int(value))),
                Err(error) => Err(LangError::new_tokenizer(INT_PARSE_ERROR.to_string())),
            }
        }
    }
}