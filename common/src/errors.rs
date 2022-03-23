use std::fmt::{Display, Debug};

pub enum ErrorKind {
    Tokenizer,
    Parser,
    Runtime,
}

pub struct LangError {
    pub kind: ErrorKind,
    pub message: String,
}

impl std::error::Error for LangError {}

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

    pub fn new_parser(message: String) -> Self  {
        Self {
            kind: ErrorKind::Parser,
            message
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
            ErrorKind::Parser => format!("Error while parsing the token\n{}", self.message),
            ErrorKind::Runtime => format!("Error while executing the node {}\n{}", /* TODO: Implement node name */"Not-Implemented", self.message),
        };
        let _ = f.write_str(message.as_str());
    }
}