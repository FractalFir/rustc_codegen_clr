use super::{
    bimap::{BiMap, BiMapIndex, IntoBiMapIndex},
    cilnode::{BinOp, ExtendKind, MethodKind, PtrCastRes, UnOp},
    class::StaticFieldDef,
    opt::{OptFuel, SideEffectInfoCache},
    Access, CILNode, CILRoot, ClassDef, ClassDefIdx, ClassRef, ClassRefIdx, Const, Exporter,
    FieldDesc, FieldIdx, FnSig, Int, IntoAsmIndex, MethodDef, MethodDefIdx, MethodRef,
    MethodRefIdx, NodeIdx, RootIdx, SigIdx, StaticFieldDesc, StaticFieldIdx, StringIdx, Type,
    TypeIdx,
};
use crate::{config, IString};
use crate::{utilis::encode, v2::MethodImpl};
use fxhash::{hash64, FxHashMap, FxHashSet};

use serde::{Deserialize, Serialize};
use std::{any::type_name, ops::Index};

pub type MissingMethodPatcher =
    FxHashMap<StringIdx, Box<dyn Fn(MethodRefIdx, &mut Assembly) -> MethodImpl>>;
type StringMap = BiMap<StringIdx, IString>;
type TypeMap = BiMap<TypeIdx, Type>;
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Assembly {
    /// A list of strings used in this assembly
    strings: StringMap,
    /// A list of all types in this assembly
    types: TypeMap,
    class_refs: BiMap<ClassRefIdx, ClassRef>,
    class_defs: FxHashMap<ClassDefIdx, ClassDef>,
    nodes: BiMap<NodeIdx, CILNode>,
    roots: BiMap<RootIdx, CILRoot>,
    sigs: BiMap<SigIdx, FnSig>,
    method_refs: BiMap<MethodRefIdx, MethodRef>,
    fields: BiMap<FieldIdx, FieldDesc>,
    statics: BiMap<StaticFieldIdx, StaticFieldDesc>,
    method_defs: FxHashMap<MethodDefIdx, MethodDef>,
    sections: FxHashMap<String, Vec<u8>>,
    // Cache containing information about the stack usage of a CIL node.
    //#[serde(skip)]
    //cache: CachedAssemblyInfo<NodeIdx, NonMaxU32, StackUsage>,
}
impl Index<StringIdx> for Assembly {
    type Output = str;

    fn index(&self, index: StringIdx) -> &Self::Output {
        &self.strings[index]
    }
}
impl Index<ClassDefIdx> for Assembly {
    type Output = ClassDef;

    fn index(&self, index: ClassDefIdx) -> &Self::Output {
        &self.class_defs[&index]
    }
}
impl Index<MethodRefIdx> for Assembly {
    type Output = MethodRef;

    fn index(&self, index: MethodRefIdx) -> &Self::Output {
        &self.method_refs[index]
    }
}
impl Index<MethodDefIdx> for Assembly {
    type Output = MethodDef;

    fn index(&self, index: MethodDefIdx) -> &Self::Output {
        &self.method_defs[&index]
    }
}
impl Index<ClassRefIdx> for Assembly {
    type Output = ClassRef;

    fn index(&self, index: ClassRefIdx) -> &Self::Output {
        &self.class_refs[index]
    }
}
impl Index<TypeIdx> for Assembly {
    type Output = Type;

    fn index(&self, index: TypeIdx) -> &Self::Output {
        &self.types[index]
    }
}
impl Index<SigIdx> for Assembly {
    type Output = FnSig;

    fn index(&self, index: SigIdx) -> &Self::Output {
        &self.sigs[index]
    }
}
impl Index<RootIdx> for Assembly {
    type Output = CILRoot;

    fn index(&self, index: RootIdx) -> &Self::Output {
        &self.roots[index]
    }
}
impl Index<NodeIdx> for Assembly {
    type Output = CILNode;

    fn index(&self, index: NodeIdx) -> &Self::Output {
        &self.nodes[index]
    }
}
impl Index<StaticFieldIdx> for Assembly {
    type Output = StaticFieldDesc;

    fn index(&self, index: StaticFieldIdx) -> &Self::Output {
        &self.statics[index]
    }
}
impl Index<FieldIdx> for Assembly {
    type Output = FieldDesc;

