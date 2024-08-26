use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    Type,
};
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct SigIdx(BiMapIndex);
impl IntoBiMapIndex for SigIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct FnSig {
    inputs: Box<[Type]>,
    output: Type,
}

impl FnSig {
    pub fn new(input: Box<[Type]>, output: Type) -> Self {
        Self {
            inputs: input,
            output,
        }
    }

    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }

    pub fn output(&self) -> &Type {
        &self.output
    }

    pub fn from_v1(signature: &crate::FnSig) -> Self {
        let input = signature.inputs().iter().copied().collect();
        Self::new(input, *signature.output())
    }

    pub fn iter_types(&self) -> impl Iterator<Item = Type> + '_ {
        self.inputs()
            .iter()
            .copied()
            .chain(std::iter::once(*self.output()))
    }
}
