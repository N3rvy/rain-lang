use core::{external_module::{ExternalModule, ExternalModuleSetFunctionMetadata}, ExternalType};

use common::{ast::types::{TypeKind, FunctionType}, module::ModuleUID};

use crate::engine::WasmEngine;

pub struct WasmExternalModule {
    pub uid: ModuleUID,
    pub definitions: Vec<(String, TypeKind)>,
}

impl ExternalModule for WasmExternalModule {
    type Engine = WasmEngine;

    fn new(_engine: &mut Self::Engine, id: &common::module::ModuleIdentifier, importer: &impl core::parser::ModuleImporter)
        -> Option<Self>
    where Self: Sized
    {
        let uid = importer.get_unique_identifier(id)?;

        Some(Self {
            uid,
            definitions: Vec::new(),
        })
    }
}

impl<R> ExternalModuleSetFunctionMetadata<(), R> for WasmExternalModule
where
    R: ExternalType,
{
    fn set_function(&mut self, name: &str) {
        let func_type = TypeKind::Function(
            FunctionType(
                vec![],
                Box::new(R::type_kind())
            )
        );

        self.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, R> ExternalModuleSetFunctionMetadata<(A0,), R> for WasmExternalModule
where
    A0: ExternalType,
    R: ExternalType,
{
    fn set_function(&mut self, name: &str) {
        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    ("".to_string(), A0::type_kind()),
                ],
                Box::new(R::type_kind())
            )
        );

        self.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, A1, R> ExternalModuleSetFunctionMetadata<(A0, A1), R> for WasmExternalModule
where
    A0: ExternalType,
    A1: ExternalType,
    R: ExternalType,
{
    fn set_function(&mut self, name: &str) {
        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    ("".to_string(), A0::type_kind()),
                    ("".to_string(), A1::type_kind()),
                ],
                Box::new(R::type_kind())
            )
        );

        self.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, A1, A2, R> ExternalModuleSetFunctionMetadata<(A0, A1, A2), R> for WasmExternalModule
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    R: ExternalType,
{
    fn set_function(&mut self, name: &str) {
        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    ("".to_string(), A0::type_kind()),
                    ("".to_string(), A1::type_kind()),
                    ("".to_string(), A2::type_kind()),
                ],
                Box::new(R::type_kind())
            )
        );

        self.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, A1, A2, A3, R> ExternalModuleSetFunctionMetadata<(A0, A1, A2, A3), R> for WasmExternalModule
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    A3: ExternalType,
    R: ExternalType,
{
    fn set_function(&mut self, name: &str) {
        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    ("".to_string(), A0::type_kind()),
                    ("".to_string(), A1::type_kind()),
                    ("".to_string(), A2::type_kind()),
                    ("".to_string(), A3::type_kind()),
                ],
                Box::new(R::type_kind())
            )
        );

        self.definitions
            .push((name.to_string(), func_type));
    }
}