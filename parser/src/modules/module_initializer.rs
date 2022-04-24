use common::ast::types::{FunctionType, LiteralKind, OperatorKind, ParenthesisKind, ParenthesisState, TypeKind};
use common::errors::{LangError, ParserErrorKind};
use common::module::{DefinitionModule, ModuleIdentifier};
use common::tokens::{TokenKind, Token};
use tokenizer::iterator::{Tokens, TokenSnapshot};
use crate::errors::ParsingErrorHelper;
use crate::{expect_indent, expect_token};
use crate::utils::{parse_parameter_names, parse_type_error, TokensExtensions};

pub enum DeclarationKind {
    Variable(TypeKind),
    Function(Vec<String>, FunctionType),
}

pub struct Declaration {
    pub kind: DeclarationKind,
    pub body: TokenSnapshot,
}

pub struct ParsableClass {
    pub declarations: Vec<(String, Declaration)>
}

pub struct ParsableModule {
    pub tokens: Tokens,
    pub imports: Vec<ModuleIdentifier>,
    pub declarations: Vec<(String, Declaration)>,
    pub classes: Vec<(String, ParsableClass)>,
}

pub struct ModuleInitializer;

impl ModuleInitializer {
    pub fn create(tokens: Tokens) -> Result<ParsableModule, LangError> {
        let mut module = ParsableModule {
            tokens,
            imports: Vec::new(),
            declarations: Vec::new(),
            classes: Vec::new(),
        };

        loop {
            let token = match module.tokens.peek() {
                Some(token) => token,
                None => break,
            };

            let result = Self::parse_declaration(&mut module.tokens, false);
            match result {
                Ok(DeclarationParseAction::Import(path)) => {
                    module.imports.push(ModuleIdentifier(path));
                },
                Ok(DeclarationParseAction::Declaration(name, declaration)) => {
                    module.declarations.push((name, declaration));
                },
                Ok(DeclarationParseAction::ClassDefinition(name, class)) => {
                    module.classes.push((name, class));
                },
                Ok(DeclarationParseAction::FunctionDefinition(_, _)) =>
                    return Err(
                        LangError::parser(
                            &token, ParserErrorKind::Unsupported("Function definition inside a normal module".to_string()))),
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(module)
    }

    pub fn create_definition(mut tokens: Tokens, id: ModuleIdentifier) -> Result<DefinitionModule, LangError> {
        let imports = Vec::new();
        let mut functions = Vec::new();

        loop {
            let token = match tokens.peek() {
                Some(token) => token,
                None => break,
            };

            let result = Self::parse_declaration(&mut tokens, true);
            match result {
                Ok(DeclarationParseAction::Import(_path)) => {
                    todo!()
                },
                Ok(DeclarationParseAction::FunctionDefinition(name, func_type)) => {
                    functions.push((name, func_type));
                },
                Ok(DeclarationParseAction::ClassDefinition(_, _)) => todo!(),
                Ok(DeclarationParseAction::Declaration(_, _)) =>
                    return Err(
                        LangError::parser(
                            &token,
                            ParserErrorKind::Unsupported(
                                "Declaration inside of a definition module".to_string()))),
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(DefinitionModule {
            id,

            imports,
            functions,
        })
    }

    fn parse_declaration(tokens: &mut Tokens, is_definition: bool) -> Result<DeclarationParseAction, LangError> {
        let token = tokens.pop_err()?;

        match token.kind {
            TokenKind::Import => {
                // import [path]

                // [path]
                let path = match tokens.pop_err()?.kind {
                    TokenKind::Literal(LiteralKind::String(path)) => path,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };

                // new line
                expect_token!(tokens.pop(), TokenKind::NewLine);

                Ok(DeclarationParseAction::Import(path))
            },
            TokenKind::Variable => {
                // var <name> (type) = [value]

                if is_definition {
                    return Err(LangError::parser(
                        &token,
                        ParserErrorKind::Unsupported(
                            "Variable inside of a definition module".to_string())));
                }

                let token = tokens.pop_err()?;

                // <name>
                let name = match token.kind {
                    TokenKind::Symbol(name) => name,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };

                // (type)
                let type_kind = parse_type_error(tokens)?;

                // =
                expect_token!(tokens.pop(), TokenKind::Operator(OperatorKind::Assign));

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
            TokenKind::Function => {
                // func <name>((<param_name> (type))*) (type): {body}

                let token = tokens.pop_err()?;

                // <name>
                let name = match token.kind {
                    TokenKind::Symbol(name) => name,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };

                // (
                expect_token!(tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

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
            TokenKind::Class => {
                /*
                class ClassName:
                    var attr1 int = 0
                    var attr2 str = "no"

                    func method1() (type): {body}
                */

                // <name>
                let name = match tokens.pop() {
                    Some(Token { kind: TokenKind::Symbol(name), start: _, end: _ }) => name,
                    Some(token) => return Err(LangError::new_parser_unexpected_token(&token)),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // :
                expect_indent!(tokens);

                let mut declarations = Vec::new();

                loop {
                    let token = tokens.peek();
                    let definition = Self::parse_declaration(tokens, is_definition)?;

                    match definition {
                        DeclarationParseAction::Nothing => (),
                        DeclarationParseAction::Declaration(name, decl) =>
                            declarations.push((name, decl)),
                        DeclarationParseAction::FunctionDefinition(_, _) => todo!(),

                        DeclarationParseAction::ClassDefinition(_, _)
                            => return Err(
                                LangError::parser(
                                    &token.unwrap(),
                                    ParserErrorKind::Unsupported("Class inside another class".to_string()))),
                        DeclarationParseAction::Import(_)
                            => return Err(LangError::new_parser_unexpected_token(&token.unwrap())),
                    }

                    if let Some(Token { kind: TokenKind::Dedent, start: _, end: _ }) = tokens.peek() {
                        break
                    }
                }
                tokens.pop();

                Ok(DeclarationParseAction::ClassDefinition(
                    name,
                    ParsableClass {
                        declarations,
                    }
                ))
            },
            TokenKind::NewLine => Ok(DeclarationParseAction::Nothing),
            _ => Err(LangError::new_parser_unexpected_token(&token)),
        }
    }

    fn pop_until_dedent(tokens: &mut Tokens) {
        let mut indentations = 0;

        loop {
            match tokens.pop() {
                Some(Token { kind: TokenKind::Indent, start: _, end: _ }) => indentations += 1,
                Some(Token { kind: TokenKind::Dedent, start: _, end: _ }) => {
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
                Some(Token { kind: TokenKind::NewLine, start: _, end: _ }) | None => break,
                Some(_) => (),
            }
        }
    }
}

enum DeclarationParseAction {
    Import(String),
    Declaration(String, Declaration),
    FunctionDefinition(String, FunctionType),
    ClassDefinition(String, ParsableClass),
    Nothing,
}
