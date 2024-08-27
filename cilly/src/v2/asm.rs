use super::{
    bimap::{calculate_hash, BiMap, BiMapIndex, IntoBiMapIndex},
    cilnode::{BinOp, MethodKind, UnOp},
    opt::{OptFuel, SideEffectInfoCache},
    Access, CILNode, CILRoot, ClassDef, ClassDefIdx, ClassRef, ClassRefIdx, Const, Exporter,
    FieldDesc, FieldIdx, FnSig, MethodDef, MethodDefIdx, MethodRef, MethodRefIdx, NodeIdx, RootIdx,
    SigIdx, StaticFieldDesc, StaticFieldIdx, StringIdx, Type, TypeIdx,
};
use crate::IString;
use crate::{asm::Assembly as V1Asm, v2::MethodImpl};
use fxhash::{FxHashMap, FxHashSet};
use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Default, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub(super) struct IStringWrapper(pub(super) IString);
impl std::hash::Hash for IStringWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        calculate_hash(&0xC0FE_BEEFu32).hash(state);
        for char in self.0.chars() {
            calculate_hash(&char).hash(state);
        }
    }
}
pub type MissingMethodPatcher =
    FxHashMap<StringIdx, Box<dyn Fn(MethodRefIdx, &mut Assembly) -> MethodImpl>>;
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Assembly {
    /// A list of strings used in this assembly
    strings: BiMap<StringIdx, IStringWrapper>,
    /// A list of all types in this assembly
    types: BiMap<TypeIdx, Type>,
    class_refs: BiMap<ClassRefIdx, ClassRef>,
    class_defs: FxHashMap<ClassDefIdx, ClassDef>,
    nodes: BiMap<NodeIdx, CILNode>,
    roots: BiMap<RootIdx, CILRoot>,
    sigs: BiMap<SigIdx, FnSig>,
    method_refs: BiMap<MethodRefIdx, MethodRef>,
    fields: BiMap<FieldIdx, FieldDesc>,
    statics: BiMap<StaticFieldIdx, StaticFieldDesc>,
    method_defs: FxHashMap<MethodDefIdx, MethodDef>,
    // Cache containing information about the stack usage of a CIL node.
    //#[serde(skip)]
    //cache: CachedAssemblyInfo<NodeIdx, NonMaxU32, StackUsage>,
}
impl Assembly {
    pub fn class_defs(&self) -> &FxHashMap<ClassDefIdx, ClassDef> {
        &self.class_defs
    }
    pub fn method_ref_to_def(&self, class: MethodRefIdx) -> Option<MethodDefIdx> {
        if self.method_defs.contains_key(&MethodDefIdx(class)) {
            Some(MethodDefIdx(class))
        } else {
            None
        }
    }
    pub fn fuel_from_env(&self) -> OptFuel {
        match std::env::var("OPT_FUEL") {
            Ok(fuel) => match fuel.parse::<u32>() {
                Ok(fuel) => OptFuel::new(fuel),
                Err(_) => self.default_fuel(),
            },
            Err(_) => self.default_fuel(),
        }
    }
    pub fn default_fuel(&self) -> OptFuel {
        OptFuel::new((self.method_defs.len() * 4 + self.roots.len() * 4) as u32)
    }
    pub(crate) fn borrow_methoddef(&mut self, def_id: MethodDefIdx) -> MethodDef {
        self.method_defs.remove(&def_id).unwrap()
    }
    pub(crate) fn return_methoddef(&mut self, def_id: MethodDefIdx, def: MethodDef) {
        assert!(
            self.method_defs.insert(def_id, def).is_none(),
            "Could not return a methoddef, because a method def is already present."
        );
    }
    /// Optimizes the assembly uitill all fuel is consumed, or no more progress can be made
    pub fn opt(&mut self, fuel: &mut OptFuel) {
        let mut cache = SideEffectInfoCache::default();
        while !fuel.exchausted() {
            let prev = fuel.clone();
            self.opt_sigle_pass(fuel, &mut cache);
            // No fuel consumed, progress can't be made, break.
            if *fuel == prev {
                break;
            }
            let _pass_min_cost: bool = fuel.consume(1);
        }
    }
    /// Optimizes the assembly, cosuming some fuel. This performs a single optimization pass.
    pub fn opt_sigle_pass(&mut self, fuel: &mut OptFuel, cache: &mut SideEffectInfoCache) {
        let method_def_idxs: Box<[_]> = self.method_defs.keys().copied().collect();
        for method in method_def_idxs {
            let mut tmp_method = self.borrow_methoddef(method);
            tmp_method.optimize(self, cache, fuel);
            self.return_methoddef(method, tmp_method);
            if fuel.exchausted() {
                break;
            }
        }
    }
    /// Finds all methods matching the closure
    pub fn methods_with<'a>(
        &'a self,
        mut filter: impl FnMut(&Self, MethodDefIdx, &MethodDef) -> bool + 'a,
    ) -> impl Iterator<Item = (&MethodDefIdx, &MethodDef)> + 'a {
        self.method_defs
            .iter()
            .filter(move |(id, def)| filter(self, **id, def))
    }
    /// Modifies the method deifinition by running the closure on it

    pub fn modify_methodef(
        &mut self,
        modify: impl FnOnce(&mut Self, &mut MethodDef),
        def_id: MethodDefIdx,
    ) {
        let mut borrowed = self.borrow_methoddef(def_id);
        modify(self, &mut borrowed);
        self.return_methoddef(def_id, borrowed);
    }
    pub fn find_methods_matching<'a, P: std::str::pattern::Pattern + Clone + 'a>(
        &self,
        pat: P,
    ) -> Option<impl Iterator<Item = MethodDefIdx> + '_> {
        let names: Box<[StringIdx]> = self.find_strs_containing(pat).collect();
        Some(self.method_defs.iter().filter_map(move |(mdefidx, mdef)| {
            if names.iter().any(|name| *name == mdef.name()) {
                Some(*mdefidx)
            } else {
                None
            }
        }))
    }
    pub fn find_strs_containing<'a, P: std::str::pattern::Pattern + Clone + 'a>(
        &'a self,
        pat: P,
    ) -> impl Iterator<Item = StringIdx> + '_ {
        self.strings
            .0
            .iter()
            .enumerate()
            .filter_map(move |(idx, str)| {
                if str.0.contains(pat.clone()) {
                    Some(StringIdx(BiMapIndex::new((idx + 1) as u32).unwrap()))
                } else {
                    None
                }
            })
    }
    pub fn get_prealllocated_string(&self, string: impl Into<IString>) -> Option<StringIdx> {
        self.strings.1.get(&IStringWrapper(string.into())).copied()
    }
    pub fn class_mut(&mut self, id: ClassDefIdx) -> &mut ClassDef {
        self.class_defs.get_mut(&id).unwrap()
    }
    pub fn get_class_def(&self, id: ClassDefIdx) -> &ClassDef {
        self.class_defs.get(&id).unwrap()
    }
    pub fn class_ref(&self, cref: ClassRefIdx) -> &ClassRef {
        self.class_refs.get(cref)
    }
    pub fn method_def(&self, dref: MethodDefIdx) -> &MethodDef {
        self.method_defs.get(&dref).unwrap()
    }
    pub fn alloc_string(&mut self, string: impl Into<IString>) -> StringIdx {
        self.strings.alloc(IStringWrapper(string.into()))
    }
    pub fn get_string(&self, key: StringIdx) -> &IString {
        &self.strings.get(key).0
    }
    pub fn sig(&mut self, input: impl Into<Box<[Type]>>, output: impl Into<Type>) -> SigIdx {
        self.sigs.alloc(FnSig::new(input.into(), output.into()))
    }
    pub fn fn_ptr(&mut self, input: impl Into<Box<[Type]>>, output: impl Into<Type>) -> Type {
        let sig = self.sig(input, output);
        Type::FnPtr(sig)
    }
    pub fn get_sig(&self, key: SigIdx) -> &FnSig {
        self.sigs.get(key)
    }
    pub fn nptr(&mut self, inner: Type) -> Type {
        Type::Ptr(self.types.alloc(inner))
    }
    pub fn nref(&mut self, inner: Type) -> Type {
        Type::Ref(self.types.alloc(inner))
    }
    pub fn get_type(&self, idx: TypeIdx) -> &Type {
        self.types.get(idx)
    }
    pub fn get_mref(&self, idx: MethodRefIdx) -> &MethodRef {
        self.method_refs.get(idx)
    }
    pub fn get_root(&self, root: RootIdx) -> &CILRoot {
        self.roots.get(root)
    }
    pub fn biop(&mut self, lhs: impl Into<CILNode>, rhs: impl Into<CILNode>, op: BinOp) -> CILNode {
        let lhs = self.nodes.alloc(lhs.into());
        let rhs = self.nodes.alloc(rhs.into());
        CILNode::BinOp(lhs, rhs, op)
    }
    pub fn unop(&mut self, val: impl Into<CILNode>, op: UnOp) -> CILNode {
        let val = self.nodes.alloc(val.into());
        CILNode::UnOp(val, op)
    }
    pub fn ldstr(&mut self, msg: impl Into<IString>) -> CILNode {
        CILNode::Const(Box::new(Const::PlatformString(self.alloc_string(msg))))
    }
    pub fn strct(&mut self, name: IString) -> ClassRefIdx {
        let class = ClassRef::new(self.alloc_string(name), None, true, vec![].into());
        self.class_refs.alloc(class)
    }

    pub fn alloc_node(&mut self, node: impl Into<CILNode>) -> NodeIdx {
        self.nodes.alloc(node.into())
    }

    pub fn alloc_class_ref(&mut self, cref: ClassRef) -> ClassRefIdx {
        self.class_refs.alloc(cref)
    }

    pub fn alloc_sig(&mut self, sig: FnSig) -> SigIdx {
        self.sigs.alloc(sig)
    }

    pub fn alloc_methodref(&mut self, method_ref: MethodRef) -> MethodRefIdx {
        self.method_refs.alloc(method_ref)
    }
    pub fn new_methodref(
        &mut self,
        class: ClassRefIdx,
        name: impl Into<IString>,
        sig: SigIdx,
        kind: MethodKind,
        generics: impl Into<Box<[Type]>>,
    ) -> MethodRefIdx {
        let name = self.alloc_string(name);

        self.alloc_methodref(MethodRef::new(class, name, sig, kind, generics.into()))
    }
    pub fn alloc_root(&mut self, val: CILRoot) -> RootIdx {
        self.roots.alloc(val)
    }

    pub fn alloc_type(&mut self, tpe: impl Into<Type>) -> TypeIdx {
        self.types.alloc(tpe.into())
    }

    pub(crate) fn get_node(&self, key: NodeIdx) -> &CILNode {
        self.nodes.get(key)
    }

    pub(crate) fn alloc_field(&mut self, field: FieldDesc) -> FieldIdx {
        self.fields.alloc(field)
    }
    pub fn get_field(&self, key: FieldIdx) -> &FieldDesc {
        self.fields.get(key)
    }
    pub fn alloc_sfld(&mut self, sfld: StaticFieldDesc) -> StaticFieldIdx {
        self.statics.alloc(sfld)
    }
    pub fn get_static_field(&self, key: StaticFieldIdx) -> &StaticFieldDesc {
        self.statics.get(key)
    }
    pub fn add_static(
        &mut self,
        tpe: Type,
        name: impl Into<IString>,
        thread_local: bool,
        in_class: ClassDefIdx,
    ) -> StaticFieldIdx {
        let name = self.alloc_string(name);
        let sfld = StaticFieldDesc::new(*in_class, name, tpe);
        let idx = self.alloc_sfld(sfld);
        if !self
            .class_mut(in_class)
            .static_fields()
            .contains(&(tpe, name, thread_local))
        {
            self.class_mut(in_class)
                .static_fields_mut()
                .push((tpe, name, thread_local));
        }

        idx
    }
    /// Adds a new class definition to this type
    pub fn class_def(&mut self, def: ClassDef) -> ClassDefIdx {
        let cref = def.ref_to();
        let cref = self.alloc_class_ref(cref);

        if self.class_defs.contains_key(&ClassDefIdx(cref)) {
            if self.get_string(def.name()).contains("core.ffi.c_void") {
                return ClassDefIdx(cref);
            }
            panic!()
        }
        self.class_defs.insert(ClassDefIdx(cref), def.clone());
        ClassDefIdx(cref)
    }
    pub fn main_module(&mut self) -> ClassDefIdx {
        let main_module = self.alloc_string(MAIN_MODULE);

        let class_def = ClassDef::new(
            main_module,
            false,
            0,
            None,
            vec![],
            vec![],
            vec![],
            Access::Public,
            None,
        );
        let cref = class_def.ref_to();
        let cref = self.class_refs.alloc(cref);
        // Check if that definition already exists
        if self.class_defs.contains_key(&ClassDefIdx(cref)) {
            ClassDefIdx(cref)
        } else {
            self.class_def(class_def)
        }
    }
    /// Adds a method definition to this assembly.
    pub fn new_method(&mut self, def: MethodDef) -> MethodDefIdx {
        let mref = def.ref_to();
        let def_class = def.class();
        let ref_idx = self.alloc_methodref(mref);
        // Check that this def is unique
        self.class_defs
            .get_mut(&def_class)
            .expect("Method added without a class")
            .add_def(MethodDefIdx(ref_idx));

        self.method_defs.insert(MethodDefIdx(ref_idx), def);

        MethodDefIdx(ref_idx)
    }
    pub fn user_init(&mut self) -> MethodDefIdx {
        let main_module = self.main_module();
        let user_init = self.alloc_string(USER_INIT);
        let ctor_sig = self.sig([], Type::Void);
        let mref = MethodRef::new(
            *main_module,
            user_init,
            ctor_sig,
            MethodKind::Static,
            vec![].into(),
        );
        let mref = self.alloc_methodref(mref);
        if self.method_defs.contains_key(&MethodDefIdx(mref)) {
            MethodDefIdx(mref)
        } else {
            let mimpl = MethodImpl::MethodBody {
                blocks: vec![super::BasicBlock::new(
                    vec![self.alloc_root(CILRoot::VoidRet)],
                    0,
                    None,
                )],
                locals: vec![],
            };
            let cctor_def = MethodDef::new(
                Access::Extern,
                main_module,
                user_init,
                ctor_sig,
                MethodKind::Static,
                mimpl,
                vec![],
            );
            self.new_method(cctor_def)
        }
    }
    /// Returns a reference to tht thread local constructor.
    pub fn tcctor(&mut self) -> MethodDefIdx {
        let main_module = self.main_module();
        let user_init = self.alloc_string(TCCTOR);
        let ctor_sig = self.sig([], Type::Void);
        let mref = MethodRef::new(
            *main_module,
            user_init,
            ctor_sig,
            MethodKind::Static,
            vec![].into(),
        );
        let mref = self.alloc_methodref(mref);
        if self.method_defs.contains_key(&MethodDefIdx(mref)) {
            MethodDefIdx(mref)
        } else {
            let mimpl = MethodImpl::MethodBody {
                blocks: vec![super::BasicBlock::new(
                    vec![self.alloc_root(CILRoot::VoidRet)],
                    0,
                    None,
                )],
                locals: vec![],
            };
            let cctor_def = MethodDef::new(
                Access::Extern,
                main_module,
                user_init,
                ctor_sig,
                MethodKind::Static,
                mimpl,
                vec![],
            );
            self.new_method(cctor_def)
        }
    }
    /// Returns a reference to the static initializer
    pub fn cctor(&mut self) -> MethodDefIdx {
        let main_module = self.main_module();
        let user_init = self.alloc_string(CCTOR);
        let ctor_sig = self.sig([], Type::Void);
        let mref = MethodRef::new(
            *main_module,
            user_init,
            ctor_sig,
            MethodKind::Static,
            vec![].into(),
        );
        let mref = self.alloc_methodref(mref);
        if self.method_defs.contains_key(&MethodDefIdx(mref)) {
            MethodDefIdx(mref)
        } else {
            let mimpl = MethodImpl::MethodBody {
                blocks: vec![super::BasicBlock::new(
                    vec![self.alloc_root(CILRoot::VoidRet)],
                    0,
                    None,
                )],
                locals: vec![],
            };
            let cctor_def = MethodDef::new(
                Access::Extern,
                main_module,
                user_init,
                ctor_sig,
                MethodKind::Static,
                mimpl,
                vec![],
            );
            self.new_method(cctor_def)
        }
    }
    /// Adds new rooots to the user init list.
    pub fn add_user_init(&mut self, roots: &[RootIdx]) {
        let user_init = self.user_init();
        let user_init = self.method_defs.get_mut(&user_init).unwrap();
        let blocks = user_init
            .implementation_mut()
            .blocks_mut()
            .expect("EROROR: {USER_INIT} has no body.");
        let last = blocks
            .iter_mut()
            .last()
            .expect("ERROR: {USER_INIT} has a body without blocks.");
        let last_root_idx = if last.roots().is_empty() {
            0
        } else {
            last.roots().len() - 1
        };
        for (idx, root) in roots.iter().enumerate() {
            last.roots_mut().insert(idx + last_root_idx, *root);
        }
    }
    /// Adds new rooots to the thread local intiailzer .
    pub fn add_tcctor(&mut self, roots: &[RootIdx]) {
        let user_init = self.tcctor();
        let user_init = self.method_defs.get_mut(&user_init).unwrap();
        let blocks = user_init
            .implementation_mut()
            .blocks_mut()
            .expect("EROROR: {TCCTOR} has no body.");
        let last = blocks
            .iter_mut()
            .last()
            .expect("ERROR: {TCCTOR} has a body without blocks.");
        let last_root_idx = if last.roots().is_empty() {
            0
        } else {
            last.roots().len() - 1
        };
        for (idx, root) in roots.iter().enumerate() {
            last.roots_mut().insert(idx + last_root_idx, *root);
        }
    }
    /// Adds new rooots to the static initializer
    pub fn add_cctor(&mut self, roots: &[RootIdx]) {
        let user_init = self.cctor();
        let user_init = self.method_defs.get_mut(&user_init).unwrap();
        let blocks = user_init
            .implementation_mut()
            .blocks_mut()
            .expect("EROROR: {CCTOR} has no body.");
        let last = blocks
            .iter_mut()
            .last()
            .expect("ERROR: {CCTOR} has a body without blocks.");
        let last_root_idx = if last.roots().is_empty() {
            0
        } else {
            last.roots().len() - 1
        };
        for (idx, root) in roots.iter().enumerate() {
            last.roots_mut().insert(idx + last_root_idx, *root);
        }
    }
    /// Serializes and saves this assembly
    pub fn save_tmp<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(&postcard::to_stdvec(&self).unwrap())
    }
    /// Converts the old assembly repr to the new one.
    pub fn from_v1(v1: &V1Asm) -> Self {
        let mut empty: Assembly = v1.inner().clone();
        // Add the user defined roots
        let roots = v1
            .initializers()
            .iter()
            .map(|root| {
                let root = CILRoot::from_v1(root, &mut empty);
                empty.alloc_root(root)
            })
            .collect::<Box<[_]>>();
        empty.add_user_init(roots.as_ref());
        // Add the global static fields
        let fields: Vec<_> = v1
            .static_fields()
            .iter()
            .map(|(name, (tpe, thread_local))| {
                let name = empty.alloc_string(name.clone());
                (*tpe, name, *thread_local)
            })
            .collect();
        let main_module = empty.main_module();
        empty
            .class_defs
            .get_mut(&main_module)
            .expect("Main module missing, even tough it has been added")
            .static_fields_mut()
            .extend(fields);
        // Convert external function refs
        v1.extern_fns()
            .iter()
            .for_each(|((fn_name, sig, preserve_errno), lib_name)| {
                let v2_sig = FnSig::from_v1(sig);
                let name = empty.alloc_string(fn_name.clone());
                let sigidx = empty.alloc_sig(v2_sig);
                let lib = empty.alloc_string(lib_name.clone());
                empty.new_method(MethodDef::new(
                    Access::Public,
                    main_module,
                    name,
                    sigidx,
                    MethodKind::Static,
                    MethodImpl::Extern {
                        lib,
                        preserve_errno: *preserve_errno,
                    },
                    sig.inputs().iter().map(|_| None).collect(),
                ));
            });

        // Convert module methods
        v1.functions().values().for_each(|method| {
            let def = MethodDef::from_v1(method, &mut empty, main_module);
            empty.new_method(def);
        });

        #[cfg(debug_assertions)]
        empty.sanity_check();
        empty
    }
    #[track_caller]
    pub fn sanity_check(&self) {
        self.class_defs.values().for_each(|class| {
            crate::utilis::assert_unique(class.methods(), class.ref_to().display(self))
        })
    }
    pub fn export(&self, out: impl AsRef<std::path::Path>, exporter: impl Exporter) {
        exporter.export(self, out.as_ref()).unwrap()
    }
    pub fn memory_info(&self) {
        let mut stats = vec![
            encoded_stats(self),
            encoded_stats(&self.strings),
            encoded_stats(&self.types),
            encoded_stats(&self.class_refs),
            encoded_stats(&self.class_defs),
            encoded_stats(&self.nodes),
            encoded_stats(&self.roots),
            encoded_stats(&self.sigs),
            encoded_stats(&self.types),
            encoded_stats(&self.fields),
            encoded_stats(&self.statics),
            encoded_stats(&self.method_defs),
        ];
        stats.sort_by(|(_, a), (_, b)| a.cmp(b));
        for stat in stats {
            println!("{}:\t{} bytes", stat.0, stat.1);
        }
    }

    pub(crate) fn iter_class_defs(&self) -> impl Iterator<Item = &ClassDef> {
        self.class_defs.values()
    }
    pub(crate) fn iter_class_def_ids(&self) -> impl Iterator<Item = &ClassDefIdx> {
        self.class_defs.keys()
    }
    pub(crate) fn method_def_from_ref(&self, mref: MethodRefIdx) -> Option<&MethodDef> {
        self.method_defs.get(&MethodDefIdx(mref))
    }
    pub(crate) fn eliminate_dead_fns(&mut self) {
        // 1st. Collect all "extern" method definitons, since those are always alive.
        let mut previosly_ressurected: FxHashSet<MethodDefIdx> = self
            .method_defs
            .iter()
            .filter(|(_, def)| def.access().is_extern())
            .map(|(idx, _)| *idx)
            .collect();
        let mut to_resurrect: FxHashSet<MethodDefIdx> = FxHashSet::default();
        let mut alive: FxHashSet<MethodDefIdx> = FxHashSet::default();
        while !previosly_ressurected.is_empty() {
            for def in previosly_ressurected
                .iter()
                .map(|def: &MethodDefIdx| self.method_defs.get(def).unwrap())
            {
                // Iterate torugh the cil of this method, if present
                let Some(cil) = def.iter_cil(self) else {
                    continue;
                };
                // Get all the ref ids of the methods used in the cil.
                let refids = cil.filter_map(|elem| match elem {
                    crate::v2::CILIterElem::Node(CILNode::Call(args)) => Some(args.0),
                    crate::v2::CILIterElem::Node(CILNode::LdFtn(mref)) => Some(mref),
                    crate::v2::CILIterElem::Node(_) => None,
                    crate::v2::CILIterElem::Root(CILRoot::Call(args)) => Some(args.0),
                    crate::v2::CILIterElem::Root(_) => None,
                });
                // Check if this method reference is also a def. If so, map it to a def
                let defids = refids.filter_map(|refid| {
                    self.method_defs
                        .get(&MethodDefIdx(refid))
                        .map(|_| MethodDefIdx(refid))
                        .and_then(|refid| {
                            if alive.contains(&refid) {
                                None
                            } else {
                                Some(refid)
                            }
                        })
                });
                to_resurrect.extend(defids);
            }
            alive.extend(previosly_ressurected);
            previosly_ressurected = to_resurrect;
            to_resurrect = FxHashSet::default();
        }
        // Some cheap sanity checks
        assert!(previosly_ressurected.is_empty());
        assert!(to_resurrect.is_empty());
        // Set the method set to only include alive methods
        self.method_defs = alive
            .iter()
            .map(|id| (*id, self.method_defs.remove(id).unwrap()))
            .collect();
        // clean up typedefs
        self.class_defs.values_mut().for_each(|tdef| {
            tdef.methods_mut()
                .retain(|def| self.method_defs.contains_key(def));
        });
    }
    pub fn eliminate_dead_code(&mut self) {
        self.eliminate_dead_fns();
        self.eliminate_dead_types();
    }
    #[allow(dead_code)]
    pub(crate) fn eliminate_dead_types(&mut self) {
        let mut previosly_ressurected: FxHashSet<ClassDefIdx> = self
            .method_defs()
            .values()
            .flat_map(|method| method.iter_types(self))
            .flat_map(|tpe| tpe.iter_class_refs(self).collect::<Vec<_>>())
            .flat_map(|cref| self.class_ref_to_def(cref))
            .collect();
        let rust_void = self.alloc_string("RustVoid");
        let rust_void = self.alloc_class_ref(ClassRef::new(rust_void, None, true, vec![].into()));
        if let Some(cref) = self.class_ref_to_def(rust_void) {
            previosly_ressurected.insert(cref);
        }
        let f128 = self.alloc_string("f128");
        let f128 = self.alloc_class_ref(ClassRef::new(f128, None, true, vec![].into()));
        if let Some(cref) = self.class_ref_to_def(f128) {
            previosly_ressurected.insert(cref);
        }

        let mut to_resurrect: FxHashSet<ClassDefIdx> = FxHashSet::default();
        let mut alive: FxHashSet<ClassDefIdx> = FxHashSet::default();
        while !previosly_ressurected.is_empty() {
            for def in previosly_ressurected.iter() {
                let defids: FxHashSet<ClassDefIdx> = self
                    .get_class_def(*def)
                    .iter_types()
                    .flat_map(|tpe| tpe.iter_class_refs(self).collect::<Vec<_>>())
                    .flat_map(|cref| self.class_ref_to_def(cref))
                    .filter(|refid| !alive.contains(refid))
                    .collect();

                to_resurrect.extend(defids);
            }
            alive.extend(previosly_ressurected);
            previosly_ressurected = to_resurrect;
            to_resurrect = FxHashSet::default();
        }
        // Some cheap sanity checks
        assert!(previosly_ressurected.is_empty());
        assert!(to_resurrect.is_empty());
        // Set the class_defs to only include alive classes
        self.class_defs = alive
            .iter()
            .map(|id| (*id, self.class_defs.remove(id).unwrap()))
            .collect();
    }
    /*pub fn realloc_nodes(&mut self){

    }*/
    /// Reallocates the roots, freeing all dead ones.
    pub fn realloc_roots(&mut self) {
        let mut new_roots = BiMap::default();
        for block in self
            .method_defs
            .values_mut()
            .flat_map(|def| def.implementation_mut().blocks_mut())
            .flatten()
        {
            let (handler, roots) = block.handler_and_root_mut();
            for root in roots.iter_mut().chain(
                handler
                    .into_iter()
                    .flat_map(|blocks| blocks.iter_mut())
                    .flat_map(|b| b.roots_mut()),
            ) {
                *root = new_roots.alloc(self.roots.get(*root).clone());
            }
        }
        self.roots = new_roots;
    }

    pub fn patch_missing_methods(
        &mut self,
        externs: FxHashMap<&str, String>,
        modifies_errno: FxHashSet<&str>,
        override_methods: MissingMethodPatcher,
    ) {
        let mref_count = self.method_refs.0.len();
        let externs: FxHashMap<_, _> = externs
            .into_iter()
            .map(|(fn_name, lib_name)| (self.alloc_string(fn_name), self.alloc_string(lib_name)))
            .collect();
        let preserve_errno: FxHashSet<_> = modifies_errno
            .into_iter()
            .map(|fn_name| self.alloc_string(fn_name))
            .collect();
        for index in 0..mref_count {
            // Get the full method refernce
            let mref = &self.method_refs.0[index];
            // Check if this method reference's class has an assembly. If it has, then the method is extern. If it has not, then it is defined in this assembly
            // and must have some kind of implementation
            let class = self.class_ref(mref.class());

            if class.asm().is_some() {
                // Is extern, skip

                continue;
            }
            let mref_idx =
                MethodRefIdx::from_index(std::num::NonZeroU32::new(index as u32 + 1).unwrap());
            // Check if this method already has an implementation.
            if self.method_defs.contains_key(&MethodDefIdx(mref_idx)) {
                // A method defintion already present, so we don't need to do anyting, so skip.

                continue;
            }
            if let Some(overrider) = override_methods.get(&mref.name()) {
                let mref = mref.clone();
                let implementation = overrider(mref_idx, self);
                self.new_method(mref.into_def(implementation, Access::Private, self));
                continue;
            }
            // Check if this method is in the extern list
            if let Some(lib) = externs.get(&mref.name()) {
                let arg_names = (0..(self.get_sig(mref.sig()).inputs().len()))
                    .map(|_| None)
                    .collect();
                let method_def = MethodDef::new(
                    Access::Public,
                    ClassDefIdx(mref.class()),
                    mref.name(),
                    mref.sig(),
                    mref.kind(),
                    MethodImpl::Extern {
                        lib: *lib,
                        preserve_errno: preserve_errno.contains(&mref.name()),
                    },
                    arg_names,
                );
                assert!(
                    self.class_defs.contains_key(&ClassDefIdx(mref.class())),
                    "Can't yet handle missing types."
                );

                self.new_method(method_def);

                continue;
            }
            // Create a replacement method.

            let arg_names = (0..(self.get_sig(mref.sig()).inputs().len()))
                .map(|_| None)
                .collect();
            let method_def = MethodDef::new(
                Access::Public,
                ClassDefIdx(mref.class()),
                mref.name(),
                mref.sig(),
                mref.kind(),
                MethodImpl::Missing,
                arg_names,
            );
            assert!(
                self.class_defs.contains_key(&ClassDefIdx(mref.class())),
                "Can't yet handle missing types. Type {} with id {:?} is missing. Method {}",
                self.class_ref(mref.class()).display(self),
                mref.class(),
                self.get_string(mref.name())
            );

            self.new_method(method_def);
        }
    }

    pub fn class_ref_to_def(&self, class: ClassRefIdx) -> Option<ClassDefIdx> {
        if self.class_defs.contains_key(&ClassDefIdx(class)) {
            Some(ClassDefIdx(class))
        } else {
            None
        }
    }
    pub fn link(mut self, other: Self) -> Self {
        let original_str = self.alloc_string(MAIN_MODULE);
        for def in other.iter_class_defs() {
            let translated = self.translate_class_def(&other, def);
            let class_ref = self.alloc_class_ref(translated.ref_to());
            match self.class_defs.entry(ClassDefIdx(class_ref)) {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    occupied.get_mut().merge_defs(translated, &self.strings)
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(translated);
                }
            }
        }
        assert_eq!(self.alloc_string(MAIN_MODULE), original_str);
        self
    }

    pub(crate) fn method_defs_mut(&mut self) -> &mut FxHashMap<MethodDefIdx, MethodDef> {
        &mut self.method_defs
    }

    pub(crate) fn method_defs(&self) -> &FxHashMap<MethodDefIdx, MethodDef> {
        &self.method_defs
    }

    pub(crate) fn contains_def(&self, cref: ClassRefIdx) -> bool {
        self.class_ref_to_def(cref).is_some()
    }
    /// Checks if this assembly contains a reference [`ClassRef`]
    pub fn contains_ref(&self, cref: &ClassRef) -> bool {
        self.class_refs.1.contains_key(cref)
    }
}
/// An initializer, which runs before everything else. By convention, it is used to initialize static / const data. Should not execute any user code
pub const CCTOR: &str = ".cctor";
/// An thread-local initializer. Runs before each thread starts. By convention, it is used to initialize thread local data. Should not execute any user code.
pub const TCCTOR: &str = ".tcctor";
/// An intializer, which runs after the [`CCTOR`] and [`TCCTOR`], but before the [`ENTRYPOINT`]. Meant to execute user code, is roughly equivalnt to `.init_array` on GNU.
pub const USER_INIT: &str = "static_init";
/// The entrypoint of a program
pub const ENTRYPOINT: &str = "entrypoint";
/// Main class of this module
pub const MAIN_MODULE: &str = "MainModule";

