use std::{collections::HashMap, cell::RefCell};

use common::{ast::{ASTNode, NodeKind, types::{TypeKind, LiteralKind, MathOperatorKind, ReturnKind}}, errors::LangError, messages::{UNEXPECTED_ERROR, INCORRECT_NUMBER_OF_PARAMETERS, INCORRECT_FUNCTION_PARAMETER_TYPE}};


macro_rules! assert_compatible_type {
    ($a:expr, $b:expr) => {
        if !$a.is_compatible($b) {
            return Err(LangError::new_parser_wrong_type());
        }
    };
}

pub fn check_types(ast: &ASTNode) -> Result<(), LangError> {
    check_node(ast, &TypeScope::new_root())?;
    Ok(())
}

struct TypeScope<'a> {
    parent: Option<&'a TypeScope<'a>>,
    types: RefCell<HashMap<String, TypeKind>>,
    ret_type: TypeKind,
}

impl<'a> TypeScope<'a> {
    fn new_root() -> Self {
        Self {
            parent: None,
            types: RefCell::new(HashMap::new()),
            ret_type: TypeKind::Unknown,
        }
    }
    
    fn new_child(&'a self) -> Self {
        Self {
            parent: Some(self),
            types: RefCell::new(HashMap::new()),
            ret_type: self.ret_type.clone(),
        }
    }
    
    fn new_child_with_ret(&'a self, ret_type: TypeKind) -> Self {
        Self {
            parent: Some(self),
            types: RefCell::new(HashMap::new()),
            ret_type,
        }
    }
    
    fn declare_type(&self, name: String, t: TypeKind) {
        self.types.borrow_mut().insert(name, t);
    }
    
    fn get_type(&self, name: &String) -> TypeKind {
        match self.types.borrow().get(name) {
            Some(val) => val.clone(),
            None => match self.parent {
                Some(parent) => parent.get_type(name),
                None => TypeKind::Unknown,
            },
        }
    }
}

