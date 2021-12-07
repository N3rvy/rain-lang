use crate::common::lang_value::LangValue;

pub enum Token {
    Function,
    Variable,
    Operator(OperatorKind),
    BoolOperator(BoolOperatorKind),
    MathOperator(MathOperatorKind),
    Symbol(String),
    Literal(LangValue),
    Parenthesis(ParenthesisKind, ParenthesisState)
}

#[derive(Clone)]
pub enum ParenthesisKind {
    Round,
    Square,
    Curly,
}

#[derive(Clone)]
pub enum ParenthesisState {
    Open,
    Close,
}

#[derive(Clone)]
pub enum OperatorKind {
    Assign,
    Range,
    Comma,
}

#[derive(Clone)]
pub enum BoolOperatorKind {
    Equal,
    Different,
    Bigger,
    Smaller,
    BiggerEq,
    SmallerEq,
}

#[derive(Clone)]
pub enum MathOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Power,
}