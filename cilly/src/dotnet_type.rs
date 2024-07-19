use serde::{Deserialize, Serialize};

use crate::{utilis::MemoryUsage, AsmString, AsmStringContainer, IString, Type};

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub enum DotnetTypeRef {
    Full {
        assembly: Option<IString>,
        name_path: IString,
        generics: Vec<Type>,
        // In cause of `System.BadImageFormatException: Expected value type but got type kind 14` check if `is_valuetype` is always correct!
        is_valuetype: bool,
    },
    // This type is a valuetype in this assmebly, without any generics.
    OptimizedRustStruct {
        name: AsmString,
    },
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
    pub fn new<S: Into<IString>, S2: Into<IString> + std::borrow::Borrow<str>>(
        assembly: Option<S>,
        name_path: S2,
    ) -> Self {
        assert!(!name_path.borrow().contains('/'));
        Self::Full {
            assembly: assembly.map(std::convert::Into::into),
            name_path: name_path.into(),
            generics: Vec::new(),
            is_valuetype: true,
        }
    }
    #[must_use]
    pub const fn is_valuetype(&self) -> bool {
        match self {
            DotnetTypeRef::Full { is_valuetype, .. } => *is_valuetype,
            DotnetTypeRef::OptimizedRustStruct { .. } => true,
        }
    }
    #[must_use]
    pub const fn tpe_prefix(&self) -> &'static str {
        if self.is_valuetype() {
            "valuetype"
        } else {
            "class"
        }
    }
    pub fn set_valuetype(&mut self, set_valuetype: bool) {
        match self {
            DotnetTypeRef::Full { is_valuetype, .. } => *is_valuetype = set_valuetype,
            DotnetTypeRef::OptimizedRustStruct { .. } => panic!(),
        }
    }
    #[must_use]
    pub fn array(element: &Type, length: usize, strings: &AsmStringContainer) -> Self {
        let name = crate::arr_name(length, element, strings);
        Self::new::<crate::IString, _>(None, name)
    }

    pub fn asm(&self) -> Option<&str> {
        match self {
            DotnetTypeRef::Full { assembly, .. } => {
                assembly.as_ref().map(std::convert::AsRef::as_ref)
            }
            DotnetTypeRef::OptimizedRustStruct { name } => None,
        }
    }
    #[must_use]
    pub fn name_path<'a, 'b: 'a>(&'a self, strings: &'b AsmStringContainer) -> &'a str {
        match self {
            DotnetTypeRef::Full { name_path, .. } => name_path,
            DotnetTypeRef::OptimizedRustStruct { name } => strings.get(*name),
        }
    }

    #[must_use]
    pub fn generics(&self) -> &[Type] {
        match self {
            DotnetTypeRef::Full { generics, .. } => generics,
            DotnetTypeRef::OptimizedRustStruct { name } => &[],
        }
    }
    pub fn set_generics(&mut self, set_generics: impl Into<Vec<Type>>) {
        match self {
            DotnetTypeRef::Full { generics, .. } => *generics = set_generics.into(),
            DotnetTypeRef::OptimizedRustStruct { name } => panic!(),
        }
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
    pub fn i_dictionary() -> Self {
        Self::new(Some("System.Runtime"), "System.Collections.IDictionary").with_valuetype(false)
    }
    #[must_use]
    pub fn dictionary(key: Type, value: Type) -> Self {
        let mut res = Self::new(
            Some("System.Collections.Concurrent"),
            "System.Collections.Concurrent.ConcurrentDictionary",
        )
        .with_valuetype(false);
        res.set_generics(&[key, value]);
        res
    }
    #[must_use]
    pub fn collection() -> Self {
        Self::new(Some("System.Runtime"), "System.Collections.ICollection").with_valuetype(false)
    }
    #[must_use]
    pub fn dictionary_iterator() -> Self {
        Self::new(
            Some("System.Runtime"),
            "System.Collections.IDictionaryEnumerator",
        )
        .with_valuetype(false)
    }
    #[must_use]
    pub fn collection_iterator() -> Self {
        Self::new(Some("System.Runtime"), "System.Collections.IEnumerator").with_valuetype(false)
    }

    #[must_use]
    pub fn native_mem() -> Self {
        Self::new(
            Some("System.Runtime.InteropServices"),
            "System.Runtime.InteropServices.NativeMemory",
        )
        .with_valuetype(false)
    }

    pub fn dotnet_tuple(elements: &[Type]) -> Self {
        let mut res = Self::new(Some("System.Runtime"), "System.ValueTuple");
        res.set_generics(elements);
        res
    }

    pub fn dictionary_entry() -> Self {
        Self::new(Some("System.Runtime"), "System.Collections.DictionaryEntry")
    }

    pub(crate) fn opt(&mut self, strings: &mut AsmStringContainer) {
        match self {
            DotnetTypeRef::Full {
                assembly,
                name_path,
                generics,
                is_valuetype,
            } => {
                if generics.is_empty() && assembly.is_none() && *is_valuetype {
                    *self = DotnetTypeRef::OptimizedRustStruct {
                        name: strings.alloc(name_path.clone()),
                    }
                }
            }

            DotnetTypeRef::OptimizedRustStruct { .. } => (),
        }
    }
}
impl MemoryUsage for DotnetTypeRef {
    fn memory_usage(&self, counter: &mut impl crate::utilis::MemoryUsageCounter) -> usize {
        match self {
            DotnetTypeRef::Full {
                assembly,
                name_path,
                generics,
                ..
            } => {
                let tpe_name = std::any::type_name::<Self>();
                let self_size = std::mem::size_of::<Self>();
                let asm_size = assembly.memory_usage(counter);
                let name_size = name_path.memory_usage(counter);
                let generic_size = generics.memory_usage(counter);

                let size = self_size + asm_size + name_size + generic_size;
                counter.add_type(tpe_name, size);
                size
            }
            DotnetTypeRef::OptimizedRustStruct { name } => {
                let tpe_name = std::any::type_name::<Self>();
                let self_size = std::mem::size_of::<Self>();
                counter.add_type(tpe_name, self_size);
                self_size
            }
        }
    }
}
