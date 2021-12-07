use crate::common::lang_value::LangValue;


pub enum Token {
    Function,
    Variable,
    Operator(OperatorKind),
    BoolOperator(BoolOperatorKind),
    MathOperator(MathOperatorKind),
    Symbol(String),
    Literal(LangValue),
}

pub enum OperatorKind {
    Assign,
    Range,
}

pub enum BoolOperatorKind {
    Equal,
    Different,
    Bigger,
    Smaller,
    BiggerEq,
    SmallerEq,
}

pub enum MathOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Power,
}