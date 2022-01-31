use crate::{lang_value::LangValue, types::{LangFunction, LangExternalFunction, LangVector}, object::LangObject};


pub trait ConvertLangValue
    where Self: Sized + 'static
{
    fn from(val: Self) -> LangValue;
    fn into(val: &LangValue) -> Option<Self>;
}


impl ConvertLangValue for LangValue {
    fn from(val: Self) -> LangValue {
        val
    }

    fn into(val: &LangValue) -> Option<Self> {
        Some(val.clone())
    }
}

impl ConvertLangValue for () {
    fn from(_: Self) -> LangValue {
        LangValue::Nothing
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_unit()
    }
}

impl ConvertLangValue for i32 {
    fn from(val: Self) -> LangValue {
        LangValue::Int(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_i32()
    }
}

impl ConvertLangValue for f32 {
    fn from(val: Self) -> LangValue {
        LangValue::Float(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_f32()
    }
}

impl ConvertLangValue for bool {
    fn from(val: Self) -> LangValue {
        LangValue::Bool(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_bool()
    }
}

impl ConvertLangValue for String {
    fn from(val: Self) -> LangValue {
        LangValue::String(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_string()
    }
}

impl ConvertLangValue for LangFunction {
    fn from(val: Self) -> LangValue {
        LangValue::Function(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_function()
    }
}

impl ConvertLangValue for LangExternalFunction {
    fn from(val: Self) -> LangValue {
        LangValue::ExtFunction(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_ext_function()
    }
}

impl ConvertLangValue for LangVector {
    fn from(val: Self) -> LangValue {
        LangValue::Vector(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        val.as_vec()
    }
}

impl ConvertLangValue for LangObject {
    fn from(val: Self) -> LangValue {
        LangValue::Object(val)
    }

    fn into(val: &LangValue) -> Option<Self> {
        match val {
            LangValue::Object(obj) => Some(obj.clone()),
            _ => None,
        }
    }
}