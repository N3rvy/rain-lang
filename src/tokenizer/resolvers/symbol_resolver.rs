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
        match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                self.add_char(char);
                AddResult::Ok
            },
            _ => {
                let token = self.end_symbol();
                
                AddResult::Change(token, char)
            },
        }
    }
    
    fn end_symbol(&self) -> Token {
        match self.chars.as_str() {
            "func" => Token::Function, 
            "var" => Token::Variable,
            "return" => Token::Return,
            "if" => Token::If,

            _ => Token::Symbol(self.chars.clone()),
        }
    }
}