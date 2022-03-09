#![feature(unboxed_closures)]
#![feature(try_trait_v2)]

use std::collections::HashMap;
use core::module::EngineModule;
use core::{ExternalType, Engine, EngineSetFunction, EngineGetFunction, InternalFunction};
use std::marker::PhantomData;
use std::sync::Arc;
use common::ast::module::ASTModule;
use common::ast::types::{Function, TypeKind, FunctionType};
use common::errors::LangError;
use common::module::ModuleUID;
use errors::CANT_CONVERT_VALUE;
use evaluate::{EvalResult, EvaluateAST};
use external_functions::IntoExternalFunctionRunner;
use lang_value::LangValue;
use scope::Scope;
use core::engine_module_loader::EngineModuleLoader;

mod scope;
mod evaluate;
mod lang_value;
mod external_functions;
mod object;
mod errors;

pub struct InterpreterEngine {
    global_module: InterpreterModule,
    global_types: Vec<(String, TypeKind)>,
    module_loader: EngineModuleLoader<Self>,
}

impl<'a> Default for InterpreterEngine {
    fn default() -> Self {
        Self {
            global_module: InterpreterModule::new(Scope::new()),
            global_types: Vec::new(),
            module_loader: EngineModuleLoader::new(),
        }
    }
}

pub struct InterpreterModule {
    scope: Arc<Scope>,
    modules: HashMap<ModuleUID, InterpreterModule>,
}

impl EngineModule for InterpreterModule {
    type Engine = InterpreterEngine;
    
    fn new(builder: &EngineModuleLoader<Self::Engine>, module: ASTModule) -> Result<Self, LangError> {
        let scope = Scope::new();

        for (func_name, func) in module.functions {
            scope.declare_var(func_name.clone(), LangValue::Function(func));
        }

        for (var_name, var) in module.variables {
            let value = match scope.evaluate_ast(&var) {
                EvalResult::Ok(value) => value,
                EvalResult::Ret(value, _) => value,
                EvalResult::Err(err) => return Err(err),
            };
            scope.declare_var(var_name.clone(), value);
        }

        for import in module.imports {
            let module = match builder.get_module(import) {
                Some(module) => module,
                None => continue,
            };

            let vars = module.scope.variables_unsecure();
            for (name, value) in vars.borrow().iter() {
                scope.declare_var(name.clone(), value.clone());
            }
        }

        Ok(InterpreterModule::new(scope))
    }
}

impl InterpreterModule {
    fn new(scope: Arc<Scope>) -> Self {
        Self {
            scope,
            modules: HashMap::new(),
        }
    }
}


impl Engine for InterpreterEngine {
    type Module = InterpreterModule;

    fn global_types(&self) -> &Vec<(String, TypeKind)> {
        &self.global_types
    }

    fn module_loader(&mut self) -> &mut EngineModuleLoader<Self> {
        &mut self.module_loader
    }

    fn new() -> Self {
        Self::default()
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
        let scope = Scope::new_child(self.module.scope.clone());
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
        let module = match self.module_loader.get_module(uid) {
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
        self.global_module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
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
        self.global_module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
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
        self.global_module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
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
        self.global_module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
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
        self.global_module.scope.declare_var(name.to_string(), LangValue::ExtFunction(ext_func));
    }
}
