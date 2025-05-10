#[derive(Debug, Clone)]
pub enum ReflValue<'a> {
    Str(&'a str),
    Int(i32),
    Float(f32),
    Bool(bool),
}
