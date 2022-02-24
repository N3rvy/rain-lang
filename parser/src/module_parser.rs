use std::{collections::HashMap, sync::Arc};
use common::{ast::{ASTNode, types::{OperatorKind, ParenthesisKind, ParenthesisState, TypeKind, Function, FunctionType}, NodeKind}, errors::LangError};
use tokenizer::{iterator::{Tokens, TokenSnapshot}, tokens::Token};
use crate::{errors::{ParsingErrorHelper, UNEXPECTED_ERROR}, expect_token, utils::{parse_type_error, parse_parameter_names}, parser::ParserScope, expect_indent};

pub enum DeclarationKind {
    Variable(TypeKind),
    Function(Vec<String>, FunctionType),
}

pub struct Declaration {
    kind: DeclarationKind,
    body: TokenSnapshot,
}

pub struct ModuleParser {
    tokens: Tokens,
    externals: Vec<(String, TypeKind)>,
    declarations: HashMap<String, Declaration>,
}

impl ModuleParser {

    pub fn from_tokens(tokens: Tokens) -> Result<Self, LangError> {
        let mut parser = Self {
            tokens,
            externals: Vec::new(),
            declarations: HashMap::new(),
        };

        loop {
            if !parser.tokens.has_next() {
                break
            }

            parser.parse_definition()?;
        }

        Ok(parser)
    }

    pub fn with_externals(mut self, externals: &Vec<(String, TypeKind)>) -> Self {
        self.externals
            .extend(externals
                .iter()
                .map(|(name, type_kind)| {
                    (name.clone(), type_kind.clone())
                }));

        self
    }

    pub fn build(mut self) -> Result<ASTNode, LangError> {
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
                    let value = Self::parse_variable_value(&mut self.tokens, scope.new_child())?;

                    variables.push((name, value));
                },
                DeclarationKind::Function(params, func_type) => {
                    let value = Self::parse_function_value(
                        &mut self.tokens,
                        scope.new_child(),
                        params,
                        func_type)?;

                    functions.push((name, value));
                },
            };
        }

        Ok(ASTNode::new(
            NodeKind::new_module(
                functions,
                variables
            ),
            TypeKind::Nothing,
        ))
    }

    fn parse_variable_value(tokens: &mut Tokens, scope: ParserScope) -> Result<ASTNode, LangError> {
        scope.parse_statement(tokens)
    }

    fn parse_function_value(tokens: &mut Tokens, scope: ParserScope, params: Vec<String>, func_type: FunctionType) -> Result<Arc<Function>, LangError> {
        if params.len() != func_type.0.len() {
            return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string()));
        }
        
        for i in 0..params.len() {
            scope.declare(params[i].clone(), func_type.0[i].clone());
        }

        let body = scope.parse_body(tokens)?;

        Ok(Function::new(body, params))
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
                // let path = match tokens.pop() {
                //     Some(Token::Literal(LiteralKind::String(path))) => path,
                //     Some(_) => return Err(LangError::new_parser_unexpected_token()),
                //     None => return Err(LangError::new_parser_end_of_file()),
                // };

                // new line
                // expect_token!(tokens.pop(), Token::NewLine);

                todo!("Imports not yet implemented");
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
                Self::pop_untill_newline(&mut self.tokens);

                self.declarations.insert(
                    name,
                    Declaration {
                        kind: DeclarationKind::Variable(type_kind),
                        body,
                    },
                );

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

                // (<parem_name>: (type))*)
                let (param_names, param_types) = parse_parameter_names(&mut self.tokens)?;

                // : (type)
                let ret_type = parse_type_error(&mut self.tokens)?;

                expect_indent!(self.tokens);

                // {body}
                let body = self.tokens.snapshot();
                Self::pop_untill_dedent(&mut self.tokens);

                let func_type = FunctionType(param_types, Box::new(ret_type));

                self.declarations.insert(
                    name,
                    Declaration {
                        kind: DeclarationKind::Function(param_names, func_type),
                        body,
                    }
                );

                Ok(())
            },
            Token::NewLine => self.parse_definition(),
            _ => Err(LangError::new_parser_unexpected_token()),
        }
    }

    fn pop_untill_dedent(tokens: &mut Tokens) {
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

    fn pop_untill_newline(tokens: &mut Tokens) {
        loop {
            match tokens.pop() {
                Some(Token::NewLine) | None => break,
                Some(_) => (),
            }
        }
    }
}