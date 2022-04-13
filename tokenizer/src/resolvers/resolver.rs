use common::{errors::{TokenizerErrorKind}, tokens::TokenKind};
use crate::tokenizer::Tokenizer;

use super::{number_resolver::NumberResolver, parenthesis_resolver::ParenthesisResolver, operator_resolver::OperatorResolver, string_resolver::StringResolver, symbol_resolver::SymbolResolver, whitespace_resolver::WhitespaceResolver};

pub enum AddResult {
    /// The operation whent ok and there is nothing to do
    Ok,
    /// The operation whent ok and there is a token but the resolver needs to stay
    OkToken(TokenKind),
    /// The operation whent ok and the token is ended
    End(TokenKind),
    /// The operation whent ok, there is a leftover character but no token is generated
    ChangeWithoutToken(char),
    /// The operation whent ok, the indentation changed and there is a leftover character
    ChangeIndentation(u32, char),
    /// The operation whent ok, the token in ended and there is a leftover character
    Change(TokenKind, char),
    /// The operation whent ok, the token in ended and there are leftover characters
    ChangeChars(TokenKind, Vec<char>),
    /// The operation whent wrong
    Err(TokenizerErrorKind),
}

pub trait Resolver {
    fn add(&mut self, char: char) -> AddResult;
}

impl<'a> Tokenizer<'a> {
    pub fn resolver_from_char(&'a self, char: char) -> Box<dyn Resolver> {
        match char {
            c if c.is_whitespace() => Box::new(WhitespaceResolver::new(self.indentation_stack.last().unwrap().clone())),
            '0'..='9' => Box::new(NumberResolver::new()),
            '=' | '.' | ',' | '!' | '>' | '<' | '+' | '-' | '*' | '/' | '%' | '^' | ':' => Box::new(OperatorResolver::new()),
            '(' | ')' | '[' | ']' | '{' | '}' => Box::new(ParenthesisResolver::new()),
            '"' => Box::new(StringResolver::new()),
            _ => Box::new(SymbolResolver::new()),
        }
    }
}