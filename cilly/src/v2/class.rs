use super::{
    access::Access,
    bimap::{BiMapIndex, IntoBiMapIndex},
    Assembly, MethodDefIdx, MethodRef, MethodRefIdx, StringIdx, Type,
};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

impl From<ClassRefIdx> for Type {
    fn from(val: ClassRefIdx) -> Self {
        Type::ClassRef(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ClassRefIdx(BiMapIndex);
impl IntoBiMapIndex for ClassRefIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct ClassRef {
    name: StringIdx,
    asm: Option<StringIdx>,
    is_valuetype: bool,
    generics: Box<[Type]>,
}

impl ClassRef {
    #[must_use]
    pub fn display(&self, asm: &Assembly) -> String {
        format!(
            "ClassRef{{name:{},asm:{:?},is_valuetype:{},generics{:?}}}",
            &asm[self.name()],
            self.asm().map(|idx| &asm[idx]),
            self.is_valuetype(),
            self.generics()
        )
    }
    #[must_use]
    pub fn new(
        name: StringIdx,
        asm: Option<StringIdx>,
        is_valuetype: bool,
        generics: Box<[Type]>,
    ) -> Self {
        Self {
            name,
            asm,
            is_valuetype,
            generics,
        }
    }
    pub fn interlocked(asm: &mut super::Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Threading.Interlocked");
        let asm_name = Some(asm.alloc_string("System.Threading"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, vec![].into()))
    }

    /// Returns the assembly containing this typedef
    #[must_use]
    pub fn asm(&self) -> Option<StringIdx> {
        self.asm
    }
    /// The name of this class definition
    #[must_use]
    pub fn name(&self) -> StringIdx {
        self.name
    }

    #[must_use]
    pub fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }

    #[must_use]
    pub fn generics(&self) -> &[Type] {
        &self.generics
    }
    /// The .NET math class
    pub fn math(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Math");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, vec![].into()))
    }
    /// Retusn a reference to the class `System.Double`
    pub fn double(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Double");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, vec![].into()))
    }
    /// Retusn a reference to the class `System.Single`
    pub fn single(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Single");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, vec![].into()))
    }
    /// Returns a reference to the class `System.MathF`
    #[must_use]
    pub fn mathf(asm: &mut Assembly) -> ClassRefIdx {
        let name: StringIdx = asm.alloc_string("System.MathF");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.UInt128` type.
    pub fn uint_128(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.UInt128");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.Int128` type.
    pub fn int_128(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Int128");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.UIntPtr` type.
    pub fn usize_type(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.UIntPtr");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.UInt16` type.
    pub fn uint16(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.UInt16");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.Int16` type.
    pub fn int16(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Int16");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.UInt32` type.
    pub fn uint32(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.UInt32");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.Int32` type.
    pub fn int32(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Int32");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.UInt64` type.
    pub fn uint64(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.UInt64");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.Int64` type.
    pub fn int64(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Int64");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.IntPtr` type.
    pub fn isize_type(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.IntPtr");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.Half` type.
    pub fn half(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Half");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.Byte` type.
    pub fn byte(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Byte");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.SByte` type.
    pub fn sbyte(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.SByte");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the GC handle class.
    pub fn gc_handle(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Runtime.InteropServices.GCHandle");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.String`
    pub fn string(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.String");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.Object`
    pub fn object(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Object");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.Threading.Thread`
    pub fn thread(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Threading.Thread");
        let asm_name = Some(asm.alloc_string("System.Threading.Thread"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.Threading.ThreadStart`
    pub fn thread_start(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Threading.ThreadStart");
        let asm_name = Some(asm.alloc_string("System.Threading.Thread"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.Type`
    pub fn type_type(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Type");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.RuntimeTypeHandle`
    pub fn runtime_type_hadle(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.RuntimeTypeHandle");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the `System.String`
    pub fn exception(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Exception");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the `System.Console`
    pub fn console(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Console");
        let asm_name = Some(asm.alloc_string("System.Console"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Collections.IDictionaryEnumerator`
    #[must_use]
    pub fn dictionary_iterator(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Collections.IDictionaryEnumerator");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Collections.IEnumerator`
    #[must_use]
    pub fn i_enumerator(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Collections.IEnumerator");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Collections.IDictionary`
    #[must_use]
    pub fn i_dictionary(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Collections.IDictionary");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Collections.ICollection`
    #[must_use]
    pub fn i_collection(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Collections.ICollection");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Environment`
    #[must_use]
    pub fn enviroment(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Environment");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Runtime.InteropServices.Marshal`
    #[must_use]
    pub fn marshal(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Runtime.InteropServices.Marshal");
        let asm_name = Some(asm.alloc_string("System.Runtime.InteropServices"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Collections.DictionaryEntry`
    #[must_use]
    pub fn dictionary_entry(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Collections.DictionaryEntry");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    /// Returns a reference to the class `System.Runtime.InteropServices.NativeMemory`
    #[must_use]
    pub fn native_mem(asm: &mut Assembly) -> ClassRefIdx {
        let name = asm.alloc_string("System.Runtime.InteropServices.NativeMemory");
        let asm_name = Some(asm.alloc_string("System.Runtime.InteropServices"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Numerics.BitOperations`
    #[must_use]
    pub fn bit_operations(asm: &mut Assembly) -> ClassRefIdx {
        let name: StringIdx = asm.alloc_string("System.Numerics.BitOperations");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.Buffers.Binary.BinaryPrimitives`
    #[must_use]
    pub fn binary_primitives(asm: &mut Assembly) -> ClassRefIdx {
        let name: StringIdx = asm.alloc_string("System.Buffers.Binary.BinaryPrimitives");
        let asm_name = Some(asm.alloc_string("System.Memory"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [].into()))
    }
    /// Returns a reference to the class `System.MidpointRounding`
    #[must_use]
    pub fn midpoint_rounding(asm: &mut Assembly) -> ClassRefIdx {
        let name: StringIdx = asm.alloc_string("System.MidpointRounding");
        let asm_name = Some(asm.alloc_string("System.Runtime"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, true, [].into()))
    }
    #[must_use]
    pub fn fixed_array(element: Type, length: usize, asm: &mut Assembly) -> ClassRefIdx {
        let name = format!("{element}_{length}", element = element.mangle(asm));
        let name = asm.alloc_string(name);
        let cref = ClassRef::new(name, None, true, [].into());
        asm.alloc_class_ref(cref)
    }
    /// Returns a reference to the constructor of this class  - `.ctor`. The explict inputs of the constructor should not include `this` - that parameter will be automaticaly provided.
    pub fn ctor(&self, explict_inputs: &[Type], asm: &mut Assembly) -> MethodRefIdx {
        let this = asm.alloc_class_ref(self.clone());
        let mut inputs = vec![Type::ClassRef(this)];
        inputs.extend(explict_inputs);
        let sig = asm.sig(inputs, Type::Void);
        let fn_name = asm.alloc_string(".ctor");
        asm.alloc_methodref(MethodRef::new(
            this,
            fn_name,
            sig,
            super::cilnode::MethodKind::Constructor,
            [].into(),
        ))
    }
    /// Returns a reference to an instance method of this class, with a given name. The explict inputs of the method should not include `this` - that parameter will be automaticaly provided.
    pub fn instance(
        &self,
        explict_inputs: &[Type],
        output: Type,
        fn_name: StringIdx,
        asm: &mut Assembly,
    ) -> MethodRefIdx {
        let this = asm.alloc_class_ref(self.clone());
        let mut inputs = if self.is_valuetype() {
            vec![asm.nref(Type::ClassRef(this))]
        } else {
            vec![Type::ClassRef(this)]
        };

        inputs.extend(explict_inputs);
        let sig = asm.sig(inputs, output);
        asm.alloc_methodref(MethodRef::new(
            this,
            fn_name,
            sig,
            super::cilnode::MethodKind::Instance,
            [].into(),
        ))
    }
    /// Returns a reference to an virtual method of this class, with a given name. The explict inputs of the method should not include `this` - that parameter will be automaticaly provided.
    pub fn virtual_mref(
        &self,
        explict_inputs: &[Type],
        output: Type,
        fn_name: StringIdx,
        asm: &mut Assembly,
    ) -> MethodRefIdx {
        let this = asm.alloc_class_ref(self.clone());
        let mut inputs = vec![Type::ClassRef(this)];
        inputs.extend(explict_inputs);
        let sig = asm.sig(inputs, output);
        asm.alloc_methodref(MethodRef::new(
            this,
            fn_name,
            sig,
            super::cilnode::MethodKind::Virtual,
            [].into(),
        ))
    }
    /// Returns a reference to an static method of this class, with a given name.
    pub fn static_mref(
        &self,
        inputs: &[Type],
        output: Type,
        fn_name: StringIdx,
        asm: &mut Assembly,
    ) -> MethodRefIdx {
        let this = asm.alloc_class_ref(self.clone());
        let sig = asm.sig(inputs, output);
        asm.alloc_methodref(MethodRef::new(
            this,
            fn_name,
            sig,
            super::cilnode::MethodKind::Static,
            [].into(),
        ))
    }
    // Returns a `System.Collections.Concurrent.ConcurrentDictionary` of key,value
    pub fn concurent_dictionary(key: Type, value: Type, asm: &mut Assembly) -> ClassRefIdx {
        let name: StringIdx =
            asm.alloc_string("System.Collections.Concurrent.ConcurrentDictionary");
        let asm_name = Some(asm.alloc_string("System.Collections.Concurrent"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [key, value].into()))
    }
    // Returns a `System.Collections.Generic.Dictionary` of key,value
    pub fn dictionary(key: Type, value: Type, asm: &mut Assembly) -> ClassRefIdx {
        let name: StringIdx = asm.alloc_string("System.Collections.Generic.Dictionary");
        let asm_name = Some(asm.alloc_string("System.Collections"));
        asm.alloc_class_ref(ClassRef::new(name, asm_name, false, [key, value].into()))
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct ClassDef {
    name: StringIdx,
    is_valuetype: bool,
    generics: u32,
    extends: Option<ClassRefIdx>,
    fields: Vec<(Type, StringIdx, Option<u32>)>,
    static_fields: Vec<(Type, StringIdx, bool)>,
    methods: Vec<MethodDefIdx>,
    access: Access,
    explict_size: Option<NonZeroU32>,
}
impl ClassDef {
    /// Checks if this class defition has a with the name and type.
    #[must_use]
    pub fn has_static_field(&self, fld_name: StringIdx, fld_tpe: Type) -> bool {
        self.static_fields
            .iter()
            .any(|(tpe, name, _)| *tpe == fld_tpe && *name == fld_name)
    }
    pub(crate) fn iter_types(&self) -> impl Iterator<Item = Type> + '_ {
        self.fields()
            .iter()
            .map(|(tpe, _, _)| tpe)
            .chain(self.static_fields().iter().map(|(tpe, _, _)| tpe))
            .copied()
            .chain(self.extends.iter().map(|cref| Type::ClassRef(*cref)))
    }
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        name: StringIdx,
        is_valuetype: bool,
        generics: u32,
        extends: Option<ClassRefIdx>,
        fields: Vec<(Type, StringIdx, Option<u32>)>,
        static_fields: Vec<(Type, StringIdx, bool)>,

        access: Access,
        explict_size: Option<NonZeroU32>,
    ) -> Self {
        //crate::utilis::assert_unique(&methods);
        Self {
            name,
            is_valuetype,
            generics,
            extends,
            fields,
            static_fields,
            methods: vec![],
            access,
            explict_size,
        }
    }

    pub(crate) fn ref_to(&self) -> ClassRef {
        assert_eq!(self.generics, 0);
        ClassRef::new(self.name, None, self.is_valuetype, vec![].into())
    }

    pub fn methods_mut(&mut self) -> &mut Vec<MethodDefIdx> {
        &mut self.methods
    }

    pub fn static_fields_mut(&mut self) -> &mut Vec<(Type, StringIdx, bool)> {
        &mut self.static_fields
    }
    pub fn fields_mut(&mut self) -> &mut Vec<(Type, StringIdx, Option<u32>)> {
        &mut self.fields
    }
    #[must_use]
    pub fn access(&self) -> &Access {
        &self.access
    }

    #[must_use]
    pub fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }

    #[must_use]
    pub fn extends(&self) -> Option<ClassRefIdx> {
        self.extends
    }

    pub(crate) fn has_explicit_layout(&self) -> bool {
        self.explict_size.is_some() || self.fields.iter().any(|(_, _, offset)| offset.is_some())
    }

    #[must_use]
    pub fn fields(&self) -> &[(Type, StringIdx, Option<u32>)] {
        &self.fields
    }

    #[must_use]
    pub fn name(&self) -> StringIdx {
        self.name
    }

    #[must_use]
    pub fn static_fields(&self) -> &[(Type, StringIdx, bool)] {
        &self.static_fields
    }

    #[must_use]
    pub fn methods(&self) -> &[MethodDefIdx] {
        &self.methods
    }

    #[must_use]
    pub fn explict_size(&self) -> Option<NonZeroU32> {
        self.explict_size
    }

    #[must_use]
    pub fn generics(&self) -> u32 {
        self.generics
    }

    pub(super) fn merge_defs(&mut self, translated: ClassDef) {
        // Check name matches
        assert_eq!(self.name(), translated.name());

        // Check valuetype matches
        assert_eq!(self.is_valuetype(), translated.is_valuetype());
        // Check generic count matches
        assert_eq!(self.generics(), translated.generics());
        // Check inheretence matches
        assert_eq!(self.extends(), translated.extends());

        // Merge the static fields, removing duplicates
        self.static_fields_mut().extend(translated.static_fields());
        make_unique(&mut self.static_fields);
        // Merge the methods, removing duplicates
        self.methods_mut().extend(translated.methods());
        make_unique(self.methods_mut());
        // Check accessibility matches
        assert_eq!(self.access(), translated.access());
    }

    pub fn add_def(&mut self, ref_idx: MethodDefIdx) {
        if self.methods.iter().any(|def| *def == ref_idx) {
            // Duplicate, skip I guess?
            // TODO: check if this duplicate matches the current function.
            return;
        }
        self.methods_mut().push(ref_idx);
    }
    /*
    /// Optimizes this class definition, consuming fuel
    pub fn opt(&mut self, fuel: &mut OptFuel, asm: &mut Assembly, cache: &mut SideEffectInfoCache) {
    } */
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ClassDefIdx(pub ClassRefIdx);

impl std::ops::Deref for ClassDefIdx {
    type Target = ClassRefIdx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn into_unique<T: Eq + std::hash::Hash>(input: Vec<T>) -> Vec<T> {
    let set: fxhash::FxHashSet<_> = input.into_iter().collect();
    set.into_iter().collect()
}
fn make_unique<T: Eq + std::hash::Hash>(input: &mut Vec<T>) {
    let mut tmp = Vec::new();
    std::mem::swap(&mut tmp, input);
    let mut tmp = into_unique(tmp);
    std::mem::swap(&mut tmp, input);
}
#[test]
fn test_into_unique() {
    assert!(into_unique::<u32>(vec![]).is_empty());
    assert_eq!(into_unique::<u32>(vec![0]), vec![0]);
    assert_eq!(into_unique::<u32>(vec![0, 0]), vec![0]);
    assert_eq!(into_unique::<u32>(vec![2, 1, 1]).len(), 2);
    let mut v = vec![];
    make_unique::<u32>(&mut v);
    assert!(v.is_empty());
    let mut v = vec![0];
    make_unique::<u32>(&mut v);
    assert_eq!(v, vec![0]);
    let mut v = vec![0, 1];
    make_unique::<u32>(&mut v);
    assert_eq!(v, vec![0, 1]);
    let mut v = vec![2, 1, 1];
    make_unique::<u32>(&mut v);
    assert_eq!(v.len(), 2);
}
#[test]
fn has_explicit_layout() {
    let vt = [true, false];
    for is_valuetype in vt {
        let mut asm = Assembly::default();
        let name = asm.alloc_string("MyClass");
        let def = ClassDef::new(
            name,
            is_valuetype,
            0,
            None,
            vec![],
            vec![],
            Access::Extern,
            None,
        );
        assert!(def.explict_size().is_none());
        assert!(!def.has_explicit_layout());
        assert_eq!(is_valuetype, def.is_valuetype());
        assert_eq!(is_valuetype, def.ref_to().is_valuetype());
        let def = ClassDef::new(
            name,
            is_valuetype,
            0,
            None,
            vec![],
            vec![],
            Access::Extern,
            Some(NonZeroU32::new(1000).unwrap()),
        );
        assert_eq!(def.fields().len(), 0);
        assert!(def.has_explicit_layout());
        assert_eq!(is_valuetype, def.is_valuetype());
        assert_eq!(is_valuetype, def.ref_to().is_valuetype());
        let def = ClassDef::new(
            name,
            is_valuetype,
            0,
            None,
            vec![(Type::Bool, name, Some(1000))],
            vec![],
            Access::Extern,
            None,
        );
        assert!(def.explict_size().is_none());
        assert_eq!(def.fields().len(), 1);
        assert!(def.has_explicit_layout());
        assert_eq!(is_valuetype, def.is_valuetype());
        assert_eq!(is_valuetype, def.ref_to().is_valuetype());
        let mut def = ClassDef::new(
            name,
            is_valuetype,
            0,
            None,
            vec![],
            vec![(Type::Bool, name, false)],
            Access::Extern,
            None,
        );
        assert!(def.explict_size().is_none());
        assert_eq!(def.static_fields().len(), 1);
        assert_eq!(def.static_fields_mut().len(), 1);
        assert!(def.has_static_field(name, Type::Bool));
        assert!(!def.has_static_field(name, Type::PlatformChar));
        assert!(!def.has_static_field(asm.alloc_string("CuteString"), Type::Bool));
        assert!(!def.has_explicit_layout());
        assert_eq!(is_valuetype, def.is_valuetype());
        assert_eq!(is_valuetype, def.ref_to().is_valuetype());
        let def = ClassDef::new(
            name,
            is_valuetype,
            0,
            None,
            vec![(Type::Bool, name, None)],
            vec![],
            Access::Extern,
            None,
        );
        assert!(def.explict_size().is_none());
        assert_eq!(def.fields().len(), 1);
        assert_eq!(is_valuetype, def.is_valuetype());
        assert_eq!(is_valuetype, def.ref_to().is_valuetype());
        assert!(!def.has_explicit_layout());
        let def = ClassDef::new(
            name,
            is_valuetype,
            0,
            None,
            vec![(Type::Bool, name, Some(1000))],
            vec![],
            Access::Extern,
            Some(NonZeroU32::new(1000).unwrap()),
        );
        assert_eq!(def.explict_size(), Some(NonZeroU32::new(1000).unwrap()));
        assert_eq!(def.fields().len(), 1);
        assert!(def.has_explicit_layout());
        assert_eq!(is_valuetype, def.is_valuetype());
        assert_eq!(is_valuetype, def.ref_to().is_valuetype());
    }
}
#[test]
fn generics() {
    let mut asm = Assembly::default();
    let name = asm.alloc_string("MyClass");
    let def = ClassDef::new(name, false, 0, None, vec![], vec![], Access::Extern, None);
    assert_eq!(def.generics(), 0);
    assert_eq!(def.ref_to().generics(), &[]);
    let def = ClassDef::new(name, false, 5, None, vec![], vec![], Access::Extern, None);
    assert_eq!(def.generics(), 5);
}
#[test]
fn display_class_ref() {
    let mut asm = Assembly::default();
    let name: StringIdx = asm.alloc_string("MyClass");
    let def = ClassDef::new(name, false, 0, None, vec![], vec![], Access::Extern, None);
    assert_eq!(
        def.ref_to().display(&asm),
        "ClassRef{name:MyClass,asm:None,is_valuetype:false,generics[]}"
    );
}
#[test]
fn type_gc() {
    let mut asm = Assembly::default();
    let name: StringIdx = asm.alloc_string("Stay");
    asm.class_def(ClassDef::new(
        name,
        false,
        0,
        None,
        vec![],
        vec![],
        Access::Extern,
        None,
    ));
    let name: StringIdx = asm.alloc_string("Gone");
    asm.class_def(ClassDef::new(
        name,
        false,
        0,
        None,
        vec![],
        vec![],
        Access::Public,
        None,
    ));
    assert_eq!(asm.class_defs().len(), 2);
    asm.eliminate_dead_types();
    assert_eq!(asm.class_defs().len(), 1);
}
#[test]
fn merge_defs() {
    let mut asm = Assembly::default();
    let name: StringIdx = asm.alloc_string("Stay");
    let def = ClassDef::new(name, false, 0, None, vec![], vec![], Access::Extern, None);

    def.clone().merge_defs(def);
}
#[test]
#[should_panic]
fn merge_defs_different() {
    let mut asm = Assembly::default();
    let name: StringIdx = asm.alloc_string("Stay");
    let mut stay = ClassDef::new(name, false, 0, None, vec![], vec![], Access::Extern, None);
    let name: StringIdx = asm.alloc_string("Gone");
    let gone = ClassDef::new(name, false, 0, None, vec![], vec![], Access::Public, None);

    stay.merge_defs(gone);
}
#[test]
fn extends() {
    let mut asm = Assembly::default();
    let name: StringIdx = asm.alloc_string("Stay");
    let exception = ClassRef::exception(&mut asm);
    let def = ClassDef::new(name, false, 0, None, vec![], vec![], Access::Extern, None);
    assert_eq!(def.iter_types().count(), 0);
    assert!(def.extends().is_none());
    let def = ClassDef::new(
        name,
        false,
        0,
        Some(exception),
        vec![],
        vec![],
        Access::Extern,
        None,
    );
    assert_eq!(def.extends(), Some(exception));
    assert_eq!(def.iter_types().count(), 1);
}
#[test]
fn class_ref() {
    let mut asm = Assembly::default();
    let names = [
        asm.alloc_string("CuteClass"),
        asm.alloc_string("SpookyClass"),
        asm.alloc_string("BraveClass"),
    ];
    let asms = [
        None,
        Some(asm.alloc_string("NiceAssembly")),
        Some(asm.alloc_string("GreatAssembly")),
    ];
    let valuetypes = [false, true];
    let generics = [
        vec![],
        vec![Type::Bool],
        vec![Type::Bool, Type::PlatformObject],
    ];
    for name in names {
        for asm in asms {
            for valuetype in valuetypes {
                for generic in &generics {
                    let cref = ClassRef::new(name, asm, valuetype, generic.clone().into());
                    assert_eq!(cref.name(), name);
                    assert_eq!(cref.asm(), asm);
                    assert_eq!(cref.is_valuetype(), valuetype);
                    assert_eq!(cref.generics(), generic);
                }
            }
        }
    }
}
