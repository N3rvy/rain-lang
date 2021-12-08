use std::fmt::{Display, Debug};

use crate::{tokenizer::tokens::Token, common::messages::{UNEXPECTED_END_OF_FILE, UNEXPECTED_ERROR, UNEXPECTED_TOKEN}};


pub enum ErrorKind {
    Tokenizer,
    Parser(Option<Token>),
    Runtime,
}

pub struct LangError {
    pub kind: ErrorKind,
    pub message: String,
}

impl Debug for LangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f);
        Ok(())
    }
}

impl Display for LangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f);
        Ok(())
    }
}

impl LangError {
    pub fn new_tokenizer(message: String) -> Self  {
        Self {
            kind: ErrorKind::Tokenizer,
            message
        }
    }

    pub fn new_parser(token: Token, message: String) -> Self  {
        Self {
            kind: ErrorKind::Parser(Some(token.clone())),
            message
        }
    }
    
    pub fn new_parser_wo_token(message: String) -> Self {
        Self {
            kind: ErrorKind::Parser(None),
            message
        }
    }
    
    pub fn new_parser_unexpected_token(token: Token) -> Self  {
        Self {
            kind: ErrorKind::Parser(Some(token.clone())),
            message: UNEXPECTED_TOKEN.to_string(),
        }
    }
    
    pub fn new_parser_end_of_file() -> Self {
        Self {
            kind: ErrorKind::Parser(None),
            message: UNEXPECTED_END_OF_FILE.to_string(),
        }
    }
    
    pub fn new_parser_unexpected_error() -> Self {
        Self {
            kind: ErrorKind::Parser(None),
            message: UNEXPECTED_ERROR.to_string(),
        }
    }

    pub fn new_runtime(message: String) -> Self  {
        Self {
            kind: ErrorKind::Runtime,
            message
        }
    }
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        let message = match &self.kind {
            ErrorKind::Tokenizer => format!("Error while tokenizing the script:\n{}", self.message),
            ErrorKind::Parser(token) => format!("Error while parsing the token {:?}\n{}", token, self.message),
            ErrorKind::Runtime => format!("Error while parsing the node {}\n{}", /* TODO: Implement node name */"Not-Implemented", self.message),
        };
        let _ = f.write_str(message.as_str());
    }
}