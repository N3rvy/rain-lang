use wasm_encoder::{BlockType, Instruction, ValType};
use common::ast::{ASTNode, NodeKind};
use common::ast::types::{BoolOperatorKind, LiteralKind, MathOperatorKind};
use common::errors::LangError;
use crate::errors::{FUNC_NOT_FOUND, INVALID_STACK_SIZE, INVALID_STACK_TYPE, LOCAL_NOT_FOUND, UNSUPPORTED_FUNC_INVOKE};

pub struct ModuleBuilder {
    functions: Vec<(String, Vec<ValType>, ValType)>,
}

impl ModuleBuilder {
    pub fn new(functions: Vec<(String, Vec<ValType>, ValType)>) -> Self {
        Self {
            functions,
        }
    }

    fn get_func(&self, name: &String) -> Result<(u32, &Vec<ValType>, &ValType), LangError> {
        let func = self.functions
            .iter()
            .enumerate()
            .find_map(|(i, (n, params, type_))| {
                if n == name {
                    Some((i as u32, params, type_))
                } else {
                    None
                }
            });

        match func {
            Some(func) => Ok(func),
            None => Err(LangError::new_runtime(FUNC_NOT_FOUND.to_string())),
        }
    }
}

pub struct FunctionBuilder<'a> {
    module_builder: &'a mut ModuleBuilder,
    type_stack: Vec<ValType>,

    // Stores all the locals (never removes)
    locals: Vec<(String, ValType)>,
    // Instructions of the function
    instructions: Vec<Instruction<'a>>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(module_builder: &'a mut ModuleBuilder, locals: Vec<(String, ValType)>) -> Self {
        Self {
            module_builder,
            locals,
            type_stack: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn end_build(&mut self) -> (&Vec<(String, ValType)>, &Vec<Instruction<'a>>) {
        self.instructions.push(Instruction::End);

        (&self.locals, &self.instructions)
    }

    pub fn build_statement(&mut self, node: &ASTNode) -> Result<(), LangError> {
        match node.kind.as_ref() {
            NodeKind::VariableDecl { name, value } => {
                // Build value
                self.build_statement(value)?;

                // Remove "value" from the stack and add it's type to the locals
                let type_ = self.type_stack.pop().unwrap();

                // Add "name" to the locals
                self.locals.push((name.clone(), type_));
                // Obtain the newly created local id
                let id = self.locals.len() as u32 - 1;

                self.instructions.push(Instruction::LocalSet(id));
            },
            NodeKind::VariableRef { module: _, name } => {
                let (id, type_) = self.get_local(name)?;

                self.instructions.push(Instruction::LocalGet(id));
                self.type_stack.push(type_);
            },
            NodeKind::VariableAsgn { name, value } => {
                self.build_statement(value)?;

                let type_ = self.type_stack.pop().unwrap();

                let (id, local_type) = self.get_local(name)?;

                Self::assert_type(type_, local_type)?;

                self.instructions.push(Instruction::LocalSet(id));
            },
            NodeKind::FunctionInvok { variable, parameters } => {
                // TODO: Support for other kinds of invocations
                let name = match variable.kind.as_ref() {
                    NodeKind::VariableRef { name, module: _ } => name,
                    _ => return Err(LangError::new_runtime(UNSUPPORTED_FUNC_INVOKE.to_string())),
                };

                for param in parameters {
                    self.build_statement(param)?;
                }

                let (func_id, param_types, ret_type) = self.module_builder.get_func(name)?;

                for param in param_types {
                    let type_ = self.type_stack.pop().unwrap();

                    Self::assert_type(type_, param.clone())?;
                }

                self.type_stack.push(ret_type.clone());

                self.instructions.push(Instruction::Call(func_id));
            },
            NodeKind::Literal { value } => {
                match value {
                    LiteralKind::Nothing => (),
                    LiteralKind::Int(i) => {
                        self.instructions.push(Instruction::I32Const(*i));
                        self.type_stack.push(ValType::I32);
                    },
                    LiteralKind::Float(f) => {
                        self.instructions.push(Instruction::F32Const(*f));
                        self.type_stack.push(ValType::F32);
                    },
                    LiteralKind::String(_) => todo!(),
                };
            },
            NodeKind::MathOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                self.type_stack.pop();
                self.type_stack.pop();

                self.type_stack.push(ValType::I32);

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

                self.type_stack.pop();
                self.type_stack.pop();

                self.type_stack.push(ValType::I32);

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

                self.type_stack.pop();

                self.instructions.push(Instruction::Return);
            },
            NodeKind::IfStatement { condition, body } => {
                self.build_statement(condition)?;

                self.type_stack.pop();

                let stack_size = self.type_stack.len();

                self.instructions.push(Instruction::If(BlockType::Empty));

                for node in body {
                    self.build_statement(node)?;
                }

                self.assert_stack_size(stack_size)?;

                self.instructions.push(Instruction::End);
            },
            NodeKind::ForStatement { iter_name, left, right, body } => {
                self.build_statement(left)?;

                let type_ = self.type_stack.pop().unwrap();
                Self::assert_type(type_, ValType::I32)?;

                // Add "iter_name" to the locals
                self.locals.push((iter_name.clone(), ValType::I32));
                // Obtain the newly created local id
                let id = self.locals.len() as u32 - 1;

                // Setting "iter_name" to "left"
                self.instructions.push(Instruction::LocalSet(id));

                // Open loop
                self.instructions.push(Instruction::Loop(BlockType::Empty));

                // Building body
                for node in body {
                    self.build_statement(node)?;
                }

                // Add 1 to "iter_name"
                self.instructions.push(Instruction::LocalGet(id));
                self.instructions.push(Instruction::I32Const(1));
                self.instructions.push(Instruction::I32Add);
                self.instructions.push(Instruction::LocalTee(id));

                // "iter_name" < "right"
                self.build_statement(right)?;
                self.instructions.push(Instruction::I32LtS);

                let stack_size = self.type_stack.len();

                // Open if
                self.instructions.push(Instruction::If(BlockType::Empty));

                // Goto block
                self.instructions.push(Instruction::Br(1));

                // Close if
                self.instructions.push(Instruction::End);

                // Close loop
                self.instructions.push(Instruction::End);

                self.assert_stack_size(stack_size)?;
            },
            NodeKind::WhileStatement { .. } => {}
            NodeKind::FieldAccess { .. } => {}
            NodeKind::VectorLiteral { .. } => {}
            NodeKind::ObjectLiteral { .. } => {}
            NodeKind::FunctionLiteral { .. } => {}
            NodeKind::ValueFieldAccess { .. } => {}
        }

        Ok(())
    }

    fn get_local(&self, name: &String) -> Result<(u32, ValType), LangError> {
        let local = self.locals
            .iter()
            .enumerate()
            .find_map(|(i, (n, type_))| {
                if n == name {
                    Some((i as u32, type_.clone()))
                } else {
                    None
                }
            });

        match local {
            Some(local) => Ok(local),
            None => Err(LangError::new_runtime(LOCAL_NOT_FOUND.to_string())),
        }
    }

    #[inline]
    fn assert_type(a: ValType, b: ValType) -> Result<(), LangError> {
        if a == b {
            Ok(())
        } else {
            Err(LangError::new_runtime(INVALID_STACK_TYPE.to_string()))
        }
    }

    #[inline]
    fn assert_stack_size(&self, size: usize) -> Result<(), LangError> {
        if self.type_stack.len() == size {
            Ok(())
        } else {
            Err(LangError::new_runtime(INVALID_STACK_SIZE.to_string()))
        }
    }
}