use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    Type,
};
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct SigIdx(BiMapIndex);
impl IntoBiMapIndex for SigIdx {
    fn from_hash(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct FnSig {
    input: Box<[Type]>,
    output: Type,
}

impl FnSig {
    pub fn new(input: Box<[Type]>, output: Type) -> Self {
        Self { input, output }
    }

    pub fn input(&self) -> &[Type] {
        &self.input
    }

    pub fn output(&self) -> &Type {
        &self.output
    }

    pub(crate) fn from_v1(signature: &crate::FnSig, asm: &mut super::Assembly) -> Self {
        let input = signature
            .inputs()
            .iter()
            .map(|input| Type::from_v1(input, asm))
            .collect();
        let output = Type::from_v1(signature.output(), asm);
        Self::new(input, output)
    }
}
