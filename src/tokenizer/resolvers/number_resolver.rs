use crate::{tokenizer::tokens::Token, common::{lang_value::LangValue, messages::{FLOAT_PARSE_ERROR, INT_PARSE_ERROR}}, error::LangError};
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
        if char.is_whitespace() {
            match self.end_number() {
                Ok(token) => AddResult::End(token),
                Err(err) => AddResult::Err(err),
            }
        } else {
            self.add_char(char);
            AddResult::Ok
        }
    }
    
    fn end_number(&self) -> Result<Token, LangError>  {
        if self.chars.contains('.') {
            match self.chars.parse::<f32>() {
                Ok(value) => Ok(Token::Literal(LangValue::Float(value))),
                Err(error) => Err(LangError::new_tokenizer(0, 0, FLOAT_PARSE_ERROR.to_string())),
            }
        } else {
            match self.chars.parse::<i32>() {
                Ok(value) => Ok(Token::Literal(LangValue::Int(value))),
                Err(error) => Err(LangError::new_tokenizer(0, 0, INT_PARSE_ERROR.to_string())),
            }
        }
    }
}