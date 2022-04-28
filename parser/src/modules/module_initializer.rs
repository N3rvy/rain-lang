use std::sync::Arc;
use common::ast::types::{ClassKind, ClassType, FunctionType, LiteralKind, OperatorKind, ParenthesisKind, ParenthesisState, TypeKind};
use common::errors::{LangError, ParserErrorKind};
use common::module::{DeclarationModule, ModuleIdentifier, ModuleUID};
use common::tokens::{TokenKind, Token};
use tokenizer::iterator::{Tokens, TokenSnapshot};
use crate::errors::ParsingErrorHelper;
use crate::{expect_indent, expect_token};
use crate::utils::{parse_parameter_names, parse_type_error, parse_type_option, TokensExtensions};

pub struct VariableDeclaration {
    pub type_kind: TypeKind,
    pub body: TokenSnapshot,
}

pub struct FunctionDeclaration {
    pub params: Vec<String>,
    pub func_type: FunctionType,
    pub body: TokenSnapshot,
}

pub struct ParsableClass {
    pub class_type: Arc<ClassType>,
    pub functions: Vec<(String, FunctionDeclaration)>
}

/// This represents a module that needs more processing to be parsed
pub struct ParsableModule {
    pub uid: ModuleUID,
    pub tokens: Tokens,
    pub imports: Vec<ModuleIdentifier>,
    pub variables: Vec<(String, VariableDeclaration)>,
    pub functions: Vec<(String, FunctionDeclaration)>,
    pub classes: Vec<(String, ParsableClass)>,
}

/// First step of a module parsing, this can either create a `ParsableModule` or a `DeclarationModule`.
/// `ParsableModule` when the module is a definition module, this will contain
/// the declarations with a corresponding token snapshot for later parsing.
/// `DeclarationModule` when the module is a declaration module, this
/// will parse all the declarations and directly create a already completely
/// parsed module.
pub struct ModuleInitializer;

