use common::types::OperatorKind;

use crate::tokens::Token;

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
            "break" => Token::Break,
            "in" => Token::Operator(OperatorKind::In),
            "if" => Token::If,
            "for" => Token::For,
            "while" => Token::While,
            "import" => Token::Import,

            _ => Token::Symbol(self.chars.clone()),
        }
    }
}