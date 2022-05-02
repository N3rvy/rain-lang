use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
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
    pub data: Option<Arc<Function>>,
    pub metadata: FunctionType,
}

pub struct VariableDefinition {
    pub data: Option<LiteralKind>,
    pub metadata: TypeKind,
}

pub struct ClassDefinition {
    pub data: Class,
    pub metadata: Arc<ClassType>,
}

impl ClassDefinition {
    pub fn get_method_def(&self, name: &String) -> Option<FunctionDefinition> {
        let metadata = self.metadata.methods
            .borrow()
            .iter()
            .find(|(n, _)| n == name)?
            .1.clone();

        let data = self.data.methods
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, f)| f.data.clone());

        Some(FunctionDefinition {
            data,
            metadata,
        })
    }
}

pub enum ModuleFeature {
    Function(FunctionDefinition),
    Variable(VariableDefinition),
    Class(ClassDefinition),
}

pub struct Module {
    pub id: ModuleIdentifier,
    pub uid: ModuleUID,

    pub imports: Vec<ModuleUID>,
    pub features: HashMap<String, ModuleFeature>,
}

impl Module {
    pub fn get_func_feature(&self, name: &String) -> Option<&FunctionDefinition> {
        self.features
            .get(name)
            .and_then(|feature| match feature {
                ModuleFeature::Function(def) => Some(def),
                _ => None,
            })
    }

    pub fn get_var_feature(&self, name: &String) -> Option<&VariableDefinition> {
        self.features
            .get(name)
            .and_then(|feature| match feature {
                ModuleFeature::Variable(def) => Some(def),
                _ => None,
            })
    }

    pub fn get_class_feature(&self, name: &String) -> Option<&ClassDefinition> {
        self.features
            .get(name)
            .and_then(|feature| match feature {
                ModuleFeature::Class(def) => Some(def),
                _ => None,
            })
    }
}