use serde::{Deserialize, Serialize};

use crate::{IString, Type};

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub struct DotnetTypeRef {
    assembly: Option<IString>,
    name_path: IString,
    generics: Vec<Type>,
    // In cause of `System.BadImageFormatException: Expected value type but got type kind 14` check if `is_valuetype` is always correct!
    is_valuetype: bool,
}
impl DotnetTypeRef {
    #[must_use]
    pub fn marshal() -> Self {
        Self::new(
            Some("System.Runtime.InteropServices"),
            "System.Runtime.InteropServices.Marshal",
        )
        .with_valuetype(false)
    }
    #[must_use]
    pub fn gc_handle() -> Self {
        Self::new(
            Some("System.Runtime"),
            "System.Runtime.InteropServices.GCHandle",
        )
        .with_valuetype(true)
    }
    #[must_use]
    pub fn thread() -> Self {
        Self::new(Some("System.Threading.Thread"), "System.Threading.Thread").with_valuetype(false)
    }
    #[must_use]
    pub fn thread_start() -> Self {
        Self::new(
            Some("System.Threading.Thread"),
            "System.Threading.ThreadStart",
        )
        .with_valuetype(false)
    }
    #[must_use]
    pub fn half() -> Self {
        Self::new(Some("System.Runtime"), "System.Half").with_valuetype(true)
    }
    #[must_use]
    pub fn single() -> Self {
        Self::new(Some("System.Runtime"), "System.Single").with_valuetype(true)
    }
    #[must_use]
    pub fn double() -> Self {
        Self::new(Some("System.Runtime"), "System.Double").with_valuetype(true)
    }
    #[must_use]
    pub fn console() -> Self {
        Self::new(Some("System.Console"), "System.Console").with_valuetype(false)
    }
    #[must_use]
    pub fn enviroment() -> Self {
        Self::new(Some("System.Runtime"), "System.Environment").with_valuetype(false)
    }
    #[must_use]
    pub fn math() -> Self {
        Self::new(Some("System.Runtime"), "System.Math").with_valuetype(false)
    }
    #[must_use]
    pub fn mathf() -> Self {
        Self::new(Some("System.Runtime"), "System.MathF").with_valuetype(false)
    }
    #[must_use]
    pub fn int_128() -> Self {
        Self::new(Some("System.Runtime"), "System.Int128")
    }
    #[must_use]
    pub fn binary_primitives() -> Self {
        Self::new(
            Some("System.Memory"),
            "System.Buffers.Binary.BinaryPrimitives",
        )
        .with_valuetype(false)
    }
    #[must_use]
    pub fn uint_128() -> Self {
        Self::new(Some("System.Runtime"), "System.UInt128")
    }
    #[must_use]
    pub fn usize_type() -> Self {
        Self::new(Some("System.Runtime"), "System.UIntPtr")
    }
    #[must_use]
    pub fn isize_type() -> Self {
        Self::new(Some("System.Runtime"), "System.IntPtr")
    }
    #[must_use]
    pub fn type_handle_type() -> Self {
        Self::new(Some("System.Runtime"), "System.RuntimeTypeHandle")
    }
    #[must_use]
    pub fn type_type() -> Self {
        Self::new(Some("System.Runtime"), "System.Type").with_valuetype(false)
    }
    #[must_use]
    pub fn object_type() -> Self {
        Self::new(Some("System.Runtime"), "System.Object").with_valuetype(false)
    }
    #[must_use]
    pub fn string_type() -> Self {
        Self::new(Some("System.Runtime"), "System.String").with_valuetype(false)
    }
    #[must_use]
    pub fn managed_array() -> Self {
        Self::new(Some("System.Runtime"), "System.Array").with_valuetype(false)
    }
    #[must_use]
    pub fn with_valuetype(mut self, valuetype: bool) -> Self {
        self.set_valuetype(valuetype);
        self
    }
    #[must_use]
    pub fn compiler_services_unsafe() -> Self {
        Self::new(
            Some("System.Runtime"),
            "System.Runtime.CompilerServices.Unsafe",
        )
        .with_valuetype(false)
    }
    pub fn new<S: Into<Box<str>>, S2: Into<Box<str>> + std::borrow::Borrow<str>>(
        assembly: Option<S>,
        name_path: S2,
    ) -> Self {
        assert!(!name_path.borrow().contains('/'));
        Self {
            assembly: assembly.map(std::convert::Into::into),
            name_path: name_path.into(),
            generics: Vec::new(),
            is_valuetype: true,
        }
    }
    #[must_use]
    pub const fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }
    #[must_use]
    pub const fn tpe_prefix(&self) -> &'static str {
        if self.is_valuetype() {
            "valuetype"
        } else {
            "class"
        }
    }
    pub fn set_valuetype(&mut self, is_valuetype: bool) {
        self.is_valuetype = is_valuetype;
    }
    #[must_use]
    pub fn array(element: &Type, length: usize) -> Self {
        let name = crate::arr_name(length, element);
        Self::new::<Box<str>, _>(None, name)
    }

    pub fn asm(&self) -> Option<&str> {
        self.assembly.as_ref().map(std::convert::AsRef::as_ref)
    }
    #[must_use]
    pub fn name_path(&self) -> &str {
        self.name_path.as_ref()
    }
    #[must_use]
    pub fn generics(&self) -> &[Type] {
        self.generics.as_ref()
    }
    pub fn set_generics(&mut self, generics: impl Into<Vec<Type>>) {
        self.generics = generics.into();
    }
    #[must_use]
    pub fn interlocked() -> Self {
        Self::new(Some("System.Threading"), "System.Threading.Interlocked").with_valuetype(false)
    }
    #[must_use]
    pub fn monitor() -> Self {
        Self::new(Some("System.Threading"), "System.Threading.Monitor").with_valuetype(false)
    }
    #[must_use]
    pub fn exception() -> Self {
        Self::new(Some("System.Runtime"), "System.Exception").with_valuetype(false)
    }
    #[must_use]
    pub fn assembly() -> Self {
        Self::new(Some("System.Runtime"), "System.Reflection.Assembly").with_valuetype(false)
    }
    #[must_use]
    pub fn native_mem() -> Self {
        Self::new(
            Some("System.Runtime.InteropServices"),
            "System.Runtime.InteropServices.NativeMemory",
        )
        .with_valuetype(false)
    }
}
