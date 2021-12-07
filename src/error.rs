use std::fmt::{Display, Debug};


pub enum ErrorKind {
    Tokenizer,
    Parser,
    Runtime,
}

pub struct LangError {
    pub kind: ErrorKind,
    pub row: i32,
    pub column: i32,
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
    pub fn new_tokenizer(row: i32, column: i32, message: String) -> Self  {
        Self {
            kind: ErrorKind::Tokenizer,
            row,
            column,
            message
        }
    }

    pub fn new_parser(row: i32, column: i32, message: String) -> Self  {
        Self {
            kind: ErrorKind::Parser,
            row,
            column,
            message
        }
    }

    pub fn new_runtime(row: i32, column: i32, message: String) -> Self  {
        Self {
            kind: ErrorKind::Runtime,
            row,
            column,
            message
        }
    }
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        let _ = f.write_str(format!(
            "Error during {} at row {}, and column {}:",
            Self::to_stage_verg(&self.kind),
            self.row,
            self.column,
        ).as_str());
        let _ = f.write_str(self.message.as_str());
    }
    
    fn to_stage_verg(kind: &ErrorKind) -> &'static str {
        match kind {
            ErrorKind::Tokenizer => "tokenization",
            ErrorKind::Parser => "parsing",
            ErrorKind::Runtime => "execution",
        }
    }
}