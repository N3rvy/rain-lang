use common::ast::types::{ClassKind, TypeKind};
use common::module::{ModuleIdentifier, ModuleUID};
use common::tokens::PrimitiveType;
use tokenizer::iterator::{Tokens, TokenSnapshot};

pub struct ParsableFunctionType(pub Vec<ParsableType>, pub Box<ParsableType>);

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

pub struct ParsableVariable {
    pub type_kind: ParsableType,
    pub body: Option<TokenSnapshot>,
}

pub struct ParsableFunction {
    pub func_type: ParsableFunctionType,
    pub params: Vec<String>,
    pub body: Option<TokenSnapshot>,
}

pub struct ParsableClass {
    pub kind: ClassKind,
    pub fields: Vec<(String, ParsableType)>,
    pub methods: Vec<(String, ParsableFunction)>
}

/// This represents a module that needs more processing to be parsed
pub struct ParsableModule {
    pub id: ModuleIdentifier,
    pub uid: ModuleUID,

    pub tokens: Tokens,
    pub imports: Vec<ModuleIdentifier>,
    pub variables: Vec<(String, ParsableVariable)>,
    pub functions: Vec<(String, ParsableFunction)>,
    pub classes: Vec<(String, ParsableClass)>,
}
