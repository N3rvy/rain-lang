use std::{collections::HashMap, cell::RefCell};
use common::{ast::{ASTNode, NodeKind, types::{TypeKind, ParenthesisKind, ParenthesisState, LiteralKind, Function, OperatorKind, ReturnKind}}, errors::LangError};
use tokenizer::tokens::Token;
use crate::{expect_token, errors::{ParsingErrorHelper, VAR_NOT_FOUND, INVALID_FIELD_ACCESS, FIELD_DOESNT_EXIST, INVALID_ASSIGN, NOT_A_FUNCTION, INVALID_ARGS_COUNT, INVALID_ARGS}};

pub fn parse(mut tokens: Vec<Token>) -> Result<ASTNode, LangError> {
    // Reversing the vector for using it as a stack
    tokens.reverse();
    
    let mut body = Vec::new(); 
    let scope = ParserScope::new_root();
    
    loop {
        if tokens.is_empty() { break }

        body.push(scope.parse_statement(&mut tokens)?); 
    }
    
    Ok(ASTNode::new(NodeKind::new_root(body), TypeKind::Unknown))
}

pub struct ParserScope<'a> {
    parent: Option<&'a ParserScope<'a>>,
    types: RefCell<HashMap<String, TypeKind>>,
    eval_type: RefCell<TypeKind>,
}

impl<'a> ParserScope<'a> {
    pub fn new_root() -> Self {
        Self {
            parent: None,
            types: RefCell::new(HashMap::new()),
            eval_type: RefCell::new(TypeKind::Nothing),
        }
    }
    
    pub fn new_child(&'a self) -> Self {
        Self {
            parent: Some(self),
            types: RefCell::new(HashMap::new()),
            eval_type: RefCell::new(TypeKind::Nothing),
        }
    }
    
