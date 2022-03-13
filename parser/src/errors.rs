use common::errors::LangError;

pub const UNEXPECTED_ERROR: &str = "Unexpected error";
pub const UNEXPECTED_TOKEN: &str = "Unexpected token";
pub const UNEXPECTED_END_OF_FILE: &str = "Unexpected end of file";
pub const WRONG_TYPE: &str = "Wrong typing";
pub const PARAMETERS_EXPECTING_COMMA: &str = "Expected a comma found a parameter name";
pub const PARAMETERS_EXPECTING_PARAMETER: &str = "Expected a parameter name found comma";
pub const VAR_NOT_FOUND: &str = "Variable not found in this scope";
pub const INVALID_FIELD_ACCESS: &str = "Invalid field access";
pub const FIELD_DOESNT_EXIST: &str = "Field does not exist inside of object";
pub const INVALID_ASSIGN: &str = "Invalid type in assignment";
pub const NOT_A_FUNCTION: &str = "Tried invoking a value that is not a function";
pub const NOT_A_VECTOR: &str = "Tried indexing a variable that is not a vector";
pub const INVALID_ARGS_COUNT: &str = "Tried invoking a function with an incorrect number of parameters";
pub const INVALID_ARGS: &str = "Tried invoking a function with an incorrect parameter type";
pub const MODULE_NOT_FOUND: &str = "Could not find the module";
pub const LOAD_MODULE_ERROR: &str = "Could not load the module";


pub trait ParsingErrorHelper {
    fn new_parser_unexpected_token() -> Self;
    fn new_parser_end_of_file() -> Self;
    fn new_parser_wrong_type() -> Self;
}

impl ParsingErrorHelper for LangError {
    fn new_parser_unexpected_token() -> Self {
        Self::new_parser(UNEXPECTED_TOKEN.to_string())
    }

    fn new_parser_end_of_file() -> Self {
        Self::new_parser(UNEXPECTED_END_OF_FILE.to_string())
    }

    fn new_parser_wrong_type() -> Self {
        Self::new_parser(WRONG_TYPE.to_string())
    }
}