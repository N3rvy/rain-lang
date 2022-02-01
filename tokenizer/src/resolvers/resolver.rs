use common::errors::LangError;

use crate::{tokens::Token, errors::UNEXPECTED_ERROR};


#[allow(dead_code)]
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
    Change(Token, char),
    ChangeChars(Token, Vec<char>),
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
    
    pub(crate) fn from_char(char: char) -> Resolver {
        match char {
            c if c.is_whitespace() => Resolver::new_empty(),
            '0'..='9' => Resolver::new_number(),
            '=' | '.' | ',' | '!' | '>' | '<' | '+' | '-' | '*' | '/' | '%' | '^' | ':' => Resolver::new_operator(),
            '(' | ')' | '[' | ']' | '{' | '}' => Resolver::new_parenthesis(),
            '"' => Resolver::new_string_literal(),
            _ => Resolver::new_symbol(),
        }
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
            add_fn: |_, _| AddResult::Err(LangError::new_tokenizer(UNEXPECTED_ERROR.to_string())),
        }
    }
}