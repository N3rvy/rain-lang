use wasm_encoder::{BlockType, Instruction, ValType};
use common::ast::{ASTNode, NodeKind};
use common::ast::types::{LiteralKind, TypeKind, FunctionType, Function};
use common::errors::LangError;
use common::module::{ModuleUID, Module};
use core::parser::ModuleLoader;
use std::sync::Arc;
use crate::build::convert_type;
use crate::errors::{FUNC_NOT_FOUND, INVALID_STACK_SIZE, INVALID_STACK_TYPE, LOCAL_NOT_FOUND, MODULE_NOT_FOUND, UNSUPPORTED_FUNC_INVOKE, UNEXPECTED_ERROR};

pub struct FunctionBuilderResult {
    pub name: String,

    pub params: Vec<ValType>,
    pub ret: ValType,

    pub locals: Vec<(String, ValType)>,
    pub instructions: Vec<Instruction<'static>>,
}

pub struct ModuleBuilderResult {
    pub functions: Vec<FunctionBuilderResult>,
}

pub struct ModuleBuilder<'a> {
    module_loader: &'a ModuleLoader,
    functions: Vec<(String, Vec<ValType>, ValType)>,

    result_funcs: Vec<FunctionBuilderResult>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(module_loader: &'a ModuleLoader) -> Self {
        Self {
            module_loader,
            functions: Vec::new(),
            result_funcs: Vec::new(),
        }
    }

    pub fn insert_module(&mut self, module: Arc<Module>) -> Result<(), LangError> {
        for (name, func) in &module.functions {
            let contains_func = self.functions
                .iter()
                .any(|(n, _, _)| n == name);
            
            if contains_func {
                continue
            }

            let (_, type_kind) = module.metadata.definitions
                .iter()
                .find(|(n, _)| n == name)
                .unwrap();

            let func_type = match type_kind {
                TypeKind::Function(func_type) => func_type,
                _ => return Err(LangError::new_runtime(UNEXPECTED_ERROR.to_string())),
            };

            self.insert_func(name, func_type, func)?;
        }

        Ok(())
    }

    pub fn insert_func(&mut self, name: &str, func_type: &FunctionType, func: &Arc<Function>) -> Result<(), LangError> {
        let locals = func_type.0
                .iter()
                .enumerate()
                .map(|(i, type_)| {
                    (
                        func.parameters
                            .get(i)
                            .unwrap()
                            .clone(),
                        convert_type(type_),
                    )
                })
                .collect();

        let params = func_type.0
                .iter()
                .map(|type_| convert_type(type_))
                .collect();

        let mut code_builder = FunctionBuilder::new(
            self,
            name.to_string(),
            locals,
            params,
            convert_type(&func_type.1));

        for node in &func.body {
            code_builder.build_statement(&node)?;
        }

        let result = code_builder.build();

        self.result_funcs.push(result);

        Ok(())
    }

    pub fn build(self) -> ModuleBuilderResult {
        ModuleBuilderResult {
            functions: self.result_funcs,
        }
    }

    fn get_func(&mut self, module_uid: ModuleUID, name: &String) -> Result<(u32, &Vec<ValType>, &ValType), LangError> {
        // This "code duplication" is done because otherwise it would complain
        // that self.functions is already borrowed
        let contains_func = self.functions
            .iter()
            .any(|(n, _, _)| n == name);

        match contains_func {
            true => {
                self.functions
                    .iter()
                    .enumerate()
                    .find_map(|(i, (n, params, type_))| {
                        if n == name {
                            Some((i as u32, params, type_))
                        } else {
                            None
                        }
                    })
                    .ok_or(LangError::new_runtime(FUNC_NOT_FOUND.to_string()))
            },
            false => {
                let module = self.module_loader
                    .get_module(module_uid);

                let module = match module {
                    Some(m) => m,
                    None => return Err(LangError::new_runtime(MODULE_NOT_FOUND.to_string())),
                };

                let metadata = module.metadata.definitions
                    .iter()
                    .find_map(|(n, type_)| {
                        if n == name {
                            Some(type_)
                        } else {
                            None
                        }
                    });

                let func_type = match metadata {
                    Some(TypeKind::Function(ft)) => ft,
                    _ => return Err(LangError::new_runtime(FUNC_NOT_FOUND.to_string())),
                };

                let func = module.functions
                    .iter()
                    .find(|(n, _)| n == name);

                let func = match func {
                    Some(f) => &f.1,
                    None => return Err(LangError::new_runtime(UNEXPECTED_ERROR.to_string())),
                };
                
                self.insert_func(name.as_ref(), func_type, func)?;

                self.functions.push((
                    name.clone(),
                    func_type.0
                        .iter()
                        .map(|type_| convert_type(type_))
                        .collect(),
                    convert_type(func_type.1.as_ref()),
                ));

                let (_, params, ret) = self.functions
                    .last()
                    .unwrap();

                Ok((
                    self.functions.len() as u32 - 1,
                    params,
                    ret
                ))
            },
        }
    }
}

