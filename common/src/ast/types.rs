use std::{collections::HashMap, sync::Arc, fmt::Debug};
use super::ASTBody;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType(pub Vec<TypeKind>, pub Box<TypeKind>);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassType(pub HashMap<String, TypeKind>);

#[derive(Clone, Debug)]
pub enum LiteralKind {
    Nothing,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
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
    Colon,
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

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    Unknown,
    Int,
    Float,
    String,
    Bool,
    Nothing,
    Vector(Box<TypeKind>),
    Function(FunctionType),
    Object(Arc<ClassType>),
}

impl TypeKind {
    pub fn is_compatible(&self, other: &TypeKind) -> bool {

        match (self, other) {
            (a, b) if a == b => true,
            (TypeKind::Unknown, _) => true,
            (_, TypeKind::Unknown) => true,
            _ => false
        }
    }
    
    pub fn is_unknown(&self) -> bool {
        match self {
            TypeKind::Unknown => true,
            _ => false,
        }
    }
}

impl From<LiteralKind> for TypeKind {
    fn from(literal_kind: LiteralKind) -> Self {
        match literal_kind {
            LiteralKind::Nothing => Self::Nothing,
            LiteralKind::Int(_) => Self::Int,
            LiteralKind::Float(_) => Self::Float,
            LiteralKind::Bool(_) => Self::Bool,
            LiteralKind::String(_) => Self::String,
        }
    }
}

pub struct Class {
    pub functions: Vec<(String, Arc<Function>)>,
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Class]")
    }
}

impl Class {
    pub fn new(functions: Vec<(String, Arc<Function>)>) -> Self {
        Self { functions }
    }
}

pub struct Function {
    pub body: ASTBody,
    pub parameters: Vec<String>,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Function]")
    }
}

impl Function {
    pub fn new(body: ASTBody, parameters: Vec<String>) -> Arc<Function> {
        Arc::new(Self { body, parameters })
    }
}