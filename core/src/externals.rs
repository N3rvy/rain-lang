
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


pub trait ExternalFunction<Args> {
    type Output: ExternalType;

    fn args_count(&self) -> usize;
}

impl<R, F> ExternalFunction<()> for F
where
    R: ExternalType,
    F: Fn<(), Output = R>,
{
    type Output = R;

    fn args_count(&self) -> usize { 0 }
}

impl<A0, R, F> ExternalFunction<(A0,)> for F
where
    A0: ExternalType,
    R: ExternalType,
    F: Fn<(A0,), Output = R>,
{
    type Output = R;

    fn args_count(&self) -> usize { 1 }
}

impl<A0, A1, R, F> ExternalFunction<(A0, A1)> for F
where
    A0: ExternalType,
    R: ExternalType,
    F: Fn<(A0, A1), Output = R>,
{
    type Output = R;

    fn args_count(&self) -> usize { 2 }
}

impl<A0, A1, A2, R, F> ExternalFunction<(A0, A1, A2)> for F
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    R: ExternalType,
    F: Fn<(A0, A1, A2), Output = R>,
{
    type Output = R;

    fn args_count(&self) -> usize { 3 }
}

impl<A0, A1, A2, A3, R, F> ExternalFunction<(A0, A1, A2, A3)> for F
where
    A0: ExternalType,
    A1: ExternalType,
    A2: ExternalType,
    A3: ExternalType,
    R: ExternalType,
    F: Fn<(A0, A1, A2, A3), Output = R>,
{
    type Output = R;

    fn args_count(&self) -> usize { 4 }
}