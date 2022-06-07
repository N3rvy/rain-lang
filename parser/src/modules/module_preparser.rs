use common::ast::types::{Attribute, ClassKind, LiteralKind, OperatorKind, ParenthesisKind, ParenthesisState};
use common::errors::{LangError, ParserErrorKind};
use common::module::{ModuleIdentifier, ModuleUID};
use common::tokens::{TokenKind, Token};
use common::tokens_iterator::Tokens;
use crate::errors::ParsingErrorHelper;
use crate::{expect_open_body, expect_token};
use common::ast::parsing_types::{ParsableFunctionType, ParsableType};
use common::parsable_types::{ParsableClass, ParsableFunction, ParsableModule, ParsableVariable};
use crate::modules::preparsing_utils::{preparse_parameter_names, preparse_type_error, preparse_type_option};
use crate::utils::TokensExtensions;

/// First step of a module parsing, this can either create a `ParsableModule` or a `DeclarationModule`.
/// `ParsableModule` when the module is a definition module, this will contain
/// the declarations with a corresponding token snapshot for later parsing.
/// `DeclarationModule` when the module is a declaration module, this
/// will parse all the declarations and directly create a already completely
/// parsed module.
pub struct ModulePreParser;

impl ModulePreParser {
    pub fn prepare_module(mut tokens: Tokens, id: ModuleIdentifier, uid: ModuleUID) -> Result<ParsableModule, LangError> {
        let mut imports = Vec::new();
        let mut variables = Vec::new();
        let mut functions = Vec::new();
        let mut classes = Vec::new();

        loop {
            if !tokens.has_next() { break }

            let result = Self::parse_declaration(&mut tokens, uid, &mut Vec::new());
            match result {
                Ok(DeclarationParseAction::Import(path)) => {
                    imports.push(ModuleIdentifier(path));
                },
                Ok(DeclarationParseAction::Function(name, func)) => {
                    functions.push((name, func));
                },
                Ok(DeclarationParseAction::Variable(name, var)) => {
                    variables.push((name, var));
                },
                Ok(DeclarationParseAction::Class(name, class)) => {
                    classes.push((name, class));
                },
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(ParsableModule {
            id,
            uid,

            tokens,
            imports,
            variables,
            functions,
            classes,
        })
    }

    fn parse_declaration(tokens: &mut Tokens, module: ModuleUID, attributes: &mut Vec<Attribute>) -> Result<DeclarationParseAction, LangError> {

        let token = tokens.pop_err()?;

        match token.kind {
            TokenKind::Import => {
                // import [path]

                // Should not have any attributes (at least for now)
                for attribute in attributes as &Vec<Attribute> {
                    return match attribute {
                        _ => Err(LangError::parser(&token, ParserErrorKind::InvalidAttribute(attribute.clone()))),
                    }
                }

                // [path]
                let path = match tokens.peek_err()?.kind {
                    TokenKind::Literal(LiteralKind::String(path)) => {
                        tokens.pop();
                        path
                    },
                    _ => {
                        attributes.push(Attribute::Import);

                        return Self::parse_declaration(tokens, module, attributes);
                    }
                };

                // new line
                expect_token!(tokens.pop(), TokenKind::NewLine);

                Ok(DeclarationParseAction::Import(path))
            },
            TokenKind::Variable => {
                // Definition:  var <name> (type) = [value]
                // Declaration: var <name> (type)

                // Should not have any attributes (at least for now)
                for attribute in attributes {
                    return match attribute {
                        _ => Err(LangError::parser(&token, ParserErrorKind::InvalidAttribute(attribute.clone()))),
                    }
                }

                let (name, decl) = Self::parse_variable(tokens)?;

                Ok(DeclarationParseAction::Variable(name, decl))
            },
            TokenKind::Function => {
                // Definition:  func <name>((<param_name> (type))*) (type) {body}
                // Declaration: func <name>((<param_name> (type))*) (type)

                let mut import = false;
                for attribute in attributes {
                    match attribute {
                        Attribute::Import => import = true,
                        _ => return Err(LangError::parser(&token, ParserErrorKind::InvalidAttribute(attribute.clone()))),
                    }
                }

                let (name, func) = Self::parse_function(tokens, import)?;

                Ok(DeclarationParseAction::Function(
                    name,
                    func,
                ))
            },
            TokenKind::Class => {
                /*
                class (data)? ClassName {
                    attr1 int
                    attr2 str

                    func method1() (type) ({body})?
                }
                */

                let mut kind = ClassKind::Normal;
                let mut import = false;
                for attribute in attributes as &Vec<Attribute> {
                    match attribute {
                        Attribute::Data => kind = ClassKind::Data,
                        Attribute::Import => import = true,
                        //_ => return Err(LangError::parser(&token, ParserErrorKind::InvalidAttribute(attribute.clone()))),
                    }
                }

                // <name>
                let name = match tokens.pop() {
                    Some(Token { kind: TokenKind::Symbol(name), start: _, end: _ }) => name,
                    Some(token) => return Err(LangError::new_parser_unexpected_token(&token)),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // {
                expect_open_body!(tokens);

                let class = Self::parse_class_declaration(tokens, kind, name.clone(), module, import)?;

                Ok(DeclarationParseAction::Class(name, class))
            },
            TokenKind::Attribute(attribute) => {
                attributes.push(attribute);
                Self::parse_declaration(tokens, module, attributes)
            }
            TokenKind::NewLine => Ok(DeclarationParseAction::Nothing),
            _ => Err(LangError::new_parser_unexpected_token(&token)),
        }
    }

    fn parse_class_declaration(tokens: &mut Tokens, kind: ClassKind, name: String, module: ModuleUID, import: bool) -> Result<ParsableClass, LangError> {
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        loop {
            let token = match tokens.pop() {
                Some(token) => token,
                None => break,
            };

            match token.kind {
                TokenKind::Symbol(name) => {
                    // (type)
                    let type_ = preparse_type_error(tokens)?;

                    fields.push((name, type_));
                },
                TokenKind::Function => {
                    if let ClassKind::Data = kind {
                        return Err(
                            LangError::parser(
                                &token,
                                ParserErrorKind::Unsupported("Methods in data classes are not yet supported".to_string())))
                    }

                    let (name, method) = Self::parse_function(tokens, import)?;

                    methods.push((
                        name,
                        method,
                    ));
                },
                TokenKind::NewLine => (),
                TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close) => break,
                _ => return Err(LangError::parser(&token, ParserErrorKind::UnexpectedToken))
            }
        }

        Ok(ParsableClass {
            kind,
            name,
            module,

            fields,
            methods,
        })
    }

    fn parse_variable(tokens: &mut Tokens) -> Result<(String, ParsableVariable), LangError> {
        let token = tokens.pop_err()?;

        // <name>
        let name = match token.kind {
            TokenKind::Symbol(name) => name,
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };

        // (type)
        let type_kind = preparse_type_error(tokens)?;

        let token = match tokens.peek() {
            Some(token) => token,
            None => return Err(LangError::new_parser_end_of_file()),
        };

        // (= [value])?
        let body = match token.kind {
            TokenKind::Operator(OperatorKind::Assign) => {
                tokens.pop();

                let body = tokens.snapshot();
                Self::pop_until_newline(tokens);

                Some(body)
            },
            _ => None,
        };

        Ok((
            name,
            ParsableVariable {
                type_kind,
                body,
            },
        ))
    }

    fn parse_function(tokens: &mut Tokens, import: bool) -> Result<(String, ParsableFunction), LangError> {
        let token = tokens.pop_err()?;

        // <name>
        let name = match token.kind {
            TokenKind::Symbol(name) => name,
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };

        // (
        expect_token!(tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

        // (<param_name> (type))*)
        let (param_names, param_types) = preparse_parameter_names(tokens)?;

        // (type)
        let ret_type = preparse_type_option(tokens).unwrap_or(ParsableType::Nothing);
        let func_type = ParsableFunctionType(param_types, Box::new(ret_type));

        let token = match tokens.peek() {
            Some(token) => token,
            None => return Err(LangError::new_parser_end_of_file()),
        };

        let body = match token.kind {
            TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open) => {
                tokens.pop();

                let body = tokens.snapshot();
                Self::pop_body(tokens)?;

                Some(body)
            },
            _ if import => None,
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };

        Ok((
            name,
            ParsableFunction {
                params: param_names,
                func_type,
                body,
            }
        ))
    }

    fn pop_body(tokens: &mut Tokens) -> Result<(), LangError> {
        let mut open_parenthesis = 1;

        loop {
            match tokens.pop() {
                Some(Token { kind: TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open), start: _, end: _ }) => open_parenthesis += 1,
                Some(Token { kind: TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close), start: _, end: _ }) => {
                    open_parenthesis -= 1;

                    if open_parenthesis == 0 {
                        break;
                    }
                },
                None => break,
                Some(_) => (),
            }
        }

        Ok(())
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
    Variable(String, ParsableVariable),
    Function(String, ParsableFunction),
    Class(String, ParsableClass),
    Nothing,
}
