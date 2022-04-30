use std::ops::Index;
use wasm_encoder::{BlockType, Instruction, ValType, MemArg};
use common::ast::{ASTNode, NodeKind};
use common::ast::types::{LiteralKind, FunctionType, Function, TypeKind, ClassKind};
use common::errors::{LangError, BuildErrorKind};
use common::module::{ModuleUID, Module, FunctionDefinition, ModuleFeature, VariableDefinition};
use core::parser::ModuleLoader;
use std::sync::Arc;
use crate::build::{convert_class, convert_type, convert_types};
use common::constants::CLASS_CONSTRUCTOR_NAME;

// TODO: Right now memory alignment is at 0 so it's 1 byte, better alignment would be cool (probably 2)

pub struct FunctionData {
    pub name: String,

    pub params: Vec<ValType>,
    pub ret: Vec<ValType>,

    pub locals: Vec<ValType>,
    pub instructions: Vec<Instruction<'static>>,
}

pub struct FunctionImport {
    pub module_name: String,
    pub name: String,

    pub params: Vec<ValType>,
    pub ret: Vec<ValType>,
}

pub struct ModuleData {
    pub offset: u32,
    pub bytes: Vec<u8>,
}

pub struct ModuleBuilderResult {
    pub function_data: Vec<FunctionData>,
    pub function_imports: Vec<FunctionImport>,
    pub data: Vec<ModuleData>
}

