
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
    Dot,
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

#[derive(Clone)]
pub enum ReturnKind {
    Return,
    Break,
    Panic,
}