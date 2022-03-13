#![feature(unboxed_closures)]
#![feature(try_trait_v2)]

use std::collections::HashMap;
use core::module::EngineModule;
use core::parser::ModuleImporter;
use core::parser::ModuleLoader;
use core::{ExternalType, Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
use std::marker::PhantomData;
use std::sync::Arc;
use common::ast::types::{Function, TypeKind, FunctionType};
use common::errors::LangError;
use common::module::{Module, ModuleIdentifier, ModuleUID};
use errors::CANT_CONVERT_VALUE;
use evaluate::EvalResult;
use external_functions::IntoExternalFunctionRunner;
use lang_value::LangValue;
use scope::Scope;
use crate::module_scope::ModuleScope;

mod scope;
mod evaluate;
mod lang_value;
mod external_functions;
mod object;
mod errors;
mod module_scope;

pub struct InterpreterEngine {
    global_types: Vec<(String, TypeKind)>,
    module_loader: ModuleLoader,
    modules: HashMap<ModuleUID, InterpreterModule>,
}

pub struct InterpreterModule {
    scope: Arc<ModuleScope>,
}

impl EngineModule for InterpreterModule {
    type Engine = InterpreterEngine;
    
    fn new(engine: &mut Self::Engine, uid: ModuleUID, module: Arc<Module>) -> Result<Self, LangError> {
        let scope = ModuleScope::new(uid);

        for (func_name, func) in &module.functions {
            scope.force_set_var(func_name.clone(), LangValue::Function(func.clone()));
        }

        for (var_name, var) in &module.variables {
            let func_scope = Scope::new_module_child(scope.clone());

            let value = match func_scope.evaluate_ast(&var) {
                EvalResult::Ok(value) => value,
                EvalResult::Ret(value, _) => value,
                EvalResult::Err(err) => return Err(err),
            };
            scope.force_set_var(var_name.clone(), value);
        }

        for import in &module.imports {
            let module = match engine.modules.get(&import) {
                Some(m) => m,
                None => continue,
            };

            for (name, value) in module.scope.variables().iter() {
                scope.declare_var(name, value.clone());
            }
        }

        Ok(InterpreterModule {
            scope,
        })
    }
}


impl Engine for InterpreterEngine {
    type Module = InterpreterModule;

    fn load_module<Importer: ModuleImporter>(&mut self, identifier: impl Into<String>) -> Result<ModuleUID, LangError> {
        let (uid, module) = self
            .module_loader()
            .get_or_load_module::<Importer>(&ModuleIdentifier(identifier.into()))?;

        let module = InterpreterModule::new(self, uid, module)?;

        self.modules.insert(uid, module);

        Ok(uid)
    }

    fn global_types(&self) -> &Vec<(String, TypeKind)> {
        &self.global_types
    }

    fn module_loader(&mut self) -> &mut ModuleLoader {
        &mut self.module_loader
    }

    fn new() -> Self {
        Self {
            global_types: Vec::new(),
            module_loader: ModuleLoader::new(),
            modules: HashMap::new(),
        }
    }
}

impl InterpreterEngine {
    fn get_module(&self, uid: ModuleUID) -> Option<&InterpreterModule> {
        self.modules.get(&uid)
    }
}

pub struct InterpreterFunction<'a, Args, R: ExternalType> {
    module: &'a InterpreterModule,
    func: Arc<Function>,
    _marker: PhantomData<(Args, R)>,
}

impl<'a, R: ExternalType> InternalFunction<(), Result<R, LangError>>
    for InterpreterFunction<'_, (), R>
{
    fn call(&self, _args: ()) -> Result<R, LangError> {
        let scope = Scope::new_module_child(self.module.scope.clone());
        let result = scope.invoke_function(
            &LangValue::Function(self.func.clone()),
            vec![],
        );

        let value = match result {
            EvalResult::Ok(value) => value,
            EvalResult::Ret(value, _) => value,
            EvalResult::Err(err) => return Err(err),
        };

        match value.into() {
            None => Err(LangError::new_runtime(CANT_CONVERT_VALUE.to_string())),
            Some(value) => match R::concretize(value) {
                None => Err(LangError::new_runtime(CANT_CONVERT_VALUE.to_string())),
                Some(value) => Ok(value),
            },
        }
    }
}

impl<'a, R: ExternalType> EngineGetFunction
    <'a, (), Result<R, LangError>, InterpreterFunction<'a, (), R>>
    for InterpreterEngine
{
    fn get_function(&'a self, uid: ModuleUID, name: &str)
        -> Option<InterpreterFunction<'a, (), R>>
    {
        let module = match self.get_module(uid) {
            Some(m) => m,
            None => return None,
        };

        let value = module.scope.get_var(&name.to_string());
        let func = match value {
            None => return None,
            Some(value) => match value {
                LangValue::Function(func) => func,
                _ => return None
            },
        };
        
        Some(InterpreterFunction {
            module,
            func,
            _marker: PhantomData::default(),
        })
    }
}

impl<'a, R> EngineSetFunction<'a, (), R> for InterpreterEngine
where
    R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<(), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(), R>::external(func);

        self.global_types.push((
            name.to_string(),
            TypeKind::Function(
                FunctionType(
                    vec![],
                    Box::new(R::type_kind())
                )
            )
        ));
        // self.global_module.scope.force_set_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0> EngineSetFunction<'a, (A0,), R> for InterpreterEngine
where
    A0: ExternalType,
    R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<(A0,), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0,), R>::external(func);

        self.global_types.push((
            name.to_string(),
            TypeKind::Function(
                FunctionType(
                    vec![
                        A0::type_kind(),
                    ],
                    Box::new(R::type_kind())
                )
            )
        ));
        // self.global_module.scope.force_set_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0, A1> EngineSetFunction<'a, (A0, A1), R> for InterpreterEngine
where
    A0: ExternalType,
    A1: ExternalType,
    R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<(A0, A1), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1), R>::external(func);

        self.global_types.push((
            name.to_string(),
            TypeKind::Function(
                FunctionType(
                    vec![
                        A0::type_kind(),
                        A1::type_kind(),
                    ],
                    Box::new(R::type_kind())
                )
            )
        ));
        // self.global_module.scope.force_set_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0, A1, A2> EngineSetFunction<'a, (A0, A1, A2), R> for InterpreterEngine
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    R: ExternalType
{
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<(A0, A1, A2), Output = R> + Send + Sync + 'static {
        let ext_func = IntoExternalFunctionRunner::<(A0, A1, A2), R>::external(func);

        self.global_types.push((
            name.to_string(),
            TypeKind::Function(
                FunctionType(
                    vec![
                        A0::type_kind(),
                        A1::type_kind(),
                        A2::type_kind(),
                    ],
                    Box::new(R::type_kind())
                )
            )
        ));
        // self.global_module.scope.force_set_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}

impl<'a, R, A0, A1, A2, A3> EngineSetFunction<'a, (A0, A1, A2, A3), R> for InterpreterEngine
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

        self.global_types.push((
            name.to_string(),
            TypeKind::Function(
                FunctionType(
                    vec![
                        A0::type_kind(),
                        A1::type_kind(),
                        A2::type_kind(),
                        A3::type_kind(),
                    ],
                    Box::new(R::type_kind())
                )
            )
        ));
        // self.global_module.scope.force_set_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}
