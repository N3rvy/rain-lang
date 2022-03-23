use common::ast::types::TypeKind;


pub trait ExternalType
where Self: Sized
{
    fn concretize(val: AnyValue) -> Option<Self>;
    fn generilize(val: Self) -> AnyValue;
    fn type_kind() -> TypeKind;
}

impl ExternalType for () {
    fn concretize(val: AnyValue) -> Option<Self> {
        match val {
            AnyValue::Nothing => Some(()),
            _ => None,
        }
    }

    fn generilize(_val: Self) -> AnyValue {
        AnyValue::Nothing
    }

    fn type_kind() -> TypeKind {
        TypeKind::Nothing
    }
}
impl ExternalType for i32 {
    fn concretize(val: AnyValue) -> Option<Self> {
        match val {
            AnyValue::Int(val) => Some(val),
            _ => None,
        }
    }

    fn generilize(val: Self) -> AnyValue {
        AnyValue::Int(val)
    }

    fn type_kind() -> TypeKind {
        TypeKind::Int
    }
}
impl ExternalType for f32 {
    fn concretize(val: AnyValue) -> Option<Self> {
        match val {
            AnyValue::Float(val) => Some(val),
            _ => None,
        }
    }

    fn generilize(val: Self) -> AnyValue {
        AnyValue::Float(val)
    }

    fn type_kind() -> TypeKind {
        TypeKind::Float
    }
}
impl ExternalType for bool {
    fn concretize(val: AnyValue) -> Option<Self> {
        match val {
            AnyValue::Bool(val) => Some(val),
            _ => None,
        }
   }

    fn generilize(val: Self) -> AnyValue {
        AnyValue::Bool(val)
    }

    fn type_kind() -> TypeKind {
        TypeKind::Bool
    }
}
impl ExternalType for String {
    fn concretize(val: AnyValue) -> Option<Self> {
        match val {
            AnyValue::String(val) => Some(val),
            _ => None,
        }
    }

    fn generilize(val: Self) -> AnyValue {
        AnyValue::String(val)
    }

    fn type_kind() -> TypeKind {
        TypeKind::String
    }
}
impl ExternalType for AnyValue {
    fn concretize(val: AnyValue) -> Option<Self> {
        Some(val)
    }
    fn generilize(val: Self) -> AnyValue {
        val
    }

    fn type_kind() -> TypeKind {
        TypeKind::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum AnyValue {
    Nothing,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

impl ToString for AnyValue {
    fn to_string(&self) -> String {
        match self {
            AnyValue::Nothing => "[Nothing]".to_string(),
            AnyValue::Int(i) => i.to_string(),
            AnyValue::Float(f) => f.to_string(),
            AnyValue::Bool(b) => b.to_string(),
            AnyValue::String(s) => s.clone(),
        }
    }
}