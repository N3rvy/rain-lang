use std::collections::HashMap;
use wasm_encoder::{Function, Instruction};
use common::ast::{ASTNode, NodeKind};
use common::ast::types::{LiteralKind};
use common::errors::LangError;

pub struct CodeBuilder<'a> {
    func: &'a mut Function,
    locals: HashMap<String, u32>,
}

impl<'a> CodeBuilder<'a> {
    pub fn new(func: &'a mut Function) -> Self {
        Self {
            func,
            locals: HashMap::new(),
        }
    }

    pub fn register_local(&mut self, name: String, value: u32) {
        self.locals.insert(name, value);
    }

    pub fn end_build(mut self) {
        self.func.instruction(&Instruction::End);
    }

    pub fn build_statement(&mut self, node: &ASTNode) -> Result<(), LangError> {
        match node.kind.as_ref() {
            NodeKind::VariableDecl { .. } => {}
            NodeKind::VariableRef { .. } => {}
            NodeKind::VariableAsgn { .. } => {}
            NodeKind::FunctionInvok { .. } => {}
            NodeKind::Literal { value } => {
                match value {
                    LiteralKind::Nothing => (),
                    LiteralKind::Int(i) => {
                        self.func.instruction(&Instruction::I32Const(*i));
                    },
                    LiteralKind::Float(f) => {
                        self.func.instruction(&Instruction::F32Const(*f));
                    },
                    LiteralKind::String(_) => todo!(),
                };
            },
            NodeKind::MathOperation { .. } => {}
            NodeKind::BoolOperation { .. } => {}
            NodeKind::ReturnStatement { kind: _ , value } => {
                match value {
                    Some(value) => {
                        self.build_statement(value)?;
                    }
                    None => ()
                }

                self.func.instruction(&Instruction::Return);
            },
            NodeKind::IfStatement { .. } => {}
            NodeKind::ForStatement { .. } => {}
            NodeKind::WhileStatement { .. } => {}
            NodeKind::FieldAccess { .. } => {}
            NodeKind::VectorLiteral { .. } => {}
            NodeKind::ObjectLiteral { .. } => {}
            NodeKind::FunctionLiteral { .. } => {}
            NodeKind::ValueFieldAccess { .. } => {}
            NodeKind::Import { .. } => {}
        }

        Ok(())
    }
}