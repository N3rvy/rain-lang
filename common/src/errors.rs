use std::cmp::{max, min};
use std::fmt::{Display, Debug};
#[cfg(feature = "colored")]
use colored::Colorize;
use crate::{tokens::Token, ast::types::TypeKind, module::ModuleUID};
use crate::ast::types::Attribute;

#[derive(Debug)]
pub enum TokenizerErrorKind {
    FloatParse(String),
    IntParse(String),
    InvalidOperatorToken,
    InvalidStringLiteral,
    InvalidIndent,
}

#[derive(Debug)]
pub enum ParserErrorKind {
    UnexpectedError(String),
    Unsupported(String),
    UnexpectedToken,
    UnexpectedEndOfFile,
    WrontType(TypeKind, TypeKind),
    ParametersExpectedComma,
    ParametersExpectedParam,
    VarNotFound,
    InvalidFieldAccess,
    VarIsNotObject,
    FieldDoesntExist,
    NotCallable,
    NotIndexable,
    InvalidArgCount(usize),
    InvalidAttribute(Attribute),
}

#[derive(Debug)]
pub enum BuildErrorKind {
    UnexpectedError(String),
    Unsupported(String),
    ClassNotFound(String),
    FuncNotFound(String),
    ModuleNotFound(ModuleUID),
    InvalidStackType, // TODO: Implement types
    InvalidStackSize(usize, usize),
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    UnexpectedError(String),
    CantConvertValue,
    FuncInvalidParamCount(usize, usize),
    ExtFuncParamCount(usize, usize),
    ExtFuncParamType,
    VarNotFound(String),
    ValueNotNumber,
    ValueNotFunc,
    ModuleNotFound(ModuleUID),
}

#[derive(Debug)]
pub enum LoadErrorKind {
    ModuleNotFound(String),
    LoadModuleError(String),
}

#[derive(Debug)]
pub enum LangError {
    Tokenizer {
        token: Token,
        kind: TokenizerErrorKind,
    },
    Parser {
        token: Token,
        kind: ParserErrorKind,
    },
    Build {
        kind: BuildErrorKind,
    },
    Load {
        kind: LoadErrorKind,
    },
    Runtime {
        kind: RuntimeErrorKind,
    },
}

impl LangError {
    pub fn tokenizer(token: &Token, kind: TokenizerErrorKind) -> Self {
        Self::Tokenizer {
            token: token.clone(),
            kind,
        }
    }

    pub fn parser(token: &Token, kind: ParserErrorKind) -> Self {
        Self::Parser {
            token: token.clone(),
            kind,
        }
    }

    pub fn load(kind: LoadErrorKind) -> Self {
        Self::Load {
            kind
        }
    }

    pub fn build(kind: BuildErrorKind) -> Self {
        Self::Build {
            kind
        }
    }

    pub fn runtime(kind: RuntimeErrorKind) -> Self {
        Self::Runtime {
            kind
        }
    }
}

impl std::error::Error for LangError {}

impl Display for LangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\nUse the format_error function for better error visualization", self)
    }
}

pub trait LangErrorFormat {
    fn format(self, source: &String) -> anyhow::Error;
}

impl LangErrorFormat for LangError {
    fn format(self, source: &String) -> anyhow::Error {
        let msg = match self {
            LangError::Tokenizer { token, kind } => format_tokenizer(source, token, kind),
            LangError::Parser { token, kind } => format_parser(source, token, kind),
            LangError::Build { kind } => format_build(kind),
            LangError::Load { kind } => format_load(kind),
            LangError::Runtime { kind } => format_runtime(kind),
        };

        anyhow::anyhow!(msg)
    }
}

fn format_token(source: &String, token: &Token) -> String {
    let (row, col) = source.chars()
        .take(token.start)
        .fold((1usize, 0usize), |(row, col), c| match c {
            '\n' => (row + 1, 0),
            _ => (row, col + 1),
        });

    let col_end = col + (token.end - token.start);

    let error_preview: String = source.lines()
        .skip(max(row, 2) - 2)
        .take(3)
        .enumerate()
        .map(|(i, line)| {
            if i == 1 {
                let col_end = min(col_end, line.len());

                let before = &line[..col];
                let err = &line[col..col_end];
                let after = &line[col_end..];

                #[cfg(feature = "colored")]
                return format!("{}{}{}\n", before, err.red().underline(), after);
                #[cfg(not(feature = "colored"))]
                return format!("{}{}{}\n", before, err, after);
            } else {
                format!("{}\n", line)
            }
        })
        .collect();

    format!("\n{}\nError from {}:{} to {}:{}", error_preview, row, col, row, col_end)
}

