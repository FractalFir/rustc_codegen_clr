use serde::{Deserialize, Serialize};

use crate::Type;

#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
/// Function signature.
pub struct FnSig {
    inputs: Vec<Type>,
    output: Type,
}
impl FnSig {
    #[must_use]
    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }
    /// Returns the function output.
    #[must_use]
    pub const fn output(&self) -> &Type {
        &self.output
    }
    /// Creates a new function signature. For non-static functions, this must include the hidden first `this` argument!
    #[must_use]
    pub fn new(inputs: impl Into<Vec<Type>>, output: impl Into<Type>) -> Self {
        Self {
            inputs: inputs.into(),
            output: output.into(),
        }
    }
    /// Sets the input list of this method.
    pub fn set_inputs(&mut self, inputs: Vec<Type>) {
        self.inputs = inputs;
    }

    pub fn inputs_mut(&mut self) -> &mut Vec<Type> {
        &mut self.inputs
    }
}
