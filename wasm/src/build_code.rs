use wasm_encoder::{BlockType, Instruction};
use common::ast::{ASTNode, NodeKind};
use common::ast::types::{BoolOperatorKind, LiteralKind, MathOperatorKind};
use common::errors::LangError;
use crate::errors::{FUNC_NOT_FOUND, LOCAL_NOT_FOUND, UNSUPPORTED_FUNC_INVOKE};

pub struct ModuleBuilder {
    functions: Vec<String>,
}

impl ModuleBuilder {
    pub fn new(functions: Vec<String>) -> Self {
        Self {
            functions,
        }
    }

    fn get_func(&self, name: &String) -> Result<u32, LangError> {
        let func = self.functions
            .iter()
            .position(|l| l == name);

        match func {
            Some(func) => Ok(func as u32),
            None => Err(LangError::new_runtime(FUNC_NOT_FOUND.to_string())),
        }
    }
}

pub struct FunctionBuilder<'a> {
    module_builder: &'a mut ModuleBuilder,
    locals: Vec<String>,

    local_count: u32,
    instructions: Vec<Instruction<'a>>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(module_builder: &'a mut ModuleBuilder, locals: Vec<String>) -> Self {
        Self {
            module_builder,
            locals,

            local_count: 0,
            instructions: Vec::new(),
        }
    }

    pub fn end_build(&mut self) -> (u32, &Vec<Instruction<'a>>) {
        self.instructions.push(Instruction::End);

        (self.local_count, &self.instructions)
    }

    pub fn build_statement(&mut self, node: &ASTNode) -> Result<(), LangError> {
        match node.kind.as_ref() {
            NodeKind::VariableDecl { name, value } => {
                self.locals.push(name.clone());
                let id = self.locals.len() as u32 - 1;

                self.build_statement(value)?;

                self.instructions.push(Instruction::LocalSet(id));
                self.local_count += 1;
            },
            NodeKind::VariableRef { module: _, name } => {
                let local = self.get_local(name)?;

                self.instructions.push(Instruction::LocalGet(local));
            },
            NodeKind::VariableAsgn { name, value } => {
                self.build_statement(value)?;

                let local = self.get_local(name)?;

                self.instructions.push(Instruction::LocalSet(local));
            },
            NodeKind::FunctionInvok { variable, parameters } => {
                // TODO: Support for other kinds of invocations
                let name = match variable.kind.as_ref() {
                    NodeKind::VariableRef { name, module: _ } => name,
                    _ => return Err(LangError::new_runtime(UNSUPPORTED_FUNC_INVOKE.to_string())),
                };

                let func_id = self.module_builder.get_func(name)?;

                for param in parameters {
                    self.build_statement(param)?;
                }

                self.instructions.push(Instruction::Call(func_id));
            },
            NodeKind::Literal { value } => {
                match value {
                    LiteralKind::Nothing => (),
                    LiteralKind::Int(i) => {
                        self.instructions.push(Instruction::I32Const(*i));
                    },
                    LiteralKind::Float(f) => {
                        self.instructions.push(Instruction::F32Const(*f));
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

                self.instructions.push(op);
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

                self.instructions.push(op);
            },
            NodeKind::ReturnStatement { kind: _ , value } => {
                match value {
                    Some(value) => {
                        self.build_statement(value)?;
                    }
                    None => ()
                }

                self.instructions.push(Instruction::Return);
            },
            NodeKind::IfStatement { condition, body } => {
                self.build_statement(condition)?;

                self.instructions.push(Instruction::If(BlockType::Empty));

                for node in body {
                    self.build_statement(node)?;
                }

                self.instructions.push(Instruction::End);
            },
            NodeKind::ForStatement { .. } => {
                self.local_count += 1;
            }
            NodeKind::WhileStatement { .. } => {}
            NodeKind::FieldAccess { .. } => {}
            NodeKind::VectorLiteral { .. } => {}
            NodeKind::ObjectLiteral { .. } => {}
            NodeKind::FunctionLiteral { .. } => {}
            NodeKind::ValueFieldAccess { .. } => {}
        }

        Ok(())
    }

    fn get_local(&self, name: &String) -> Result<u32, LangError> {
        let local = self.locals
            .iter()
            .position(|l| l == name);

        match local {
            Some(local) => Ok(local as u32),
            None => Err(LangError::new_runtime(LOCAL_NOT_FOUND.to_string())),
        }
    }
}