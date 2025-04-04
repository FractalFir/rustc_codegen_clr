use fxhash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::{IsPure, MethodKind},
    Access, Assembly, BasicBlock, CILIterElem, CILNode, ClassDefIdx, ClassRef, ClassRefIdx, Int,
    SigIdx, StringIdx, Type, TypeIdx,
};
use crate::{cil_node::CallOpArgs, v2::iter::TpeIter};
use crate::v2::CILRoot;
pub type LocalId = u32;
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct MethodRef {
    class: ClassRefIdx,
    name: StringIdx,
    sig: SigIdx,
    kind: MethodKind,
    generics: Box<[Type]>,
}

impl MethodRef {
    #[must_use]
    pub fn into_def(
        &self,
        implementation: MethodImpl,
        access: Access,
        asm: &Assembly,
    ) -> MethodDef {
        let class = asm.class_ref_to_def(self.class()).unwrap();
        let arg_names = (0..(asm[self.sig()].inputs().len()))
            .map(|_| None)
            .collect();
        MethodDef::new(
            access,
            class,
            self.name,
            self.sig,
            self.kind,
            implementation,
            arg_names,
        )
    }
    #[must_use]
    pub fn new(
        class: ClassRefIdx,
        name: StringIdx,
        sig: SigIdx,
        kind: MethodKind,
        generics: Box<[Type]>,
    ) -> Self {
        Self {
            class,
            name,
            sig,
            kind,
            generics,
        }
    }

    #[must_use]
    pub fn class(&self) -> ClassRefIdx {
        self.class
    }

    #[must_use]
    pub fn name(&self) -> StringIdx {
        self.name
    }

    #[must_use]
    pub fn sig(&self) -> SigIdx {
        self.sig
    }

    #[must_use]
    pub fn kind(&self) -> MethodKind {
        self.kind
    }

    #[must_use]
    pub fn generics(&self) -> &[Type] {
        &self.generics
    }
    /// Returns the inputs of this methods, excluding this for constructors.
    pub fn stack_inputs<'s, 'asm: 's>(&'s self, asm: &'asm Assembly) -> &'s [Type] {
        let sig = &asm[self.sig];
        match self.kind() {
            MethodKind::Static => sig.inputs(),
            MethodKind::Instance => sig.inputs(),
            MethodKind::Virtual => sig.inputs(),
            MethodKind::Constructor => &sig.inputs()[1..],
        }
    }
    /// Returns the output of this method.
    pub fn output(&self, asm: &Assembly) -> Type {
        let sig = &asm[self.sig];
        match self.kind() {
            MethodKind::Static => *sig.output(),
            MethodKind::Instance => *sig.output(),
            MethodKind::Virtual => *sig.output(),
            MethodKind::Constructor => Type::ClassRef(self.class()),
        }
    }

