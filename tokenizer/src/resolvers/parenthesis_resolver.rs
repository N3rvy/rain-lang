use common::ast::types::{ParenthesisKind, ParenthesisState};
use crate::tokens::Token;
use super::resolver::{Resolver, AddResult};

pub struct ParenthesisResolver;

impl ParenthesisResolver {
    pub fn new() -> Self { Self }
}

impl Resolver for ParenthesisResolver {
    fn add(&mut self, char: char) -> AddResult {
        let token = match char {
            '(' => Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open),
            ')' => Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close),
            '[' => Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open),
            ']' => Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close),
            '{' => Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open),
            '}' => Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close),
          
            // Fallback
            c => return AddResult::ChangeWithoutToken(c),
        };
        
        AddResult::End(token)
    }
}