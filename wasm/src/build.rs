use std::sync::Arc;
use wasm_encoder::{CodeSection, Export, ExportSection, Function, FunctionSection, Module, TypeSection, ValType};
use common::ast::types::TypeKind;
use common::errors::LangError;
use crate::build_code::{FunctionBuilder, ModuleBuilder};
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

        let func_names = self.module.functions
            .iter()
            .map(|(name, _)| {
                let (_, type_kind) = self.module.metadata.definitions
                    .iter()
                    .find(|(n, _)| n == name)
                    .unwrap();

                let func_type = match type_kind {
                    TypeKind::Function(func_type) => func_type,
                    _ => panic!(),
                };

                (
                    name.clone(),
                    func_type.0
                        .iter()
                        .map(|type_| convert_type(type_))
                        .collect(),
                    convert_type(func_type.1.as_ref()),
                )
            })
            .collect();

        let mut module_builder = ModuleBuilder::new(func_names);

        for (name, func) in &self.module.functions {
            let (_, type_kind) = self.module.metadata.definitions
                .iter()
                .find(|(n, _)| n == name)
                .unwrap();

            let func_type = match type_kind {
                TypeKind::Function(func_type) => func_type,
                _ => panic!(),
            };

            let param_types = func_type.0
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

            let mut code_builder = FunctionBuilder::new(
                &mut module_builder,
                param_types);

            for node in &func.body {
                code_builder.build_statement(&node)?;
            }

            let (locals , instructions) = code_builder.end_build();

            let locals = locals
                .iter()
                .enumerate()
                .map(|(i, (_, type_))| (i as u32, type_.clone()))
                .collect::<Vec<(u32, ValType)>>();

            let mut func_builder = Function::new(locals);

            for inst in instructions {
                func_builder.instruction(inst);
            }

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