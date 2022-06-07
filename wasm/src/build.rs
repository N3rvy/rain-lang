use std::sync::Arc;
use wasm_encoder::{CodeSection, DataSection, EntityType, Export, ExportSection, Function, FunctionSection, ImportSection, Instruction, MemorySection, MemoryType, Module, TypeSection, ValType};
use common::ast::types::{ClassKind, ClassType, TypeKind};
use common::errors::LangError;
use core::parser::ModuleLoader;
use crate::build_code::{FunctionData, ModuleBuilder, ModuleBuilderResult, ModuleData, ModuleDataKind};

pub struct WasmBuilder<'a> {
    module_loader: &'a ModuleLoader,
    module: Arc<common::module::Module>,
    core_module: Arc<common::module::Module>,
}

impl<'a> WasmBuilder<'a> {
    pub fn new(
        module_loader: &'a ModuleLoader,
        main_module: Arc<common::module::Module>,
        core_module: Arc<common::module::Module>) -> Self
    {
        Self {
            module_loader,
            core_module,
            module: main_module,
        }
    }

    pub fn build(self) -> Result<Vec<u8>, LangError> {
        let mut module_builder = ModuleBuilder::new(&self.module_loader)?;
        module_builder.insert_module(self.core_module.clone())?;
        module_builder.insert_module(self.module.clone())?;

        let result = module_builder.build();

        let mut module = Module::new();

        module
            .section(&Self::build_types(&result)?)
            .section(&Self::build_imports(&result)?)
            .section(&Self::build_functions(result.function_imports.len() as u32, &result)?)
            .section(&Self::build_memory(64))
            .section(&Self::build_exports(&result)?)
            .section(&self.build_code(result.function_data)?)
            .section(&Self::build_data(result.data));

        Ok(module.finish())
    }

    fn build_types(result: &ModuleBuilderResult) -> Result<TypeSection, LangError> {
        let mut types = TypeSection::new();

        for func in &result.function_imports {
            types.function(
                func.params.clone(),
                func.ret.clone(),
            );
        }

        for func in &result.function_data {
            types.function(
                func.params.clone(),
                func.ret.clone(),
            );
        }

        Ok(types)
    }

    fn build_imports(result: &ModuleBuilderResult) -> Result<ImportSection, LangError> {
        let mut imports = ImportSection::new();

        for (i, func) in result.function_imports.iter().enumerate() {
            imports.import(
                func.module_name.as_ref(),
                Some(func.name.as_ref()),
                EntityType::Function(i as u32),
            );
        }

        Ok(imports)
    }

    fn build_functions(offset: u32, result: &ModuleBuilderResult) -> Result<FunctionSection, LangError> {
        let mut functions = FunctionSection::new();

        for (i, _) in result.function_data.iter().enumerate() {
            functions.function(offset + i as u32);
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

        let offset = result.function_imports.len() as u32;

        for (i, func) in result.function_data.iter().enumerate() {
            exports.export(func.name.as_ref(), Export::Function(offset + i as u32));
        }

        exports.export("mem", Export::Memory(0));

        Ok(exports)
    }

    fn build_code(&self, functions: Vec<FunctionData>) -> Result<CodeSection, LangError> {
        let mut codes = CodeSection::new();

        for func in functions {
            let locals: Vec<(u32, ValType)> = func.locals
                .into_iter()
                .skip(func.params.len())
                .map(|local| (1u32, local))
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

        let mut total_size = 0;
        for data in &result {
            total_size += data.bytes.len();
        }

        for data in result {
            let offset = data.offset as i32;
            let data = match data.kind {
                ModuleDataKind::Standard => data.bytes,
                ModuleDataKind::StaticMemoryOffset => i32::to_le_bytes(total_size as i32).to_vec(),
            };

            data_sec.active(0, &Instruction::I32Const(offset), data);
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
        TypeKind::Vector(_) => vec![ValType::I32],
        TypeKind::Function(_) => todo!(),
        TypeKind::Class(obj) => convert_class(obj),
    }
}

pub(crate) fn convert_class(class_type: &Arc<ClassType>) -> Vec<ValType> {
    if let ClassKind::Data = class_type.kind {
        convert_types(&class_type.fields
            .borrow()
            .iter()
            .map(|(_, type_)| type_.clone())
            .collect())
    } else {
        vec![ValType::I32]
    }
}