pub struct FunctionBuilder<'a, 'b> {
    pub(crate) module_builder: &'a mut ModuleBuilder<'b>,
    pub(crate) type_stack: Vec<ValType>,

    pub(crate) name: String,

    pub(crate) params: Vec<ValType>,
    pub(crate) ret: ValType,

    // Stores all the locals (never removes)
    pub(crate) locals: Vec<(String, ValType)>,
    // Instructions of the function
    pub(crate) instructions: Vec<Instruction<'static>>,
}

impl<'a, 'b> FunctionBuilder<'a, 'b> {
    pub fn new(
        module_builder: &'a mut ModuleBuilder<'b>,
        name: String,
        locals: Vec<(String, ValType)>,
        params: Vec<ValType>,
        ret: ValType,
    ) -> Self {
        Self {
            name,

            module_builder,
            locals,

            params,
            ret,

            type_stack: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn build(mut self) -> FunctionBuilderResult {
        self.instructions.push(Instruction::End);

        FunctionBuilderResult {
            name: self.name,

            params: self.params,
            ret: self.ret,

            locals: self.locals,
            instructions: self.instructions,
        }
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
                let (module, name) = match variable.kind.as_ref() {
                    NodeKind::VariableRef { name, module } => (module, name),
                    _ => return Err(LangError::new_runtime(UNSUPPORTED_FUNC_INVOKE.to_string())),
                };

                for param in parameters {
                    self.build_statement(param)?;
                }

                let (func_id, param_types, ret_type) = self.module_builder.get_func(*module, name)?;

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
                    LiteralKind::Bool(b) => {
                        let value = if *b { 1 } else { 0 };

                        self.instructions.push(Instruction::I32Const(value));
                        self.type_stack.push(ValType::I32);
                    },
                    LiteralKind::String(_) => todo!(),
                };
            },
            NodeKind::MathOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                let right = self.type_stack.pop().unwrap();
                let left = self.type_stack.pop().unwrap();

                self.type_stack.push(left);

                self.build_math_op(operation, left, right);
            },
            NodeKind::BoolOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                let right = self.type_stack.pop().unwrap();
                let left = self.type_stack.pop().unwrap();

                self.type_stack.push(left);

                self.build_bool_op(operation, left, right);
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

                // "iter_name" < "right"
                self.instructions.push(Instruction::LocalGet(id));
                self.build_statement(right)?;
                self.instructions.push(Instruction::I32LtS);

                let stack_size = self.type_stack.len();

                // Open if
                self.instructions.push(Instruction::If(BlockType::Empty));

                // Building body
                for node in body {
                    self.build_statement(node)?;
                }

                // Add 1 to "iter_name"
                self.instructions.push(Instruction::LocalGet(id));
                self.instructions.push(Instruction::I32Const(1));
                self.instructions.push(Instruction::I32Add);
                self.instructions.push(Instruction::LocalSet(id));

                // Goto block
                self.instructions.push(Instruction::Br(1));

                // Close if
                self.instructions.push(Instruction::End);

                // Close loop
                self.instructions.push(Instruction::End);

                self.assert_stack_size(stack_size)?;
            },
            NodeKind::WhileStatement { condition, body} => {

                // Open loop
                self.instructions.push(Instruction::Loop(BlockType::Empty));

                let condition_stack_size = self.type_stack.len();

                // Building condition
                self.build_statement(condition)?;

                self.assert_stack_size(condition_stack_size + 1)?;

                let stack_size = self.type_stack.len();

                // Open if
                self.instructions.push(Instruction::If(BlockType::Empty));

                // Building body
                for node in body {
                    self.build_statement(node)?;
                }

                // Goto block
                self.instructions.push(Instruction::Br(1));

                // Close if
                self.instructions.push(Instruction::End);

                self.assert_stack_size(stack_size)?;

                // Close loop
                self.instructions.push(Instruction::End);
            }
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