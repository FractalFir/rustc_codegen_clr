use crate::{v2::ClassRefIdx, IString, Type};
use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
/// This struct desribes a static .NET field.  It contains information about the type this static field belongs to, the name of the field, and the fields type.
pub struct StaticFieldDescriptor {
    owner: Option<ClassRefIdx>,
    tpe: Type,
    name: IString,
}
impl StaticFieldDescriptor {
    /// Returns the name of the static field
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the type of the static field. For getting the type this field belongs to, see [self.owner]
    #[must_use]
    pub const fn tpe(&self) -> &Type {
        &self.tpe
    }
    /// Returns the the type this static field belongs to. For getting the type of this field, see [self.tpe]
    #[must_use]
    pub const fn owner(&self) -> Option<&ClassRefIdx> {
        self.owner.as_ref()
    }
    /// Constructs a new static fieldref, reffering to field of type `tpe`, belonging to `owner`, and named `name`
    #[must_use]
    pub const fn new(owner: Option<ClassRefIdx>, tpe: Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    /// The same as [`Self::new`], but also boxes the field descriptor.
    #[must_use]
    pub fn boxed(owner: Option<ClassRefIdx>, tpe: Type, name: IString) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
