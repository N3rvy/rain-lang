use crate::{tokenizer::tokens::Token, error::LangError};
use super::resolver::{Resolver, AddAction, ResolverKind};

impl Resolver {
    pub(super) fn new_symbol() -> Self {
        Self {
            kind: ResolverKind::Symbol,
            add_fn: Self::add_symbol,
            end_fn: Self::end_symbol,
        }
    }
    
    fn add_symbol(char: char, _: &Vec<char>) -> Result<AddAction, LangError> {
        Ok(AddAction::Add)
    }
    
    fn end_symbol(string: String) -> Result<Token, LangError> {
        Ok(match string.as_str() {
            "func" => Token::Function, 
            "var" => Token::Variable,

            _ => Token::Symbol(string),
        })
    }
}