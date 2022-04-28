use std::cell::RefCell;
use common::errors::ParserErrorKind;
use common::tokens::{TokenKind, Token};
use common::{ast::{ASTNode, NodeKind, types::{TypeKind, ParenthesisKind, ParenthesisState, Function, OperatorKind, ReturnKind, FunctionType, LiteralKind}}, errors::LangError, constants::SCOPE_SIZE};
use smallvec::SmallVec;
use common::constants::CLASS_CONSTRUCTOR_NAME;
use common::module::ModuleUID;
use tokenizer::iterator::Tokens;
use crate::utils::{parse_type_option, TokensExtensions};
use crate::{expect_token, errors::ParsingErrorHelper, expect_indent, utils::parse_parameter_names};
use crate::parser_module_scope::{ParserModuleScope, ScopeGetResult};

pub enum ScopeParent<'a> {
    Module(&'a ParserModuleScope),
    Scope(&'a ParserScope<'a>),
}

/// This is the scope used for parsing code (this only handles codes inside definitions)
pub struct ParserScope<'a> {
    parent: ScopeParent<'a>,
    pub eval_type: RefCell<TypeKind>,
    module_uid: ModuleUID,
    
    names: RefCell<SmallVec<[String; SCOPE_SIZE]>>,
    types: RefCell<SmallVec<[TypeKind; SCOPE_SIZE]>>,
}

impl<'a> ParserScope<'a> {
    pub fn new_module_child(module: &'a ParserModuleScope) -> Self {
        Self {
            parent: ScopeParent::Module(module),
            eval_type: RefCell::new(TypeKind::Nothing),
            module_uid: module.uid,

            names: RefCell::new(SmallVec::new()),
            types: RefCell::new(SmallVec::new()),
        }
    }
    
    pub fn new_child(&'a self) -> Self {
        Self {
            parent: ScopeParent::Scope(self),
            eval_type: RefCell::new(TypeKind::Nothing),
            module_uid: self.module_uid,

            names: RefCell::new(SmallVec::new()),
            types: RefCell::new(SmallVec::new()),
        }
    }
    
    pub fn get(&self, name: &String) -> ScopeGetResult {
        let types = self.types.borrow();
        
        let value = self.names.borrow()
            .iter()
            .rev()
            .enumerate()
            .find(|(_, value)| (**value).eq(name))
            .and_then(|(i, _)| Some(types[types.len() - 1 - i].clone()));

        match value {
            Some(value) => ScopeGetResult::Ref(self.module_uid, value),
            None => match self.parent {
                ScopeParent::Module(module) => module.get(name),
                ScopeParent::Scope(scope) => scope.get(name),
            },
        }
    }
    
    pub fn declare(&self, name: String, type_kind: TypeKind) {
        self.names.borrow_mut().push(name);
        self.types.borrow_mut().push(type_kind);
    }