    fn index(&self, index: FieldIdx) -> &Self::Output {
        &self.fields[index]
    }
}
impl Assembly {
    pub fn typecheck(&mut self) {
        let method_def_idxs: Box<[_]> = self.method_defs.keys().copied().collect();
        for method in method_def_idxs {
            let mut tmp_method = self.method_def(method).clone();
            tmp_method.typecheck(self);
        }
    }
    #[must_use]
    pub fn class_defs(&self) -> &FxHashMap<ClassDefIdx, ClassDef> {
        &self.class_defs
    }

    #[must_use]
    pub fn method_ref_to_def(&self, method: MethodRefIdx) -> Option<MethodDefIdx> {
        if self.method_defs.contains_key(&MethodDefIdx(method)) {
            Some(MethodDefIdx(method))
        } else {
            None
        }
    }
    #[must_use]
    pub fn fuel_from_env(&self) -> OptFuel {
        match std::env::var("OPT_FUEL") {
            Ok(fuel) => match fuel.parse::<u32>() {
                Ok(fuel) => OptFuel::new(fuel),
                Err(_) => self.default_fuel(),
            },
            Err(_) => self.default_fuel(),
        }
    }
    #[must_use]
    pub fn default_fuel(&self) -> OptFuel {
        OptFuel::new((self.method_defs.len() * 4 + self.roots.len() * 16) as u32)
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
            //let _pass_min_cost: bool = fuel.consume(1);
        }
    }
    /// Optimizes the assembly, cosuming some fuel. This performs a single optimization pass.
    pub fn opt_sigle_pass(&mut self, fuel: &mut OptFuel, cache: &mut SideEffectInfoCache) {
        let method_def_idxs: Box<[_]> = self.method_defs.keys().copied().collect();
        for method in method_def_idxs {
            let mut tmp_method = self.borrow_methoddef(method);
            tmp_method.optimize(self, cache, fuel);
            tmp_method.remove_dead_blocks(self);
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
    ) -> impl Iterator<Item = (&'a MethodDefIdx, &'a MethodDef)> + 'a {
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
    ) -> impl Iterator<Item = StringIdx> + 'a {
        self.strings
            .0
            .iter()
            .enumerate()
            .filter_map(move |(idx, str)| {
                if str.contains(pat.clone()) {
                    Some(StringIdx(BiMapIndex::new((idx + 1) as u32).unwrap()))
                } else {
                    None
                }
            })
    }
    pub fn get_prealllocated_string(&self, string: impl Into<IString>) -> Option<StringIdx> {
        self.strings.1.get(&(string.into())).copied()
    }
    pub fn class_mut(&mut self, id: ClassDefIdx) -> &mut ClassDef {
        self.class_defs.get_mut(&id).unwrap()
    }
    #[must_use]
    pub fn get_class_def(&self, id: ClassDefIdx) -> &ClassDef {
        &self.class_defs[&id]
    }
    #[must_use]
    pub fn class_ref(&self, cref: ClassRefIdx) -> &ClassRef {
        self.class_refs.get(cref)
    }
    #[must_use]
    pub fn method_def(&self, dref: MethodDefIdx) -> &MethodDef {
        self.method_defs.get(&dref).unwrap()
    }
    pub fn alloc_string(&mut self, string: impl Into<IString>) -> StringIdx {
        self.strings.alloc(string.into())
    }

    pub fn sig(&mut self, input: impl Into<Box<[Type]>>, output: impl Into<Type>) -> SigIdx {
        self.sigs.alloc(FnSig::new(input.into(), output.into()))
    }
    pub fn fn_ptr(&mut self, input: impl Into<Box<[Type]>>, output: impl Into<Type>) -> Type {
        let sig = self.sig(input, output);
        Type::FnPtr(sig)
    }
    pub fn nptr(&mut self, inner: impl IntoAsmIndex<TypeIdx>) -> Type {
        Type::Ptr(inner.into_idx(self))
    }
    pub fn nref(&mut self, inner: impl IntoAsmIndex<TypeIdx>) -> Type {
        Type::Ref(inner.into_idx(self))
    }

    #[must_use]
    pub fn get_root(&self, root: RootIdx) -> &CILRoot {
        self.roots.get(root)
    }
    pub fn size_of(&mut self, tpe: impl IntoAsmIndex<TypeIdx>) -> CILNode {
        let idx = tpe.into_idx(self);
        assert_ne!(self[idx], Type::Void);
        CILNode::SizeOf(idx)
    }
    pub fn biop(
        &mut self,
        lhs: impl IntoAsmIndex<NodeIdx>,
        rhs: impl IntoAsmIndex<NodeIdx>,
        op: BinOp,
    ) -> CILNode {
        let lhs = lhs.into_idx(self);
        let rhs = rhs.into_idx(self);
        CILNode::BinOp(lhs, rhs, op)
    }
    pub fn unop(&mut self, val: impl Into<CILNode>, op: UnOp) -> CILNode {
        let val = self.nodes.alloc(val.into());
        CILNode::UnOp(val, op)
    }
    pub fn int_cast(
        &mut self,
        input: impl IntoAsmIndex<NodeIdx>,
        target: Int,
        extend: ExtendKind,
    ) -> CILNode {
        CILNode::IntCast {
            input: input.into_idx(self),
            target,
            extend,
        }
    }
    pub fn ptr_cast(&mut self, input: impl IntoAsmIndex<NodeIdx>, res: PtrCastRes) -> CILNode {
        CILNode::PtrCast(input.into_idx(self), Box::new(res))
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

    pub fn alloc_field(&mut self, field: FieldDesc) -> FieldIdx {
        self.fields.alloc(field)
    }
    #[must_use]
    pub fn get_field(&self, key: FieldIdx) -> &FieldDesc {
        self.fields.get(key)
    }
    pub fn alloc_sfld(&mut self, sfld: StaticFieldDesc) -> StaticFieldIdx {
        self.statics.alloc(sfld)
    }
    #[must_use]
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
            .contains(&StaticFieldDef {
                tpe,
                name,
                is_tls: thread_local,
            })
        {
            self.class_mut(in_class)
                .static_fields_mut()
                .push(StaticFieldDef {
                    tpe,
                    name,
                    is_tls: thread_local,
                });
        }

        idx
    }
    /// Adds a new class definition to this type
    pub fn class_def(&mut self, def: ClassDef) -> ClassDefIdx {
        let cref = def.ref_to();
        let cref = self.alloc_class_ref(cref);

        if self.class_defs.contains_key(&ClassDefIdx(cref)) {
            if self[def.name()].contains("core.ffi.c_void") {
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
            Access::Public,
            None,
            None,
            true,
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
        if !self.method_defs.contains_key(&MethodDefIdx(ref_idx)) {
            self.class_defs
                .get_mut(&def_class)
                .expect("Method added without a class")
                .add_def(MethodDefIdx(ref_idx));
        }

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
    fn cctor_mref(&mut self) -> MethodRefIdx {
        let main_module = self.main_module();
        let user_init = self.alloc_string(CCTOR);
        let ctor_sig = self.sig([], Type::Void);
        self.alloc_methodref(MethodRef::new(
            *main_module,
            user_init,
            ctor_sig,
            MethodKind::Static,
            vec![].into(),
        ))
    }
    fn has_builtin(&self, name: &str, input: impl Into<Box<[Type]>>, output: Type) -> bool {
        let Some(main_module) = self.get_prealllocated_string(MAIN_MODULE) else {
            return false;
        };
        let class_def = ClassDef::new(
            main_module,
            false,
            0,
            None,
            vec![],
            vec![],
            Access::Public,
            None,
            None,
            true,
        );

        let Some(cref) = self.get_prealllocated_class_ref(class_def.ref_to()) else {
            return false;
        };
        // Check if that definition already exists
        let main_module = if self.class_defs.contains_key(&ClassDefIdx(cref)) {
            ClassDefIdx(cref)
        } else {
            return false;
        };

        let Some(user_init) = self.get_prealllocated_string(name) else {
            return false;
        };

        let Some(ctor_sig) = self.get_prealllocated_sig(FnSig::new(input.into(), output)) else {
            return false;
        };
        let Some(cctor) = self.get_prealllocated_methodref(MethodRef::new(
            *main_module,
            user_init,
            ctor_sig,
            MethodKind::Static,
            vec![].into(),
        )) else {
            return false;
        };

        self.method_ref_to_def(cctor).is_some()
    }
    pub fn has_cctor(&self) -> bool {
        self.has_builtin(CCTOR, [], Type::Void)
    }
    pub fn has_tcctor(&self) -> bool {
        self.has_builtin(TCCTOR, [], Type::Void)
    }
    pub fn get_prealllocated_class_ref(&self, cref: ClassRef) -> Option<ClassRefIdx> {
        self.class_refs.1.get(&cref).copied()
    }
    pub fn get_prealllocated_sig(&self, sig: FnSig) -> Option<SigIdx> {
        self.sigs.1.get(&sig).copied()
    }
    pub fn get_prealllocated_methodref(&self, mref: MethodRef) -> Option<MethodRefIdx> {
        self.method_refs.1.get(&mref).copied()
    }
    /// Returns a reference to the static initializer
    pub fn cctor(&mut self) -> MethodDefIdx {
        let mref = self.cctor_mref();
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
            let main_module = self.main_module();
            let user_init = self.alloc_string(CCTOR);
            let ctor_sig = self.sig([], Type::Void);
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
    #[must_use]
    pub fn from_v1(v1: &Assembly) -> Self {
        let mut empty: Assembly = v1.clone();
        let rust_void = empty.alloc_string("RustVoid");
        empty.class_def(ClassDef::new(
            rust_void,
            true,
            0,
            None,
            vec![],
            vec![],
            Access::Public,
            None,
            None,
            true,
        ));

        #[cfg(debug_assertions)]
        empty.sanity_check();
        empty
    }
    #[track_caller]
    pub fn sanity_check(&self) {
        self.class_defs.values().for_each(|class| {
            crate::utilis::assert_unique(class.methods(), class.ref_to().display(self));
        });
    }
    #[cfg(not(miri))]
    pub fn export(&self, out: impl AsRef<std::path::Path>, exporter: impl Exporter) {
        if *LINKER_RECOVER {
            eprintln!("{:?}", exporter.export(self, out.as_ref()));
        } else {
            exporter.export(self, out.as_ref()).unwrap();
        }
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
    pub(crate) fn eliminate_dead_fns(&mut self, only_imports: bool) {
        // 1st. Collect all "extern" method definitons, since those are always alive.
        let mut previosly_ressurected: FxHashSet<MethodDefIdx> = self
            .method_defs
            .iter()
            .filter(|(_, def)| def.access().is_extern())
            .map(|(idx, _)| *idx)
            .collect();
        let mut to_resurrect: FxHashSet<MethodDefIdx> = FxHashSet::default();
        let mut alive: FxHashSet<MethodDefIdx> = FxHashSet::default();
        // If only cleaning up imports, assume all non-import fns are alive.
        if only_imports {
            alive.extend(
                self.method_defs
                    .iter()
                    .filter(|(_, def)| !matches!(def.implementation(), MethodImpl::Extern { .. }))
                    .map(|(id, _)| *id),
            );
        }
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
        self.eliminate_dead_fns(false);
        self.eliminate_dead_types();
    }
    #[allow(dead_code)]
    pub(crate) fn eliminate_dead_types(&mut self) {
        let mut previosly_ressurected: FxHashSet<ClassDefIdx> = self
            .method_defs()
            .values()
            .flat_map(|method| method.iter_types(self))
            .flat_map(|tpe| tpe.iter_class_refs(self).collect::<Vec<_>>())
            .filter_map(|cref| self.class_ref_to_def(cref))
            .collect();
        previosly_ressurected.extend(self.class_defs().iter().filter_map(|(defid, def)| {
            if def.access().is_extern() {
                Some(defid)
            } else {
                None
            }
        }));
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
            for def in &previosly_ressurected {
                let defids: FxHashSet<ClassDefIdx> = self.class_defs[def]
                    .iter_types()
                    .flat_map(|tpe| tpe.iter_class_refs(self).collect::<Vec<_>>())
                    .filter_map(|cref| self.class_ref_to_def(cref))
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
            .filter_map(|def| def.implementation_mut().blocks_mut())
            .flatten()
        {
            let (handler, roots) = block.handler_and_root_mut();
            for root in roots.iter_mut().chain(
                handler
                    .into_iter()
                    .flat_map(|blocks| blocks.iter_mut())
                    .flat_map(super::basic_block::BasicBlock::roots_mut),
            ) {
                *root = new_roots.alloc(self.roots.get(*root).clone());
            }
        }
        self.roots = new_roots;
    }

    pub fn patch_missing_methods(
        &mut self,
        externs: &FxHashMap<&str, String>,
        modifies_errno: &FxHashSet<&str>,
        override_methods: &MissingMethodPatcher,
    ) {
        let mref_count = self.method_refs.0.len();
        let externs: FxHashMap<_, _> = externs
            .iter()
            .map(|(fn_name, lib_name)| {
                (
                    self.alloc_string(*fn_name),
                    self.alloc_string(lib_name.clone()),
                )
            })
            .collect();
        let preserve_errno: FxHashSet<_> = modifies_errno
            .iter()
            .map(|fn_name| self.alloc_string(*fn_name))
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
                let arg_names = (0..(self[mref.sig()].inputs().len()))
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

            let arg_names = (0..(self[mref.sig()].inputs().len()))
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
                &self[mref.name()]
            );

            self.new_method(method_def);
        }
    }

    #[must_use]
    pub fn class_ref_to_def(&self, class: ClassRefIdx) -> Option<ClassDefIdx> {
        if self.class_defs.contains_key(&ClassDefIdx(class)) {
            Some(ClassDefIdx(class))
        } else {
            None
        }
    }

    #[must_use]
    pub fn link(mut self, other: Self) -> Self {
        let original_str = self.alloc_string(MAIN_MODULE);
        for def in other.iter_class_defs() {
            let translated = self.translate_class_def(&other, def);
            let class_ref = self.alloc_class_ref(translated.ref_to());
            match self.class_defs.entry(ClassDefIdx(class_ref)) {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    occupied.get_mut().merge_defs(translated);
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(translated);
                }
            }
        }
        assert_eq!(self.alloc_string(MAIN_MODULE), original_str);
        self.sections.extend(other.sections);
        self
    }

    pub fn method_defs(&self) -> &FxHashMap<MethodDefIdx, MethodDef> {
        &self.method_defs
    }

    /// Checks if this assembly contains a reference [`ClassRef`]
    #[must_use]
    pub fn contains_ref(&self, cref: &ClassRef) -> bool {
        self.class_refs.1.contains_key(cref)
    }

    pub(crate) fn class_defs_mut_strings(
        &mut self,
    ) -> (
        &mut FxHashMap<ClassDefIdx, ClassDef>,
        &BiMap<StringIdx, IString>,
    ) {
        (&mut self.class_defs, &self.strings)
    }
    pub fn iter_nodes(&self) -> impl Iterator<Item = &CILNode> {
        self.nodes.0.iter()
    }
    pub fn iter_roots(&self) -> impl Iterator<Item = &CILRoot> {
        self.roots.0.iter()
    }
    pub fn remove_dead_statics(&mut self) {
        /*// Check which statics are referenced by real code.
                let alive_statics: FxHashSet<StaticFieldIdx> = self
                    .iter_nodes()
                    .filter_map(|node| match node {
                        CILNode::LdStaticField(fld) | CILNode::LdStaticFieldAdress(fld) => Some(*fld),
                        _ => None,
                    })
                    .collect();
                let defs: Vec<_> = self.iter_class_def_ids().copied().collect();
                for class_id in defs {
                    let class = self.get_class_def(class_id).clone();
                    // Collect all statics which, to which there exists a corresponding static field desc.
                    let statics: Vec<_> = class
                        .static_fields()
                        .iter()
                        .copied()
                        .filter(|(tpe, name, _)| {
                            alive_statics
                                .contains(&self.alloc_sfld(StaticFieldDesc::new(*class_id, *name, *tpe)))
                        })
                        .collect();
                    let class = self.class_mut(class_id);
                    *class.static_fields_mut() = statics;
                }
                // After removing all statics whose address nor value is not taken, replace any writes to those statics with pops.
        */
    }
    /// Preforms a "shallow" GC pass on all method defs, removing them if and only if:
    /// 1. They are not referenced by anything inside this assembly
    /// 2. They are not accessible from outside of it.
    ///
    /// **WARNING**: This gc is highly conservative, and will often not collect some things.
    /// To improve its accuracy, first do `link_gc`.
    pub fn shallow_methodef_gc(&mut self) {
        let live: FxHashSet<MethodRefIdx> = self
            .iter_nodes()
            .filter_map(|node| match node {
                CILNode::Call(boxed) => Some(boxed.0),
                CILNode::LdFtn(method_ref_idx) => Some(*method_ref_idx),
                CILNode::Const(_)
                | CILNode::BinOp(_, _, _)
                | CILNode::UnOp(_, _)
                | CILNode::LdLoc(_)
                | CILNode::LdLocA(_)
                | CILNode::LdArg(_)
                | CILNode::LdArgA(_)
                | CILNode::IntCast { .. }
                | CILNode::FloatCast { .. }
                | CILNode::RefToPtr(_)
                | CILNode::PtrCast(_, _)
                | CILNode::LdFieldAdress { .. }
                | CILNode::LdField { .. }
                | CILNode::LdInd { .. }
                | CILNode::SizeOf(_)
                | CILNode::GetException
                | CILNode::IsInst(_, _)
                | CILNode::CheckedCast(_, _)
                | CILNode::CallI(_)
                | CILNode::LocAlloc { .. }
                | CILNode::LdStaticField(_)
                | CILNode::LdStaticFieldAdress(_)
                | CILNode::LdTypeToken(_)
                | CILNode::LdLen(_)
                | CILNode::LocAllocAlgined { .. }
                | CILNode::LdElelemRef { .. }
                | CILNode::UnboxAny { .. } => None,
            })
            .chain(self.iter_roots().filter_map(|root| match root {
                CILRoot::Call(boxed) => Some(boxed.0),
                CILRoot::StLoc(_, _)
                | CILRoot::InitObj(_, _)
                | CILRoot::StArg(_, _)
                | CILRoot::Ret(_)
                | CILRoot::Pop(_)
                | CILRoot::Throw(_)
                | CILRoot::VoidRet
                | CILRoot::Break
                | CILRoot::Nop
                | CILRoot::Branch(_)
                | CILRoot::SourceFileInfo { .. }
                | CILRoot::SetField(_)
                | CILRoot::StInd(_)
                | CILRoot::InitBlk(_)
                | CILRoot::CpBlk(_)
                | CILRoot::CallI(_)
                | CILRoot::ExitSpecialRegion { .. }
                | CILRoot::ReThrow
                | CILRoot::SetStaticField { .. }
                | CILRoot::CpObj { .. }
                | CILRoot::Unreachable(_) => None,
            }))
            .collect();

        let mut live: FxHashSet<MethodDefIdx> = live
            .into_iter()
            .filter_map(|mref| self.method_ref_to_def(mref))
            .collect();
        if live.len() == self.method_defs.len() {
            println!("shallow_methodref_gc failed(no unreferenced methods)");
            return;
        }
        self.method_defs.retain(|id, def| {
            if live.contains(id) {
                true
            } else if !matches!(def.implementation(), MethodImpl::Extern { .. }) {
                live.insert(*id);
                true
            } else {
                false
            }
        });
        if live.len() == self.method_defs.len() {
            println!("shallow_methodref_gc failed(no unreferenced, externaly invisible methods)");
        }
        self.class_defs.values_mut().for_each(|tdef| {
            tdef.methods_mut()
                .retain(|methodef| live.contains(methodef));
        });
    }
    pub fn split_to_parts(&self, parts: u32) -> impl Iterator<Item = Self> + use<'_> {
        let lib_name = StringIdx::from_index(std::num::NonZeroU32::new(1).unwrap());
        // Since 1st part is dedicated to methods which access statics, split the rest into n-1 parts.
        let div = (self.method_refs.len().div_ceil(parts as usize - 1)) as u32;
        // Into 1st. Only split out the methods where it is known, for sure, that they don't access any statics.
        (0..parts).map(move |rem| {
            let mut part = self.clone();
            part.method_defs.iter_mut().for_each(|(idx, def)| {
                if def.accesses_statics(self) {
                    if 0 != rem {
                        *def.implementation_mut() = MethodImpl::Extern {
                            lib: lib_name,
                            preserve_errno: false,
                        }
                    }
                } else if idx.as_bimap_index().get() / div + 1 != rem {
                    *def.implementation_mut() = MethodImpl::Extern {
                        lib: lib_name,
                        preserve_errno: false,
                    }
                }
            });
            if 0 != rem {
                part.class_defs
                    .iter_mut()
                    .for_each(|(_, def)| *def.static_fields_mut() = vec![]);
            }
            part.eliminate_dead_types();
            //part.eliminate_dead_fns(true);
            part = part.link_gc();
            part.shallow_methodef_gc();
            part
        })
    }
    pub fn only_statics(&self) -> Self {
        let lib_name = StringIdx::from_index(std::num::NonZeroU32::new(1).unwrap());
        let mut empty = self.clone();
        empty.method_defs.iter_mut().for_each(|(_, def)| {
            *def.implementation_mut() = MethodImpl::Extern {
                lib: lib_name,
                preserve_errno: false,
            }
        });
        empty.eliminate_dead_types();
        empty = empty.link_gc();
        empty
    }
    pub fn fix_aligement(&mut self) {
        let method_def_idxs: Box<[_]> = self.method_defs.keys().copied().collect();
        for method in method_def_idxs {
            let mut tmp_method = self.borrow_methoddef(method);
            tmp_method.adjust_aligement(self);
            self.return_methoddef(method, tmp_method);
        }
    }
    pub fn alignof_type(&self, tpe: TypeIdx) -> u64 {
        match self[tpe] {
            Type::FnPtr(_) | Type::Ptr(_) | Type::Ref(_) => 8, // ASSUMES alignof<*T>() = 8.
            Type::Int(int) => int.size().unwrap_or(8) as u64,  // ASSUMES alignof<usize>() = 8.
            Type::ClassRef(class_ref_idx) => match self.class_ref_to_def(class_ref_idx) {
                Some(def) => self[def]
                    .align()
                    .unwrap_or(std::num::NonZeroU32::new(8).unwrap())
                    .get() as u64,
                None => 8,
            },
            Type::Float(float) => float.size() as u64,
            Type::PlatformString | Type::PlatformObject | Type::PlatformArray { .. } => 8, // ASSUMES alignof<&managed T>() = 8.
            Type::PlatformChar => 2,
            Type::PlatformGeneric(_, _) => 8,
            Type::Bool => 1,
            Type::Void => 0,
            Type::SIMDVector(simdvector) => match simdvector.elem() {
                super::tpe::simd::SIMDElem::Int(int) => int.size().unwrap_or(8) as u64, // ASSUMES alignof<usize>() = 8.
                super::tpe::simd::SIMDElem::Float(float) => float.size() as u64,
            },
        }
    }

    pub fn method_refs(&self) -> &BiMap<MethodRefIdx, MethodRef> {
        &self.method_refs
    }

    pub fn strings(&self) -> &StringMap {
        &self.strings
    }

    pub fn shorten_strings(&mut self, size_cap: usize) {
        self.strings.map_values(|string| {
            if string.len() > size_cap {
                eprint!("shortening {string}");
                *string = encode(hash64(string)).into();
                eprintln!("to {string}");
            }
        })
    }

    fn link_gc(self) -> Self {
        let mut clone = self.clone();
        clone = clone.link(self);
        clone
    }
    pub(crate) fn ptr_size(&self) -> u32 {
        8
    }
    pub(crate) fn sizeof_type(&self, field_tpe: Type) -> u32 {
        match field_tpe {
            Type::Ref(_) | Type::Ptr(_) => self.ptr_size(),
            Type::Int(int) => int
                .size()
                .unwrap_or(self.ptr_size().try_into().unwrap())
                .into(),
            Type::ClassRef(class_ref_idx) => self[self.class_ref_to_def(class_ref_idx).unwrap()]
                .explict_size()
                .unwrap()
                .into(),
            Type::Float(float) => float.size().into(),
            Type::PlatformString => self.ptr_size(),
            Type::PlatformChar => 1,
            Type::PlatformGeneric(_, _) => todo!(),
            Type::PlatformObject => self.ptr_size(),
            Type::Bool => 1,
            Type::Void => 0,
            Type::PlatformArray { .. } => todo!(),
            Type::FnPtr(_) => self.ptr_size(),
            Type::SIMDVector(simdvector) => (simdvector.bits() / 8).into(),
        }
    }

    pub fn add_section(&mut self, arg: &str, packed_metadata: impl Into<Vec<u8>>) {
        self.sections.insert(arg.into(), packed_metadata.into());
    }

    pub(crate) fn get_section(&self, arg: &str) -> Option<&Vec<u8>> {
        self.sections.get(arg)
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
#[test]
fn test_encoded_stats() {
    assert_eq!(encoded_stats(&u64::MAX), (type_name::<u64>(), 10));
    assert_eq!(encoded_stats(&0_i32), (type_name::<i32>(), 1));
}
pub fn encoded_stats<T: Serialize + for<'a> Deserialize<'a>>(val: &T) -> (&'static str, usize) {
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

pub static ILASM_FLAVOUR: std::sync::LazyLock<IlasmFlavour> = std::sync::LazyLock::new(|| {
    if String::from_utf8_lossy(
            &std::process::Command::new(ilasm_path()).arg("--help")
                .output()
                .unwrap_or_else(|_| panic!("Could not run the IL assembler(ilasm) at path {:?}. Is ilasm propely installed? If so, try specifying a precise path by seting the ILASM_PATH enviroment variable",*ILASM_PATH))
                .stdout,
        )
        .contains("PDB")
        {
            IlasmFlavour::Modern
        } else {
            IlasmFlavour::Clasic
        }
});

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IlasmFlavour {
    Clasic,
    Modern,
}
#[must_use]
pub fn ilasm_path() -> &'static str {
    ILASM_PATH.as_str()
}
#[allow(dead_code)]
fn chunked_range(top: u32, parts: u32) -> impl Iterator<Item = std::ops::Range<u32>> {
    let chunk_size = top.div_ceil(parts); // Ceiling of n / m

    assert!(parts < top);
    (0..top).filter_map(move |i| {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, top);
        if start < top {
            Some(start..end)
        } else {
            None
        }
    })
}
#[doc = "Specifies the path to the IL assembler."]
pub static ILASM_PATH: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::vars()
        .find_map(|(key, value)| {
            if key == "ILASM_PATH" {
                Some(value)
            } else {
                None
            }
        })
        .unwrap_or(get_default_ilasm())
});

