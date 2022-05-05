use common::ast::types::{OperatorKind, ParenthesisKind, ParenthesisState};
use common::errors::{LangError, ParserErrorKind};
use common::tokens::TokenKind;
use tokenizer::iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use common::ast::parsing_types::ParsableType;
use crate::utils::TokensExtensions;

pub fn preparse_type_error(tokens: &mut Tokens) -> Result<ParsableType, LangError> {
    let token = tokens.pop_err()?;

    // type
    match token.kind {
        TokenKind::Type(tk) => Ok(ParsableType::from(&tk)),
        TokenKind::Symbol(name) => Ok(ParsableType::Custom(name)),
        _ => Err(LangError::new_parser_unexpected_token(&token)),
    }
}

pub fn preparse_type_option(tokens: &mut Tokens) -> Option<ParsableType> {
    let token = match tokens.peek() {
        Some(token) => {
            token
        },
        None => return None,
    };

    // type
    match token.kind {
        TokenKind::Type(tk) => {
            tokens.pop();
            Some(ParsableType::from(&tk))
        }
        TokenKind::Symbol(name) =>{
            tokens.pop();
            Some(ParsableType::Custom(name))
        }
        _ => None,
    }
}

/** Parses a list of parameter names (something like "(arg0, arg1, arg2)").
* It consumes only the last parenthesis and expects the first token to be the first argument,
    in this case it will be "arg0"
 */
pub fn preparse_parameter_names(tokens: &mut Tokens) -> Result<(Vec<String>, Vec<ParsableType>), LangError> {
    let mut names = Vec::new();
    let mut types = Vec::new();
    let mut next_is_argument = true;

    loop {
        let token = match tokens.pop() {
            Some(token) => token,
            None => return Err(LangError::new_parser_end_of_file()),
        };

        match &token.kind {
            TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close) => break,
            TokenKind::Symbol(name) => {
                if next_is_argument {
                    next_is_argument = false;

                    let t = preparse_type_error(tokens)?;

                    names.push(name.clone());
                    types.push(t);
                } else {
                    return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedComma));
                }
            },
            TokenKind::Operator(OperatorKind::Comma) => {
                if next_is_argument {
                    return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedParam));
                } else {
                    next_is_argument = true;
                }
            },
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };
    }

    Ok((names, types))
}
