use common::types::{OperatorKind, BoolOperatorKind, MathOperatorKind, ParenthesisKind, ParenthesisState, TypeKind, LiteralKind};

#[derive(Clone, Debug)]
pub enum Token {
    Function,
    Variable,
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