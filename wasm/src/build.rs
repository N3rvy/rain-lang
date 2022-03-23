use core::LangError;
use std::{sync::Arc, collections::HashMap};
use common::ast::{ASTNode, NodeKind, types::{TypeKind, FunctionType}};
use walrus::{ModuleConfig, Module, FunctionBuilder, ValType, LocalId, InstrSeqBuilder};

use crate::errors::UNEXPECTED_ERROR;

pub struct FunctionBuilderScope {
    vars: HashMap<String, LocalId>,
}

pub fn build_module(module: Arc<common::module::Module>) -> Result<Vec<u8>, LangError> {
    let config = ModuleConfig::default();
    let mut module_build = Module::with_config(config);

    for (name, func) in &module.functions {
        let func_type = module.metadata.definitions
            .iter()
            .find(|def| def.0 == *name)
            .and_then(|def| match &def.1 {
                TypeKind::Function(func) => Some(func),
                _ => None,
            });

        let func_type = match func_type {
            Some(t) => t,
            None => return Err(LangError::new_runtime(UNEXPECTED_ERROR.to_string())),
        };

        let builder = FunctionBuilder::new(&mut module_build.types, &[ValType::I32], &[ValType::I32]);

        build_function(&mut module_build, builder, func.clone(), func_type)?;
    }

    todo!()
}

pub fn build_function(
    module: &mut Module,
    mut builder: FunctionBuilder,
    func: Arc<common::ast::types::Function>,
    func_type: &FunctionType,
) -> Result<(), LangError> {
    let mut scope = FunctionBuilderScope {
        vars: HashMap::new(),
    };

    for i in 0..func_type.0.len() {
        let name = &func.parameters[i];
        let type_ = &func_type.0[i];

        let local = module.locals.add(convert_type(type_));

        scope.vars.insert(
            name.clone(),
            local,
        );
    }

    let mut body = builder.func_body();

    for node in &func.body {
        build_statement(&mut scope, &mut body, node)?;
    }

    Ok(())
}

#[allow(unused_variables)]
pub fn build_statement(scope: &FunctionBuilderScope, builder: &mut InstrSeqBuilder, node: &ASTNode) -> Result<(), LangError> {
    match node.kind.as_ref() {
        NodeKind::VariableDecl { name, value } => todo!(),
        NodeKind::VariableRef { module, name } => {
            match scope.vars.get(name) {
                Some(id) => {
                    builder
                        .local_get(*id);
                },
                None => todo!(),
            }
        },
        NodeKind::VariableAsgn { name, value } => todo!(),
        NodeKind::FunctionInvok { variable, parameters } => todo!(),
        NodeKind::Literal { value } => todo!(),
        NodeKind::MathOperation { operation, left, right } => todo!(),
        NodeKind::BoolOperation { operation, left, right } => todo!(),
        NodeKind::ReturnStatement { value, kind } => {
            match value {
                Some(value) => {
                    build_statement(scope, builder, value)?
                },
                None => todo!(),
            }
        }
        NodeKind::IfStatement { condition, body } => todo!(),
        NodeKind::ForStatement { left, right, body, iter_name } => todo!(),
        NodeKind::WhileStatement { condition, body } => todo!(),
        NodeKind::FieldAccess { variable, field_name } => todo!(),
        NodeKind::VectorLiteral { values } => todo!(),
        NodeKind::ObjectLiteral { values } => todo!(),
        NodeKind::FunctionLiteral { value } => todo!(),
        NodeKind::ValueFieldAccess { variable, value } => todo!(),
        NodeKind::Import { identifier } => todo!(),
    };

    Ok(())
}

fn convert_type(type_: &TypeKind) -> ValType {
    match type_ {
        TypeKind::Unknown => todo!(),
        TypeKind::Int => ValType::I32,
        TypeKind::Float => todo!(),
        TypeKind::String => todo!(),
        TypeKind::Bool => todo!(),
        TypeKind::Nothing => todo!(),
        TypeKind::Vector(_) => todo!(),
        TypeKind::Function(_) => todo!(),
        TypeKind::Object(_) => todo!(),
    }
}