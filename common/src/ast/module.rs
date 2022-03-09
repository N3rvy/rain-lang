use std::sync::Arc;
use crate::module::ModuleUID;
use super::{types::Function, ASTNode};

pub struct ASTModule {
    pub imports: Vec<ModuleUID>,
    pub functions: Vec<(String, Arc<Function>)>,
    pub variables: Vec<(String, ASTNode)>,
}