use common::{errors::LangError, ast::types::{ParenthesisKind, ParenthesisState}};

use crate::{tokens::Token, errors::INVALID_OPERATOR_TOKEN_ERROR};

use super::resolver::{Resolver, ResolverKind, AddResult};

impl Resolver {
    pub(crate) fn new_parenthesis() -> Self {
        Self {
            kind: ResolverKind::StringLiteral,
            add_fn: Self::add_parenthesis,
            chars: Default::default(),
        }
    }
    
    fn add_parenthesis(&mut self, char: char) -> AddResult {
        let token = match char {
            '(' => Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open),
            ')' => Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close),
            '[' => Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open),
            ']' => Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close),
            '{' => Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open),
            '}' => Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close),
          
            // Fallback
            _ => return AddResult::Err(LangError::new_tokenizer(INVALID_OPERATOR_TOKEN_ERROR.to_string()))
        };
        
        AddResult::End(token)
    }
}