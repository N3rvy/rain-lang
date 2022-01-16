use common::script::Script;


pub enum ImportResult {
    Imported(Script),
    AlreadyImported,
    NotFound,
}

pub trait Importer {
    fn import(&self, identifier: &String) -> ImportResult;
}