use common::ast::types::{OperatorKind, TypeKind};
use crate::tokens::Token;
use super::resolver::{Resolver, AddResult};

pub struct SymbolResolver {
    chars: String,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self {
            chars: String::new(),
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

            "int" => Token::Type(TypeKind::Int),
            "float" => Token::Type(TypeKind::Float),
            "str" => Token::Type(TypeKind::String),
            "any" => Token::Type(TypeKind::Unknown),

            _ => Token::Symbol(self.chars.clone()),
        }
    }
}

impl Resolver for SymbolResolver {
    fn add(&mut self, char: char) -> AddResult {
        match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                self.chars.push(char);
                AddResult::Ok
            },
            _ => {
                let token = self.end_symbol();
                
                AddResult::Change(token, char)
            },
        }
    }
}