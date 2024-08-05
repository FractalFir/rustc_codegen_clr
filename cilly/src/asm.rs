use fxhash::{FxBuildHasher, FxHashMap};

use serde::{Deserialize, Serialize};

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call_site::CallSite,
    cil_node::{CILNode, CallOpArgs},
    cil_root::CILRoot,
    method::{Method, MethodType},
    static_field_desc::StaticFieldDescriptor,
    type_def::TypeDef,
    utilis::MemoryUsage,
    DotnetTypeRef, FnSig, IString, Type,
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
impl MemoryUsage for AssemblyExternRef {
    fn memory_usage(&self, counter: &mut impl crate::utilis::MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let name = std::any::type_name::<Self>();
        let version = self.version.memory_usage(counter);
        counter.add_field(name, "version", version);
        total_size += version;
        counter.add_type(name, total_size);
        total_size
    }
}
impl AssemblyExternRef {
    /// Returns the version information of this assembly.
    #[must_use]
    pub fn version(&self) -> (u16, u16, u16, u16) {
        self.version
    }
}
pub type ExternFnDef = (IString, FnSig, bool);
#[derive(Serialize, Deserialize, Debug)]
/// Representation of a .NET assembly.
pub struct Assembly {
    /// List of types desined within the assembly.
    types: FxHashMap<IString, TypeDef>,
    /// List of functions defined within this assembly.
    functions: FxHashMap<CallSite, Method>,
    /// Callsite representing the entrypoint of this assebmly if any present.
    entrypoint: Option<CallSite>,
    /// List of references to external assemblies
    extern_refs: FxHashMap<IString, AssemblyExternRef>,
    extern_fns: FxHashMap<ExternFnDef, IString>,
    /// List of all static fields within the assembly
    static_fields: FxHashMap<IString, (Type, bool)>,
    /// Initializers. Call order not guarnateed(but should match the order they are added in), but should be called after most of `.cctor` runs.
    initializers: Vec<CILRoot>,
}
impl Assembly {
    /// Returns the `.cctor` function used to initialize static data
    #[must_use]
    pub fn cctor(&self) -> Option<&Method> {
        self.functions.get(&CallSite::new(
            None,
            ".cctor".into(),
            FnSig::new(&[], Type::Void),
            true,
        ))
    }
    /// Returns the `.tcctor` function used to initialize thread-local static data
    #[must_use]
    pub fn tcctor(&self) -> Option<&Method> {
        self.functions.get(&CallSite::new(
            None,
            ".tcctor".into(),
            FnSig::new(&[], Type::Void),
            true,
        ))
    }
    /// Returns the external assembly reference
    #[must_use]
    pub fn extern_refs(&self) -> &FxHashMap<IString, AssemblyExternRef> {
        &self.extern_refs
    }
    /// Creates a new, empty assembly.
    #[must_use]
    pub fn empty() -> Self {
        let mut res = Self {
            types: FxHashMap::with_hasher(FxBuildHasher::default()),
            functions: FxHashMap::with_hasher(FxBuildHasher::default()),
            entrypoint: None,
            extern_refs: FxHashMap::with_hasher(FxBuildHasher::default()),
            static_fields: FxHashMap::with_hasher(FxBuildHasher::default()),
            extern_fns: FxHashMap::with_hasher(FxBuildHasher::default()),
            initializers: vec![],
        };
        res.static_fields.insert(
            "GlobalAtomicLock".into(),
            (
                Type::DotnetType(Box::new(DotnetTypeRef::object_type())),
                false,
            ),
        );
        let dotnet_ver = AssemblyExternRef {
            version: (6, 12, 0, 0),
        };
        res.extern_refs.insert("System.Runtime".into(), dotnet_ver);
        //res.extern_refs.insert("mscorlib".into(),dotnet_ver);
        res.extern_refs
            .insert("System.Runtime.InteropServices".into(), dotnet_ver);
        // Needed to get C-Mode to work
        res.add_cctor();
        res.add_tcctor();
        res
    }
    /// Joins 2 assemblies together.
    #[must_use]
    pub fn join(self, other: Self) -> Self {
        let static_initializer = link_static_initializers(self.cctor(), other.cctor());
        let tcctor = link_static_initializers(self.tcctor(), other.tcctor());
        let mut types = self.types;
        types.extend(other.types);
        let mut functions = self.functions;
        functions.extend(other.functions);
        if let Some(static_initializer) = static_initializer {
            functions.insert(static_initializer.call_site(), static_initializer);
        }
        if let Some(tcctor) = tcctor {
            functions.insert(tcctor.call_site(), tcctor);
        }
        let entrypoint = self.entrypoint.or(other.entrypoint);
        let mut extern_refs = self.extern_refs;
        let mut static_fields = self.static_fields;
        let mut extern_fns = self.extern_fns;
        static_fields.extend(other.static_fields);
        extern_refs.extend(other.extern_refs);
        extern_fns.extend(other.extern_fns);
        let mut initializers = self.initializers;
        initializers.extend(other.initializers);

        Self {
            types,
            functions,
            entrypoint,
            extern_refs,
            extern_fns,
            static_fields,
            initializers,
        }
    }

