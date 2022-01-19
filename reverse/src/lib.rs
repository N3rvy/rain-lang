pub use common::lang_value::LangValue;
pub use common::errors::LangError;
pub use common::external_functions::IntoExternalFunctionRunner;
pub use common::external_functions::ExternalFunctionRunner;
pub use common::external_functions::AsMethod;
pub use common::lang_value::Function;
pub use common::script::Script;
use parser::type_check::check_types;
pub use vm::scope::Scope;
pub use vm::Vm;
pub use vm::import::Importer;
pub use vm::import::ImportResult;
pub use common::object::LangObject;

use parser::parser::parse;
use tokenizer::tokenizer::tokenize;


pub trait IntoScript {
    fn script(self) -> Result<Script, LangError>;
}

impl IntoScript for String {
    fn script(self) -> Result<Script, LangError> {
        let tokens = tokenize(self)?;
        let ast = parse(tokens)?;
        
        check_types(&ast)?;

        Ok(Script { ast })
    }
}