    pub fn get(&self, name: &String) -> Option<TypeKind> {
        match self.types.borrow().get(name) {
            Some(t) => Some(t.clone()),
            None => match self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }
    
    pub fn declare(&self, name: String, type_kind: TypeKind) {
        self.types.borrow_mut().insert(name, type_kind);
    }

    pub fn parse_statement(&self, tokens: &mut Vec<Token>) -> Result<ASTNode, LangError> {
        let token = tokens.pop();
        if let None = token {
            return Err(LangError::new_parser_end_of_file());
        }
        
        let token = token.unwrap();
        
        let result = match &token {
            Token::Function => {
                let next= tokens.pop();
                
                // "name" | (
                match next {
                    Some(Token::Symbol(name)) => {
                        // (
                        expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));
                        
                        // ...)
                        let (param_names, param_types) = self.parse_parameter_names(tokens)?;

                        // return type?
                        let ret_type = self.parse_type_error(tokens)?;
                        
                        // {
                        expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));

                        // creating the child scope
                        let body_scope = self.new_child();
                        // declaring the argument types
                        for i in 0..param_names.len() {
                            body_scope.declare(param_names[i].clone(), param_types[i].clone());
                        }
                        // ...}
                        let body = body_scope.parse_body(tokens)?;

                        ASTNode::new(
                            NodeKind::new_function_decl(
                                name,
                                Function::new(body, param_names)
                            ),
                            TypeKind::Function(param_types, Box::new(ret_type)),
                        )
                    },
                    Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open)) => {
                        // ...)
                        let (param_names, param_types) = self.parse_parameter_names(tokens)?;

                        // return type?
                        let ret_type = self.parse_type_error(tokens)?;
                        
                        // {
                        expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));

                        // creating the child scope
                        let body_scope = self.new_child();
                        // declaring the argument types
                        for i in 0..param_names.len() {
                            body_scope.declare(param_names[i].clone(), param_types[i].clone());
                        }
                        // ...}
                        let body = body_scope.parse_body(tokens)?;
                        
                        ASTNode::new(
                            NodeKind::new_literal(
                                LiteralKind::Function(Function::new(body, param_names))
                            ),
                            TypeKind::Function(param_types, Box::new(ret_type))
                        )
                    },
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                }
            },
            Token::Variable => {
                // name
                let name = tokens.pop();
                
                let name = match name {
                    Some(Token::Symbol(name)) => name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };

                // ?(: type)
                let assign_type = self.parse_type_option(tokens)?;

                // =
                expect_token!(tokens.pop(), Token::Operator(OperatorKind::Assign));

                // value
                let value = self.parse_statement(tokens)?;
                
                let eval_type = match assign_type {
                    Some(type_kind) => {
                        if !type_kind.is_compatible(&value.eval_type) {
                            return Err(LangError::new_parser(INVALID_ASSIGN.to_string()))
                        }
                        type_kind
                    },
                    None => value.eval_type.clone(),
                };

                ASTNode::new(NodeKind::new_variable_decl(name, value), eval_type)
            },
            Token::Operator(_) | Token::BoolOperator(_) | Token::MathOperator(_) => return Err(LangError::new_parser_unexpected_token()),
            Token::Symbol(name) => {
                let var_ref = NodeKind::new_variable_ref(name.clone());
                let var_type = match self.get(name) {
                    Some(t) => t,
                    None => return Err(LangError::new_parser(VAR_NOT_FOUND.to_string())),
                };

                ASTNode::new(var_ref, var_type)
            }
            Token::Literal(value) => ASTNode::new(NodeKind::new_literal(value.clone()), value.clone().into()),
            Token::Parenthesis(kind, state) => {
                match (kind, state) {
                    (ParenthesisKind::Round, ParenthesisState::Open) => {
                        let result = self.parse_statement(tokens);
                        
                        match tokens.pop() {
                            Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close)) => (),
                            Some(_) => return Err(LangError::new_parser_unexpected_token()),
                            None => return Err(LangError::new_parser_end_of_file()),
                        }
                        
                        result?
                    },
                    (ParenthesisKind::Square, ParenthesisState::Open) => {
                        let values = self.parse_parameter_values(tokens, ParenthesisKind::Square)?;
                        
                        ASTNode::new(NodeKind::new_vector_literal(values), TypeKind::Vector)
                    },
                    (ParenthesisKind::Curly, ParenthesisState::Open) => {
                        let values = self.parse_object_values(tokens)?;
                        
                        ASTNode::new(NodeKind::new_object_literal(values), TypeKind::Object(HashMap::new()))
                    },
                    _ => return Err(LangError::new_parser_unexpected_token())
                }
            },
            Token::Return | Token::Break => {
                let value = match tokens.last() {
                    Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close)) => {
                        None
                    },
                    Some(_) => {
                        Some(self.parse_statement(tokens)?)
                    },
                    None => return Err(LangError::new_parser_end_of_file()),
                };
                
                let kind = match &token {
                    Token::Return => ReturnKind::Return,
                    Token::Break => ReturnKind::Break,
                    _ => panic!("Like WTF"),
                };
                
                let value_type = match &value {
                    Some(node) => node.eval_type.clone(),
                    None => TypeKind::Nothing,
                };
                self.eval_type.replace(value_type);

                ASTNode::new(NodeKind::new_return_statement(value, kind), TypeKind::Nothing)
            },
            Token::If => {
                // condition
                let condition = self.parse_statement(tokens)?;
                // {
                expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
                // ...}
                let body = self.new_child().parse_body(tokens)?;
                
                ASTNode::new(NodeKind::new_if_statement(condition, body), TypeKind::Nothing)
            },
            Token::For => {
                // iter name
                let iter_name = match tokens.pop() {
                    Some(Token::Symbol(name)) => name,
                    _ => return Err(LangError::new_parser_unexpected_token()),
                };
                
                // in
                expect_token!(tokens.pop(), Token::Operator(OperatorKind::In));
                
                // min value
                let min = self.parse_statement(tokens)?;
                
                // ..
                expect_token!(tokens.pop(), Token::Operator(OperatorKind::Range));
                
                // max value
                let max = self.parse_statement(tokens)?;
                
                // {
                expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
                
                // ...}
                let body = self.new_child().parse_body(tokens)?;
                
                ASTNode::new(NodeKind::new_for_statement(min, max, body, iter_name), TypeKind::Nothing)
            },
            Token::While => {
                // condition 
                let condition = self.parse_statement(tokens)?;
                // {
                expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
                // ...}
                let body = self.new_child().parse_body(tokens)?;
                
                ASTNode::new(NodeKind::new_while_statement(condition, body), TypeKind::Nothing)
            },
            Token::Import => {
                // identifier
                let identifier = match tokens.pop() {
                    Some(Token::Literal(LiteralKind::String(ident))) => ident,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };
                
                ASTNode::new(NodeKind::new_import(identifier), TypeKind::Nothing)
            },
            Token::Type(_) => return Err(LangError::new_parser_unexpected_token()),
        };
        

        let mut node = result;
        
        Ok(loop {
            let res = self.parse_infix(node, tokens)?; 
            if res.1 {
                node = res.0;
            } else {
                break res.0;
            }
        })
    }

    /// The bool in the tuple is a bool representing whether the infix was valid or not
    pub fn parse_infix(&self, node: ASTNode, tokens: &mut Vec<Token>) -> Result<(ASTNode, bool), LangError> {

        // Getting the infix and returning if it's None
        let infix = tokens.last().cloned();
        if matches!(infix, None) { return Ok((node, false)) }
        
        let infix = infix.unwrap();

        match infix {
            Token::MathOperator(operator) => {
                tokens.pop();
                let right = self.parse_statement(tokens);
                
                match right {
                    Ok(right) => Ok((
                            ASTNode::new(
                                NodeKind::new_math_operation(operator.clone(), node, right),
                                TypeKind::Int), // TODO: Calculate type from values
                            true)),
                    Err(err) => Err(err),
                }
            },
            Token::BoolOperator(operator) => {
                tokens.pop();
                let right = self.parse_statement(tokens);
                
                match right {
                    Ok(right) => Ok((
                        ASTNode::new(
                            NodeKind::new_bool_operation(operator.clone(), node, right),
                            TypeKind::Bool),
                        true)),
                    Err(err) => Err(err),
                }

            },
            Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open) => {
                tokens.pop();
                
                let value = self.parse_statement(tokens)?;
                
                expect_token!(tokens.pop(), Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close));
                
                Ok((
                    ASTNode::new(
                        NodeKind::new_value_field_access(node, value),
                        TypeKind::Unknown), // TODO
                    true)) 
            },
            Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open) => {
                tokens.pop();

                let parameters = self.parse_parameter_values(tokens, ParenthesisKind::Round)?;
                
                // check that node is function
                let (arg_types, ret_type) = match &node.eval_type {
                    TypeKind::Function(arg_types, ret_value) => (arg_types, ret_value),
                    _ => return Err(LangError::new_parser(NOT_A_FUNCTION.to_string())),
                };
                
                // Check parameters types
                if parameters.len() != arg_types.len() {
                    return Err(LangError::new_parser(INVALID_ARGS_COUNT.to_string()))
                }
                
                for i in 0..parameters.len() {
                    if parameters[i].eval_type.is_compatible(&arg_types[i]) {
                        return Err(LangError::new_parser(INVALID_ARGS.to_string()))
                    }
                }
                
                let ret_type = ret_type.as_ref().clone();

                Ok((
                    ASTNode::new(
                        NodeKind::new_function_invok(node, parameters),
                        ret_type),
                    true
                ))
            },
            Token::Operator(OperatorKind::Dot) => {
                tokens.pop();

                let field_name = match tokens.pop() {
                    Some(Token::Symbol(field_name)) => field_name,
                    Some(_) => return Err(LangError::new_parser_unexpected_token()),
                    None => return Err(LangError::new_parser_end_of_file()),
                };
                
                match &node.eval_type {
                    TypeKind::Object(field_types) => {
                        let field_type = match field_types.get(&field_name) {
                            Some(t) => t.clone(),
                            None => return Err(LangError::new_parser(FIELD_DOESNT_EXIST.to_string())),
                        };

                        Ok((
                            ASTNode::new(
                                NodeKind::new_field_access(node, field_name),
                                field_type),
                            true))
                    },
                    _ => return Err(LangError::new_parser(INVALID_FIELD_ACCESS.to_string())),
                }
            },
            Token::Operator(OperatorKind::Assign) => {
                let name = match node.kind.as_ref() {
                    NodeKind::VaraibleRef { name } => name.to_string(),
                    _ => return Ok((node, false)),
                };

                tokens.pop();

                let value = self.parse_statement(tokens)?;

                Ok((
                    ASTNode::new(
                        NodeKind::new_variable_asgn(name, value),
                        TypeKind::Nothing),
                    true))
            },
            
            _ => Ok((node, false)),
        }
    }
}