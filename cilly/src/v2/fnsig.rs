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
    #[must_use]
    pub fn new(input: Box<[Type]>, output: Type) -> Self {
        Self {
            inputs: input,
            output,
        }
    }

    #[must_use]
    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }

    #[must_use]
    pub fn output(&self) -> &Type {
        &self.output
    }

    pub fn iter_types(&self) -> impl Iterator<Item = Type> + '_ {
        self.inputs()
            .iter()
            .copied()
            .chain(std::iter::once(*self.output()))
    }

    pub fn inputs_mut(&mut self) -> &mut Box<[Type]> {
        &mut self.inputs
    }

    pub fn set_inputs(&mut self, inputs: Box<[Type]>) {
        self.inputs = inputs;
    }
}