    /// Adds a global static field named *name* of type *tpe*
    pub fn add_static(&mut self, tpe: Type, name: &str, thread_local: bool) {
        self.static_fields.insert(name.into(), (tpe, thread_local));
    }
    pub fn add_cctor(&mut self) -> &mut Method {
        self.functions
            .entry(CallSite::new(
                None,
                ".cctor".into(),
                FnSig::new(&[], Type::Void),
                true,
            ))
            .or_insert_with(|| {
                Method::new(
                    AccessModifer::Extern,
                    MethodType::Static,
                    FnSig::new(&[], Type::Void),
                    ".cctor",
                    vec![
                        (None, Type::Ptr(Type::U8.into())),
                        (None, Type::Ptr(Type::U8.into())),
                    ],
                    vec![BasicBlock::new(
                        vec![
                            CILRoot::SetStaticField {
                                descr: Box::new(StaticFieldDescriptor::new(
                                    None,
                                    Type::DotnetType(Box::new(DotnetTypeRef::object_type())),
                                    "GlobalAtomicLock".into(),
                                )),
                                value: CILNode::NewObj(Box::new(CallOpArgs {
                                    args: [].into(),
                                    site: Box::new(CallSite::new(
                                        Some(DotnetTypeRef::object_type()),
                                        ".ctor".into(),
                                        FnSig::new(
                                            &[Type::DotnetType(Box::new(
                                                DotnetTypeRef::object_type(),
                                            ))],
                                            Type::Void,
                                        ),
                                        false,
                                    )),
                                })),
                            }
                            .into(),
                            CILRoot::VoidRet.into(),
                        ],
                        0,
                        None,
                    )],
                    vec![],
                )
            })
    }
    /// Addds a per-thread static initailzer
    pub fn add_tcctor(&mut self) -> &mut Method {
        self.functions
            .entry(CallSite::new(
                None,
                ".tcctor".into(),
                FnSig::new(&[], Type::Void),
                true,
            ))
            .or_insert_with(|| {
                Method::new(
                    AccessModifer::Extern,
                    MethodType::Static,
                    FnSig::new(&[], Type::Void),
                    ".tcctor",
                    vec![
                        (None, Type::Ptr(Type::U8.into())),
                        (None, Type::Ptr(Type::U8.into())),
                    ],
                    vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
                    vec![],
                )
            })
    }

    /// Returns true if assembly contains function named `name`
    #[must_use]
    pub fn contains_fn(&self, site: &CallSite) -> bool {
        self.functions.contains_key(site)
    }
    /// Adds a method to the assebmly.
    pub fn add_method(&mut self, method: Method) {
        if let Err(err) = method.validate() {
            eprintln!(
                "Could not validate the method {name} because {err}",
                name = method.name()
            );
        }

        let cs = method.call_site();

        self.functions.insert(cs, method);
    }

