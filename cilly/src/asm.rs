use std::ops::DerefMut;

use fxhash::{FxBuildHasher, FxHashMap};

use serde::{Deserialize, Serialize};

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    cil_root::CILRoot,
    method::{Method, MethodType},
    v2::{cilnode::MethodKind, ClassDef, FnSig, Int, MethodRef, MethodRefIdx},
    IString, Type,
};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
/// Data representing a reference to an external assembly.
pub struct AssemblyExternRef {
    /// A tuple describing the referenced assebmly.
    /// Tuple contains:
    /// (Major Version, Minor Version, Revision number, Build number)
    /// In that order.
    version: (u16, u16, u16, u16),
}

impl AssemblyExternRef {
    /// Returns the version information of this assembly.
    #[must_use]
    pub fn version(&self) -> (u16, u16, u16, u16) {
        self.version
    }
}
pub type ExternFnDef = (IString, FnSig, bool);
#[derive(Serialize, Deserialize)]
/// Representation of a .NET assembly.
pub struct Assembly {
    /// List of functions defined within this assembly.
    functions: FxHashMap<MethodRefIdx, Method>,
    /// MethodRefIdx representing the entrypoint of this assebmly if any present.
    entrypoint: Option<MethodRefIdx>,
    /// List of references to external assemblies
    extern_refs: FxHashMap<IString, AssemblyExternRef>,
    extern_fns: FxHashMap<ExternFnDef, IString>,
    /// List of all static fields within the assembly
    static_fields: FxHashMap<IString, (Type, bool)>,
    /// Initializers. Call order not guarnateed(but should match the order they are added in), but should be called after most of `.cctor` runs.
    initializers: Vec<CILRoot>,
    // Inner v2 assembly
    inner: super::v2::Assembly,
}

impl __Deref for Assembly {
    type Target = super::v2::Assembly;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Assembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl Assembly {
    /// Returns the external assembly reference
    #[must_use]
    pub fn extern_refs(&self) -> &FxHashMap<IString, AssemblyExternRef> {
        &self.extern_refs
    }
    /// Creates a new, empty assembly.
    #[must_use]
    pub fn empty() -> Self {
        let mut res = Self {
            functions: FxHashMap::with_hasher(FxBuildHasher::default()),
            entrypoint: None,
            extern_refs: FxHashMap::with_hasher(FxBuildHasher::default()),
            static_fields: FxHashMap::with_hasher(FxBuildHasher::default()),
            extern_fns: FxHashMap::with_hasher(FxBuildHasher::default()),
            initializers: vec![],
            inner: Default::default(),
        };

        let dotnet_ver = AssemblyExternRef {
            version: (6, 12, 0, 0),
        };
        res.extern_refs.insert("System.Runtime".into(), dotnet_ver);
        //res.extern_refs.insert("mscorlib".into(),dotnet_ver);
        res.extern_refs
            .insert("System.Runtime.InteropServices".into(), dotnet_ver);
        // Needed to get C-Mode to work

        res
    }

    /// Adds a global static field named *name* of type *tpe*
    pub fn add_static(&mut self, tpe: Type, name: &str, thread_local: bool) {
        self.static_fields.insert(name.into(), (tpe, thread_local));
    }

    /// Addds a per-thread static initailzer
    pub fn add_tcctor(&mut self) -> &mut Method {
        let mref = MethodRef::new(
            *self.main_module(),
            self.alloc_string(".tcctor"),
            self.alloc_sig(FnSig::new(Box::new([]), Type::Void)),
            MethodKind::Static,
            vec![].into(),
        );
        let mref = self.alloc_methodref(mref);
        self.functions.entry(mref).or_insert_with(|| {
            Method::new(
                AccessModifer::Extern,
                MethodType::Static,
                FnSig::new(Box::new([]), Type::Void),
                ".tcctor",
                vec![
                    (None, self.inner.nptr(Type::Int(Int::U8))),
                    (None, self.inner.nptr(Type::Int(Int::U8))),
                ],
                vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
                vec![],
            )
        })
    }

    /// Returns true if assembly contains function named `name`
    #[must_use]
    pub fn contains_fn(&self, site: MethodRefIdx) -> bool {
        self.functions.contains_key(&site)
    }
    /// Adds a method to the assebmly.
    pub fn add_method(&mut self, method: Method) {
        let main_module = self.inner.main_module();
        let def = crate::v2::MethodDef::from_v1(&method, &mut self.inner, main_module);
        //def.optimize(&mut self.inner, &mut SideEffectInfoCache::default(), 4);
        self.inner.new_method(def);
    }

    /// Adds a definition of a type to the assembly.
    pub fn add_typedef(&mut self, type_def: ClassDef) {
        let cref = self.alloc_class_ref(type_def.ref_to());
        if !self.inner().contains_def(cref) {
            self.inner.class_def(type_def);
        }
    }

    /// Sets the entrypoint of the assembly to the method behind `MethodRefIdx`.
    pub fn set_entrypoint(&mut self, entrypoint: MethodRefIdx) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        let wrapper = crate::entrypoint::wrapper(self[entrypoint].clone(), self.inner_mut());
        self.entrypoint = Some(wrapper.call_site(self));
        self.add_method(wrapper);
    }

    #[must_use]
    pub(crate) fn extern_fns(&self) -> &FxHashMap<ExternFnDef, IString> {
        &self.extern_fns
    }

    pub fn add_initialzer(&mut self, root: CILRoot) {
        self.initializers.push(root);
    }
    pub fn cctor_mut(&mut self) -> Option<&mut Method> {
        let mref = MethodRef::new(
            *self.main_module(),
            self.alloc_string(".cctor"),
            self.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let mref = self.alloc_methodref(mref);
        self.functions.get_mut(&mref)
    }

    pub(crate) fn functions(&self) -> &FxHashMap<MethodRefIdx, Method> {
        &self.functions
    }

    pub(crate) fn initializers(&self) -> &[CILRoot] {
        &self.initializers
    }

    pub fn static_fields(&self) -> &FxHashMap<IString, (Type, bool)> {
        &self.static_fields
    }

    pub(crate) fn inner(&self) -> &super::v2::Assembly {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut super::v2::Assembly {
        &mut self.inner
    }
}
use lazy_static::*;
lazy_static! {
    #[doc = "Tells the codegen to remove dead code before export."]pub static ref DEAD_CODE_ELIMINATION:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(DEAD_CODE_ELIMINATION){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(DEAD_CODE_ELIMINATION),value),
        }).unwrap_or(true)
    };
}

lazy_static! {
    #[doc = "Tells the codegen to use the new version of cilly."]
    pub static ref CILLY_V2:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(CILLY_V2){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(CILLY_V2),value),
        }).unwrap_or(false)
    };
}
