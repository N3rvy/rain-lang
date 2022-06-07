use crate::ast::types::{OperatorKind, BoolOperatorKind, MathOperatorKind, LiteralKind, ParenthesisKind, ParenthesisState, Attribute};

#[derive(Clone, Copy, Debug)]
pub struct TokenSnapshot(pub usize);

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            start,
            end
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    NewLine,
    Function,
    Variable,
    Class,
    Attribute(Attribute),
    Return,
    Break,
    If,
    Else,
    For,
    While,
    Import,
    Operator(OperatorKind),
    BoolOperator(BoolOperatorKind),
    MathOperator(MathOperatorKind),
    Type(PrimitiveType),
    Symbol(String),
    Literal(LiteralKind),
    Parenthesis(ParenthesisKind, ParenthesisState)
}

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    Nothing,
    Int,
    Float,
    Bool,
    String,
}