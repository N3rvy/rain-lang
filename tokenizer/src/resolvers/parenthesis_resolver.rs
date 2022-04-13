use common::ast::types::{ParenthesisKind, ParenthesisState};
use crate::tokens::TokenKind;
use super::resolver::{Resolver, AddResult};

pub struct ParenthesisResolver;

impl ParenthesisResolver {
    pub fn new() -> Self { Self }
}

impl Resolver for ParenthesisResolver {
    fn add(&mut self, char: char) -> AddResult {
        let token = match char {
            '(' => TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open),
            ')' => TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close),
            '[' => TokenKind::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open),
            ']' => TokenKind::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close),
            '{' => TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open),
            '}' => TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close),
          
            // Fallback
            c => return AddResult::ChangeWithoutToken(c),
        };
        
        AddResult::End(token)
    }
}