fn encoded_stats<T: Serialize + for<'a> Deserialize<'a>>(val: &T) -> (&'static str, usize) {
    let buff = postcard::to_allocvec(val).unwrap();
    let start = std::time::Instant::now();
    let _: T = postcard::from_bytes(&buff).unwrap();
    let end = std::time::Instant::now();
    println!(
        "Decoding {} took {} ms",
        type_name::<T>(),
        end.duration_since(start).as_millis()
    );
    (type_name::<T>(), buff.len())
}
lazy_static! {
    pub static ref ILASM_FLAVOUR: IlasmFlavour = {
        if String::from_utf8_lossy(
            &std::process::Command::new(&*ILASM_PATH)
                .output()
                .unwrap()
                .stdout,
        )
        .contains("PDB")
        {
            IlasmFlavour::Modern
        } else {
            IlasmFlavour::Clasic
        }
    };
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IlasmFlavour {
    Clasic,
    Modern,
}
pub fn ilasm_path() -> &'static str {
    ILASM_PATH.as_str()
}
lazy_static! {
    #[doc = "Specifies the path to the IL assembler."]
    pub static ref ILASM_PATH:String = {
        std::env::vars().find_map(|(key,value)|if key == "ILASM_PATH"{Some(value)}else{None}).unwrap_or("ilasm".into())
    };
}