fn format_tokenizer(source: &String, token: Token, kind: TokenizerErrorKind) -> String {
    let res = format_token(source, &token);
    let err = match kind {
        TokenizerErrorKind::FloatParse(str) => format!("Error while parsing float literal ({})", str),
        TokenizerErrorKind::IntParse(str) => format!("Error while parsing int literal ({})", str),
        TokenizerErrorKind::InvalidOperatorToken => "Invalid operator".to_string(),
        TokenizerErrorKind::InvalidStringLiteral => "Invalid string literal".to_string(),
        TokenizerErrorKind::InvalidIndent => "Invalid indentation".to_string(),
    };

    res + "\n" + &err
}

fn format_parser(source: &String, token: Token, kind: ParserErrorKind) -> String {
    let res = format_token(source, &token);
    let err = match kind {
        ParserErrorKind::UnexpectedError(err) => format!("Unexpected error ({})", err),
        ParserErrorKind::Unsupported(feature) => format!("Unsupported feature ({})", feature),
        ParserErrorKind::UnexpectedToken => format!("Unexpected token {:?}", token.kind),
        ParserErrorKind::UnexpectedEndOfFile => format!("Unexpected end of file"),
        ParserErrorKind::WrontType(expected, found) => format!("Expected type {:?}, instead found {:?}", expected, found),
        ParserErrorKind::ParametersExpectedComma => "Expected comma".to_string(),
        ParserErrorKind::ParametersExpectedParam => "Expected parameter".to_string(),
        ParserErrorKind::VarNotFound => "Variable not found".to_string(),
        ParserErrorKind::InvalidFieldAccess => "Variable is not indexable by field".to_string(),
        ParserErrorKind::VarIsNotObject => "Variable is not an object".to_string(),
        ParserErrorKind::FieldDoesntExist => "Field doesn't exist".to_string(),
        ParserErrorKind::NotCallable => "Variable is not callable".to_string(),
        ParserErrorKind::NotIndexable => "Variable is not indexable".to_string(),
        ParserErrorKind::InvalidArgCount(expected) => format!("Expected {} parameters", expected),
        ParserErrorKind::InvalidAttribute(attribute) => format!("Invalid attribute ({:?})", attribute),
    };

    res + "\n" + &err
}

pub fn format_build(kind: BuildErrorKind) -> String {
    match kind {
        BuildErrorKind::UnexpectedError(err) => format!("Unexpected error ({})", err),
        BuildErrorKind::Unsupported(feature) => format!("Unsupported feature ({})", feature),
        BuildErrorKind::ClassNotFound(class_name) => format!("Class not found ({})", class_name),
        BuildErrorKind::FuncNotFound(name) => format!("Function not found ({})", name),
        BuildErrorKind::ModuleNotFound(uid) => format!("Module not found ({:?})", uid),
        BuildErrorKind::InvalidStackType => "Invalid stack type".to_string(),
        BuildErrorKind::InvalidStackSize(expected, found) => format!("Expected {} items on the stack found {}", expected, found),
    }
}

pub fn format_load(kind: LoadErrorKind) -> String {
    match kind {
        LoadErrorKind::ModuleNotFound(name) => format!("Module not found ({})", name),
        LoadErrorKind::LoadModuleError(err) => format!("Error while loading module: {}", err),
    }
}

pub fn format_runtime(kind: RuntimeErrorKind) -> String {
    match kind {
        RuntimeErrorKind::UnexpectedError(str) => format!("Unexpected error: {}", str),
        RuntimeErrorKind::CantConvertValue => "Can't convert value".to_string(),
        RuntimeErrorKind::FuncInvalidParamCount(expected, found)
            => format!("Incorrect number of parameters, expected {} found {}", expected, found),
        RuntimeErrorKind::ExtFuncParamCount(expected, found)
            => format!("Incorrect number of parameters in external function, expected {} found {}", expected, found),
        RuntimeErrorKind::ExtFuncParamType => "Wrong type in external function paramter".to_string(),
        RuntimeErrorKind::VarNotFound(name) => format!("Variable not found ({})", name),
        RuntimeErrorKind::ValueNotNumber => "Variable is not a number".to_string(),
        RuntimeErrorKind::ValueNotFunc => "Variable is not a function".to_string(),
        RuntimeErrorKind::ModuleNotFound(uid) => format!("Module not found ({:?})", uid),
    }
}