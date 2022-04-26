use std::ops::Index;
use wasm_encoder::{Instruction, MemArg, ValType};
use common::ast::types::TypeKind;
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

    pub(crate) fn build_memory_alloc(&mut self, size: i32, tee: bool) -> Result<u32, LangError> {
        let (alloc_func_id, _, _) = self.module_builder.get_func(
            ModuleUID::from_string("core".to_string()),
            &"__internal_memory_alloc".to_string())?;

        self.instructions.push(Instruction::I32Const(size));
        self.instructions.push(Instruction::Call(alloc_func_id));

        // TODO: Support multiple allocations in the same method
        let ids = self.push_local("__internal_alloc_location".to_string(), TypeKind::Int);
        let id = *ids.index(0);

        if tee {
            self.instructions.push(Instruction::LocalTee(id));
        } else {
            self.instructions.push(Instruction::LocalSet(id));
        }

        Ok(id)
    }
}