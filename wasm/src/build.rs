use core::LangError;
use std::{sync::Arc, collections::HashMap};
use common::ast::{ASTNode, NodeKind, types::{TypeKind, FunctionType, MathOperatorKind, BoolOperatorKind}};
use walrus::{ModuleConfig, Module, FunctionBuilder, ValType, LocalId, InstrSeqBuilder, ir::{BinaryOp, InstrSeqType}};
use common::ast::types::LiteralKind;

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

        build_function(&mut module_build, builder, name, func.clone(), func_type)?;
    }

    Ok(module_build.emit_wasm())
}

pub fn build_function(
    module: &mut Module,
    mut builder: FunctionBuilder,
    name: &str,
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

    let func_id = builder.finish(Vec::new(), &mut module.funcs);

    module.exports.add(name, func_id);

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
        NodeKind::VariableAsgn { name, value } => {
            build_statement(scope, builder, value)?;

            match scope.vars.get(name) {
                Some(id) => builder.local_set(*id),
                None => todo!(),
            };
        },
        NodeKind::FunctionInvok { variable, parameters } => todo!(),
        NodeKind::Literal { value } => {
            match value {
                LiteralKind::Nothing => todo!(),
                LiteralKind::Int(i) => builder.i32_const(*i),
                LiteralKind::Float(f) => builder.f32_const(*f),
                LiteralKind::String(_) => todo!(),
            };
        },
        NodeKind::MathOperation { operation, left, right } => {
            build_statement(scope, builder, left)?;
            build_statement(scope, builder, right)?;

            let op = match operation {
                MathOperatorKind::Plus => BinaryOp::I32Add,
                MathOperatorKind::Minus => BinaryOp::I32Sub,
                MathOperatorKind::Multiply => BinaryOp::I32Mul,
                MathOperatorKind::Divide => BinaryOp::I32DivS,
                MathOperatorKind::Modulus => todo!(),
                MathOperatorKind::Power => todo!(),
            };

            builder.binop(op);
        },
        NodeKind::BoolOperation { operation, left, right } => {
            build_statement(scope, builder, left)?;
            build_statement(scope, builder, right)?;

            let op = match operation {
                BoolOperatorKind::Equal => BinaryOp::I32Eq,
                BoolOperatorKind::Different => BinaryOp::I32Ne,
                BoolOperatorKind::Bigger => BinaryOp::I32GtS,
                BoolOperatorKind::Smaller => BinaryOp::I32LtS,
                BoolOperatorKind::BiggerEq => BinaryOp::I32GeS,
                BoolOperatorKind::SmallerEq => BinaryOp::I32LeS,
            };
        },
        NodeKind::ReturnStatement { value, kind } => {
            match value {
                Some(value) => {
                    build_statement(scope, builder, value)?
                },
                None => todo!(),
            }
        }
        NodeKind::IfStatement { condition, body } => {
            build_statement(scope, builder, condition)?;
            let mut result = Ok(());

            builder.if_else(
                InstrSeqType::Simple(None),
                |then| {
                    for node in body {
                        match build_statement(scope, then, node) {
                            Ok(_) => (),
                            Err(err) => {
                                result = Err(err);
                                break;
                            },
                        }
                    }
                },
                |else_| {})
            .drop();

            result?;
        },
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