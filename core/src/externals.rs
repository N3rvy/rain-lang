
pub trait ExternalType
where Self: Sized
{
    fn concretize(val: AnyValue) -> Option<Self>;
    fn generilize(val: Self) -> AnyValue;
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
}
impl ExternalType for AnyValue {
    fn concretize(val: AnyValue) -> Option<Self> {
        Some(val)
    }
    fn generilize(val: Self) -> AnyValue {
        val
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