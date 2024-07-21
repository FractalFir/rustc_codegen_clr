use super::Type;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Int {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}
impl From<Int> for Type {
    fn from(value: Int) -> Self {
        Self::Int(value)
    }
}
