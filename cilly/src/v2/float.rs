use super::Type;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Float {
    F16,
    F32,
    F64,
}
impl From<Float> for Type {
    fn from(value: Float) -> Self {
        Type::Float(value)
    }
}
