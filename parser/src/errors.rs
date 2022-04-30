use common::{errors::{LangError, ParserErrorKind}, tokens::{TokenKind, Token}, ast::types::TypeKind};
use common::ast::parsing_types::ParsableType;

pub trait ParsingErrorHelper {
    fn new_parser_unexpected_token(token: &Token) -> Self;
    fn new_parser_end_of_file() -> Self;
    fn wrong_type(token: &Token, expected: &TypeKind, got: &TypeKind) -> Self;
}

impl ParsingErrorHelper for LangError {
    fn new_parser_unexpected_token(token: &Token) -> Self {
        Self::parser(&token, ParserErrorKind::UnexpectedToken)
    }

    fn new_parser_end_of_file() -> Self {
        Self::parser(&Token::new(TokenKind::NewLine, usize::MAX, usize::MAX), ParserErrorKind::UnexpectedEndOfFile)
    }

    fn wrong_type(token: &Token, expected: &TypeKind, got: &TypeKind) -> Self {
        Self::parser(token, ParserErrorKind::WrontType(expected.clone(), got.clone()))
    }
}