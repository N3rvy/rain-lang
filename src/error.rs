
pub enum ErrorKind {
    Tokenizer,
    Parser,
    Runtime,
}

pub struct LangError {
    kind: ErrorKind,
    row: i32,
    column: i32,
    message: String,
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
}