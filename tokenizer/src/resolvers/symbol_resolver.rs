use common::ast::types::{LiteralKind, OperatorKind, TypeKind};
use crate::tokens::TokenKind;
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
    
    fn end_symbol(&self) -> TokenKind {
        match self.chars.as_str() {
            "func" => TokenKind::Function, 
            "var" => TokenKind::Variable,
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "in" => TokenKind::Operator(OperatorKind::In),
            "if" => TokenKind::If,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "import" => TokenKind::Import,

            "int" => TokenKind::Type(TypeKind::Int),
            "float" => TokenKind::Type(TypeKind::Float),
            "bool" => TokenKind::Type(TypeKind::Bool),
            "str" => TokenKind::Type(TypeKind::String),
            "none" => TokenKind::Type(TypeKind::Nothing),
            "any" => TokenKind::Type(TypeKind::Unknown),

            "true" => TokenKind::Literal(LiteralKind::Bool(true)),
            "false" => TokenKind::Literal(LiteralKind::Bool(false)),

            _ => TokenKind::Symbol(self.chars.clone()),
        }
    }
}

impl Resolver for SymbolResolver {
    fn add(&mut self, char: char) -> AddResult {
        match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
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