use crate::tokenizer::tokens::Token;
use super::resolver::{Resolver, ResolverKind, AddResult};

impl Resolver {
    pub(crate) fn new_symbol() -> Self {
        Self {
            kind: ResolverKind::Symbol,
            add_fn: Self::add_symbol,
            chars: Default::default(),
        }
    }
    
    fn add_symbol(&mut self, char: char) -> AddResult {
        if char.is_whitespace() {
            AddResult::End(self.end_symbol())
        } else {
            self.add_char(char);
            AddResult::Ok
        }
    }
    
    fn end_symbol(&self) -> Token {
        match self.chars.as_str() {
            "func" => Token::Function, 
            "var" => Token::Variable,

            _ => Token::Symbol(self.chars.clone()),
        }
    }
}