impl ModuleInitializer {
    pub fn initialize_module(tokens: Tokens, uid: ModuleUID) -> Result<ParsableModule, LangError> {
        let mut module = ParsableModule {
            uid,
            tokens,
            imports: Vec::new(),
            variables: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
        };

        loop {
            let token = match module.tokens.peek() {
                Some(token) => token,
                None => break,
            };

            let result = Self::parse_declaration(&mut module.tokens, module.uid, false);
            match result {
                Ok(DeclarationParseAction::Import(path)) => {
                    module.imports.push(ModuleIdentifier(path));
                },
                Ok(DeclarationParseAction::Function(name, func)) => {
                    module.functions.push((name, func));
                },
                Ok(DeclarationParseAction::Variable(name, var)) => {
                    module.variables.push((name, var));
                },
                Ok(DeclarationParseAction::Class(name, class)) => {
                    module.classes.push((name, class));
                },
                Ok(DeclarationParseAction::FunctionDeclaration(_, _)) =>
                    return Err(
                        LangError::parser(
                            &token, ParserErrorKind::Unsupported("Function declaration inside a definition module".to_string()))),
                Ok(DeclarationParseAction::ClassDeclaration(_)) =>
                    return Err(
                        LangError::parser(
                            &token, ParserErrorKind::Unsupported("Class declaration inside a definition module".to_string()))),
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(module)
    }

    pub fn parse_declaration_module(mut tokens: Tokens, id: ModuleIdentifier, uid: ModuleUID) -> Result<DeclarationModule, LangError> {
        let imports = Vec::new();
        let mut functions = Vec::new();
        let mut classes = Vec::new();

        loop {
            let token = match tokens.peek() {
                Some(token) => token,
                None => break,
            };

            let result = Self::parse_declaration(&mut tokens, uid, true);
            match result {
                Ok(DeclarationParseAction::Import(_path)) => {
                    todo!()
                },
                Ok(DeclarationParseAction::FunctionDeclaration(name, func_type)) => {
                    functions.push((name, func_type));
                },
                Ok(DeclarationParseAction::ClassDeclaration(class_type)) => {
                    classes.push((class_type.name.clone(), Arc::new(class_type)));
                },
                Ok(DeclarationParseAction::Function(_, _)) |
                Ok(DeclarationParseAction::Variable(_, _)) |
                Ok(DeclarationParseAction::Class(_, _)) =>
                    return Err(
                        LangError::parser(
                            &token,
                            ParserErrorKind::Unsupported(
                                "Definition inside of a declaration module".to_string()))),
                Ok(DeclarationParseAction::Nothing) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(DeclarationModule {
            id,

            imports,
            functions,
            classes,
        })
    }

    fn parse_declaration(tokens: &mut Tokens, module: ModuleUID, declaration: bool) -> Result<DeclarationParseAction, LangError> {
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

                if declaration {
                    return Err(LangError::parser(
                        &token,
                        ParserErrorKind::Unsupported(
                            "Variable inside of a definition module".to_string())));
                }

                let (name, decl) = Self::parse_variable(tokens)?;

                Ok(DeclarationParseAction::Variable(name, decl))
            },
            TokenKind::Function => {
                // func <name>((<param_name> (type))*) (type): {body}

                if declaration {
                    let (name, type_) = Self::parse_function_declaration(tokens)?;

                    return Ok(DeclarationParseAction::FunctionDeclaration(name, type_))
                }

                let (name, decl) = Self::parse_function(tokens)?;

                Ok(DeclarationParseAction::Function(
                    name,
                    decl,
                ))
            },
            TokenKind::Class => {
                /*
                class (data)? ClassName:
                    var attr1 int = 0
                    var attr2 str = "no"

                    func method1() (type): {body}
                */

                let kind = match tokens.peek() {
                    Some(Token { kind: TokenKind::Data, start: _, end: _ }) => {
                        tokens.pop();
                        ClassKind::Data
                    },
                    _ => ClassKind::Normal,
                };

                // <name>
                let name = match tokens.pop() {
                    Some(Token { kind: TokenKind::Symbol(name), start: _, end: _ }) => name,
                    Some(token) => return Err(LangError::new_parser_unexpected_token(&token)),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // :
                expect_indent!(tokens);

                if declaration {
                    let class_type = Self::parse_class_declaration(tokens, module, &name, kind)?;

                    Ok(DeclarationParseAction::ClassDeclaration(class_type))
                } else {
                    let class = Self::parse_class_definition(tokens, module, &name, kind)?;

                    Ok(DeclarationParseAction::Class(name, class))
                }
            },
            TokenKind::NewLine => Ok(DeclarationParseAction::Nothing),
            _ => Err(LangError::new_parser_unexpected_token(&token)),
        }
    }

    fn parse_class_declaration(tokens: &mut Tokens, module: ModuleUID, name: &String, kind: ClassKind) -> Result<ClassType, LangError> {
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        loop {
            let token = match tokens.pop() {
                Some(token) => token,
                None => break,
            };

            match token.kind {
                TokenKind::Variable => {
                    let (name, type_) = Self::parse_variable_definition(tokens)?;

                    fields.push((name, type_));
                },
                TokenKind::Function => {
                    let (name, func_type) = Self::parse_function_declaration(tokens)?;

                    methods.push((
                        name,
                        func_type,
                    ));
                },
                TokenKind::NewLine => (),
                TokenKind::Dedent => break,
                _ => return Err(LangError::parser(&token, ParserErrorKind::UnexpectedToken))
            }
        }

        Ok(ClassType {
            name: name.clone(),
            module,
            kind,
            fields,
            methods,
        })
    }

    fn parse_class_definition(tokens: &mut Tokens, module: ModuleUID, name: &String, kind: ClassKind) -> Result<ParsableClass, LangError> {
        let mut fields = Vec::new();
        let mut functions = Vec::new();
        let mut function_types = Vec::new();

        loop {
            let token = match tokens.pop() {
                Some(token) => token,
                None => break,
            };

            match token.kind {
                TokenKind::Variable => {
                    let (name, type_) = Self::parse_variable_definition(tokens)?;

                    fields.push((name, type_));
                },
                TokenKind::Function => {
                    let (name, decl) = Self::parse_function(tokens)?;

                    function_types.push((
                        name.clone(),
                        decl.func_type.clone(),
                    ));

                    functions.push((
                        name,
                        decl,
                    ));
                },
                TokenKind::NewLine => (),
                TokenKind::Dedent => break,
                _ => return Err(LangError::parser(&token, ParserErrorKind::UnexpectedToken))
            }
        }

        let class_type = Arc::new(ClassType {
            name: name.clone(),
            module,
            kind,
            fields,
            methods: function_types,
        });

        Ok(ParsableClass {
            class_type,
            functions,
        })
    }

    fn parse_variable_definition(tokens: &mut Tokens) -> Result<(String, TypeKind), LangError> {
        let token = tokens.pop_err()?;

        // <name>
        let name = match token.kind {
            TokenKind::Symbol(name) => name,
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };

        // (type)
        let type_kind = parse_type_error(tokens)?;

        Ok((name, type_kind))
    }

    fn parse_variable(tokens: &mut Tokens) -> Result<(String, VariableDeclaration), LangError> {
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

        Ok((
            name,
            VariableDeclaration {
                type_kind,
                body,
            },
        ))
    }

    fn parse_function_declaration(tokens: &mut Tokens) -> Result<(String, FunctionType), LangError> {
        let token = tokens.pop_err()?;

        // <name>
        let name = match token.kind {
            TokenKind::Symbol(name) => name,
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };

        // (
        expect_token!(tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

        // (<param_name> (type))*)
        let (_, param_types) = parse_parameter_names(tokens)?;

        // (type)
        let ret_type = parse_type_option(tokens).unwrap_or(TypeKind::Nothing);

        let func_type = FunctionType(param_types, Box::new(ret_type));

        Ok((name, func_type))
    }

    fn parse_function(tokens: &mut Tokens) -> Result<(String, FunctionDeclaration), LangError> {
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
        let ret_type = parse_type_option(tokens).unwrap_or(TypeKind::Nothing);
        let func_type = FunctionType(param_types, Box::new(ret_type));

        expect_indent!(tokens);

        // {body}
        let body = tokens.snapshot();
        Self::pop_until_dedent(tokens);

        Ok((
            name,
            FunctionDeclaration {
                params: param_names,
                func_type,
                body,
            }
        ))
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
    Variable(String, VariableDeclaration),
    Function(String, FunctionDeclaration),
    Class(String, ParsableClass),
    FunctionDeclaration(String, FunctionType),
    ClassDeclaration(ClassType),
    Nothing,
}
