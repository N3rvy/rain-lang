use std::sync::Arc;

use crate::ast::node::ASTBody;

pub enum LangValue {
    Nothing,
    String(String),
    Int(i32),
    Float(f32),
    NaN,
    Bool(bool),
    Function(Arc<ASTBody>),
}

impl LangValue {
    pub fn sum(&self, other: Self) -> LangValue  {
        let values = (self, other);

        match values {
            // Int -> Int
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Int(left + right),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float(*left as f32 + right),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(*left + right as f32),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left + right),
            
            // Others -> String
            (left, right) => LangValue::String(left.to_string() + &right.to_string()),
        }
    }
    
    pub fn minus(&self, other: Self) -> LangValue {
        match (self, other) {
            // Int -> Int
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Int(left - right),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float(*left as f32 - right),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(*left - right as f32),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left - right),
            
            // Others -> String
            (_, _) => LangValue::NaN,
        }
    }
    
    pub fn multiply(&self, other: Self) -> LangValue {
        match (self, other) {
            // Int -> Int
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Int(left * right),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float(*left as f32 * right),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(*left * right as f32),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left * right),
            
            // Others -> String
            (_, _) => LangValue::NaN,
        }
    }
    
    pub fn divide(&self, other: Self) -> LangValue {
        match (self, other) {
            // Int -> Float
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Float(*left as f32 / right as f32),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float(*left as f32 / right),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(*left / right as f32),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left / right),
            
            // Others -> String
            (_, _) => LangValue::NaN,
        }
    }
    
    pub fn modulus(&self, other: Self) -> LangValue {
        match (self, other) {
            // Int -> Int
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Int(left % right),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float(*left as f32 % right),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(*left % right as f32),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left % right),
            
            // Others -> String
            (_, _) => LangValue::NaN,
        }
    }
    
    pub fn power(&self, other: Self) -> LangValue {
        match (self, other) {
            // If the exponent is less than 0 then the result will be NaN
            (_, LangValue::Int(int)) if int < 0 => LangValue::NaN,
            (_, LangValue::Float(float)) if float < 0.0 => LangValue::NaN,

            // Int -> Int
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Float((*left as f32).powf(right as f32)),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float((*left as f32).powf(right)),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(left.powf(right as f32)),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left.powf(right)),
            
            // Others -> String
            (_, _) => LangValue::NaN,
        }
    }
}

impl ToString for LangValue {
    fn to_string(&self) -> String {
        match self {
            LangValue::String(string) => string.clone(),
            LangValue::Int(int) => int.to_string(),
            LangValue::Float(float) => float.to_string(),
            LangValue::Bool(bool) => bool.to_string(),
            LangValue::Function(function) => "[Function]".to_string(),
            LangValue::Nothing => "Nothing".to_string(),
            LangValue::NaN => "NaN".to_string(),
        }
    }
}

impl Clone for LangValue {
    fn clone(&self) -> Self {
        match self {
            Self::String(string) => Self::String(string.clone()),
            Self::Int(int) => Self::Int(int.clone()),
            Self::Float(float) => Self::Float(float.clone()),
            Self::Bool(bool) => Self::Bool(bool.clone()),
            Self::Function(body) => Self::Function(body.clone()),
            Self::Nothing => Self::Nothing,
            Self::NaN => Self::NaN,
        }
    }
}