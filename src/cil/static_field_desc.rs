use serde::{Deserialize, Serialize};

use crate::{
    r#type::{DotnetTypeRef, Type},
    IString,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// This struct desribes a static .NET field.  It contains information about the type this static field belongs to, the name of the field, and the fields type.
pub struct StaticFieldDescriptor {
    owner: Option<DotnetTypeRef>,
    tpe: Type,
    name: IString,
}
impl StaticFieldDescriptor {
    /// Returns the name of the static field
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the type of the static field. For getting the type this field belongs to, see [self.owner]
    pub fn tpe(&self) -> &Type {
        &self.tpe
    }
    /// Returns the the type this static field belongs to. For getting the type of this field, see [self.tpe]
    pub fn owner(&self) -> Option<&DotnetTypeRef> {
        self.owner.as_ref()
    }
    /// Constructs a new static fieldref, reffering to field of type `tpe`, belonging to `owner`, and named `name`
    #[must_use]
    pub fn new(owner: Option<DotnetTypeRef>, tpe: Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    /// The same as [`Self::new`], but also boxes the field descriptor.
    #[must_use]
    pub fn boxed(owner: Option<DotnetTypeRef>, tpe: Type, name: IString) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
