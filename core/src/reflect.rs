use crate::ReflValue;

pub trait Reflection {
    fn get_field(&self, field_name: &str) -> Option<ReflValue>;
}
