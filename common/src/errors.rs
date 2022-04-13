use std::fmt::{Display, Debug};

use crate::{tokens::Token, ast::types::TypeKind, module::ModuleUID};

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
}

#[derive(Debug)]
pub enum BuildErrorKind {
    UnexpectedError,
    Unsupported(String),
    FuncNotFound(String),
    ModuleNotFound(ModuleUID),
    InvalidStackType, // TODO: Implement types
    InvalidStackSize(usize, usize),
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
    }
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
}

impl std::error::Error for LangError {}

impl Display for LangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}