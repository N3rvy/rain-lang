use common::{types::{OperatorKind, BoolOperatorKind, MathOperatorKind, ParenthesisKind, ParenthesisState}, lang_value::LangValue, ast::TypeKind};

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
    Literal(LangValue),
    Parenthesis(ParenthesisKind, ParenthesisState)
}