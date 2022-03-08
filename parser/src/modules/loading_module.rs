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
    kind: DeclarationKind,
    body: TokenSnapshot,
}

pub struct LoadingModule {
    tokens: Tokens,
    imports: Vec<String>,
    declarations: Vec<(String, Declaration)>
}

impl LoadingModule {
    pub fn from_tokens(tokens: Tokens) -> Result<Self, LangError> {
        let mut parser = Self {
            tokens,
            imports: Vec::new(),
            declarations: Vec::new(),
        };

        loop {
            if !parser.tokens.has_next() {
                break
            }

            parser.parse_definition()?;
        }

        Ok(parser)
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

    pub fn declarations(&self) -> &Vec<(String, Declaration)> {
        &self.declarations
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

    pub fn build(mut self) -> Result<ASTModule, LangError> {
        let scope = ParserScope::new_root();

        // Declaring every type into the scope
        for (name, def) in &self.declarations {
            let type_kind = match &def.kind {
                DeclarationKind::Variable(t) => t.clone(),
                DeclarationKind::Function(_, ft) => TypeKind::Function(ft.clone()),
            };

            scope.declare(name.clone(), type_kind);
        }

        let mut functions = Vec::new();
        let mut variables = Vec::new();

        // Parsing every definition
        for (name, decl) in self.declarations{
            self.tokens.rollback(decl.body);

            match decl.kind {
                DeclarationKind::Variable(_) => {
                    let value = Self::parse_variable_value(&mut self.tokens, &scope.new_child())?;

                    variables.push((name, value));
                },
                DeclarationKind::Function(params, func_type) => {
                    let scope = scope.new_child();

                    let value = Self::parse_function_value(
                        &mut self.tokens,
                        &scope,
                        params,
                        func_type.clone())?;

                    if !scope.eval_type.borrow().is_compatible(func_type.1.as_ref()) {
                        return Err(LangError::new_parser(WRONG_TYPE.to_string()));
                    }

                    functions.push((name, value));
                },
            };
        }

        Ok(ASTModule::new(
            functions,
            variables,
        ))
    }

    fn parse_variable_value(tokens: &mut Tokens, scope: &ParserScope) -> Result<ASTNode, LangError> {
        scope.parse_statement(tokens)
    }

    fn parse_function_value(tokens: &mut Tokens, scope: &ParserScope, params: Vec<String>, func_type: FunctionType) -> Result<Arc<Function>, LangError> {
        if params.len() != func_type.0.len() {
            return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string()));
        }

        for i in 0..params.len() {
            scope.declare(params[i].clone(), func_type.0[i].clone());
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body, params))
    }
}