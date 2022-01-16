use std::{ops::{Try, FromResidual, ControlFlow}, borrow::Borrow, sync::Arc, collections::HashMap};
use common::{lang_value::LangValue, types::{ReturnKind, MathOperatorKind, BoolOperatorKind}, errors::LangError, ast::{ASTNode, ASTBody}, messages::{VARIABLE_NOT_DECLARED, VARIABLE_IS_NOT_A_FUNCTION, INCORRECT_NUMBER_OF_PARAMETERS, VARIABLE_IS_NOT_A_NUMBER, INVALID_IMPORT}, external_functions::ExternalFunctionRunner, object::LangObject};

use crate::{Vm, import::{Importer, ImportResult}};

use super::scope::Scope;


pub enum EvalResult {
    Ok(LangValue),
    Ret(LangValue, ReturnKind),
    Err(LangError),
}

impl FromResidual for EvalResult {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl Try for EvalResult {
    type Output = LangValue;
    type Residual = EvalResult;

    fn from_output(output: Self::Output) -> Self {
        EvalResult::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            EvalResult::Ok(value) => ControlFlow::Continue(value),
            EvalResult::Ret(value, kind) => ControlFlow::Break(EvalResult::Ret(value, kind)),
            EvalResult::Err(err) => ControlFlow::Break(EvalResult::Err(err)),
        }
    }
}

macro_rules! expect_some {
    ($value:expr, $err:expr) => {
        match $value {
            Some(val) => val,
            None => return EvalResult::Err(LangError::new_runtime($err)),
        }
    };
}


impl<'a, Imp: Importer> Vm<'a, Imp> {
    pub fn evaluate_ast(&self, scope: &Scope, ast: &Box<ASTNode>) -> EvalResult {
        match ast.as_ref() {
            ASTNode::Root { body } => {
                for child in body {
                    self.evaluate_ast(scope, child)?;
                }
                
                EvalResult::Ok(LangValue::Nothing)
            },
            ASTNode::VariableDecl { name, value } => {
                let value = self.evaluate_ast(scope, value)?;
                scope.declare_var(name.clone(), value.clone());

                EvalResult::Ok(LangValue::Nothing)
            },
            ASTNode::VaraibleRef { name } => {
                match scope.get_var(name) {
                    Some(value) => EvalResult::Ok(value.clone()),
                    None => EvalResult::Err(LangError::new_runtime(VARIABLE_NOT_DECLARED.to_string())),
                }
            },
            ASTNode::VariableAsgn { name, value } => {
                let value = self.evaluate_ast(scope, value)?;
                scope.set_var(name, value);
                
                EvalResult::Ok(LangValue::Nothing)
            },
            ASTNode::MethodInvok { object, name, parameters } => {
                let object = self.evaluate_ast(scope, object)?;
                let func = object.get_field(&self.registry, name);
                
                let mut param_values = Vec::new();
                param_values.push(object);
                for param in parameters {
                    let value = self.evaluate_ast(scope, param)?;
                    param_values.push(value);
                }
                
                self.invoke_function(scope, &func, parameters, param_values)
            },
            ASTNode::FunctionInvok { variable, parameters } => {
                let func = self.evaluate_ast (scope, variable)?;
                        
                let mut param_values = Vec::new();
                for param in parameters {
                    let value = self.evaluate_ast(scope, param)?;
                    param_values.push(value);
                }

                self.invoke_function(scope, &func, parameters, param_values)
            },
            ASTNode::Literal { value } => {
                EvalResult::Ok(value.clone())
            },
            ASTNode::MathOperation { operation, left, right } => {
                let left = self.evaluate_ast(scope, left)?;
                let right = self.evaluate_ast(scope, right)?;
                
                let value = match operation {
                    MathOperatorKind::Plus => left.sum(right),
                    MathOperatorKind::Minus => left.minus(right),
                    MathOperatorKind::Multiply => left.multiply(right),
                    MathOperatorKind::Divide => left.divide(right),
                    MathOperatorKind::Modulus => left.modulus(right),
                    MathOperatorKind::Power => left.power(right),
                };
                
                EvalResult::Ok(value)
            },
            ASTNode::BoolOperation { operation, left, right } => {
                let left = self.evaluate_ast(scope, left)?;
                let right = self.evaluate_ast(scope, right)?;
                
                let value = match operation {
                    BoolOperatorKind::Equal => left.equals(&right),
                    BoolOperatorKind::Different => left.not_equals(&right),
                    BoolOperatorKind::Bigger => left.bigger(&right),
                    BoolOperatorKind::Smaller => left.smaller(&right),
                    BoolOperatorKind::BiggerEq => left.bigger_eq(&right),
                    BoolOperatorKind::SmallerEq => left.smaller_eq(&right),
                };
                
                EvalResult::Ok(LangValue::Bool(value))
            },
            ASTNode::ReturnStatement { value: Some(value ), kind } => EvalResult::Ret(self.evaluate_ast(scope, value)?, kind.clone()),
            ASTNode::ReturnStatement { value: None, kind } => EvalResult::Ret(LangValue::Nothing, kind.clone()),
            ASTNode::IfStatement { condition, body } => {
                let condition = self.evaluate_ast(scope, condition)?;
                
                if condition.truthy() {
                    let if_scope = Scope::new_child(scope);

                    for child in body {
                        self.evaluate_ast(&if_scope, child)?;
                    }
                }
                
                EvalResult::Ok(LangValue::Nothing)
            },
            ASTNode::ForStatement { left, right, body, iter_name } => {
                let left = self.evaluate_ast(scope, left)?.as_i32();
                let right = self.evaluate_ast(scope, right)?.as_i32();
                
                let min = expect_some!(left, VARIABLE_IS_NOT_A_NUMBER.to_string());
                let max = expect_some!(right, VARIABLE_IS_NOT_A_NUMBER.to_string());
                
                for i in min..max {
                    let for_scope = Scope::new_child(scope.clone());
                    for_scope.declare_var(iter_name.clone(), LangValue::Int(i));
                    
                    for child in body {
                        match self.evaluate_ast(&for_scope, child) {
                            EvalResult::Ok(_) => (),
                            EvalResult::Ret(value, ReturnKind::Break) => return EvalResult::Ok(value),
                            EvalResult::Ret(value, kind) => return EvalResult::Ret(value, kind),
                            EvalResult::Err(err) => return EvalResult::Err(err),
                        }
                    }
                }
                
                EvalResult::Ok(LangValue::Nothing)
            },
            ASTNode::WhileStatement { condition, body } => {
                while self.evaluate_ast(scope, condition)?.truthy() {
                    let while_scope = Scope::new_child(scope.clone());
                    
                    for child in body {
                        match self.evaluate_ast(&while_scope, child) {
                            EvalResult::Ok(_) => (),
                            EvalResult::Ret(value, ReturnKind::Break) => return EvalResult::Ok(value),
                            EvalResult::Ret(value, kind) => return EvalResult::Ret(value, kind),
                            EvalResult::Err(err) => return EvalResult::Err(err),
                        }
                    }
                }

                EvalResult::Ok(LangValue::Nothing)
            },
            ASTNode::FieldAccess { variable, field_name } => {
                let value = self.evaluate_ast(scope, variable)?;
                let result = value.get_field(&self.registry, field_name);
                
                EvalResult::Ok(result)
            },
            ASTNode::VectorLiteral { values } => {
                let mut eval_values = Vec::new();
                
                for val in values {
                    eval_values.push(self.evaluate_ast(scope, val)?);
                }
                
                EvalResult::Ok(LangValue::Vector(Arc::new(eval_values)))
            },
            ASTNode::ValueFieldAccess { variable, value } => {
                let variable = self.evaluate_ast(scope, variable)?;
                let value = self.evaluate_ast(scope, value)?;

                EvalResult::Ok(variable.get_value_field(value))
            },
            ASTNode::ObjectLiteral { values } => {
                let mut map = HashMap::new();
                
                for value in values {
                    map.insert(value.0.clone(), self.evaluate_ast(scope, &value.1)?);
                }
                
                EvalResult::Ok(LangValue::Object(LangObject::from_map(map)))
            },
            ASTNode::Import { identifier } => {
                match self.importer.import(&identifier) {
                    ImportResult::Imported(script) => {
                        match self.evaluate(&script) {
                            Ok(_) => EvalResult::Ok(LangValue::Nothing),
                            Err(err) => EvalResult::Err(err),
                        }
                    },
                    ImportResult::AlreadyImported => EvalResult::Ok(LangValue::Nothing),
                    ImportResult::NotFound => EvalResult::Err(LangError::new_runtime(INVALID_IMPORT.to_string())),
                    ImportResult::ImportError(err) => EvalResult::Err(err),
                }
            },
        }
    }