pub struct ModuleBuilder<'a> {
    module_loader: &'a ModuleLoader,
    function_names: Vec<String>,
    functions: Vec<(Vec<TypeKind>, TypeKind)>,
    global_names: Vec<String>,
    globals: Vec<(TypeKind, u32)>,

    data_offset_accumulator: u32,
    data: Vec<ModuleData>,
    function_data: Vec<FunctionData>,
    function_imports: Vec<FunctionImport>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(module_loader: &'a ModuleLoader) -> Result<Self, LangError> {
        let mut builder = Self {
            module_loader,
            function_names: Vec::new(),
            functions: Vec::new(),
            global_names: Vec::new(),
            globals: Vec::new(),

            data_offset_accumulator: 0,
            data: Vec::new(),
            function_data: Vec::new(),
            function_imports: Vec::new(),
        };

        // TODO: This is not how it should be done
        // This loads all the declaration ad imports
        for module in module_loader.modules() {

            for (name, feature) in &module.features {
                match feature {
                    ModuleFeature::Function(func @ FunctionDefinition { data: None, .. }) => {
                        builder.function_names.push(name.clone());
                        builder.functions.push((func.metadata.0.clone(), (*func.metadata.1).clone()));

                        builder.insert_imported_func(module.id.0.as_ref(), name.as_ref(), &func.metadata)?;
                    },
                    ModuleFeature::Variable(VariableDefinition { data: None, .. }) => todo!(),
                    ModuleFeature::Class(_) => {
                        todo!();
                        // for (method_name, method_type) in &class.metadata.methods {
                        //     let name = format!("{}::{}", class_name, method_name);
                        //
                        //     builder.function_names.push(name.clone());
                        //     builder.functions.push((method_type.0.clone(), (*method_type.1).clone()));
                        //
                        //     builder.insert_imported_func(module.id.0.as_ref(), name.as_ref(), method_type)?;
                        // }
                    },
                    _ => (),
                }
            }
        }

        Ok(builder)
    }

    pub fn insert_module(&mut self, module: Arc<Module>) -> Result<(), LangError> {

        for (name, feature) in &module.features {
            match feature {
                ModuleFeature::Variable(var @ VariableDefinition { data: Some(ref data), .. }) => {
                    self.insert_var(name, &var.metadata, data)?;
                },
                ModuleFeature::Function(func @ FunctionDefinition { data: Some(ref data), .. }) => {
                    let contains_func = self.function_names
                        .iter()
                        .any(|n| n == name);

                    if contains_func {
                        continue
                    }

                    self.insert_func(name, &func.metadata, data)?;

                    self.function_names.push(name.clone());
                    self.functions.push((func.metadata.0.clone(), (*func.metadata.1).clone()));
                }
                ModuleFeature::Class(_) => {}
                _ => (),
            }
        }

        Ok(())
    }

    pub fn insert_imported_func(&mut self, module_name: &str, name: &str, func_type: &FunctionType) -> Result<(), LangError> {
        self.function_imports.push(FunctionImport {
            module_name: module_name.to_string(),
            name: name.to_string(),

            params: convert_types(&func_type.0),
            ret: convert_type(func_type.1.as_ref()),
        });

        Ok(())
    }

    fn insert_var(&mut self, name: &str, var_type: &TypeKind, literal: &LiteralKind) -> Result<(), LangError> {
        let data = match literal {
            LiteralKind::Nothing => Vec::new(),
            LiteralKind::Int(i) => i.to_le_bytes().to_vec(),
            LiteralKind::Float(f) => f.to_le_bytes().to_vec(),
            LiteralKind::Bool(b) => (if *b { 1u32 } else { 0u32 }).to_le_bytes().to_vec(),
            LiteralKind::String(s) => {
                let data = FunctionBuilder::string_to_bytes(s.clone());
                let offset = self.push_data(data);

                offset.to_le_bytes().to_vec()
            },
        };

        let offset = self.push_data(data);

        self.global_names.push(name.to_string());
        self.globals.push((var_type.clone(), offset));

        Ok(())
    }

    fn insert_func(&mut self, name: &str, func_type: &FunctionType, func: &Arc<Function>) -> Result<(), LangError> {
        let mut code_builder = FunctionBuilder::new(
            self,
            name.to_string(),
            func_type.0.clone(),
            func.parameters.clone(),
            *func_type.1.clone());

        for node in &func.body {
            code_builder.build_statement(&node)?;
        }

        let result = code_builder.build();

        self.function_data.push(result);

        Ok(())
    }

    pub fn build(self) -> ModuleBuilderResult {
        ModuleBuilderResult {
            function_data: self.function_data,
            function_imports: self.function_imports,
            data: self.data,
        }
    }

    fn push_data(&mut self, data: Vec<u8>) -> u32 {
        let data_len = data.len() as u32;

        let offset = self.data_offset_accumulator;

        self.data.push(ModuleData {
            bytes: data,
            offset: self.data_offset_accumulator,
        });

        self.data_offset_accumulator += data_len;

        offset
    }

    pub(crate) fn get_func(&mut self, module_uid: ModuleUID, name: &String) -> Result<(u32, &Vec<TypeKind>, &TypeKind), LangError> {
        let func_id = self.function_names
            .iter()
            .position(|n| n == name);

        match func_id {
            Some(func_id) => {
                let (params, ret) = self.functions.index(func_id);
                Ok((
                    func_id as u32,
                    params,
                    ret,
                ))
            },
            None => {
                let module = self.module_loader
                    .get_module(module_uid)
                    .ok_or(LangError::build(BuildErrorKind::ModuleNotFound(module_uid)))?;

                let func = match module.get_func_feature(name) {
                    Some(f) => f,
                    None => return Err(LangError::build(BuildErrorKind::FuncNotFound(name.clone()))),
                };

                self.load_func(func, name)
            },
        }
    }

    fn get_method(&mut self, module_uid: ModuleUID, class_name: &String, name: &String) -> Result<(u32, &Vec<TypeKind>, &TypeKind), LangError> {
        let func_name = format!("{}::{}", class_name, name);

        let func_id = self.function_names
            .iter()
            .position(|n| n == &func_name);

        match func_id {
            Some(func_id) => {
                let (params, ret) = self.functions.index(func_id);
                Ok((
                    func_id as u32,
                    params,
                    ret,
                ))
            },
            None => {
                let module = self.module_loader
                    .get_module(module_uid)
                    .ok_or(LangError::build(BuildErrorKind::ModuleNotFound(module_uid)))?;

                let class = match module.get_class_feature(class_name) {
                    Some(f) => f,
                    None => return Err(LangError::build(BuildErrorKind::ClassNotFound(class_name.clone()))),
                };

                let func = match class.get_method_def(name) {
                    Some(f) => f,
                    None => return Err(LangError::build(BuildErrorKind::FuncNotFound(func_name.clone()))),
                };

                self.load_func(&func, &func_name)
            },
        }
    }

    fn load_func(&mut self, func: &FunctionDefinition, name: &String) -> Result<(u32, &Vec<TypeKind>, &TypeKind), LangError> {
        let data = match &func.data {
            Some(data) => data,
            None => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
        };

        self.insert_func(name.as_ref(), &func.metadata, data)?;

        self.function_names.push(name.clone());
        self.functions.push((
            func.metadata.0.clone(),
            *func.metadata.1.clone(),
        ));

        let (params, ret) = self.functions
            .last()
            .unwrap();

        Ok((
            self.functions.len() as u32 - 1,
            params,
            ret
        ))
    }
}

