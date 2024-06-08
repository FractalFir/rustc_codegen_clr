use crate::{DotnetTypeRef, IString, Type};
use serde::{Deserialize, Serialize};
/// This struct descibes a .NET field. It contains information about the type this field belongs to, the name of the field, and the fields type.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct FieldDescriptor {
    owner: DotnetTypeRef,
    tpe: Type,
    name: IString,
}
impl FieldDescriptor {
    /// Returns the name of the field
    #[must_use]
    pub const fn name(&self) -> &str {
        &self.name
    }
    /// Returns the type of the field. For getting the type this field belongs to, see [self.owner]
    #[must_use]
    pub const fn tpe(&self) -> &Type {
        &self.tpe
    }
    /// Returns the the type this field belongs to. For getting the type of this field, see [self.tpe]
    #[must_use]
    pub const fn owner(&self) -> &DotnetTypeRef {
        &self.owner
    }
    /// Constructs a new fieldref, reffering to field of type `tpe`, belonging to `owner`, and named `name`
    #[must_use]
    pub fn new(owner: DotnetTypeRef, tpe: Type, name: IString) -> Self {
        assert_ne!(tpe, Type::Void);
        Self { owner, tpe, name }
    }
    /// The same as [`Self::new`], but also boxes the field descriptor.
    #[must_use]
    pub fn boxed(owner: DotnetTypeRef, tpe: Type, name: IString) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
