use wasm_encoder::{Instruction, MemArg, ValType};
use common::ast::types::TypeKind;
use common::constants::{CORE_MODULE_ID, INTERNAL_MEMORY_ALLOC};
use common::errors::LangError;
use common::module::ModuleUID;
use crate::build::convert_type;
use crate::build_code::FunctionBuilder;

impl<'a, 'b> FunctionBuilder<'a, 'b> {

    pub(crate) fn build_mem_load(&mut self, type_: &TypeKind, mem_arg: MemArg) {
        for type_ in convert_type(type_) {
            self.instructions.push(Self::convert_load_type(type_, mem_arg));
        }
    }

    fn convert_load_type(type_: ValType, mem_arg: MemArg) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Load(mem_arg),
            ValType::I64 => Instruction::I64Load(mem_arg),
            ValType::F32 => Instruction::F32Load(mem_arg),
            ValType::F64 => Instruction::F64Load(mem_arg),
            ValType::V128 => Instruction::V128Load { memarg: mem_arg },
            ValType::FuncRef => Instruction::I32Load(mem_arg), // TODO: Is this right?
            ValType::ExternRef => Instruction::I32Load(mem_arg),
        }
    }

    pub(crate) fn build_mem_store(&mut self, type_: &TypeKind, mem_arg: MemArg) {
        for type_ in convert_type(type_) {
            self.instructions.push(Self::convert_store_type(type_, mem_arg));
        }
    }

    pub fn build_default_value(&mut self, type_: ValType) {
        let inst = match type_ {
            ValType::I32 => Instruction::I32Const(0),
            ValType::I64 => Instruction::I64Const(0),
            ValType::F32 => Instruction::F32Const(0f32),
            ValType::F64 => Instruction::F64Const(0f64),
            ValType::V128 => Instruction::V128Const(0i128),
            ValType::FuncRef => Instruction::RefFunc(0),
            ValType::ExternRef => todo!(),
        };

        self.instructions.push(inst);
    }

    fn convert_store_type(type_: ValType, mem_arg: MemArg) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Store(mem_arg),
            ValType::I64 => Instruction::I64Store(mem_arg),
            ValType::F32 => Instruction::F32Store(mem_arg),
            ValType::F64 => Instruction::F64Store(mem_arg),
            ValType::V128 => Instruction::V128Store { memarg: mem_arg },
            ValType::FuncRef => Instruction::I32Store(mem_arg), // TODO: Is this right?
            ValType::ExternRef => Instruction::I32Store(mem_arg),
        }
    }

    pub(crate) fn build_memory_alloc(&mut self, size: i32) -> Result<(), LangError> {
        let (alloc_func_id, _, _) = self.module_builder.get_func(
            ModuleUID::from_string(CORE_MODULE_ID.to_string()),
            &INTERNAL_MEMORY_ALLOC.to_string())?;

        self.instructions.push(Instruction::I32Const(size));
        self.instructions.push(Instruction::Call(alloc_func_id));

        Ok(())
    }
}