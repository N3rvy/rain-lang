use crate::common::lang_value::LangValue;

#[derive(Clone, Debug)]
pub enum Token {
    Function,
    Variable,
    Return,
    Break,
    If,
    For,
    While,
    Operator(OperatorKind),
    BoolOperator(BoolOperatorKind),
    MathOperator(MathOperatorKind),
    Symbol(String),
    Literal(LangValue),
    Parenthesis(ParenthesisKind, ParenthesisState)
}

#[derive(Clone, Debug)]
pub enum ParenthesisKind {
    Round,
    Square,
    Curly,
}

#[derive(Clone, Debug)]
pub enum ParenthesisState {
    Open,
    Close,
}

#[derive(Clone, Debug)]
pub enum OperatorKind {
    Assign,
    In,
    Range,
    Comma,
}

#[derive(Clone, Debug)]
pub enum BoolOperatorKind {
    Equal,
    Different,
    Bigger,
    Smaller,
    BiggerEq,
    SmallerEq,
}

#[derive(Clone, Debug)]
pub enum MathOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Power,
}