#[cfg(not(target_os = "windows"))]
/// Finds the default instance of the IL assembler.
fn get_default_ilasm() -> String {
    "ilasm".into()
}
#[test]
#[cfg(not(miri))]
fn test_chunked_range() {
    for count in 1..100 {
        for parts in 1..count {
            let range = chunked_range(count, parts);
            assert_eq!(
                range.flatten().max().unwrap(),
                count - 1,
                "count:{count},parts:{parts},range:"
            );
            let range = chunked_range(count, parts);
            assert_eq!(
                range.flatten().count(),
                count.try_into().unwrap(),
                "count:{count},parts:{parts},range:"
            );
        }
    }
}
#[test]
fn test_get_default_ilasm() {
    assert!(get_default_ilasm().contains("ilasm"));
}
#[cfg(target_os = "windows")]
fn get_default_ilasm() -> String {
    if std::process::Command::new("ilasm")
        .arg("--help")
        .output()
        .is_ok()
    {
        return "ilasm".into();
    }
    // Framework Path
    let framework_path = std::path::PathBuf::from("C:\\Windows\\Microsoft.NET\\Framework");
    let framework_dir = std::fs::read_dir(&framework_path).unwrap_or_else(|_| panic!("Could not find the .NET framework directory at {framework_path:?}, when searching for ilasm."));
    for entry in framework_dir {
        let entry = entry.unwrap();
        // TODO: find the most recent framework
        if entry.metadata().unwrap().is_dir() {
            let mut ilasm_path = entry.path();
            ilasm_path.push("ilasm");
            ilasm_path.set_extension("exe");
            if !std::fs::exists(&ilasm_path).unwrap_or(false) {
                eprintln!("Could not find ilasm at:{ilasm_path:?}");
                continue;
            }
            return ilasm_path.display().to_string();
        }
    }
    panic!("Could not find a .NET framework in directory {framework_path:?}, when searching for ilasm.")
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
    #[cfg(not(miri))]
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
    #[cfg(not(miri))]
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
    #[cfg(not(miri))]
    asm.export("/tmp/link_test.exe", ILExporter::new(*ILASM_FLAVOUR, false));
}
config! {LINKER_RECOVER,bool,false}
