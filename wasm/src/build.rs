use std::sync::Arc;
use wasm_encoder::{CodeSection, DataSection, Export, ExportSection, Function, FunctionSection, Instruction, MemorySection, MemoryType, Module, TypeSection, ValType};
use common::ast::types::TypeKind;
use common::errors::LangError;
use core::parser::ModuleLoader;
use crate::build_code::{FunctionBuilderResult, ModuleBuilder, ModuleBuilderResult, ModuleData};

pub struct WasmBuilder<'a> {
    module_loader: &'a ModuleLoader,
    module: Arc<common::module::Module>,
}

impl<'a> WasmBuilder<'a> {
    pub fn new(module_loader: &'a ModuleLoader, main_module: Arc<common::module::Module>) -> Self {
        Self {
            module_loader,
            module: main_module,
        }
    }

    pub fn build(self) -> Result<Vec<u8>, LangError> {
        let mut module_builder = ModuleBuilder::new(&self.module_loader);
        module_builder.insert_module(self.module.clone())?;

        let result = module_builder.build();

        let mut module = Module::new();

        module
            .section(&Self::build_types(&result)?)
            .section(&Self::build_functions(&result)?)
            .section(&Self::build_memory(64))
            .section(&Self::build_exports(&result)?)
            .section(&self.build_code(result.functions)?)
            .section(&Self::build_data(result.data));

        Ok(module.finish())
    }

    fn build_types(result: &ModuleBuilderResult) -> Result<TypeSection, LangError> {
        let mut types = TypeSection::new();

        for func in &result.functions {
            types.function(
                func.params.clone(),
                func.ret.clone(),
            );
        }

        Ok(types)
    }

    fn build_functions(result: &ModuleBuilderResult) -> Result<FunctionSection, LangError> {
        let mut functions = FunctionSection::new();

        for i in 0..result.functions.len() {
            functions.function(i as u32);
        }

        Ok(functions)
    }

    fn build_memory(size: u64) -> MemorySection {
        let mut memory = MemorySection::new();
        memory.memory(MemoryType {
            minimum: size,
            maximum: None,
            memory64: false,
        });
        memory
    }

    fn build_exports(result: &ModuleBuilderResult) -> Result<ExportSection, LangError> {
        let mut exports = ExportSection::new();

        for (i, func) in result.functions.iter().enumerate() {
            exports.export(func.name.as_ref(), Export::Function(i as u32));
        }

        exports.export("mem", Export::Memory(0));

        Ok(exports)
    }

    fn build_code(&self, functions: Vec<FunctionBuilderResult>) -> Result<CodeSection, LangError> {
        let mut codes = CodeSection::new();

        for func in functions {
            let locals: Vec<(u32, ValType)> = func.locals
                .into_iter()
                .enumerate()
                .map(|(i, local)| (i as u32, local))
                .collect();

            let mut func_builder = Function::new(locals);

            for inst in &func.instructions {
                func_builder.instruction(inst);
            }

            codes.function(&func_builder);
        }

        Ok(codes)
    }

    fn build_data(result: Vec<ModuleData>) -> DataSection {
        let mut data_sec = DataSection::new();

        for data in result {
            data_sec.active(0, &Instruction::I32Const(data.offset as i32), data.bytes);
        }

        data_sec
    }
}

pub(crate) fn convert_types(types: &Vec<TypeKind>) -> Vec<ValType> {
    let mut result = Vec::with_capacity(types.len() + 1);
    for type_ in types {
        result.append(&mut convert_type(type_));
    }

    result
}

pub(crate) fn convert_type(type_: &TypeKind) -> Vec<ValType> {
    match type_ {
        TypeKind::Int => vec![ValType::I32],
        TypeKind::Float => vec![ValType::F32],
        TypeKind::String => vec![ValType::I32],
        TypeKind::Bool => vec![ValType::I32],
        TypeKind::Unknown |
        TypeKind::Nothing => vec![],
        TypeKind::Vector(_) => todo!(),
        TypeKind::Function(_) => todo!(),
        TypeKind::Object(_) => todo!(),
    }
}