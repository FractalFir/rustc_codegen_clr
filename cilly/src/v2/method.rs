use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::MethodKind,
    Access, Assembly, BasicBlock, CILIterElem, CILNode, ClassDefIdx, ClassRefIdx, SigIdx,
    StringIdx, Type, TypeIdx,
};
use crate::v2::iter::TpeIter;
use crate::v2::CILRoot;
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
        let arg_names = (0..(asm.get_sig(self.sig()).inputs().len()))
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
        let sig = asm.get_sig(self.sig());
        let sig_types = sig.iter_types();
        let local_types = self.iter_locals(asm).map(|(_, tpe)| asm.get_type(*tpe));
        let body_types = self
            .iter_cil(asm)
            .into_iter()
            .map(|cil| cil.iter_types(asm));
        let body_types = body_types.flatten();
        std::iter::once(defining_class)
            .chain(sig_types)
            .chain(local_types.copied())
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
        let sig = asm.alloc_sig(v1.call_site().signature().clone());
        let acceess = match v1.access() {
            crate::access_modifier::AccessModifer::Private => Access::Private,
            crate::access_modifier::AccessModifer::Public => Access::Public,
            crate::access_modifier::AccessModifer::Extern => Access::Extern,
        };

        let kind = if v1.call_site().is_static() {
            MethodKind::Static
        } else if v1.call_site().name() == ".ctor" {
            MethodKind::Constructor
        } else {
            MethodKind::Instance
        };
        let name = asm.alloc_string(v1.call_site().name());
        let blocks = v1
            .blocks()
            .iter()
            .map(|block| crate::v2::BasicBlock::from_v1(block, asm))
            .collect();
        let locals = v1
            .locals()
            .iter()
            .map(|(name, tpe)| {
                (
                    name.as_ref().map(|name| asm.alloc_string(name.clone())),
                    asm.alloc_type(*tpe),
                )
            })
            .collect();
        let implementation = MethodImpl::MethodBody { blocks, locals };
        let mut arg_names: Vec<_> = v1
            .arg_names()
            .iter()
            .map(|name| name.as_ref().map(|name| asm.alloc_string(name.clone())))
            .collect();
        let arg_debug_count = arg_names.len();
        let arg_sig_count = v1.call_site().signature().inputs().len();
        match arg_debug_count.cmp(&arg_sig_count) {
            std::cmp::Ordering::Less => {
                println!(
                    "WARNING: argument debug info count invalid(Too few). Expected {}, got {}. fn name:{}",
                    arg_sig_count,
                    arg_debug_count,
                    v1.call_site().name()
                );
                arg_names.extend((arg_debug_count..arg_sig_count).map(|_| None));
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                println!(
                "WARNING: argument debug info count invalid(Too many). Expected {}, got {}. fn name:{}",
                arg_sig_count,
                arg_debug_count,
                v1.call_site().name()
                );
                for arg in &arg_names {
                    println!("{:?}", arg.map(|arg| asm.get_string(arg)));
                }
                arg_names.truncate(arg_sig_count);
            }
        }
        assert_eq!(arg_names.len(), v1.call_site().signature().inputs().len());
        MethodDef::new(acceess, class, name, sig, kind, implementation, arg_names)
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
    pub fn blocks_mut(&mut self) -> Option<&mut Vec<BasicBlock>> {
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
