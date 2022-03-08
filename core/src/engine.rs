use common::ast::module::ASTModule;
use common::ast::types::TypeKind;
use common::errors::LangError;
use parser::modules::module_importer::{ModuleIdentifier, ModuleImporter, ModuleUID};
use parser::modules::module_loader::{LoadModuleResult, ModuleLoader};

use crate::{externals::ExternalType, module::EngineModule};
use crate::errors::MODULE_NOT_FOUND;


pub trait Engine<'a>
where
    Self: Sized,
{
    type Module: EngineModule;

    fn load_module<Importer: ModuleImporter>(&'a mut self, identifier: &ModuleIdentifier) -> Result<ModuleUID, LangError> {
        let mut loader = ModuleLoader::<Importer>::new();

        let main_uid = loader.load_module(identifier);

        let main_uid = match main_uid {
            LoadModuleResult::Ok(uid) |
            LoadModuleResult::AlreadyLoaded(uid) => uid,
            LoadModuleResult::NotFound => return Err(LangError::new_parser(MODULE_NOT_FOUND.to_string())),
            LoadModuleResult::Err(err) => return Err(err),
        };

        for (uid, module) in loader.modules_owned() {
            let module = module.load()?;

            self.insert_module(uid, module)?;
        }

        Ok(main_uid)
    }

    fn global_types(&'a self) -> &'a Vec<(String, TypeKind)>;
    fn insert_module(&mut self, uid: ModuleUID, module: ASTModule) -> Result<(), LangError>;
    fn get_module(&self, uid: ModuleUID) -> Option<&Self::Module>;

    fn new() -> Self;
}

pub trait EngineGetFunction<'a, Args, R, Ret: InternalFunction<Args, R>> : Engine<'a> {
    fn get_function(&'a self, uid: ModuleUID, name: &str)
                    -> Option<Ret>;
}

pub trait InternalFunction<Args, R> {
    fn call(&self, args: Args) -> R;
}

pub trait EngineSetFunction<'a, Args, R: ExternalType> : Engine<'a> {
    fn set_function<F>(&mut self, name: &str, func: F)
    where F: Fn<Args, Output = R> + Send + Sync + 'static;
}