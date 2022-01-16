use common::{script::Script, errors::LangError};


pub enum ImportResult {
    Imported(Script),
    ImportError(LangError),
    AlreadyImported,
    NotFound,
}

pub trait Importer {
    fn import(&self, identifier: &String) -> ImportResult;
}