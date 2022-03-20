#![feature(unboxed_closures)]
#![feature(try_trait_v2)]

use std::cell::RefCell;
use core::module::EngineModule;
use core::parser::ModuleImporter;
use core::parser::ModuleLoader;
use core::module_store::ModuleStore;
use core::{ExternalType, Engine, EngineGetFunction, InternalFunction};
use std::marker::PhantomData;
use std::sync::Arc;
use common::errors::LangError;
use common::module::{Module, ModuleIdentifier, ModuleUID};
use errors::CANT_CONVERT_VALUE;
use evaluate::EvalResult;
use external_functions::IntoExternalFunctionRunner;
use lang_value::LangValue;
use scope::Scope;
use crate::errors::{INVALID_IDENTIFIER, MODULE_NOT_FOUND, VARIABLE_IS_NOT_A_FUNCTION, VARIABLE_NOT_DECLARED};
use crate::external_module::InterpreterExternalModule;
use crate::module_scope::ModuleScope;

mod scope;
mod evaluate;
mod lang_value;
mod external_functions;
mod object;
mod errors;
mod module_scope;
pub mod external_module;

pub struct InterpreterEngine {
    module_loader: ModuleLoader,
    pub(crate) module_store: Arc<RefCell<ModuleStore<InterpreterModule>>>,
}

pub struct InterpreterModule {
    scope: Arc<ModuleScope>,
}

impl EngineModule for InterpreterModule {
    type Engine = InterpreterEngine;

    fn new(engine: &mut Self::Engine, id: &ModuleIdentifier, importer: &impl ModuleImporter) -> Result<Self, LangError> {
        let uid = match importer.get_unique_identifier(id) {
            Some(uid) => uid,
            None => return Err(LangError::new_runtime(INVALID_IDENTIFIER.to_string())),
        };

        Ok(Self {
            scope: ModuleScope::new(uid, engine),
        })
    }

    fn from_module(engine: &mut Self::Engine, module: Arc<Module>) -> Result<Self, LangError> {
        let scope = ModuleScope::new(module.uid, engine);

        for (func_name, func) in &module.functions {
            scope.set_var(func_name.clone(), LangValue::Function(func.clone()));
        }

        for (var_name, var) in &module.variables {
            let func_scope = Scope::new_module_child(scope.clone());

            let value = match func_scope.evaluate_ast(&var) {
                EvalResult::Ok(value) => value,
                EvalResult::Ret(value, _) => value,
                EvalResult::Err(err) => return Err(err),
            };
            scope.set_var(var_name.clone(), value);
        }

        Ok(InterpreterModule {
            scope,
        })
    }
}


impl Engine for InterpreterEngine {
    type Module = InterpreterModule;
    type ExternalModule = InterpreterExternalModule;

    fn load_module(&mut self, identifier: impl Into<String>, importer: &impl ModuleImporter) -> Result<ModuleUID, LangError> {
        let (uid, modules) = self
            .module_loader()
            .load_module(&ModuleIdentifier(identifier.into()), importer)?;

        for module in &modules {
            let eng_module = InterpreterModule::from_module(self, module.clone())?;

            (*self.module_store)
                .borrow_mut()
                .insert(module.uid, eng_module);
        }

        Ok(uid)
    }

    fn insert_module(&mut self, module: Arc<Module>) -> Result<(), LangError> {
        let uid = module.uid;
        let eng_module = InterpreterModule::from_module(self, module)?;

        (*self.module_store)
            .borrow_mut()
            .insert(uid, eng_module);

        Ok(())
    }

    fn module_loader(&mut self) -> &mut ModuleLoader {
        &mut self.module_loader
    }

    fn insert_external_module(&mut self, module: Self::ExternalModule) {
        self.module_loader()
            .insert_module(module.uid, module.module);

        (*self.module_store)
            .borrow_mut()
            .insert(module.uid, module.engine_module);
    }

    fn new() -> Self {
        Self {
            module_loader: ModuleLoader::new(),
            module_store: Arc::new(RefCell::new(ModuleStore::new())),
        }
    }
}

pub struct InterpreterFunction<Args, R: ExternalType> {
    module_store: Arc<RefCell<ModuleStore<InterpreterModule>>>,
    module: ModuleUID,
    name: String,
    _marker: PhantomData<(Args, R)>,
}

impl<R: ExternalType> InternalFunction<(), Result<R, LangError>>
    for InterpreterFunction<(), R>
{
    fn call(&self, _args: ()) -> Result<R, LangError> {
        let module = (*self.module_store)
            .borrow()
            .get(self.module);

        let module = match module {
            Some(m) => m,
            None => return Err(LangError::new_runtime(MODULE_NOT_FOUND.to_string())),
        };

        let value = module.scope.get_var(&self.name);
        let func = match value {
            None => return Err(LangError::new_runtime(VARIABLE_NOT_DECLARED.to_string())),
            Some(value) => match value {
                LangValue::Function(func) => func,
                _ => return Err(LangError::new_runtime(VARIABLE_IS_NOT_A_FUNCTION.to_string()))
            },
        };

        let scope = Scope::new_module_child(module.scope.clone());
        let result = scope.invoke_function(
            &LangValue::Function(func.clone()),
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

impl<R: ExternalType> EngineGetFunction
    <(), Result<R, LangError>, InterpreterFunction<(), R>>
    for InterpreterEngine
{
    fn get_function(&self, uid: ModuleUID, name: &str)
        -> Option<InterpreterFunction<(), R>>
    {
        Some(InterpreterFunction {
            module_store: self.module_store.clone(),
            module: uid,
            name: name.to_string(),
            _marker: PhantomData::default(),
        })
    }
}