use std::collections::HashMap;
use wasm_encoder::{BlockType, Function, Instruction};
use common::ast::{ASTNode, NodeKind};
use common::ast::types::{BoolOperatorKind, LiteralKind, MathOperatorKind};
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

    pub fn end_build(self) {
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
            NodeKind::MathOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                let op = match operation {
                    MathOperatorKind::Plus => Instruction::I32Add,
                    MathOperatorKind::Minus => Instruction::I32Sub,
                    MathOperatorKind::Multiply => Instruction::I32Mul,
                    MathOperatorKind::Divide => Instruction::I32DivS,
                    MathOperatorKind::Modulus => todo!(),
                    MathOperatorKind::Power => todo!(),
                };

                self.func.instruction(&op);
            },
            NodeKind::BoolOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                let op = match operation {
                    BoolOperatorKind::Equal => Instruction::I32Eq,
                    BoolOperatorKind::Different => Instruction::I32Ne,
                    BoolOperatorKind::Bigger => Instruction::I32GtS,
                    BoolOperatorKind::Smaller => Instruction::I32LtS,
                    BoolOperatorKind::BiggerEq => Instruction::I32GeS,
                    BoolOperatorKind::SmallerEq => Instruction::I32LeS,
                };

                self.func.instruction(&op);
            },
            NodeKind::ReturnStatement { kind: _ , value } => {
                match value {
                    Some(value) => {
                        self.build_statement(value)?;
                    }
                    None => ()
                }

                self.func.instruction(&Instruction::Return);
            },
            NodeKind::IfStatement { condition, body } => {
                self.build_statement(condition)?;

                self.func.instruction(&Instruction::If(BlockType::Empty));

                for node in body {
                    self.build_statement(node)?;
                }

                self.func.instruction(&Instruction::End);
            },
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