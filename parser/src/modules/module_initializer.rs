use common::ast::types::{FunctionType, LiteralKind, OperatorKind, ParenthesisKind, ParenthesisState, TypeKind};
use common::errors::LangError;
use common::module::ModuleIdentifier;
use tokenizer::iterator::{Tokens, TokenSnapshot};
use tokenizer::tokens::Token;
use crate::errors::ParsingErrorHelper;
use crate::{expect_indent, expect_token};
use crate::utils::{parse_parameter_names, parse_type_error};

pub enum DeclarationKind {
    Variable(TypeKind),
    Function(FunctionType),
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

            let result = Self::parse_declaration(&mut module);
            match result {
                Ok(DeclarationParseAction::Import(path)) => {
                    module.imports.push(ModuleIdentifier(path));
                },
                Ok(DeclarationParseAction::Declaration(name, declaration)) => {
                    module.declarations.push((name, declaration));
                },
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(module)
    }

    fn parse_declaration(module: &mut ParsableModule) -> Result<DeclarationParseAction, LangError> {
        let token = match module.tokens.pop() {
            Some(t) => t,
            None => return Err(LangError::new_parser_end_of_file()),
        };

        match token {
            Token::Import => {
                // import [path]

                // [path]
                let path = match module.tokens.pop() {
                    Some(Token::Literal(LiteralKind::String(path))) => path,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // new line
                expect_token!(module.tokens.pop(), Token::NewLine);

                Ok(DeclarationParseAction::Import(path))
            },
            Token::Variable => {
                // var <name> (type) = [value]

                // <name>
                let name = match module.tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // (type)
                let type_kind = parse_type_error(&mut module.tokens)?;

                // =
                expect_token!(module.tokens.pop(), Token::Operator(OperatorKind::Assign));

                // [value]
                let body = module.tokens.snapshot();
                Self::pop_until_newline(&mut module.tokens);

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
                let name = match module.tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // (
                expect_token!(module.tokens.pop(), Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

                // (<param_name> (type))*)
                let params = parse_parameter_names(&mut module.tokens)?;

                // (type)
                let ret_type = parse_type_error(&mut module.tokens)?;

                expect_indent!(module.tokens);

                // {body}
                let body = module.tokens.snapshot();
                Self::pop_until_dedent(&mut module.tokens);

                let func_type = FunctionType(params, Box::new(ret_type));

                Ok(DeclarationParseAction::Declaration(
                    name,
                    Declaration {
                        kind: DeclarationKind::Function(func_type),
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
    Nothing,
}
