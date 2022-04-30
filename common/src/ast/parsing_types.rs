use crate::ast::types::{FunctionType, LiteralKind, TypeKind};
use crate::tokens::PrimitiveType;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsableFunctionType(pub Vec<ParsableType>, pub Box<ParsableType>);

impl From<&FunctionType> for ParsableFunctionType {
    fn from(function_type: &FunctionType) -> Self {
        ParsableFunctionType(
            function_type
                .0
                .iter()
                .map(|parameter| ParsableType::from(parameter))
                .collect(),
            Box::new(ParsableType::from(function_type.1.as_ref())),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParsableType {
    Unknown,
    Int,
    Float,
    String,
    Bool,
    Nothing,
    Vector(Box<ParsableType>),
    Function(ParsableFunctionType),
    Custom(String),
}

impl From<&LiteralKind> for ParsableType {
    fn from(literal_kind: &LiteralKind) -> Self {
        match literal_kind {
            LiteralKind::Int(_) => ParsableType::Int,
            LiteralKind::Float(_) => ParsableType::Float,
            LiteralKind::String(_) => ParsableType::String,
            LiteralKind::Bool(_) => ParsableType::Bool,
            LiteralKind::Nothing => ParsableType::Nothing,
        }
    }
}

impl From<&TypeKind> for ParsableType {
    fn from(kind: &TypeKind) -> Self {
        match kind {
            TypeKind::Unknown => ParsableType::Unknown,
            TypeKind::Nothing => ParsableType::Nothing,
            TypeKind::Int => ParsableType::Int,
            TypeKind::Float => ParsableType::Float,
            TypeKind::String => ParsableType::String,
            TypeKind::Bool => ParsableType::Bool,
            TypeKind::Vector(inner) => ParsableType::Vector(Box::new(ParsableType::from(inner.as_ref()))),
            TypeKind::Function(func_type) => {
                ParsableType::Function(ParsableFunctionType(
                    func_type.0.iter().map(|arg| ParsableType::from(arg)).collect(),
                    Box::new(ParsableType::from(func_type.1.as_ref())),
                ))
            }
            TypeKind::Object(class_type) => ParsableType::Custom(class_type.name.to_string()),
        }
    }
}

impl From<&PrimitiveType> for ParsableType {
    fn from(tk: &PrimitiveType) -> Self {
        match tk {
            PrimitiveType::Nothing => ParsableType::Nothing,
            PrimitiveType::Int => ParsableType::Int,
            PrimitiveType::Float => ParsableType::Float,
            PrimitiveType::Bool => ParsableType::Bool,
            PrimitiveType::String => ParsableType::String,
        }
    }
}