    fn invoke_function(&self, scope: &Scope, func: &LangValue, parameters: &ASTBody, param_values: Vec<LangValue>) -> EvalResult {
        match func {
            LangValue::Function(func) => {
                // Parameters
                if parameters.len() != func.parameters.len() {
                    return EvalResult::Err(LangError::new_runtime(INCORRECT_NUMBER_OF_PARAMETERS.to_string()));
                }
        
                let func_scope = Scope::new_child(scope);
                for i in 0..parameters.len() {
                    // TODO: PLS BETTER PERFORMANCE! THANKS ME OF THE FUTURE
                    func_scope.declare_var(func.parameters[i].to_string(), param_values[i].clone());
                }

                for child in &func.body {
                    // Matching to make the return statement stop
                    match self.evaluate_ast(&func_scope, child) {
                        EvalResult::Ok(_) => (),
                        EvalResult::Ret(value, ReturnKind::Return) => return EvalResult::Ok(value),
                        EvalResult::Ret(value, kind) => return EvalResult::Ret(value, kind),
                        EvalResult::Err(err) => return EvalResult::Err(err),
                    }
                }
                
                EvalResult::Ok(LangValue::Nothing)
            },
            LangValue::ExtFunction(func) => {
                match (func.borrow() as &ExternalFunctionRunner).run(param_values) {
                    Ok(value ) => EvalResult::Ok(value),
                    Err(err) => EvalResult::Err(err),
                }
            },
            _ => return EvalResult::Err(LangError::new_runtime(VARIABLE_IS_NOT_A_FUNCTION.to_string())),
        }
    }
}