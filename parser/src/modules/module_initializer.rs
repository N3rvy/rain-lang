use common::ast::types::{FunctionType, LiteralKind, OperatorKind, ParenthesisKind, ParenthesisState, TypeKind};
use common::errors::LangError;
use common::module::{DefinitionModule, ModuleIdentifier, ModuleUID};
use tokenizer::iterator::{Tokens, TokenSnapshot};
use tokenizer::tokens::Token;
use crate::errors::{ParsingErrorHelper, UNEXPECTED_ERROR, VAR_INSIDE_DEF_MODULE};
use crate::{expect_indent, expect_token};
use crate::utils::{parse_parameter_names, parse_type_error};

pub enum DeclarationKind {
    Variable(TypeKind),
    Function(Vec<String>, FunctionType),
}

pub struct Declaration {
    pub kind: DeclarationKind,
    pub body: TokenSnapshot,
}

pub struct ParsableModule {
    pub tokens: Tokens,
    pub imports: Vec<ModuleIdentifier>,
    pub declarations: Vec<(String, Declaration)>
}

pub struct ModuleInitializer;

impl ModuleInitializer {
    pub fn create(tokens: Tokens) -> Result<ParsableModule, LangError> {
        let mut module = ParsableModule {
            tokens,
            imports: Vec::new(),
            declarations: Vec::new(),
        };

        loop {
            if !module.tokens.has_next() {
                break
            }

            let result = Self::parse_declaration(&mut module.tokens, false);
            match result {
                Ok(DeclarationParseAction::Import(path)) => {
                    module.imports.push(ModuleIdentifier(path));
                },
                Ok(DeclarationParseAction::Declaration(name, declaration)) => {
                    module.declarations.push((name, declaration));
                },
                Ok(DeclarationParseAction::FunctionDefinition(_, _)) => return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string())),
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(module)
    }

    pub fn create_definition(mut tokens: Tokens, uid: ModuleUID) -> Result<DefinitionModule, LangError> {
        let imports = Vec::new();
        let mut functions = Vec::new();

        loop {
            if !tokens.has_next() {
                break
            }

            let result = Self::parse_declaration(&mut tokens, true);
            match result {
                Ok(DeclarationParseAction::Import(_path)) => {
                    todo!()
                },
                Ok(DeclarationParseAction::Declaration(_, _)) => return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string())),
                Ok(DeclarationParseAction::FunctionDefinition(name, func_type)) => {
                    functions.push((name, func_type));
                },
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(DefinitionModule {
            uid,

            imports,
            functions,
        })
    }

    fn parse_declaration(tokens: &mut Tokens, is_definition: bool) -> Result<DeclarationParseAction, LangError> {
        let token = match tokens.pop() {
            Some(t) => t,
            None => return Err(LangError::new_parser_end_of_file()),
        };

        match token {
            Token::Import => {
                // import [path]

                // [path]
                let path = match tokens.pop() {
                    Some(Token::Literal(LiteralKind::String(path))) => path,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // new line
                expect_token!(tokens.pop(), Token::NewLine);

                Ok(DeclarationParseAction::Import(path))
            },
            Token::Variable => {
                // var <name> (type) = [value]

                if is_definition {
                    return Err(LangError::new_parser(VAR_INSIDE_DEF_MODULE.to_string()));
                }

                // <name>
                let name = match tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // (type)
                let type_kind = parse_type_error(tokens)?;

                // =
                expect_token!(tokens.pop(), Token::Operator(OperatorKind::Assign));

                // [value]
                let body = tokens.snapshot();
                Self::pop_until_newline(tokens);

                Ok(DeclarationParseAction::Declaration(
                    name,
                    Declaration {
                        kind: DeclarationKind::Variable(type_kind),
                        body,
                    },
                ))
            },
            Token::Function => {
                // func <name>((<param_name> (type))*) (type): {body}

                // <name>
                let name = match tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // (
                expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

                // (<param_name> (type))*)
                let (param_names, param_types) = parse_parameter_names(tokens)?;

                // (type)
                let ret_type = parse_type_error(tokens)?;

                let func_type = FunctionType(param_types, Box::new(ret_type));

                if is_definition {
                    return Ok(DeclarationParseAction::FunctionDefinition(name, func_type))
                }

                expect_indent!(tokens);

                // {body}
                let body = tokens.snapshot();
                Self::pop_until_dedent(tokens);

                Ok(DeclarationParseAction::Declaration(
                    name,
                    Declaration {
                        kind: DeclarationKind::Function(param_names, func_type),
                        body,
                    }
                ))
            },
            Token::NewLine => Ok(DeclarationParseAction::Nothing),
            _ => Err(LangError::new_parser_unexpected_token()),
        }
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

enum DeclarationParseAction {
    Import(String),
    Declaration(String, Declaration),
    FunctionDefinition(String, FunctionType),
    Nothing,
}
