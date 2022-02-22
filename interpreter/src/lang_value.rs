use core::AnyValue;
use std::{sync::Arc, fmt::Debug};

use common::ast::types::{Function, LiteralKind};

use crate::{external_functions::ExternalFunctionRunner, object::LangObject};


pub type LangFunction = Arc<Function>;
pub type LangExternalFunction = Arc<ExternalFunctionRunner>;
pub type LangVector = Arc<Vec<LangValue>>;

pub enum LangValue {
    Nothing,
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Function(LangFunction),
    ExtFunction(LangExternalFunction),
    Vector(LangVector),
    Object(LangObject),
}

#[derive(PartialEq, Eq, Hash)]
pub enum LangValueDiscriminant {
    Nothing,
    String,
    Int,
    Float,
    Bool,
    Function,
    ExtFunction,
    Vector,
    Object,
}

impl From<&LangValue> for LangValueDiscriminant {
    fn from(val: &LangValue) -> Self {
        match val {
            LangValue::Nothing => LangValueDiscriminant::Nothing,
            LangValue::String(_) => LangValueDiscriminant::String,
            LangValue::Int(_) => LangValueDiscriminant::Int,
            LangValue::Float(_) => LangValueDiscriminant::Float,
            LangValue::Bool(_) => LangValueDiscriminant::Bool,
            LangValue::Function(_) => LangValueDiscriminant::Function,
            LangValue::ExtFunction(_) => LangValueDiscriminant::ExtFunction,
            LangValue::Vector(_) => LangValueDiscriminant::Vector,
            LangValue::Object(_) => LangValueDiscriminant::Object,
        }
    }
}


#[allow(dead_code)]
impl LangValue {
    pub fn truthy(&self) -> bool {
        match self {
            LangValue::Nothing => false,
            LangValue::String(string) => string.len() > 0,
            LangValue::Int(int) => *int != 0,
            LangValue::Float(float) => *float != 0.0,
            LangValue::Bool(bool) => *bool,
            LangValue::Function(_) => true,
            LangValue::ExtFunction(_) => true,
            LangValue::Vector(vec) => vec.len() != 0,
            LangValue::Object(obj) => obj.len() != 0,
        }
    }

    pub fn get_field<'a>(&'a self, name: &String) -> LangValue {
        match self {
            LangValue::Object(obj) => {
                obj.get(name)
            },
            _ => LangValue::Nothing,
        }
    }
    
    pub fn get_value_field(&self, value: LangValue) -> LangValue {
        match self {
            LangValue::Vector(vec) => {
                match value.as_i32() {
                    Some(i) => match vec.get(i as usize) {
                        Some(value) => value.clone(),
                        None => LangValue::Nothing,
                    },
                    None => LangValue::Nothing,
                }
            },
            LangValue::Object(obj) => {
                match value.as_string() {
                    Some(name) => obj.get(&name),
                    None => LangValue::Nothing,
                }
            },
            _ => LangValue::Nothing,
        }
    }
    
    pub fn as_unit(&self) -> Option<()> {
        match self {
            LangValue::Nothing => Some(()),
            _ => None,
        }
    }
    
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            LangValue::Int(int) => Some(*int),
            LangValue::Float(float) => Some(*float as i32),
            _ => None,
        }
    }
    
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            LangValue::Int(int) => Some(*int as f32),
            LangValue::Float(float) => Some(*float),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LangValue::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            LangValue::String(string) => Some(string.clone()),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<LangFunction> {
        match self {
            LangValue::Function(function) => Some(function.clone()),
            _ => None,
        }
    }
    
    pub fn as_ext_function(&self) -> Option<LangExternalFunction> {
        match self {
            LangValue::ExtFunction(ext_func) => Some(ext_func.clone()),
            _ => None,
        }
    }
    
    pub fn as_vec(&self) -> Option<LangVector> {
        match self {
            LangValue::Vector(vec) => Some(vec.clone()),
            _ => None,
        }
    }
    
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
            (_, _) => LangValue::Nothing,
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
            (_, _) => LangValue::Nothing,
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
            (_, _) => LangValue::Nothing,
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
            (_, _) => LangValue::Nothing,
        }
    }
    
    pub fn power(&self, other: Self) -> LangValue {
        match (self, other) {
            // If the exponent is less than 0 then the result will be NaN
            (_, LangValue::Int(int)) if int < 0 => LangValue::Nothing,
            (_, LangValue::Float(float)) if float < 0.0 => LangValue::Nothing,

            // Int -> Int
            (LangValue::Int(left), LangValue::Int(right)) => LangValue::Float((*left as f32).powf(right as f32)),
            
            // Int/Float -> Float
            (LangValue::Int(left), LangValue::Float(right)) => LangValue::Float((*left as f32).powf(right)),
            (LangValue::Float(left), LangValue::Int(right)) => LangValue::Float(left.powf(right as f32)),
            
            // Float -> Float
            (LangValue::Float(left), LangValue::Float(right )) => LangValue::Float(left.powf(right)),
            
            // Others -> String
            (_, _) => LangValue::Nothing,
        }
    }
    
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (LangValue::Int(x), LangValue::Int(y)) => x == y,
            (LangValue::Float(x), LangValue::Float(y)) => x == y,
            
            (LangValue::Int(x), LangValue::Float(y)) => *x as f32 == *y,
            (LangValue::Float(x), LangValue::Int(y)) => *x == *y as f32,

            (LangValue::Nothing, LangValue::Nothing) => true,
            (LangValue::String(x), LangValue::String(y)) => x == y,
            (LangValue::Bool(x), LangValue::Bool(y)) => x == y,
            (LangValue::Function(x), LangValue::Function(y)) => {
                x as *const _ == y as *const _
            },
            
            _ => false,
        }
    }
    
    pub fn bigger(&self, other: &Self) -> bool {
        match (self, other) {
            (LangValue::Int(x), LangValue::Int(y)) => x > y,
            (LangValue::Float(x), LangValue::Float(y)) => x > y,
            
            (LangValue::Int(x), LangValue::Float(y)) => *x as f32 > *y,
            (LangValue::Float(x), LangValue::Int(y)) => *x > *y as f32,

            (LangValue::String(x), LangValue::String(y)) => x.len() > y.len(),
            (LangValue::Bool(x), LangValue::Bool(y)) => *x && !y,
            
            _ => false,
        }
    }
    
    pub fn smaller(&self, other: &Self) -> bool {
        match (self, other) {
            (LangValue::Int(x), LangValue::Int(y)) => x < y,
            (LangValue::Float(x), LangValue::Float(y)) => x < y,
            
            (LangValue::Int(x), LangValue::Float(y)) => (*x as f32) < *y,
            (LangValue::Float(x), LangValue::Int(y)) => *x < *y as f32,

            (LangValue::String(x), LangValue::String(y)) => x.len() < y.len(),
            (LangValue::Bool(x), LangValue::Bool(y)) => !*x && *y,
            
            _ => false,
        }
    }
    
    pub fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
    
    pub fn smaller_eq(&self, other: &Self) -> bool {
        !self.bigger(other)
    }
    
    pub fn bigger_eq(&self, other: &Self) -> bool {
        !self.smaller(other)
    }
}

