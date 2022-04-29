use common::tokens::TokenKind;

use super::resolver::{Resolver, AddResult};

pub struct WhitespaceResolver;

impl WhitespaceResolver {
    pub fn new() -> Self { Self }
}

impl Resolver for WhitespaceResolver {
    fn add(&mut self, char: char) -> AddResult {
        let ret = match char {
            '\n' => AddResult::OkToken(TokenKind::NewLine),
            c if c.is_whitespace() => AddResult::Ok,
            c => AddResult::ChangeWithoutToken(c),
        };

        ret
    }
}