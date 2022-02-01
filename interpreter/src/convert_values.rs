use core::ExternalType;

use crate::lang_value::LangValue;


pub trait ConvertLangValue : ExternalType
    where Self: Sized + 'static
{
    fn from(val: Self) -> LangValue;
    fn into(val: &LangValue) -> Option<Self>;
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