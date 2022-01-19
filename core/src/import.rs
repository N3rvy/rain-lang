use common::errors::LangError;


pub enum ImportResult {
    Imported(String),
    ImportError(LangError),
    AlreadyImported,
    NotFound,
}

pub trait Importer {
    fn import(&self, identifier: &String) -> ImportResult;
}