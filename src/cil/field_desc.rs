use serde::{Deserialize, Serialize};

use crate::{
    r#type::{DotnetTypeRef, Type},
    IString,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// This struct descibes a .NET field. It contains information about the type this field belongs to, the name of the field, and the fields type.
pub struct FieldDescriptor {
    owner: DotnetTypeRef,
    tpe: Type,
    name: IString,
}
impl FieldDescriptor {
    /// Returns the name of the field
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the type of the field. For getting the type this field belongs to, see [self.owner]
    pub fn tpe(&self) -> &Type {
        &self.tpe
    }
    /// Returns the the type this field belongs to. For getting the type of this field, see [self.tpe]
    pub fn owner(&self) -> &DotnetTypeRef {
        &self.owner
    }
    /// Constructs a new fieldref, reffering to field of type `tpe`, belonging to `owner`, and named `name`
    pub fn new(owner: DotnetTypeRef, tpe: Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    /// The same as [`Self::new`], but also boxes the field descriptor.
    pub fn boxed(owner: DotnetTypeRef, tpe: Type, name: IString) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
