use std::sync::Arc;
use wasm_encoder::{CodeSection, Export, ExportSection, Function, FunctionSection, Module, TypeSection, ValType};
use common::ast::types::TypeKind;
use common::errors::LangError;
use crate::build_code::CodeBuilder;
use crate::errors::UNEXPECTED_ERROR;

pub struct WasmBuilder {
    module: Arc<common::module::Module>,
}

impl WasmBuilder {
    pub fn from_module(module: Arc<common::module::Module>) -> Self {
        Self {
            module,
        }
    }

    pub fn build(self) -> Result<Vec<u8>, LangError> {
        let mut module = Module::new();

        module.section(&self.build_types()?);
        module.section(&self.build_functions()?);
        module.section(&self.build_exports()?);
        module.section(&self.build_code()?);

        Ok(module.finish())
    }

    fn build_types(&self) -> Result<TypeSection, LangError> {
        let mut types = TypeSection::new();

        for (func_name, _) in &self.module.functions {
            let func_type = self.module.metadata.definitions
                .iter()
                .find(|def| def.0 == *func_name)
                .and_then(|def| match &def.1 {
                    TypeKind::Function(func) => Some(func),
                    _ => None,
                });

            let func_type = match func_type {
                Some(t) => t,
                None => return Err(LangError::new_runtime(UNEXPECTED_ERROR.to_string())),
            };

            let mut params = Vec::with_capacity(func_type.0.len());
            for param_type in &func_type.0 {
                params.push(convert_type(param_type));
            }

            let result = [convert_type(&*func_type.1)];

            types.function(params, result);
        }

        Ok(types)
    }

    fn build_functions(&self) -> Result<FunctionSection, LangError> {
        let mut functions = FunctionSection::new();

        for i in 0..self.module.functions.len() {
            functions.function(i as u32);
        }

        Ok(functions)
    }

    fn build_exports(&self) -> Result<ExportSection, LangError> {
        let mut exports = ExportSection::new();

        for (i, (func_name, _)) in self.module.functions.iter().enumerate() {
            exports.export(func_name, Export::Function(i as u32));
        }

        Ok(exports)
    }

    fn build_code(&self) -> Result<CodeSection, LangError> {
        let mut codes = CodeSection::new();

        for (_, func) in &self.module.functions {
            let locals = Vec::new();
            let mut func_builder = Function::new(locals);

            let mut code_builder = CodeBuilder::new(&mut func_builder);

            for node in &func.body {
                code_builder.build_statement(&node)?;
            }

            code_builder.end_build();

            codes.function(&func_builder);
        }

        Ok(codes)
    }
}

fn convert_type(type_: &TypeKind) -> ValType {
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