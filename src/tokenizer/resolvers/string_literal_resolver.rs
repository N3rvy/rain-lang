use crate::{tokenizer::tokens::Token, common::lang_value::LangValue, error::LangError};
use super::resolver::{Resolver, AddAction, ResolverKind};

impl Resolver {
    pub(super) fn new_string_literal() -> Self {
        Self {
            kind: ResolverKind::StringLiteral,
            add_fn: Self::add_string_literal,
            end_fn: Self::end_string_literal,
        }
    }
    
    fn add_string_literal(char: char, chars: &Vec<char>) -> Result<AddAction, LangError> {
        if char == '"' {
            return if chars.len() == 0 {
                Ok(AddAction::Ignore)
            } else {
                Ok(AddAction::End)
            }
        }
        
        Ok(AddAction::Add)
    }
    
    fn end_string_literal(string: String) -> Result<Token, LangError> {
        Ok(Token::Literal(LangValue::String(string)))
    }
}