use common::ast::types::{FunctionType, TypeKind};
use common::module::{Module, ModuleIdentifier, ModuleMetadata, ModuleUID};
use core::external_module::{ExternalModule, ExternalModuleSetFunction, ExternalModuleSetValue};
use crate::{ExternalType, InterpreterEngine, InterpreterModule, IntoExternalFunctionRunner, LangValue, ModuleImporter, ModuleScope};

pub struct InterpreterExternalModule {
    pub uid: ModuleUID,
    pub module: Module,
    pub engine_module: InterpreterModule,
}

impl ExternalModule for InterpreterExternalModule {
    type Engine = InterpreterEngine;

    fn new(engine: &mut Self::Engine, id: &ModuleIdentifier, importer: &impl ModuleImporter) -> Option<Self> {
        let uid = importer.get_unique_identifier(id)?;

        Some(Self {
            uid,
            module: Module {
                uid,
                imports: Vec::new(),
                metadata: ModuleMetadata {
                    definitions: Vec::new(),
                },
                functions: Vec::new(),
                variables: Vec::new(),
            },
            engine_module: InterpreterModule {
                scope: ModuleScope::new(uid, engine),
            },
        })
    }
}

impl<R: ExternalType> ExternalModuleSetValue<R> for InterpreterExternalModule {
    fn set_value(&mut self, name: &str, value: R) {
        self.engine_module.scope
            .set_var(name.to_string(), R::generilize(value).into());

        self.module.metadata.definitions
            .push((name.to_string(), R::type_kind()));
    }
}

impl<R> ExternalModuleSetFunction<(), R> for InterpreterExternalModule
    where
        R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
        where F: Fn<(), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(), R>::external(func);

        self.engine_module.scope
            .set_var(name.to_string(), LangValue::ExtFunction(ext_func));

        let func_type = TypeKind::Function(
            FunctionType(
                vec![],
                Box::new(R::type_kind())
            )
        );

        self.module.metadata.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, R> ExternalModuleSetFunction<(A0,), R> for InterpreterExternalModule
    where
        A0: ExternalType,
        R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
        where F: Fn<(A0,), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0,), R>::external(func);

        self.engine_module.scope
            .set_var(name.to_string(), LangValue::ExtFunction(ext_func));

        let func_type = TypeKind::Function(
            FunctionType(
                vec![A0::type_kind()],
                Box::new(R::type_kind())
            )
        );

        self.module.metadata.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, A1, R> ExternalModuleSetFunction<(A0, A1), R> for InterpreterExternalModule
    where
        A0: ExternalType,
        A1: ExternalType,
        R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
        where F: Fn<(A0, A1), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1), R>::external(func);

        self.engine_module.scope
            .set_var(name.to_string(), LangValue::ExtFunction(ext_func));

        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    A0::type_kind(),
                    A1::type_kind(),
                ],
                Box::new(R::type_kind())
            )
        );

        self.module.metadata.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, A1, A2, R> ExternalModuleSetFunction<(A0, A1, A2), R> for InterpreterExternalModule
    where
        A0: ExternalType,
        A1: ExternalType,
        A2: ExternalType,
        R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
        where F: Fn<(A0, A1, A2), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1, A2), R>::external(func);

        self.engine_module.scope
            .set_var(name.to_string(), LangValue::ExtFunction(ext_func));

        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    A0::type_kind(),
                    A1::type_kind(),
                    A2::type_kind(),
                ],
                Box::new(R::type_kind())
            )
        );

        self.module.metadata.definitions
            .push((name.to_string(), func_type));
    }
}

impl<A0, A1, A2, A3, R> ExternalModuleSetFunction<(A0, A1, A2, A3), R> for InterpreterExternalModule
    where
        A0: ExternalType,
        A1: ExternalType,
        A2: ExternalType,
        A3: ExternalType,
        R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
        where F: Fn<(A0, A1, A2, A3), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1, A2, A3), R>::external(func);

        self.engine_module.scope
            .set_var(name.to_string(), LangValue::ExtFunction(ext_func));

        let func_type = TypeKind::Function(
            FunctionType(
                vec![
                    A0::type_kind(),
                    A1::type_kind(),
                    A2::type_kind(),
                    A3::type_kind(),
                ],
                Box::new(R::type_kind())
            )
        );

        self.module.metadata.definitions
            .push((name.to_string(), func_type));
    }
}
