use common::ast::types::{OperatorKind, BoolOperatorKind, MathOperatorKind, TypeKind, LiteralKind, ParenthesisKind, ParenthesisState};


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