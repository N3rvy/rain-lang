use std::sync::Arc;
use wasm_encoder::{CodeSection, Export, ExportSection, Function, FunctionSection, Module, TypeSection, ValType};
use common::ast::types::TypeKind;
use common::errors::LangError;
use core::parser::ModuleLoader;
use crate::build_code::{ModuleBuilder, ModuleBuilderResult};

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

        module.section(&Self::build_types(&result)?);
        module.section(&Self::build_functions(&result)?);
        module.section(&Self::build_exports(&result)?);
        module.section(&self.build_code(result)?);

        Ok(module.finish())
    }

    fn build_types(result: &ModuleBuilderResult) -> Result<TypeSection, LangError> {
        let mut types = TypeSection::new();

        for func in &result.functions {
            types.function(
                func.locals.clone(),
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

    fn build_exports(result: &ModuleBuilderResult) -> Result<ExportSection, LangError> {
        let mut exports = ExportSection::new();

        for (i, func) in result.functions.iter().enumerate() {
            exports.export(func.name.as_ref(), Export::Function(i as u32));
        }

        Ok(exports)
    }

    fn build_code(&self, result: ModuleBuilderResult) -> Result<CodeSection, LangError> {
        let mut codes = CodeSection::new();

        for func in result.functions {
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