impl Debug for LangValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str()) 
    }
}

impl ToString for LangValue {
    fn to_string(&self) -> String {
        match self {
            LangValue::String(string) => string.clone(),
            LangValue::Int(int) => int.to_string(),
            LangValue::Float(float) => float.to_string(),
            LangValue::Bool(bool) => bool.to_string(),
            LangValue::Function(_) => "[Function]".to_string(),
            LangValue::Nothing => "Nothing".to_string(),
            LangValue::ExtFunction(_) => "[External Function]".to_string(),
            LangValue::Vector(_) => "[Vector]".to_string(),
            LangValue::Object(_) => "[Object]".to_string(),
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
            Self::ExtFunction(func) => Self::ExtFunction(func.clone()),
            Self::Vector(vec) => Self::Vector(vec.clone()),
            Self::Object(obj) => Self::Object(obj.clone()),
        }
    }
}


impl Into<Option<AnyValue>> for LangValue {
    fn into(self) -> Option<AnyValue> {
        match self {
            LangValue::Nothing => Some(AnyValue::Nothing),
            LangValue::String(str) => Some(AnyValue::String(str)),
            LangValue::Int(i) => Some(AnyValue::Int(i)),
            LangValue::Float(f) => Some(AnyValue::Float(f)),
            LangValue::Bool(b) => Some(AnyValue::Bool(b)),
            _ => None,
        }
    }
}

impl Into<AnyValue> for LangValue {
    fn into(self) -> AnyValue {
        match self {
            LangValue::Nothing => AnyValue::Nothing,
            LangValue::String(str) => AnyValue::String(str),
            LangValue::Int(i) => AnyValue::Int(i),
            LangValue::Float(f) => AnyValue::Float(f),
            LangValue::Bool(b) => AnyValue::Bool(b),
            _ => AnyValue::Nothing,
        }
    }
}

impl From<AnyValue> for LangValue {
    fn from(val: AnyValue) -> Self {
        match val {
            AnyValue::Nothing => LangValue::Nothing,
            AnyValue::Int(i) => LangValue::Int(i),
            AnyValue::Float(f) => LangValue::Float(f),
            AnyValue::Bool(b) => LangValue::Bool(b),
            AnyValue::String(s) => LangValue::String(s),
        }
    }
}

impl From<LiteralKind> for LangValue {
    fn from(val: LiteralKind) -> Self {
        match val {
            LiteralKind::Nothing => LangValue::Nothing,
            LiteralKind::Int(i) => LangValue::Int(i),
            LiteralKind::Float(f) => LangValue::Float(f),
            LiteralKind::String(s) => LangValue::String(s),
        }
    }
}
