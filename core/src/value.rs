#[derive(Debug, Clone)]
pub enum ReflValue<'a> {
    Str(&'a str),
    Int(i32),
    Float(f32),
    Bool(bool),
}

/// A helper to get what type is it
impl ReflValue<'_> {
    pub fn kind(&self) -> &'static str {
        match self {
            ReflValue::Str(_) => "str",
            ReflValue::Int(_) => "int",
            ReflValue::Float(_) => "float",
            ReflValue::Bool(_) => "bool",
        }
    }
}
