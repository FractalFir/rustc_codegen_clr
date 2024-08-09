use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
/// f32, which is hashed as its bit representation, and compared bitwise. Usefull for consts in nodes
pub struct HashableF32(pub f32);

impl std::ops::Deref for HashableF32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::hash::Hash for HashableF32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
impl PartialEq for HashableF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl Eq for HashableF32 {}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
/// f64, which is hashed as its bit representation, and compared bitwise. Usefull for consts in nodes
pub struct HashableF64(pub f64);

impl std::ops::Deref for HashableF64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::hash::Hash for HashableF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
impl PartialEq for HashableF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl Eq for HashableF64 {}
