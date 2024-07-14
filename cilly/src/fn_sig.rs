use serde::{Deserialize, Serialize};

use crate::{utilis::MemoryUsage, Type};

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
impl MemoryUsage for FnSig {
    fn memory_usage(&self, counter: &mut impl crate::utilis::MemoryUsageCounter) -> usize {
        let total_size = std::mem::size_of::<Self>();
        let name = std::any::type_name::<Self>();
        let inputs = self.inputs.memory_usage(counter);
        counter.add_field(name, "inputs", inputs);
        let output = self.output.memory_usage(counter);
        counter.add_field(name, "output", output);
        total_size
    }
}
