use crate::ast::types::{OperatorKind, BoolOperatorKind, MathOperatorKind, TypeKind, LiteralKind, ParenthesisKind, ParenthesisState};


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
    Indent,
    Dedent,
    Function,
    Variable,
    Class,
    Data,
    Return,
    Break,
    If,
    For,
    While,
    Import,
    Operator(OperatorKind),
    BoolOperator(BoolOperatorKind),
    MathOperator(MathOperatorKind),
    Type(TypeKind),
    Symbol(String),
    Literal(LiteralKind),
    Parenthesis(ParenthesisKind, ParenthesisState)
}