#[test]
fn user_init() {
    let mut asm = Assembly::default();
    asm.user_init();
}
#[test]
fn add_user_init() {
    let mut asm = Assembly::default();
    let roots = vec![
        asm.alloc_root(CILRoot::VoidRet),
        asm.alloc_root(CILRoot::Break),
        asm.alloc_root(CILRoot::Nop),
    ];
    asm.add_user_init(&roots);
}
#[test]
fn export() {
    use super::il_exporter::*;

    let mut asm = Assembly::default();
    let main_module = asm.main_module();
    let name = asm.alloc_string("entrypoint");
    let sig = asm.sig([], Type::Void);
    let body = vec![asm.alloc_root(CILRoot::VoidRet)];
    asm.new_method(MethodDef::new(
        Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        MethodImpl::MethodBody {
            blocks: vec![super::BasicBlock::new(body, 0, None)],
            locals: vec![],
        },
        vec![],
    ));
    let type_idx = asm.alloc_type(Type::Int(super::Int::I8));
    let sig = asm.sig([Type::Ptr(type_idx)], Type::Void);
    let name = asm.alloc_string("pritnf");
    let lib = asm.alloc_string("/lib/libc.so");
    asm.new_method(MethodDef::new(
        Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        MethodImpl::Extern {
            lib,
            preserve_errno: false,
        },
        vec![None],
    ));
    asm.export("/tmp/export.exe", ILExporter::new(*ILASM_FLAVOUR, false));
}
#[test]
fn export2() {
    use super::il_exporter::*;

    let mut asm = Assembly::default();
    let main_module = asm.main_module();
    let name = asm.alloc_string("entrypoint");
    let sig = asm.sig([], Type::Void);
    let body1 = vec![asm.alloc_root(CILRoot::VoidRet)];
    let hbody = vec![asm.alloc_root(CILRoot::ExitSpecialRegion {
        target: 2,
        source: 0,
    })];
    asm.alloc_root(CILRoot::Break);
    let body2 = vec![asm.alloc_root(CILRoot::VoidRet)];
    asm.new_method(MethodDef::new(
        Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        MethodImpl::MethodBody {
            blocks: vec![
                super::BasicBlock::new(
                    body1,
                    0,
                    Some(vec![super::BasicBlock::new(hbody, 1, None)]),
                ),
                super::BasicBlock::new(body2, 2, None),
            ],
            locals: vec![],
        },
        vec![],
    ));
    let type_idx = asm.alloc_type(Type::Int(super::Int::I8));
    let sig = asm.sig([Type::Ptr(type_idx)], Type::Void);
    let name = asm.alloc_string("pritnf");
    let lib = asm.alloc_string("/lib/libc.so");
    asm.new_method(MethodDef::new(
        Access::Public,
        main_module,
        name,
        sig,
        MethodKind::Static,
        MethodImpl::Extern {
            lib,
            preserve_errno: false,
        },
        vec![None],
    ));
    let uinit = asm.alloc_root(CILRoot::Break);
    asm.add_user_init(&[uinit]);
    asm.eliminate_dead_code();
    asm.realloc_roots();
    asm.export("/tmp/export2.exe", ILExporter::new(*ILASM_FLAVOUR, false));
}
#[test]
fn link() {
    use super::il_exporter::*;

    let asm1 = {
        let mut asm = Assembly::default();
        let main_module = asm.main_module();
        let name = asm.alloc_string("entrypoint");
        let sig = asm.sig([], Type::Void);
        let body1 = vec![asm.alloc_root(CILRoot::VoidRet)];
        let hbody = vec![asm.alloc_root(CILRoot::ExitSpecialRegion {
            target: 2,
            source: 0,
        })];
        asm.alloc_root(CILRoot::Break);
        let body2 = vec![asm.alloc_root(CILRoot::VoidRet)];
        asm.new_method(MethodDef::new(
            Access::Extern,
            main_module,
            name,
            sig,
            MethodKind::Static,
            MethodImpl::MethodBody {
                blocks: vec![
                    super::BasicBlock::new(
                        body1,
                        0,
                        Some(vec![super::BasicBlock::new(hbody, 1, None)]),
                    ),
                    super::BasicBlock::new(body2, 2, None),
                ],
                locals: vec![],
            },
            vec![],
        ));
        asm
    };
    let asm2 = {
        let mut asm = Assembly::default();
        let main_module = asm.main_module();
        let type_idx = asm.alloc_type(Type::Int(super::Int::I8));
        let sig = asm.sig([Type::Ptr(type_idx)], Type::Void);
        let name = asm.alloc_string("pritnf");
        let lib = asm.alloc_string("/lib/libc.so");
        asm.new_method(MethodDef::new(
            Access::Public,
            main_module,
            name,
            sig,
            MethodKind::Static,
            MethodImpl::Extern {
                lib,
                preserve_errno: false,
            },
            vec![None],
        ));
        let uinit = asm.alloc_root(CILRoot::Break);
        asm.add_user_init(&[uinit]);
        asm
    };
    let mut asm = asm1.link(asm2);
    asm.eliminate_dead_code();
    asm.realloc_roots();
    asm.export("/tmp/link_test.exe", ILExporter::new(*ILASM_FLAVOUR, false));
}
