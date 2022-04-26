use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use crate::ast::types::{Class, Function, FunctionType, LiteralKind, ClassType, TypeKind};

#[derive(Clone)]
pub struct ModuleIdentifier(pub String);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct ModuleUID(u64);

impl ModuleUID {
    pub fn from_string(string: String) -> Self {
        let mut hasher = DefaultHasher::new();
        string.hash(&mut hasher);

        ModuleUID(hasher.finish())
    }
}

pub struct FunctionDefinition {
    pub data: Arc<Function>,
    pub metadata: FunctionType,
}

pub struct VariableDefinition {
    pub data: LiteralKind,
    pub metadata: TypeKind,
}

pub struct ClassDefinition {
    pub data: Class,
    pub metadata: Arc<ClassType>,
}

impl ClassDefinition {
    pub fn get_method_def(&self, name: &String) -> Option<FunctionDefinition> {
        self.data.functions
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, def)| {
                let (_, func_type) = self.metadata.methods
                    .iter()
                    .find(|(n, _)| n == name)
                    .unwrap();

                Some(FunctionDefinition {
                    data: def.clone(),
                    metadata: func_type.clone(),
                })
            })
    }
}

pub struct Module {
    pub uid: ModuleUID,

    pub imports: Vec<ModuleUID>,
    pub functions: Vec<(String, FunctionDefinition)>,
    pub variables: Vec<(String, VariableDefinition)>,
    pub classes: Vec<(String, ClassDefinition)>
}

impl Module {
    pub fn get_func_def(&self, name: &String) -> Option<&FunctionDefinition> {
        self.functions
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, def)| Some(def))
    }

    pub fn get_var_def(&self, name: &String) -> Option<&VariableDefinition> {
        self.variables
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, def)| Some(def))
    }

    pub fn get_class_def(&self, name: &String) -> Option<&ClassDefinition> {
        self.classes
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, def)| Some(def))
    }
}

pub struct DefinitionModule {
    pub id: ModuleIdentifier,

    pub imports: Vec<ModuleUID>,
    pub functions: Vec<(String, FunctionType)>,
}

impl DefinitionModule {
    pub fn get_func_type(&self, name: &String) -> Option<&FunctionType> {
        self.functions
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, type_)| Some(type_))
    }
}
