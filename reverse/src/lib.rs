pub use common::lang_value::LangValue;
pub use common::errors::LangError;
pub use common::external_functions::IntoExternalFunctionRunner;
use parser::parser::parse;
use tokenizer::tokenizer::tokenize;
pub use vm::scope::Scope;
pub use vm::Vm;

use common::script::Script;


pub trait IntoScript {
    fn script(self) -> Result<Script, LangError>;
}

impl IntoScript for String {
    fn script(self) -> Result<Script, LangError> {
        let tokens = tokenize(self)?;
        let ast = parse(tokens)?;

        Ok(Script { ast })
    }
}