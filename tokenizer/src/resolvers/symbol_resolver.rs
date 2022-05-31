use common::{ast::types::{LiteralKind, OperatorKind}, tokens::TokenKind};
use common::ast::types::Attribute;
use common::tokens::PrimitiveType;
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
            "class" => TokenKind::Class,
            "var" => TokenKind::Variable,
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "in" => TokenKind::Operator(OperatorKind::In),
            "if" => TokenKind::If,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "import" => TokenKind::Import,

            "data" => TokenKind::Attribute(Attribute::Data),

            "Int" => TokenKind::Type(PrimitiveType::Int),
            "Float" => TokenKind::Type(PrimitiveType::Float),
            "Bool" => TokenKind::Type(PrimitiveType::Bool),
            "String" => TokenKind::Type(PrimitiveType::String),
            "None" => TokenKind::Type(PrimitiveType::Nothing),

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