enum VarKind {
    Local(Vec<u32>),
    Global(u32),
}

pub struct FunctionBuilder<'a, 'b> {
    pub(crate) module_builder: &'a mut ModuleBuilder<'b>,

    /// This is like locals but is kept untouched
    params: Vec<TypeKind>,
    ret: TypeKind,

    name: String,
    id_accumulator: u32,
    locals: Vec<TypeKind>,
    local_names: Vec<String>,
    local_ids: Vec<Vec<u32>>,

    type_stack: Vec<TypeKind>,
    pub(crate) instructions: Vec<Instruction<'static>>,
}

impl<'a, 'b> FunctionBuilder<'a, 'b> {
    pub fn new(
        module_builder: &'a mut ModuleBuilder<'b>,
        name: String,
        params: Vec<TypeKind>,
        param_names: Vec<String>,
        ret: TypeKind,
    ) -> Self {
        let mut id_accumulator = 0u32;
        let mut local_ids = Vec::with_capacity(params.len());
        for param in &params {
            let len = convert_type(param).len() as u32;
            let ids = (id_accumulator..id_accumulator + len).collect();
            local_ids.push(ids);

            id_accumulator += len;
        }

        Self {
            module_builder,

            params: params.clone(),
            ret,

            name,
            id_accumulator,
            locals: params,
            local_names: param_names,
            local_ids,

            type_stack: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn build(mut self) -> FunctionData {
        self.instructions.push(Instruction::End);

        FunctionData {
            name: self.name,

            params: convert_types(&self.params),
            ret: convert_type(&self.ret),

            locals: convert_types(&self.locals),
            instructions: self.instructions,
        }
    }

    pub fn push_local(&mut self, name: String, type_: TypeKind) -> &Vec<u32> {
        let len = convert_type(&type_).len() as u32;
        let ids = (self.id_accumulator..self.id_accumulator + len).collect();
        self.local_ids.push(ids);
        self.id_accumulator += len;

        self.locals.push(type_);
        self.local_names.push(name);

        &self.local_ids.last().unwrap()
    }

    pub fn build_statement(&mut self, node: &ASTNode) -> Result<(), LangError> {
        match node.kind.as_ref() {
            NodeKind::VariableDecl { name, value } => {
                // Build value
                self.build_statement(value)?;

                // Remove "value" from the stack and add it's type to the locals
                let type_ = self.type_stack.pop().unwrap();

                let ids = self.push_local(name.clone(), type_).clone();

                for id in ids {
                    self.instructions.push(Instruction::LocalSet(id));
                }
            },
            NodeKind::VariableRef { module: _, name } => {
                let (var_type, var_kind) = match self.get_var(name) {
                    Some((vr, vk)) => (vr, vk),
                    None => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
                };

                self.type_stack.push(var_type.clone());

                match var_kind {
                    VarKind::Local(ids) => {
                        for id in ids {
                            self.instructions.push(Instruction::LocalGet(id));
                        }
                    },
                    VarKind::Global(offset) => {
                        self.instructions.push(Instruction::I32Const(offset as i32));
                        self.build_mem_load(&var_type, MemArg {
                            memory_index: 0,
                            offset: 0,
                            align: 0,
                        });
                    },
                };
            },
            NodeKind::VariableAsgn { name, value } => {
                let (local_type, var_kind) = match self.get_var(name) {
                    Some((lt, vk)) => (lt, vk),
                    None => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
                };

                match var_kind {
                    VarKind::Local(ids) => {
                        self.build_statement(value)?;
                        for id in ids {
                            self.instructions.push(Instruction::LocalSet(id));
                        }
                    },
                    VarKind::Global(offset) => {
                        self.instructions.push(Instruction::I32Const(offset as i32));

                        self.build_statement(value)?;

                        self.build_mem_store(&local_type, MemArg {
                            align: 0,
                            memory_index: 0,
                            offset: 0,
                        });
                    },
                }

                let type_ = self.type_stack.pop().unwrap();

                Self::assert_type(&type_, &local_type)?;
            },
            NodeKind::FunctionInvok { variable, parameters } => {
                // TODO: Support for other kinds of invocations
                let (func_id, param_types, ret_type) = match variable.kind.as_ref() {
                    NodeKind::VariableRef { name, module } => {
                        for param in parameters {
                            self.build_statement(param)?;
                        }

                        self.module_builder.get_func(*module, name)?
                    },
                    NodeKind::FieldAccess { variable, class_type, field_name } => {
                        self.build_statement(variable)?;

                        for param in parameters {
                            self.build_statement(param)?;
                        }

                        self.module_builder.get_method(class_type.module, &class_type.name, field_name)?
                    },
                    _ => return Err(LangError::build(BuildErrorKind::Unsupported("Not static function call".to_string()))),
                };

                for param in param_types.iter().skip(1) {
                    let type_ = self.type_stack.pop().unwrap();

                    Self::assert_type(&type_, &param)?;
                }

                self.type_stack.push(ret_type.clone());

                self.instructions.push(Instruction::Call(func_id));
            },
            NodeKind::Literal { value } => {
                match value {
                    LiteralKind::Nothing => (),
                    LiteralKind::Int(i) => {
                        self.instructions.push(Instruction::I32Const(*i));
                        self.type_stack.push(TypeKind::Int);
                    },
                    LiteralKind::Float(f) => {
                        self.instructions.push(Instruction::F32Const(*f));
                        self.type_stack.push(TypeKind::Float);
                    },
                    LiteralKind::Bool(b) => {
                        let value = if *b { 1 } else { 0 };

                        self.instructions.push(Instruction::I32Const(value));
                        self.type_stack.push(TypeKind::Bool);
                    },
                    LiteralKind::String(string) => {
                        let data = Self::string_to_bytes(string.clone());

                        let offset = self.module_builder.push_data(data);

                        self.instructions.push(Instruction::I32Const(offset as i32));
                        self.type_stack.push(TypeKind::String);
                    },
                };
            },
            NodeKind::MathOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                let right = self.type_stack.pop().unwrap();
                let left = self.type_stack.pop().unwrap();

                let left_convert = convert_type(&left);
                let right_convert = convert_type(&right);

                self.type_stack.push(left);

                match (left_convert.as_slice(), right_convert.as_slice()) {
                    ([left], [right]) => {
                        self.build_math_op(operation, left.clone(), right.clone());
                    }
                    _ => return Err(LangError::build(BuildErrorKind::InvalidStackType)),
                }
            },
            NodeKind::BoolOperation { operation, left, right } => {
                self.build_statement(left)?;
                self.build_statement(right)?;

                let right = self.type_stack.pop().unwrap();
                let left = self.type_stack.pop().unwrap();

                let left_convert = convert_type(&left);
                let right_convert = convert_type(&right);

                self.type_stack.push(left);

                match (left_convert.as_slice(), right_convert.as_slice()) {
                    ([left], [right]) => {
                        self.build_bool_op(operation, left.clone(), right.clone());
                    }
                    _ => return Err(LangError::build(BuildErrorKind::InvalidStackType)),
                }
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
                Self::assert_type(&type_, &TypeKind::Int)?;

                // Add "iter_name" to the locals
                let ids = self.push_local(iter_name.clone(), TypeKind::Int);

                // Ids should always be 1 long
                let id = ids.first().unwrap().clone();

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
            NodeKind::FieldAccess { variable, class_type, field_name } => {
                match &class_type.kind {
                    ClassKind::Normal => {
                        self.build_statement(variable)?;

                        let class_type = match self.type_stack.pop().unwrap() {
                            TypeKind::Object(obj) => obj,
                            _ => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
                        };

                        let mut field_type = TypeKind::Unknown;

                        let mut offset = 0;
                        for (name, type_) in &class_type.fields {
                            if field_name == name {
                                field_type = type_.clone();
                                break
                            }
                            offset += Self::get_type_byte_size(type_) as u64;
                        }

                        if matches!(field_type, TypeKind::Unknown) {
                            return Err(LangError::build(BuildErrorKind::UnexpectedError));
                        }

                        self.build_mem_load(&field_type, MemArg {
                            offset,
                            align: 0,
                            memory_index: 0,
                        });

                        self.type_stack.push(field_type);
                    }
                    ClassKind::Data => {
                        match variable.kind.as_ref() {
                            NodeKind::VariableRef { module: _, name } => {
                                let (var_type, var_kind) = match self.get_var(name) {
                                    Some(var) => var,
                                    None => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
                                };

                                match var_kind {
                                    VarKind::Local(ids) => {
                                        let mut field_type = TypeKind::Nothing;
                                        let mut idx_offset = 0u32;

                                        for (name, type_) in &class_type.fields {
                                            if name == field_name {
                                                field_type = type_.clone();
                                                break
                                            }

                                            idx_offset += convert_type(type_).len() as u32;
                                        }

                                        let start_id = ids.index(0) + idx_offset;
                                        let idx_count = convert_type(&field_type).len() as u32;

                                        for i in start_id..(start_id + idx_count) {
                                            self.instructions.push(Instruction::LocalGet(i));
                                        }

                                        self.type_stack.push(field_type);
                                    },
                                    VarKind::Global(location) => {
                                        self.instructions.push(Instruction::I32Const(location as i32));

                                        let mut field_type = TypeKind::Nothing;
                                        let mut offset = 0;

                                        for (name, type_) in &class_type.fields {
                                            if name == field_name {
                                                field_type = type_.clone();
                                                break
                                            }

                                            offset += Self::get_type_byte_size(type_) as u64;
                                        }

                                        self.build_mem_load(&var_type, MemArg {
                                            offset,
                                            align: 0,
                                            memory_index: 0
                                        });

                                        self.type_stack.push(field_type);
                                    },
                                }
                            },
                            _ => todo!(),
                        }
                    }
                }
            },
            NodeKind::FieldAsgn { variable, class_type, field_name, value } => {
                // TODO: Future me please fix this shit, this is just a copy of the thing above

                match &class_type.kind {
                    ClassKind::Normal => {
                        self.build_statement(variable)?;

                        let class_type = match self.type_stack.pop().unwrap() {
                            TypeKind::Object(obj) => obj,
                            _ => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
                        };

                        let mut field_type = TypeKind::Unknown;

                        let mut offset = 0;
                        for (name, type_) in &class_type.fields {
                            if field_name == name {
                                field_type = type_.clone();
                                break
                            }
                            offset += Self::get_type_byte_size(type_) as u64;
                        }

                        self.build_statement(value)?;

                        if field_type != self.type_stack.pop().unwrap() {
                            return Err(LangError::build(BuildErrorKind::InvalidStackType));
                        }

                        self.build_mem_store(&field_type, MemArg {
                            offset,
                            align: 0,
                            memory_index: 0,
                        });
                    },
                    ClassKind::Data => {
                        match variable.kind.as_ref() {
                            NodeKind::VariableRef { module: _, name } => {
                                let (var_type, var_kind) = match self.get_var(name) {
                                    Some(var) => var,
                                    None => return Err(LangError::build(BuildErrorKind::UnexpectedError)),
                                };

                                match var_kind {
                                    VarKind::Local(ids) => {
                                        let mut field_type = TypeKind::Nothing;
                                        let mut idx_offset = 0u32;

                                        for (name, type_) in &class_type.fields {
                                            if name == field_name {
                                                field_type = type_.clone();
                                                break
                                            }

                                            idx_offset += convert_type(type_).len() as u32;
                                        }

                                        let start_id = ids.index(0) + idx_offset;
                                        let idx_count = convert_type(&field_type).len() as u32;

                                        self.build_statement(value)?;

                                        for i in start_id..(start_id + idx_count) {
                                            self.instructions.push(Instruction::LocalSet(i));
                                        }

                                        self.type_stack.push(field_type);
                                    },
                                    VarKind::Global(location) => {
                                        self.instructions.push(Instruction::I32Const(location as i32));

                                        let mut field_type = TypeKind::Nothing;
                                        let mut offset = 0;

                                        for (name, type_) in &class_type.fields {
                                            if name == field_name {
                                                field_type = type_.clone();
                                                break
                                            }

                                            offset += Self::get_type_byte_size(type_) as u64;
                                        }

                                        self.build_statement(value)?;

                                        self.build_mem_store(&var_type, MemArg {
                                            offset,
                                            align: 0,
                                            memory_index: 0
                                        });

                                        self.type_stack.push(field_type);
                                    },
                                }
                            },
                            _ => todo!(),
                        }
                    },
                }
            },
            NodeKind::VectorLiteral { values } => {
                // TODO: Make the vector take other types (for now only ints)

                self.build_memory_alloc(values.len() as i32 * 4)?;

                // TODO: Support multiple allocations in the same method
                let ids = self.push_local("__internal_alloc_location".to_string(), TypeKind::Int);
                let id = *ids.index(0);

                self.instructions.push(Instruction::LocalTee(id));

                for (offset, val) in values.iter().enumerate() {
                    self.instructions.push(Instruction::LocalGet(id));

                    self.build_statement(val)?;

                    self.instructions.push(Instruction::I32Store(MemArg {
                        offset: offset as u64 * 4,
                        align: 0,
                        memory_index: 0
                    }));
                }

                self.type_stack.push(TypeKind::Vector(Box::new(TypeKind::Int)));
            },
            NodeKind::ObjectLiteral { .. } => todo!(),
            NodeKind::FunctionLiteral { .. } => todo!(),
            NodeKind::ValueFieldAccess { variable, value } => {
                // TODO: Make other types work (for now only ints)

                // TODO: Support other types of values
                let index = match value.kind.as_ref() {
                    NodeKind::Literal { value: LiteralKind::Int(i) } => *i as u64,
                    _ => todo!(),
                };

                self.build_statement(variable)?;

                let var_type = self.type_stack.pop().unwrap();

                if let TypeKind::Vector(type_) = var_type {
                    if *type_ != TypeKind::Int { todo!() }

                    self.instructions.push(Instruction::I32Load(MemArg {
                        align: 0,
                        offset: index * 4,
                        memory_index: 0,
                    }));
                }
            },
            NodeKind::ConstructClass { parameters, class_type } => {
                match &class_type.kind {
                    ClassKind::Normal => {
                        let size = class_type.fields
                            .iter()
                            .map(|(_, type_)| Self::get_type_byte_size(type_) as i32)
                            .sum();

                        self.build_memory_alloc(size)?;

                        if let Some((_, _)) = class_type.methods.iter().find(|(n, _)| n == CLASS_CONSTRUCTOR_NAME) {
                            // TODO: Support multiple allocations in the same method
                            let ids = self.push_local("__internal_alloc_location".to_string(), TypeKind::Int);
                            let id = *ids.index(0);

                            self.instructions.push(Instruction::LocalTee(id));

                            for param in parameters {
                                self.build_statement(param)?;
                            }

                            let (constructor_id, _, _) = self.module_builder.get_method(
                                class_type.module,
                                &class_type.name,
                                &CLASS_CONSTRUCTOR_NAME.to_string())?;

                            self.instructions.push(Instruction::Call(constructor_id));

                            self.instructions.push(Instruction::LocalGet(id));
                        }

                        self.type_stack.push(TypeKind::Object(class_type.clone()));
                    },
                    ClassKind::Data => {
                        for type_ in convert_class(class_type) {
                            self.build_default_value(type_);
                        }

                        self.type_stack.push(TypeKind::Object(class_type.clone()));
                    },
                }
            },
        }

        Ok(())
    }

    fn string_to_bytes(string: String) -> Vec<u8> {
        let string_len = string.len() as u32;

        let mut data = string_len.to_le_bytes().to_vec();
        data.append(&mut string.clone().into_bytes());

        data
    }

    fn get_var(&self, name: &String) -> Option<(TypeKind, VarKind)> {
        let id = self.local_names
            .iter()
            .position(|n| n == name);

        if let Some(id) = id {
            return Some((
                self.locals.index(id).clone(),
                VarKind::Local(self.local_ids.index(id).clone()),
            ))
        };

        let id = self.module_builder.global_names
            .iter()
            .position(|n| n == name);

        match id {
            Some(id) => {
                let (type_, offset) = self.module_builder.globals.index(id);

                Some((
                    type_.clone(),
                    VarKind::Global(*offset),
                ))
            },
            None => None,
        }
    }

    fn get_type_byte_size(type_: &TypeKind) -> usize {
        match type_ {
            TypeKind::Int => 4,
            TypeKind::Float => 4,
            TypeKind::String => 4,
            TypeKind::Bool => 4,
            TypeKind::Vector(_) => 4,
            TypeKind::Object(_) => 4,
            TypeKind::Nothing => 0,
            TypeKind::Unknown => 0,
            TypeKind::Function(_) => 4,
        }
    }

    #[inline]
    fn assert_type(a: &TypeKind, b: &TypeKind) -> Result<(), LangError> {
        if a == b {
            Ok(())
        } else {
            Err(LangError::build(BuildErrorKind::InvalidStackType))
        }
    }

    #[inline]
    fn assert_stack_size(&self, size: usize) -> Result<(), LangError> {
        if self.type_stack.len() > size {
            Ok(())
        } else {
            Err(LangError::build(BuildErrorKind::InvalidStackSize(size, self.type_stack.len())))
        }
    }
}