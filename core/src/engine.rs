use common::ast::types::TypeKind;
use common::errors::LangError;
use parser::modules::module_importer::{ModuleIdentifier, ModuleImporter, ModuleUID};
use parser::modules::module_loader::{LoadModuleResult, ModuleLoader};

use crate::{externals::ExternalType, module::EngineModule};
use crate::errors::MODULE_NOT_FOUND;
use crate::module_builder::ModuleBuilder;


pub trait Engine
where
    Self: Sized,
{
    type Module: EngineModule<Engine = Self>;

    fn load_module<Importer: ModuleImporter>(&mut self, identifier: &ModuleIdentifier) -> Result<ModuleUID, LangError> {
        let mut loader = ModuleLoader::<Importer>::new();

        let main_uid = loader.load_module(identifier);

        let main_uid = match main_uid {
            LoadModuleResult::Ok(uid) |
            LoadModuleResult::AlreadyLoaded(uid) => uid,
            LoadModuleResult::NotFound => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string())),
            LoadModuleResult::Err(err) => return Err(err),
        };

        let module_builder = self.module_builder_mut();

        for (uid, module) in loader.modules_owned() {
            module_builder
                .load_module(uid, module)?;
        }

        Ok(main_uid)
    }

    fn global_types(&self) -> &Vec<(String, TypeKind)>;
    fn module_builder(&self) -> &ModuleBuilder<Self>;
    fn module_builder_mut(&mut self) -> &mut ModuleBuilder<Self>;

    fn new() -> Self;
}

pub trait EngineGetFunction<'a, Args, R, Ret: InternalFunction<Args, R>> : Engine {
    fn get_function(&'a self, uid: ModuleUID, name: &str)
                    -> Option<Ret>;
}

pub trait InternalFunction<Args, R> {
    fn call(&self, args: Args) -> R;
}

pub trait EngineSetFunction<'a, Args, R: ExternalType> : Engine {
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}