    pub fn aligned_alloc(asm: &mut crate::v2::Assembly) -> MethodRef {
        let void_ptr = asm.nptr(Type::Void);
        let sig = asm.sig([Type::Int(Int::USize), Type::Int(Int::USize)], void_ptr);
        MethodRef::new(
            ClassRef::native_mem(asm),
            asm.alloc_string("AlignedAlloc"),
            sig,
            MethodKind::Static,
            vec![].into(),
        )
    }
    pub fn alloc(asm: &mut crate::v2::Assembly) -> MethodRef {
        let sig = asm.sig([Type::Int(Int::ISize)], Type::Int(Int::ISize));
        MethodRef::new(
            ClassRef::marshal(asm),
            asm.alloc_string("AllocHGlobal"),
            sig,
            MethodKind::Static,
            vec![].into(),
        )
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MethodRefIdx(BiMapIndex);
impl IntoBiMapIndex for MethodRefIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct MethodDef {
    access: Access,
    class: ClassDefIdx,
    name: StringIdx,
    sig: SigIdx,
    arg_names: Vec<Option<StringIdx>>,
    kind: MethodKind,
    implementation: MethodImpl,
}

impl MethodDef {
    pub fn iter_types<'a, 'asm: 'a>(
        &'a self,
        asm: &'asm Assembly,
    ) -> impl Iterator<Item = Type> + 'a {
        let defining_class = Type::ClassRef(*self.class());
        let sig = &asm[self.sig()];
        let sig_types = sig.iter_types();
        let local_types = self.iter_locals(asm).map(|(_, tpe)| asm[*tpe]);
        let body_types = self
            .iter_cil(asm)
            .into_iter()
            .map(|cil| cil.iter_types(asm));
        let body_types = body_types.flatten();
        std::iter::once(defining_class)
            .chain(sig_types)
            .chain(local_types)
            .chain(body_types)
    }
    #[must_use]
    pub fn iter_cil<'asm: 'method, 'method>(
        &'method self,
        asm: &'asm Assembly,
    ) -> Option<impl Iterator<Item = CILIterElem> + 'method> {
        match self.resolved_implementation(asm) {
            MethodImpl::MethodBody { blocks, .. } => Some(
                blocks
                    .iter()
                    .flat_map(super::basic_block::BasicBlock::iter_roots)
                    .flat_map(|root| super::CILIter::new(asm.get_root(root).clone(), asm)),
            ),
            MethodImpl::Extern { .. } => None,
            MethodImpl::AliasFor(_) => {
                panic!("Unresolved alias returned by MethodDef::resolved_implementation")
            }
            MethodImpl::Missing => None,
        }
    }
    #[must_use]
    pub fn ref_to(&self) -> MethodRef {
        MethodRef::new(
            *self.class(),
            self.name(),
            self.sig(),
            self.kind(),
            vec![].into(),
        )
    }
    #[must_use]
    pub fn new(
        access: Access,
        class: ClassDefIdx,
        name: StringIdx,
        sig: SigIdx,
        kind: MethodKind,
        implementation: MethodImpl,
        arg_names: Vec<Option<StringIdx>>,
    ) -> Self {
        Self {
            access,
            class,
            name,
            sig,
            arg_names,
            kind,
            implementation,
        }
    }

    #[must_use]
    pub fn class(&self) -> ClassDefIdx {
        self.class
    }

    #[must_use]
    pub fn name(&self) -> StringIdx {
        self.name
    }

    #[must_use]
    pub fn sig(&self) -> SigIdx {
        self.sig
    }

    #[must_use]
    pub fn kind(&self) -> MethodKind {
        self.kind
    }

    #[must_use]
    pub fn implementation(&self) -> &MethodImpl {
        &self.implementation
    }
    #[must_use]
    pub fn resolved_implementation<'asm: 'method, 'method>(
        &'method self,
        asm: &'asm Assembly,
    ) -> &'method MethodImpl {
        match self.implementation {
            MethodImpl::MethodBody { .. } | MethodImpl::Extern { .. } | MethodImpl::Missing => {
                &self.implementation
            }
            MethodImpl::AliasFor(method) => asm
                .method_def_from_ref(method)
                .expect("ERROR: a method is an alias for an extern function")
                .resolved_implementation(asm),
        }
    }
    pub fn implementation_mut(&mut self) -> &mut MethodImpl {
        &mut self.implementation
    }

    pub fn from_v1(
        v1: &crate::method::Method,
        asm: &mut super::Assembly,
        class: ClassDefIdx,
    ) -> Self {
        let sig = v1.sig().clone();
        let sig_idx = asm.alloc_sig(v1.sig().clone());
        let acceess = match v1.access() {
            crate::access_modifier::AccessModifer::Private => Access::Private,
            crate::access_modifier::AccessModifer::Public => Access::Public,
            crate::access_modifier::AccessModifer::Extern => Access::Extern,
        };

        let kind = if v1.is_static() {
            MethodKind::Static
        } else if v1.name() == ".ctor" {
            MethodKind::Constructor
        } else {
            MethodKind::Instance
        };
        let name = asm.alloc_string(v1.name());
        let blocks = v1
            .blocks()
            .iter()
            .map(|block| crate::v2::BasicBlock::from_v1(block, asm))
            .collect();
        let locals = v1.locals().to_vec();
        let implementation = MethodImpl::MethodBody { blocks, locals };
        let mut arg_names: Vec<_> = v1.arg_names().to_vec();
        let arg_debug_count = arg_names.len();
        let arg_sig_count = sig.inputs().len();
        match arg_debug_count.cmp(&arg_sig_count) {
            std::cmp::Ordering::Less => {
                println!(
                    "WARNING: argument debug info count invalid(Too few). Expected {}, got {}. fn name:{}",
                    arg_sig_count,
                    arg_debug_count,
                    v1.name()
                );
                arg_names.extend((arg_debug_count..arg_sig_count).map(|_| None));
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                println!(
                "WARNING: argument debug info count invalid(Too many). Expected {}, got {}. fn name:{}",
                arg_sig_count,
                arg_debug_count,
                v1.name()
                );
                for arg in &arg_names {
                    println!("{:?}", arg.map(|arg| &asm[arg]));
                }
                arg_names.truncate(arg_sig_count);
            }
        }
        assert_eq!(arg_names.len(), v1.sig().inputs().len());
        MethodDef::new(
            acceess,
            class,
            name,
            sig_idx,
            kind,
            implementation,
            arg_names,
        )
    }

    #[must_use]
    pub fn access(&self) -> &Access {
        &self.access
    }

    #[must_use]
    pub fn arg_names(&self) -> &[Option<StringIdx>] {
        &self.arg_names
    }

    pub(crate) fn iter_locals<'a>(
        &'a self,
        asm: &'a Assembly,
    ) -> impl Iterator<Item = &'a (Option<StringIdx>, TypeIdx)> {
        match self.resolved_implementation(asm) {
            MethodImpl::MethodBody { blocks: _, locals } => locals.iter(),
            MethodImpl::Extern { .. } | MethodImpl::Missing => [].iter(),
            MethodImpl::AliasFor(_) => panic!(),
        }
    }

    /// Sets the accesibility of this method to `access`.
    pub fn set_access(&mut self, access: Access) {
        self.access = access;
    }

    pub fn stack_inputs(&self, asm: &mut Assembly) -> Vec<(Type, Option<StringIdx>)> {
        let mut arg_names = self.arg_names().to_vec();
        let sig = asm[self.sig()].clone();
        arg_names.extend((arg_names.len()..(sig.inputs().len())).map(|_| None));
        sig.inputs()
            .iter()
            .copied()
            .zip(arg_names.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn blocks<'s, 'asm: 's>(&'s self, asm: &'asm Assembly) -> Option<&'s [BasicBlock]> {
        self.resolved_implementation(asm)
            .blocks()
            .map(|vec| vec.as_ref())
    }
    pub fn adjust_aligement(&mut self, asm: &mut Assembly) {
        let MethodImpl::MethodBody { blocks, locals } = self.implementation_mut() else {
            return;
        };
        assert!(!blocks.is_empty());
        let to_map: Vec<_> = locals
            .iter()
            .map(|(name, tpe)| (*name, *tpe, asm.alignof_type(*tpe)))
            .enumerate()
            .collect();
        // Check which locals get their address taken.
        let mut local_address_of = vec![false; locals.len()];
        for node in blocks
            .iter()
            .flat_map(super::basic_block::BasicBlock::iter_roots)
            .flat_map(|root| super::CILIter::new(asm.get_root(root).clone(), asm))
            .filter_map(super::iter::CILIterElem::as_node)
        {
            if let CILNode::LdLocA(loc) = node {
                local_address_of[loc as usize] = true
            }
        }
        let mut preamble = vec![];
        for (local_id, (_, tpe_idx, align)) in to_map {
            if align <= asm.guaranted_align() as u64 {
                // Aligement guanrateed by .NET, skip.
                continue;
            }
            // Check that the address of this local is ever taken. If not, just ignore it.
            if !local_address_of[local_id] {
                continue;
            }
            // Change the type of the local var.
            let tpe_ptr = asm.nptr(tpe_idx);
            let tpe_ptr = asm.alloc_type(tpe_ptr);
            locals[local_id].1 = tpe_ptr;
            // Allocate a new buffer for the local var, aligned to align.
            let tpe = asm[tpe_idx];
            let local_buff = asm.alloc_node(CILNode::LocAllocAlgined {
                tpe: tpe_idx,
                align,
            });
            preamble.push(asm.alloc_root(CILRoot::StLoc(local_id as u32, local_buff)));
            // Map all usages of this local, to ensure it is propely alligned.
            blocks
                .iter_mut()
                .flat_map(|block| block.roots_mut())
                .for_each(|root_idx| {
                    let root = asm[*root_idx].clone();
                    let local_addr = asm.alloc_node(CILNode::LdLoc(local_id as u32));
                    let root = root.map(
                        asm,
                        &mut |root, _| match root {
                            CILRoot::StLoc(loc, val) if loc == local_id as u32 => {
                                CILRoot::StInd(Box::new((local_addr, val, tpe, false)))
                            }
                            _ => root,
                        },
                        &mut |node, _| match node {
                            CILNode::LdLocA(loc) if loc == local_id as u32 => {
                                CILNode::LdLoc(local_id as u32)
                            }
                            CILNode::LdLoc(loc) if loc == local_id as u32 => CILNode::LdInd {
                                addr: local_addr,
                                tpe: tpe_idx,
                                volatile: false,
                            },
                            _ => node,
                        },
                    );
                    *root_idx = asm.alloc_root(root);
                });
        }
        preamble.extend(blocks[0].roots().iter().copied());
        *blocks[0].roots_mut() = preamble;
    }

    pub(crate) fn remove_dead_blocks(&mut self, asm: &Assembly) {
        // This opt only makes sense if this method has an impl
        let Some(blocks) = self.implementation().blocks() else {
            return;
        };
        // Check if the entry block does not jump anywhere(no targets) and has no handler - if so, only keep it.
        if blocks[0].targets(asm).count() == 0 && blocks[0].handler().is_none() {
            let entry = blocks[0].clone();
            *self.implementation_mut().blocks_mut().unwrap() = vec![entry];
            return;
        }
        let mut alive: FxHashSet<_> = blocks.iter().flat_map(|block| block.targets(asm)).collect();
        // entry block is always live
        alive.insert(blocks[0].block_id());
        // if alive < total, then there are some dead blocks, then remove them.
        if alive.len() >= blocks.len() {
            return;
        }
        // If handlers jump to normal blocks, do not GC.
        if blocks
            .iter()
            .flat_map(|block| block.handler())
            .flatten()
            .flat_map(|block| block.roots())
            .any(|root| {
                matches!(
                    asm[*root],
                    CILRoot::ExitSpecialRegion {
                        target: _,
                        source: _
                    }
                )
            })
        {
            return;
        }
        //let blocks_copy = blocks.clone();
        self.implementation_mut()
            .blocks_mut()
            .unwrap()
            .retain(|block| alive.contains(&block.block_id()));
    }

    pub(crate) fn locals(&self) -> Option<&[LocalDef]> {
        let MethodImpl::MethodBody { blocks: _, locals } = self.implementation() else {
            return None;
        };
        Some(locals)
    }

    pub fn accesses_statics(&self, asm: &Assembly) -> bool {
        let Some(mut cil) = self.iter_cil(asm) else {
            return false;
        };
        cil.any(|node| {
            matches!(
                node,
                CILIterElem::Node(CILNode::LdStaticField(_) | CILNode::LdStaticFieldAdress(_))
                    | CILIterElem::Root(CILRoot::SetStaticField { .. })
            )
        })
    }
}
pub type LocalDef = (Option<StringIdx>, TypeIdx);
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum MethodImpl {
    MethodBody {
        blocks: Vec<BasicBlock>,
        locals: Vec<LocalDef>,
    },
    Extern {
        lib: StringIdx,
        preserve_errno: bool,
    },
    AliasFor(MethodRefIdx),
    Missing,
}
impl MethodImpl {
    pub fn root_count(&self) -> usize {
        match self {
            MethodImpl::MethodBody { blocks, .. } => {
                blocks.iter().map(|block| block.roots().len()).sum()
            }
            MethodImpl::Extern { .. } => 0,
            MethodImpl::AliasFor(_) => 0,
            MethodImpl::Missing => 3,
        }
    }
    pub fn blocks_mut(&mut self) -> Option<&mut Vec<BasicBlock>> {
        match self {
            Self::MethodBody { blocks, .. } => Some(blocks),
            _ => None,
        }
    }
    pub fn blocks(&self) -> Option<&Vec<BasicBlock>> {
        match self {
            Self::MethodBody { blocks, .. } => Some(blocks),
            _ => None,
        }
    }

    /// Returns `true` if the method impl is [`Extern`].
    ///
    /// [`Extern`]: MethodImpl::Extern
    #[must_use]
    pub fn is_extern(&self) -> bool {
        matches!(self, Self::Extern { .. })
    }
    // While this function is a bit long, this is not an issue.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn merge_cctor_impls(&mut self, implementation: &MethodImpl, asm: &Assembly) {
        let tmp = match (&self, &implementation) {
            (
                MethodImpl::MethodBody { blocks, locals },
                MethodImpl::MethodBody {
                    blocks: other_blocks,
                    locals: other_locals,
                },
            ) => {
                assert_eq!(locals, other_locals);
                let mut blocks = blocks.clone();
                // First, check that 1st blocks end in `VoidRet`, and remove it.
                let last_root = if blocks.is_empty() {
                    blocks.push(BasicBlock::new(vec![], 0, None));
                    CILRoot::VoidRet
                } else {
                    blocks
                        .last_mut()
                        .unwrap()
                        .roots_mut()
                        .pop()
                        .map_or(CILRoot::VoidRet, |root_id| asm.get_root(root_id).clone())
                };
                assert_eq!(last_root, CILRoot::VoidRet);
                assert_eq!(
                    other_blocks.len(),
                    1,
                    "Only merging one block method impls is currently supported"
                );
                blocks
                    .last_mut()
                    .unwrap()
                    .roots_mut()
                    .extend(other_blocks.last().unwrap().roots());
                MethodImpl::MethodBody {
                    blocks,
                    locals: locals.clone(),
                }
            }
            (MethodImpl::MethodBody { .. }, MethodImpl::Extern { .. }) => {
                panic!("Unmergable method impl: Can't merge MethodBody with Extern.")
            }
            (MethodImpl::MethodBody { .. }, MethodImpl::AliasFor(_)) => {
                panic!("Unmergable method impl: Can't merge MethodBody with AliasFor.")
            }

            (MethodImpl::Extern { .. }, MethodImpl::MethodBody { .. }) => {
                panic!("Unmergable method impl: Can't merge Extern with MethodBody.")
            }
            (
                MethodImpl::Extern {
                    lib,
                    preserve_errno,
                },
                MethodImpl::Extern {
                    lib: liba,
                    preserve_errno: preserve_errnoa,
                },
            ) => {
                assert_eq!(lib, liba);
                assert_eq!(preserve_errno, preserve_errnoa);
                self.clone()
            }
            (MethodImpl::Extern { .. }, MethodImpl::AliasFor(_)) => {
                panic!("Unmergable method impl: Can't merge Extern with AliasFor.")
            }
            (
                MethodImpl::Extern {
                    lib,
                    preserve_errno,
                },
                MethodImpl::Missing,
            )
            | (
                MethodImpl::Missing,
                MethodImpl::Extern {
                    lib,
                    preserve_errno,
                },
            ) => MethodImpl::Extern {
                lib: *lib,
                preserve_errno: *preserve_errno,
            },
            (
                MethodImpl::AliasFor(_),
                MethodImpl::MethodBody { .. } | MethodImpl::Extern { .. },
            ) => {
                panic!("Unmergable method impl: can't merge alias.")
            }
            (MethodImpl::AliasFor(a), MethodImpl::AliasFor(b)) => {
                assert_eq!(a, b);
                self.clone()
            }
            (MethodImpl::AliasFor(alias), MethodImpl::Missing)
            | (MethodImpl::Missing, MethodImpl::AliasFor(alias)) => MethodImpl::AliasFor(*alias),
            (MethodImpl::Missing, MethodImpl::MethodBody { blocks, locals })
            | (MethodImpl::MethodBody { blocks, locals }, MethodImpl::Missing) => {
                MethodImpl::MethodBody {
                    blocks: blocks.clone(),
                    locals: locals.clone(),
                }
            }

            (MethodImpl::Missing, MethodImpl::Missing) => MethodImpl::Missing,
        };
        *self = tmp;
    }

    pub(crate) fn realloc_locals(&mut self, asm: &mut Assembly) {
        // Optimization only suported for methods with locals
        let MethodImpl::MethodBody {
            blocks,
            ref mut locals,
        } = self
        else {
            return;
        };
        let mut new_locals = std::sync::Mutex::new(Vec::new());
        let local_map = std::sync::Mutex::new(FxHashMap::default());
        for block in blocks.iter_mut() {
            block.map_roots(
                asm,
                &mut |root, _| match root {
                    CILRoot::StLoc(loc, tree) => CILRoot::StLoc(
                        match local_map.lock().unwrap().entry(loc) {
                            std::collections::hash_map::Entry::Occupied(val) => *val.get(),
                            std::collections::hash_map::Entry::Vacant(empty) => {
                                let mut new_locals = new_locals.lock().unwrap();
                                let new_idx = new_locals.len();
                                new_locals.push(locals[loc as usize]);
                                *empty.insert(new_idx as u32)
                            }
                        },
                        tree,
                    ),
                    _ => root,
                },
                &mut |node, _| match node {
                    CILNode::LdLoc(loc) => {
                        CILNode::LdLoc(match local_map.lock().unwrap().entry(loc) {
                            std::collections::hash_map::Entry::Occupied(val) => *val.get(),
                            std::collections::hash_map::Entry::Vacant(empty) => {
                                let mut new_locals = new_locals.lock().unwrap();
                                let new_idx = new_locals.len();
                                new_locals.push(locals[loc as usize]);
                                *empty.insert(new_idx as u32)
                            }
                        })
                    }
                    CILNode::LdLocA(loc) => {
                        CILNode::LdLocA(match local_map.lock().unwrap().entry(loc) {
                            std::collections::hash_map::Entry::Occupied(val) => *val.get(),
                            std::collections::hash_map::Entry::Vacant(empty) => {
                                let mut new_locals = new_locals.lock().unwrap();
                                let new_idx = new_locals.len();
                                new_locals.push(locals[loc as usize]);
                                *empty.insert(new_idx as u32)
                            }
                        })
                    }
                    _ => node,
                },
            );
        }
        // Swap new and locals
        std::mem::swap(locals, new_locals.get_mut().unwrap());
    }
    
    pub(crate) fn wrapper(mref: MethodRefIdx, asm: &mut Assembly) -> MethodImpl {
        let sig = asm[asm[mref].sig()].clone();
        let args = sig.inputs().iter().enumerate().map(|(idx,_)|asm.alloc_node(CILNode::LdArg(idx.try_into().unwrap()))).collect();
        let roots = if asm.sizeof_type(*sig.output()) == 0{
            let call = asm.alloc_root(CILRoot::Call(Box::new((mref,args,  IsPure::NOT))));
            vec![call,asm.alloc_root(CILRoot::VoidRet)]
        } else{
            let val = asm.alloc_node(CILNode::Call(Box::new((mref,args,  IsPure::NOT))));
            vec![asm.alloc_root(CILRoot::Ret(val))]
        };
        MethodImpl::MethodBody { blocks: vec![BasicBlock::new(roots,0,None)], locals: vec![].into() }
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MethodDefIdx(pub MethodRefIdx);

impl std::ops::Deref for MethodDefIdx {
    type Target = MethodRefIdx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl MethodRefIdx {
    /// # Safety
    /// This function can't cause UB, but it can corrupt the internal assembly representation. Can only be used to rebuild a [`MethodRefIdx`] obtained in some other way.
    #[must_use]
    pub unsafe fn from_raw(raw: BiMapIndex) -> Self {
        Self(raw)
    }
}
#[test]
fn locals() {
    fn method(locals: &[LocalDef], asm: &mut Assembly) -> MethodDef {
        let name: StringIdx = asm.alloc_string("DoSomething");
        let mimpl = MethodImpl::MethodBody {
            blocks: vec![],
            locals: locals.into(),
        };
        let main_module = asm.main_module();
        let sig = asm.sig([], Type::Void);
        MethodDef::new(
            Access::Extern,
            main_module,
            name,
            sig,
            MethodKind::Static,
            mimpl,
            vec![],
        )
    }
    let mut asm = Assembly::default();
    assert_eq!(method(&[], &mut asm).iter_locals(&asm).count(), 0);
    let tpe = asm.alloc_type(Type::Bool);
    let tpe2 = asm.alloc_type(Type::Bool);
    assert_eq!(
        method(&[(None, tpe)], &mut asm).iter_locals(&asm).count(),
        1
    );
    assert_eq!(
        method(&[(None, tpe), (None, tpe2)], &mut asm)
            .iter_locals(&asm)
            .cloned()
            .collect::<Vec<(Option<StringIdx>, _)>>(),
        vec![(None, tpe), (None, tpe2)]
    );
    let mut method = method(&[(None, tpe), (None, tpe2)], &mut asm);
    assert_eq!(
        method
            .iter_locals(&asm)
            .cloned()
            .collect::<Vec<(Option<StringIdx>, _)>>(),
        vec![(None, tpe), (None, tpe2)]
    );
    method.implementation.realloc_locals(&mut asm);
    assert_eq!(method.iter_locals(&asm).count(), 0);
}
#[test]
fn test_extern() {
    assert!(!MethodImpl::MethodBody {
        blocks: vec![],
        locals: vec![],
    }
    .is_extern());
    let mut asm = Assembly::default();
    let name: StringIdx = asm.alloc_string("libsomething.so");
    assert!(MethodImpl::Extern {
        lib: name,
        preserve_errno: false,
    }
    .is_extern())
}
#[test]
fn cil() {
    use super::RootIdx;
    fn method(roots: &[RootIdx], asm: &mut Assembly) -> MethodDef {
        let name: StringIdx = asm.alloc_string("DoSomething");
        let mimpl = MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(roots.to_vec(), 0, None)],
            locals: vec![],
        };
        let main_module = asm.main_module();
        let sig = asm.sig([], Type::Void);
        MethodDef::new(
            Access::Extern,
            main_module,
            name,
            sig,
            MethodKind::Static,
            mimpl,
            vec![],
        )
    }
    let mut asm = Assembly::default();
    assert_eq!(
        method(&[], &mut asm)
            .iter_cil(&asm)
            .map(|iter| iter.count()),
        Some(0)
    );
    let void_ret = asm.alloc_root(CILRoot::VoidRet);
    assert_eq!(
        method(&[void_ret], &mut asm)
            .iter_cil(&asm)
            .map(|iter| iter.collect::<Vec<_>>()),
        Some(vec![CILIterElem::Root(CILRoot::VoidRet)])
    );
    let const0 = asm.alloc_node(crate::v2::Const::I32(0));
    let const0_ret = asm.alloc_root(CILRoot::Ret(const0));
    assert_eq!(
        method(&[const0_ret], &mut asm)
            .iter_cil(&asm)
            .map(|iter| iter.collect::<Vec<_>>()),
        Some(vec![
            CILIterElem::Root(CILRoot::Ret(const0)),
            CILIterElem::Node(crate::v2::Const::I32(0).into()),
        ])
    );
    let name: StringIdx = asm.alloc_string("DoSomething");
    let main_module = asm.main_module();
    let sig = asm.sig([], Type::Void);
    assert_eq!(
        MethodDef::new(
            Access::Extern,
            main_module,
            name,
            sig,
            MethodKind::Static,
            MethodImpl::Extern {
                lib: name,
                preserve_errno: false,
            },
            vec![],
        )
        .iter_cil(&asm)
        .map(|iter| iter.count()),
        None,
    );
    assert_eq!(
        MethodDef::new(
            Access::Extern,
            main_module,
            name,
            sig,
            MethodKind::Static,
            MethodImpl::Missing,
            vec![],
        )
        .iter_cil(&asm)
        .map(|iter| iter.count()),
        None,
    );
}
