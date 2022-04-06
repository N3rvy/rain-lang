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
        module.section(&self.build_code(&result)?);

        Ok(module.finish())
    }

    fn build_types(result: &ModuleBuilderResult) -> Result<TypeSection, LangError> {
        let mut types = TypeSection::new();

        for func in &result.functions {
            types.function(
                func.params.clone(),
                [func.ret],
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

    fn build_code(&self, result: &ModuleBuilderResult) -> Result<CodeSection, LangError> {
        let mut codes = CodeSection::new();

        for func in &result.functions {
            let locals = func.locals
                .iter()
                .enumerate()
                .map(|(i, (_, type_))| (i as u32, *type_))
                .collect::<Vec<(u32, ValType)>>();

            let mut func_builder = Function::new(locals);

            for inst in &func.instructions {
                func_builder.instruction(inst);
            }

            codes.function(&func_builder);
        }

        Ok(codes)
    }
}

pub(crate) fn convert_type(type_: &TypeKind) -> ValType {
    match type_ {
        TypeKind::Unknown => todo!(),
        TypeKind::Int => ValType::I32,
        TypeKind::Float => ValType::F32,
        TypeKind::String => ValType::I32,
        TypeKind::Bool => ValType::I32,
        TypeKind::Nothing => todo!(),
        TypeKind::Vector(_) => todo!(),
        TypeKind::Function(_) => todo!(),
        TypeKind::Object(_) => todo!(),
    }
}