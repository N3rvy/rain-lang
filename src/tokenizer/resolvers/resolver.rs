use crate::{tokenizer::tokens::Token, error::LangError, common::messages::UNEXPECTED_ERROR};

pub(crate) enum ResolverKind {
    None,
    Symbol,
    StringLiteral,
    NumberLiteral,
    Operator,
}

pub(crate) enum AddResult {
    Ok,
    End(Token),
    Err(LangError),
}

pub(crate) struct Resolver {
    pub kind: ResolverKind,
    pub chars: String,
    pub add_fn: fn(&mut Self, char: char) -> AddResult,
}

impl Resolver {
    pub(crate) fn new_empty() -> Self {
        Default::default()
    }

    pub(crate) fn add(&mut self, char: char) -> AddResult {
        (self.add_fn)(self, char)
    }
    
    pub(super) fn add_char(&mut self, char: char) {
        self.chars.push(char)
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            kind: ResolverKind::None,
            chars: Default::default(),
            add_fn: |_, _| AddResult::Err(LangError::new_tokenizer(0, 0, UNEXPECTED_ERROR.to_string())),
        }
    }
}