    pub fn parse_statement(&self, tokens: &mut Tokens) -> Result<ASTNode, LangError> {
        let token = tokens.pop();
        if let None = token {
            return Err(LangError::new_parser_end_of_file());
        }
        
        let token = token.unwrap();
        
        let result = match &token.kind {
            TokenKind::Function => {
                let next= tokens.pop_err()?;

                let name = match next.kind {
                    TokenKind::Symbol(name) => {
                        expect_token!(tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));
                        Some(name)
                    },
                    TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open) => None,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };

                // ...)
                let (param_names, param_types) = parse_parameter_names(tokens)?;

                // return type?
                let ret_type = parse_type_option(tokens).unwrap_or(TypeKind::Nothing);
                
                // Indentation
                expect_indent!(tokens);

                // creating the child scope
                let body_scope = self.new_child();
                // declaring the argument types
                for i in 0..param_names.len() {
                    body_scope.declare(param_names[i].clone(), param_types[i].clone());
                }

                // ...}
                let body = body_scope.parse_body(tokens)?;

                if !body_scope.eval_type.borrow().is_compatible(&ret_type) {
                    return Err(LangError::wrong_type(&token, &body_scope.eval_type.borrow(), &ret_type));
                }
                
                let eval_type = TypeKind::Function(FunctionType(param_types.clone(), Box::new(ret_type)));

                let func_literal = ASTNode::new(
                    NodeKind::new_function_literal(
                        Function::new(body, param_names, false)),
                    eval_type.clone(),
                );

                match name {
                    Some(name) => {
                        self.declare(name.clone(), eval_type);

                        ASTNode::new(
                            NodeKind::new_variable_decl(
                                name,
                                func_literal,
                            ),
                            TypeKind::Nothing
                        )
                    },
                    None => func_literal,
                }
            },
            TokenKind::Variable => {
                // name
                let name = tokens.pop_err()?;
                
                let name = match name.kind {
                    TokenKind::Symbol(name) => name,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };

                // ?(type)
                let assign_type = parse_type_option(tokens);

                // =
                expect_token!(tokens.pop(), TokenKind::Operator(OperatorKind::Assign));

                // value
                let value = self.parse_statement(tokens)?;
                
                let eval_type = match assign_type {
                    Some(type_kind) => {
                        if !type_kind.is_compatible(&value.eval_type) {
                            return Err(LangError::wrong_type(&token, &type_kind, &value.eval_type))
                        }
                        type_kind
                    },
                    None => value.eval_type.clone(),
                };
                    
                self.declare(name.clone(), eval_type.clone());

                ASTNode::new(NodeKind::new_variable_decl(name, value), eval_type)
            },
            TokenKind::Symbol(name) => {
                match self.get(name) {
                    ScopeGetResult::Class(_, class_type) => {
                        expect_token!(tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open));

                        let parameters = self.parse_parameter_values(tokens)?;

                        // TODO: Make this a bit better
                        let constructor = class_type.methods
                            .iter()
                            .find(|(name, _)| name == CLASS_CONSTRUCTOR_NAME)
                            .cloned();

                        match constructor {
                            Some((_, constructor)) => {
                                // Check parameters types
                                if parameters.len() != constructor.0.len() {
                                    return Err(LangError::parser(&token, ParserErrorKind::InvalidArgCount(constructor.0.len())))
                                }

                                for i in 0..parameters.len() {
                                    if !parameters[i].eval_type.is_compatible(&constructor.0[i]) {
                                        return Err(LangError::wrong_type(&token, &constructor.0[i], &parameters[i].eval_type))
                                    }
                                }
                            }
                            None => {
                                if parameters.len() != 0 {
                                    return Err(LangError::parser(&token, ParserErrorKind::InvalidArgCount(0)))
                                }
                            }
                        }

                        ASTNode::new(
                            NodeKind::new_construct_class(parameters, class_type.clone()),
                            TypeKind::Object(class_type.clone()))
                    },
                    ScopeGetResult::Ref(uid, type_) => {
                        let var_ref = NodeKind::new_variable_ref(uid, name.clone());
                        ASTNode::new(var_ref, type_)
                    },
                    ScopeGetResult::None => return Err(LangError::parser(&token, ParserErrorKind::VarNotFound)),
                }
            }
            TokenKind::Literal(value) => ASTNode::new(NodeKind::new_literal(value.clone()), value.clone().into()),
            TokenKind::Parenthesis(kind, state) => {
                match (kind, state) {
                    (ParenthesisKind::Round, ParenthesisState::Open) => {
                        let result = self.parse_statement(tokens);
                        
                        match tokens.pop_err()?.kind {
                            TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close) => (),
                            _ => return Err(LangError::new_parser_unexpected_token(&token)),
                        }
                        
                        result?
                    },
                    (ParenthesisKind::Square, ParenthesisState::Open) => {
                        let (vector_type, values) = self.parse_vector_values(tokens)?;
                        
                        ASTNode::new(
                            NodeKind::new_vector_literal(values),
                            TypeKind::Vector(Box::new(vector_type))
                        )
                    },
                    _ => return Err(LangError::new_parser_unexpected_token(&token))
                }
            },
            TokenKind::Return | TokenKind::Break => {
                let value = match tokens.peek() {
                    Some(Token { kind: TokenKind::NewLine, start: _, end: _ }) | None => {
                        None
                    },
                    Some(_) => {
                        Some(self.parse_statement(tokens)?)
                    },
                };
                
                let kind = match &token.kind {
                    TokenKind::Return => ReturnKind::Return,
                    TokenKind::Break => ReturnKind::Break,
                    _ => panic!("Like WTF"),
                };
                
                let value_type = match &value {
                    Some(node) => node.eval_type.clone(),
                    None => TypeKind::Nothing,
                };
                self.eval_type.replace(value_type);

                ASTNode::new(NodeKind::new_return_statement(value, kind), TypeKind::Nothing)
            },
            TokenKind::If => {
                // condition
                let condition = self.parse_statement(tokens)?;
                // Indent
                expect_indent!(tokens);
                // ...}
                let body = self.new_child().parse_body(tokens)?;
                
                ASTNode::new(NodeKind::new_if_statement(condition, body), TypeKind::Nothing)
            },
            TokenKind::For => {
                // iter name
                let iter_name = match tokens.pop_err()?.kind {
                    TokenKind::Symbol(name) => name,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };
                
                // in
                expect_token!(tokens.pop(), TokenKind::Operator(OperatorKind::In));
                
                // min value
                let min = self.parse_statement(tokens)?;
                
                // ..
                expect_token!(tokens.pop(), TokenKind::Operator(OperatorKind::Range));
                
                // max value
                let max = self.parse_statement(tokens)?;
                
                // {
                expect_indent!(tokens);
                
                // ...}
                let for_scope = self.new_child();
                for_scope.declare(iter_name.clone(), TypeKind::Int);
                let body = for_scope.parse_body(tokens)?;
                
                ASTNode::new(NodeKind::new_for_statement(min, max, body, iter_name), TypeKind::Nothing)
            },
            TokenKind::While => {
                // condition 
                let condition = self.parse_statement(tokens)?;
                // {
                expect_indent!(tokens);
                // ...}
                let body = self.new_child().parse_body(tokens)?;
                
                ASTNode::new(NodeKind::new_while_statement(condition, body), TypeKind::Nothing)
            },
            TokenKind::Type(TypeKind::Nothing) => ASTNode::new(NodeKind::new_literal(LiteralKind::Nothing), TypeKind::Nothing),
            TokenKind::NewLine => self.parse_statement(tokens)?,
            TokenKind::Operator(_) |
            TokenKind::BoolOperator(_) |
            TokenKind::MathOperator(_) |
            TokenKind::Type(_) |
            TokenKind::Indent |
            TokenKind::Import |
            TokenKind::Class |
            TokenKind::Data |
            TokenKind::Dedent => return Err(LangError::new_parser_unexpected_token(&token)),
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
    pub fn parse_infix(&self, node: ASTNode, tokens: &mut Tokens) -> Result<(ASTNode, bool), LangError> {

        // Getting the infix and returning if it's None
        let infix = tokens.peek();
        if matches!(infix, None) { return Ok((node, false)) }
        
        let infix = infix.unwrap();

        match infix.kind {
            TokenKind::MathOperator(operator) => {
                tokens.pop();
                let right = self.parse_statement(tokens)?;
                
                let eval_type = Self::predict_math_result(operator.clone(), &node.eval_type, &right.eval_type);
                
                Ok((
                    ASTNode::new(
                        NodeKind::new_math_operation(operator.clone(), node, right),
                        eval_type
                    ),
                    true
                ))
            },
            TokenKind::BoolOperator(operator) => {
                tokens.pop();
                let right = self.parse_statement(tokens)?;
                
                Ok((
                    ASTNode::new(
                        NodeKind::new_bool_operation(operator.clone(), node, right),
                        TypeKind::Bool
                    ),
                    true
                ))
            },
            TokenKind::Parenthesis(ParenthesisKind::Square, ParenthesisState::Open) => {
                let token = tokens.pop().unwrap();
                
                let value = self.parse_statement(tokens)?;
                
                expect_token!(tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close));
                
                let vec_type = match &node.eval_type {
                    TypeKind::Vector(vt) => (**vt).clone(),
                    _ => return Err(LangError::parser(&token, ParserErrorKind::NotIndexable)),
                };
                
                Ok((
                    ASTNode::new(
                        NodeKind::new_value_field_access(node, value),
                        vec_type),
                    true)) 
            },
            TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Open) => {
                let token = tokens.pop().unwrap();

                let parameters = self.parse_parameter_values(tokens)?;

                // check that node is function
                let (arg_types, ret_type) = match &node.eval_type {
                    TypeKind::Function(FunctionType(arg_types, ret_value)) => (arg_types, ret_value),
                    _ => return Err(LangError::parser(&token, ParserErrorKind::NotCallable)),
                };
                
                // Check parameters types
                if parameters.len() != arg_types.len() {
                    return Err(LangError::parser(&token, ParserErrorKind::InvalidArgCount(arg_types.len())))
                }
                
                for i in 0..parameters.len() {
                    if !parameters[i].eval_type.is_compatible(&arg_types[i]) {
                        return Err(LangError::wrong_type(&token, &arg_types[i], &parameters[i].eval_type))
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
            TokenKind::Operator(OperatorKind::Dot) => {
                tokens.pop();

                let token = tokens.pop_err()?;

                let field_name = match &token.kind {
                    TokenKind::Symbol(field_name) => field_name,
                    _ => return Err(LangError::new_parser_unexpected_token(&token)),
                };
                
                match &node.eval_type {
                    TypeKind::Object(class_type) => {
                        let field_type = match class_type.fields.iter().find(|(name, _)| name == field_name) {
                            Some((_, t)) => t.clone(),
                            None => {
                                // If the field doesn't exist search for a method
                                match class_type.methods.iter().find(|(name, _)| name == field_name) {
                                    Some((_, ft)) => TypeKind::Function(ft.clone()),
                                    None => return Err(LangError::parser(&token, ParserErrorKind::FieldDoesntExist)),
                                }
                            }
                        };

                        Ok((
                            ASTNode::new(
                                NodeKind::new_field_access(node, field_name.clone()),
                                field_type),
                            true))
                    }
                    _ => return Err(LangError::parser(&token, ParserErrorKind::InvalidFieldAccess)),
                }
            },
            TokenKind::Operator(OperatorKind::Assign) => {
                tokens.pop();

                let value = self.parse_statement(tokens)?;

                match *node.kind {
                    NodeKind::VariableRef { module: _, name } => {
                        Ok((
                            ASTNode::new(
                                NodeKind::new_variable_asgn(name, value),
                                TypeKind::Nothing),
                            true))
                    },
                    NodeKind::FieldAccess { variable, field_name } => {
                        Ok((
                            ASTNode::new(
                                NodeKind::new_field_asgn(variable, field_name, value),
                                TypeKind::Nothing),
                            true))
                    },
                    _ => Ok((node, false)),
                }
            },
            
            _ => Ok((node, false)),
        }
    }
}