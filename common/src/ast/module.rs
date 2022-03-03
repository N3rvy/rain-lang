use std::sync::Arc;

use super::{types::Function, ASTNode};


pub struct ASTModule {
    pub functions: Vec<(String, Arc<Function>)>,
    pub variables: Vec<(String, ASTNode)>,
}

impl ASTModule {
    pub fn new(functions: Vec<(String, Arc<Function>)>, variables: Vec<(String, ASTNode)>) -> Self {
        Self {
            functions,
            variables,
        }
    }
}