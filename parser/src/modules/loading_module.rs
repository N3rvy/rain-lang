use std::sync::Arc;
use common::ast::ASTNode;
use common::ast::module::ASTModule;
use common::ast::types::{Function, FunctionType, LiteralKind, OperatorKind, ParenthesisKind, ParenthesisState, TypeKind};
use common::errors::LangError;
use tokenizer::iterator::{Tokens, TokenSnapshot};
use tokenizer::tokens::Token;
use crate::errors::{ParsingErrorHelper, UNEXPECTED_ERROR, WRONG_TYPE};
use crate::{expect_indent, expect_token};
use crate::parser::ParserScope;
use crate::utils::{parse_parameter_names, parse_type_error};

pub enum DeclarationKind {
    Variable(TypeKind),
    Function(Vec<String>, FunctionType),
}

pub struct Declaration {
    pub kind: DeclarationKind,
    pub body: TokenSnapshot,
}

pub struct LoadingModule {
    pub tokens: Tokens,
    imports: Vec<String>,
    pub declarations: Vec<(String, Declaration)>
}

impl LoadingModule {
    pub fn from_tokens(tokens: Tokens) -> Result<Self, LangError> {
        let mut module = Self {
            tokens,
            imports: Vec::new(),
            declarations: Vec::new(),
        };

        loop {
            if !module.tokens.has_next() {
                break
            }

            module.parse_definition()?;
        }

        Ok(module)
    }

    fn parse_definition(&mut self) -> Result<(), LangError> {
        let token = match self.tokens.pop() {
            Some(t) => t,
            None => return Err(LangError::new_parser_end_of_file()),
        };

        match token {
            Token::Import => {
                // import [path]

                // [path]
                let path = match self.tokens.pop() {
                    Some(Token::Literal(LiteralKind::String(path))) => path,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // new line
                expect_token!(self.tokens.pop(), Token::NewLine);

                self.imports.push(path);

                Ok(())
            },
            Token::Variable => {
                // var <name>: (type) = [value]

                // <name>
                let name = match self.tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // : (type)
                let type_kind = parse_type_error(&mut self.tokens)?;

                // =
                expect_token!(self.tokens.pop(), Token::Operator(OperatorKind::Assign));

                // [value]
                let body = self.tokens.snapshot();
                Self::pop_until_newline(&mut self.tokens);

                self.declarations.push((
                    name,
                    Declaration {
                        kind: DeclarationKind::Variable(type_kind),
                        body,
                    },
                ));

                Ok(())
            },
            Token::Function => {
                // func <name>((<param_name>: (type))*): (type) {body}

                // <name>
                let name = match self.tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // (
                expect_token!(self.tokens.pop(), Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

                // (<param_name>: (type))*)
                let (param_names, param_types) = parse_parameter_names(&mut self.tokens)?;

                // : (type)
                let ret_type = parse_type_error(&mut self.tokens)?;

                expect_indent!(self.tokens);

                // {body}
                let body = self.tokens.snapshot();
                Self::pop_until_dedent(&mut self.tokens);

                let func_type = FunctionType(param_types, Box::new(ret_type));

                self.declarations.push((
                    name,
                    Declaration {
                        kind: DeclarationKind::Function(param_names, func_type),
                        body,
                    }
                ));

                Ok(())
            },
            Token::NewLine => self.parse_definition(),
            _ => Err(LangError::new_parser_unexpected_token()),
        }
    }

    pub fn imports(&self) -> &Vec<String> {
        &self.imports
    }

    fn pop_until_dedent(tokens: &mut Tokens) {
        let mut indentations = 0;

        loop {
            match tokens.pop() {
                Some(Token::Indent) => indentations += 1,
                Some(Token::Dedent) => {
                    if indentations == 0 {
                        break;
                    }

                    indentations -= 1;
                },
                None => break,
                Some(_) => (),
            }
        }
    }

    fn pop_until_newline(tokens: &mut Tokens) {
        loop {
            match tokens.pop() {
                Some(Token::NewLine) | None => break,
                Some(_) => (),
            }
        }
    }
}