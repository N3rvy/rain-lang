use crate::ast::parsing_types::{ParsableFunctionType, ParsableType};
use crate::ast::types::{Attribute, ClassKind};
use crate::module::{ModuleIdentifier, ModuleUID};
use crate::tokens::TokenSnapshot;
use crate::tokens_iterator::Tokens;

pub struct ParsableVariable {
    pub attributes: Vec<Attribute>,
    pub type_kind: ParsableType,
    pub body: Option<TokenSnapshot>,
}

pub struct ParsableFunction {
    pub attributes: Vec<Attribute>,
    pub func_type: ParsableFunctionType,
    pub params: Vec<String>,
    pub body: Option<TokenSnapshot>,
}

pub struct ParsableClass {
    pub attributes: Vec<Attribute>,
    pub kind: ClassKind,
    pub name: String,
    pub module: ModuleUID,

    pub fields: Vec<(String, ParsableType)>,
    pub methods: Vec<(String, ParsableFunction)>
}

pub struct ParsableEnum {
    pub name: String,
    pub variants: Vec<(String, Option<ParsableType>)>,
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
    pub enums: Vec<(String, ParsableEnum)>,
}
