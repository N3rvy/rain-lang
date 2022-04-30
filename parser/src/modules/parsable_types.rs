use common::ast::parsing_types::{ParsableFunctionType, ParsableType};
use common::ast::types::ClassKind;
use common::module::{ModuleIdentifier, ModuleUID};
use common::tokens::TokenSnapshot;
use tokenizer::iterator::Tokens;

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
