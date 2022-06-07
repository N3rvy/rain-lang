use std::{sync::Arc, fmt::Debug};
use std::cell::RefCell;
use crate::module::{FunctionDefinition, ModuleUID};
use crate::tokens::PrimitiveType;
use super::ASTBody;

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Data,
    Import,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassKind {
    Normal,
    Data,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType(pub Vec<TypeKind>, pub Box<TypeKind>);

#[derive(Debug)]
pub struct ClassType {
    pub name: String,
    pub module: ModuleUID,
    pub kind: ClassKind,
    pub fields: RefCell<Vec<(String, TypeKind)>>,
    pub methods: RefCell<Vec<(String, FunctionType)>>,
}

impl PartialEq for ClassType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.module == other.module
    }
}

unsafe impl Send for ClassType {}
unsafe impl Sync for ClassType {}

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
    Class(Arc<ClassType>),
}

impl From<&PrimitiveType> for TypeKind {
    fn from(primitive: &PrimitiveType) -> Self {
        match primitive {
            PrimitiveType::Int => TypeKind::Int,
            PrimitiveType::Float => TypeKind::Float,
            PrimitiveType::String => TypeKind::String,
            PrimitiveType::Bool => TypeKind::Bool,
            PrimitiveType::Nothing => TypeKind::Nothing,
        }
    }
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

impl From<&LiteralKind> for TypeKind {
    fn from(literal_kind: &LiteralKind) -> Self {
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
    pub methods: Vec<(String, FunctionDefinition)>,
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Class]")
    }
}

impl Class {
    pub fn new(methods: Vec<(String, FunctionDefinition)>) -> Self {
        Self { methods }
    }
}

pub struct Function {
    pub body: ASTBody,
    pub parameters: Vec<String>,
    pub method: Option<Arc<ClassType>>,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(_) = &self.method {
            f.write_str("[Method]")
        } else {
            f.write_str("[Function]")
        }
    }
}

impl Function {
    pub fn new(body: ASTBody, parameters: Vec<String>, method: Option<Arc<ClassType>>) -> Arc<Function> {
        Arc::new(Self {
            body,
            parameters,
            method,
        })
    }
}