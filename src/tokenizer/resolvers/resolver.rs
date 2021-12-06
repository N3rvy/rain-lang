use crate::{tokenizer::tokens::Token, error::LangError, common::messages::UNEXPECTED_ERROR};

pub(crate) enum ResolverKind {
    None,
    Symbol,
    StringLiteral,
    NumberLiteral,
    Operator,
}

pub(super) enum AddAction {
    Ignore,
    Add,
    AddOther(char),
    End,
    AddAndEnd,
    AddAndEndOther(char),
}

pub(crate) enum AddResult {
    Ok,
    Ended(Token),
    Error(LangError),
}

pub(crate) struct Resolver {
    pub kind: ResolverKind,
    pub add_fn: fn(char: char, &Vec<char>) -> Result<AddAction, LangError>,
    pub end_fn: fn(String) -> Result<Token, LangError>,
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {
            kind: ResolverKind::None,
            add_fn: |_, _| Err(LangError::new_tokenizer(0, 0, UNEXPECTED_ERROR.to_string())),
            end_fn: |_| Err(LangError::new_tokenizer(0, 0, UNEXPECTED_ERROR.to_string())),
        }
    }

    pub(crate) fn add(&mut self, char: char, chars: &mut Vec<char>) -> AddResult {
        let result = (self.add_fn)(char, &chars);

        match result {
            Ok(action) => match action {
                AddAction::Ignore => AddResult::Ok,
                AddAction::Add => {
                    chars.push(char);
                    AddResult::Ok
                },
                AddAction::AddOther(new_char) => {
                   chars.push(new_char);
                   AddResult::Ok
                },
                AddAction::End => {
                   Some(self.end(chars));
                   AddResult::Ok
                },
                AddAction::AddAndEnd => {
                    chars.push(char);
                    match self.end(chars) {
                        Ok(token) => AddResult::Ended(token),
                        Err(error ) => AddResult::Error(error),
                    }
                },
                AddAction::AddAndEndOther(new_char) => {
                    chars.push(new_char);
                    match self.end(chars) {
                        Ok(token) => AddResult::Ended(token),
                        Err(error ) => AddResult::Error(error),
                    }
                },
            }
            Err(error) => AddResult::Error(error),
        }
    }
    
    pub(crate) fn end(&mut self, chars: &Vec<char>) -> Result<Token, LangError> {
        let string = chars.into_iter().collect();
        (self.end_fn)(string)
    }
}