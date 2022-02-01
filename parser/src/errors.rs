use common::errors::LangError;

pub const UNEXPECTED_TOKEN: &str = "Unexpected token";
pub const UNEXPECTED_END_OF_FILE: &str = "Unexpected end of file";
pub const WRONG_TYPE: &str = "Wrong typing";
pub const INCORRECT_NUMBER_OF_PARAMETERS: &str = "Incorrect number of parameters passed to function";
pub const INCORRECT_FUNCTION_PARAMETER_TYPE: &str = "A parameter type passed to a function is incorrect";
pub const PARAMETERS_EXPECTING_COMMA: &str = "Expected a comma found a parameter name";
pub const PARAMETERS_EXPECTING_PARAMETER: &str = "Expected a parameter name found comma";
pub const UNEXPECTED_ERROR: &str = "Unexpected error";


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