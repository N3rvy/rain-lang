use crate::{tokenizer::tokens::Token, common::{lang_value::LangValue, messages::{FLOAT_PARSE_ERROR, INT_PARSE_ERROR}}, error::LangError};
use super::resolver::{Resolver, AddAction, ResolverKind};

impl Resolver {
    pub(super) fn new_number() -> Self {
        Self {
            kind: ResolverKind::NumberLiteral,
            add_fn: Self::add_number,
            end_fn: Self::end_number,
        }
    }
    
    fn add_number(char: char, chars: &Vec<char>) -> Result<AddAction, LangError> {
        Ok(AddAction::Add)
    }
    
    fn end_number(string: String) -> Result<Token, LangError>  {
        if string.contains('.') {
            match string.parse::<f32>() {
                Ok(value) => Ok(Token::Literal(LangValue::Float(value))),
                Err(error) => Err(LangError::new_tokenizer(0, 0, FLOAT_PARSE_ERROR.to_string())),
            }
        } else {
            match string.parse::<i32>() {
                Ok(value) => Ok(Token::Literal(LangValue::Int(value))),
                Err(error) => Err(LangError::new_tokenizer(0, 0, INT_PARSE_ERROR.to_string())),
            }
        }
    }
}