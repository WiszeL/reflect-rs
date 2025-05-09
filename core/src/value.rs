#[derive(Debug, Clone)]
pub enum ReflValue {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}