    /// Returns an interator over all methods within the assembly.
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.values()
    }
    /// Returns an interator over all methods within the assembly.
    pub fn methods_mut(&mut self) -> impl Iterator<Item = &mut Method> {
        self.functions.values_mut()
    }
    /// Returns an iterator over all types witin the assembly.
    pub fn types(&self) -> impl Iterator<Item = (&IString, &TypeDef)> {
        self.types.iter()
    }
    /// Optimizes all the methods witin the assembly.
    pub fn opt(&mut self) {
        self.functions.iter_mut().for_each(|method| {
            let (_site, method) = method;
            let mut method = method.clone();
            method.opt();
            //crate::opt::opt_method(&mut method, self);
        });
    }
    /// Adds a definition of a type to the assembly.
    pub fn add_typedef(&mut self, mut type_def: TypeDef) {
        self.types.insert(type_def.name().into(), type_def);
    }

    /// Sets the entrypoint of the assembly to the method behind `CallSite`.
    pub fn set_entrypoint(&mut self, entrypoint: &CallSite) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        let wrapper = crate::entrypoint::wrapper(entrypoint);
        self.entrypoint = Some(wrapper.call_site());
        self.add_method(wrapper);
    }

    #[must_use]
    pub fn extern_fns(&self) -> &FxHashMap<ExternFnDef, IString> {
        &self.extern_fns
    }

    pub fn add_extern_fn(&mut self, name: IString, sig: FnSig, lib: IString, preserve_errno: bool) {
        self.extern_fns.insert((name, sig, preserve_errno), lib);
    }

    pub fn add_initialzer(&mut self, root: CILRoot) {
        self.initializers.push(root);
    }
    pub fn cctor_mut(&mut self) -> Option<&mut Method> {
        self.functions.get_mut(&CallSite::new(
            None,
            ".cctor".into(),
            FnSig::new(&[], Type::Void),
            true,
        ))
    }

    pub fn static_fields_mut(&mut self) -> &mut FxHashMap<IString, (Type, bool)> {
        &mut self.static_fields
    }

    pub fn functions(&self) -> &FxHashMap<CallSite, Method> {
        &self.functions
    }

    pub fn initializers(&self) -> &[CILRoot] {
        &self.initializers
    }

    pub fn static_fields(&self) -> &FxHashMap<IString, (Type, bool)> {
        &self.static_fields
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
fn link_static_initializers(a: Option<&Method>, b: Option<&Method>) -> Option<Method> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (Some(a), Some(b)) => {
            let mut merged: Method = a.clone();
            let mut blocks = merged.blocks_mut();
            let trees = blocks[0].trees_mut();
            trees.pop();
            trees.extend(b.blocks()[0].trees().iter().cloned());
            drop(blocks);
            Some(merged)
        }
    }
}
impl MemoryUsage for Assembly {
    fn memory_usage(&self, counter: &mut impl crate::utilis::MemoryUsageCounter) -> usize {
        let self_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let types = self.types.memory_usage(counter);
        counter.add_field(tpe_name, "types", types);
        let functions = self.functions.memory_usage(counter);
        counter.add_field(tpe_name, "types", functions);
        let extern_fns = self.extern_fns.memory_usage(counter);
        counter.add_field(tpe_name, "extern_fns", extern_fns);
        let extern_refs = self.extern_refs.memory_usage(counter);
        counter.add_field(tpe_name, "extern_refs", extern_refs);
        let entrypoint = self.entrypoint.memory_usage(counter);
        counter.add_field(tpe_name, "entrypoint", entrypoint);
        let initializers = self.initializers.memory_usage(counter);
        counter.add_field(tpe_name, "initializers", initializers);
        let total_size =
            self_size + functions + extern_fns + extern_refs + entrypoint + initializers;
        counter.add_type(tpe_name, total_size);
        total_size
    }
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