fn check_node(node: &ASTNode, scope: &TypeScope) -> Result<TypeKind, LangError> {
    
    match node.kind.as_ref() {
        NodeKind::Root { body } => {
            for child in body {
                check_node(child, scope)?;
            }
            
            Ok(TypeKind::Nothing)
        },
        NodeKind::VariableDecl { name, value } => {
            let val_type = check_node(value, scope)?;
            assert_compatible_type!(val_type, &node.eval_type);
            scope.declare_type(name.clone(), val_type);
            
            Ok(TypeKind::Nothing)
        },
        NodeKind::VaraibleRef { name } => Ok(scope.get_type(name)),
        NodeKind::VariableAsgn { name, value } => {
            let val_type = check_node(value, scope)?;
            let var_type = scope.get_type(name);
            assert_compatible_type!(val_type, &var_type);
                
            Ok(TypeKind::Nothing)
        },
        NodeKind::FunctionInvok { variable, parameters } => {
            let var_type = check_node(variable, scope)?;
            check_function(&var_type, parameters, scope)
        },
        NodeKind::MethodInvok { object, name, parameters } => {
            let eval_type = match check_node(object, scope)? {
                TypeKind::Object(types) => {
                    match types.get(name) {
                        Some(t) => t.clone(),
                        None => TypeKind::Unknown,
                    }
                },
                _ => TypeKind::Unknown,
            };
            
            check_function(&eval_type, parameters, scope)
        },
        NodeKind::Literal { value } => {
            match (value, &node.eval_type) {
                (LiteralKind::Function(func), TypeKind::Function(func_types)) => {
                    let ret_type = match func_types.last() {
                        Some(t) => t,
                        None => return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string())),
                    };

                    let func_scope = scope.new_child_with_ret(ret_type.clone());

                    for child in &func.body {
                        check_node(child, &func_scope)?;
                    }
                },
                _ => ()
            }

            Ok(node.eval_type.clone())
        },
        NodeKind::MathOperation { operation, left, right } => {
            let left_type = check_node(left, scope)?;
            let right_type = check_node(right, scope)?;

            Ok(match operation {
                MathOperatorKind::Plus => {
                    match (left_type, right_type) {
                        (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                        (TypeKind::Float, TypeKind::Int) | (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                        _ => TypeKind::String,
                    }
                },
                MathOperatorKind::Minus => {
                    match (left_type, right_type) {
                        (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                        (TypeKind::Float, TypeKind::Int) | (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                        _ => TypeKind::Nothing,
                    }
                },
                MathOperatorKind::Multiply => {
                    match (left_type, right_type) {
                        (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                        (TypeKind::Float, TypeKind::Int) | (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                        _ => TypeKind::String,
                    }
                },
                MathOperatorKind::Divide => {
                    match (left_type, right_type) {
                        (TypeKind::Int, TypeKind::Int) => TypeKind::Float,
                        (TypeKind::Float, TypeKind::Int) | (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                        _ => TypeKind::Nothing,
                    }
                },
                MathOperatorKind::Modulus => {
                    match (left_type, right_type) {
                        (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                        (TypeKind::Float, TypeKind::Int) | (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                        _ => TypeKind::Nothing,
                    }
                },
                MathOperatorKind::Power => {
                    match (left_type, right_type) {
                        (TypeKind::Int, TypeKind::Int) | (TypeKind::Float, TypeKind::Int) | (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                        _ => TypeKind::Nothing,
                    }
                },
            })
        },
        NodeKind::BoolOperation { operation: _, left: _, right: _ } => Ok(TypeKind::Bool),
        NodeKind::ReturnStatement { value, kind } => {
            match kind {
                ReturnKind::Return => {
                    let val = match value {
                        Some(node) => check_node(node, scope)?,
                        None => TypeKind::Nothing,
                    };
                    
                    assert_compatible_type!(val, &scope.ret_type);
                },
                _ => ()
            }

            Ok(TypeKind::Nothing)
        },
        NodeKind::IfStatement { condition, body } => {
            let if_scope = scope.new_child();
            
            check_node(condition, scope)?;
            
            for child in body {
                check_node(child, &if_scope)?;
            }

            Ok(TypeKind::Nothing)
        },
        NodeKind::ForStatement { left, right, body, iter_name } => {
            let for_scope = scope.new_child();
            
            for_scope.declare_type(iter_name.clone(), TypeKind::Unknown);
            
            check_node(left, scope)?;
            check_node(right, scope)?;

            for child in body {
                check_node(child, &for_scope)?;
            }

            Ok(TypeKind::Unknown)
        },
        NodeKind::WhileStatement { condition, body } => {
            let while_scope = scope.new_child();
            
            check_node(condition, scope)?;
            
            for child in body {
                check_node(child, &while_scope)?;
            }

            Ok(TypeKind::Unknown)
        },
        NodeKind::FieldAccess { variable, field_name } => {
            let value = check_node(variable, scope)?;
            
            match value {
                TypeKind::Object(types) => {
                    Ok(match types.get(field_name) {
                        Some(value) => value.clone(),
                        None => TypeKind::Unknown,
                    })
                },
                _ => Ok(TypeKind::Unknown)
            }
        },
        NodeKind::VectorLiteral { values: _ } => Ok(TypeKind::Vector),
        NodeKind::ObjectLiteral { values } => {
            let mut types = HashMap::new();
            
            for (name, node) in values {
                types.insert(name.clone(), check_node(node, scope)?);
            }
            
            Ok(TypeKind::Object(types))
        },
        NodeKind::ValueFieldAccess { variable: _, value: _ } => Ok(TypeKind::Unknown),
        NodeKind::Import { identifier: _ } => Ok(TypeKind::Nothing),
    }
}

fn check_function(var_type: &TypeKind, parameters: &Vec<ASTNode>, scope: &TypeScope) -> Result<TypeKind, LangError> {
    match var_type {
        TypeKind::Function(types) => {
            let ret_type = match types.last() {
                Some(t) => t,
                None => return Err(LangError::new_parser(UNEXPECTED_ERROR.to_string())),
            };
            
            if types.len() - 1 != parameters.len() {
                return Err(LangError::new_parser(INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
            }

            for (i, param) in parameters.iter().enumerate() {
                let param_type = match check_node(param, scope) {
                    Ok(t) => t,
                    Err(err) => return Err(err),
                };
                
                if !types[i].is_compatible(&param_type) {
                    return Err(LangError::new_parser(INCORRECT_FUNCTION_PARAMETER_TYPE.to_string()))
                }
            }
            
            Ok(ret_type.clone())
        },
        _ => Ok(TypeKind::Unknown),
    }
}