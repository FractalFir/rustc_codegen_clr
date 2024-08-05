use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::MethodKind,
    Access, Assembly, BasicBlock, CILIterElem, ClassDefIdx, ClassRefIdx, FnSig, SigIdx, StringIdx,
    Type, TypeIdx,
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

    pub fn class(&self) -> ClassRefIdx {
        self.class
    }

    pub fn name(&self) -> StringIdx {
        self.name
    }

    pub fn sig(&self) -> SigIdx {
        self.sig
    }

    pub fn kind(&self) -> MethodKind {
        self.kind
    }

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
    pub fn iter_cil<'asm: 'method, 'method>(
        &'method self,
        asm: &'asm Assembly,
    ) -> Option<impl Iterator<Item = CILIterElem> + 'method> {
        match self.resolved_implementation(asm) {
            MethodImpl::MethodBody { blocks, .. } => Some(
                blocks
                    .iter()
                    .flat_map(|block| block.iter_roots())
                    .flat_map(|root| super::CILIter::new(asm.get_root(root).clone(), asm)),
            ),
            MethodImpl::Extern { .. } => None,
            MethodImpl::AliasFor(_) => {
                panic!("Unresolved alias returned by MethodDef::resolved_implementation")
            }
            MethodImpl::Missing => None,
        }
    }
    pub fn ref_to(&self) -> MethodRef {
        MethodRef::new(
            *self.class(),
            self.name(),
            self.sig(),
            self.kind(),
            vec![].into(),
        )
    }
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
            kind,
            implementation,
            arg_names,
        }
    }

    pub fn class(&self) -> ClassDefIdx {
        self.class
    }

    pub fn name(&self) -> StringIdx {
        self.name
    }

    pub fn sig(&self) -> SigIdx {
        self.sig
    }

    pub fn kind(&self) -> MethodKind {
        self.kind
    }

    pub fn implementation(&self) -> &MethodImpl {
        &self.implementation
    }
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

    pub(crate) fn from_v1(
        v1: &crate::method::Method,
        asm: &mut super::Assembly,
        class: ClassDefIdx,
    ) -> Self {
        let sig = FnSig::from_v1(v1.call_site().signature(), asm);
        let sig = asm.alloc_sig(sig);
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
                let tpe = Type::from_v1(tpe, asm);
                (
                    name.as_ref().map(|name| asm.alloc_string(name.clone())),
                    asm.alloc_type(tpe),
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
                arg_names.extend((arg_debug_count..arg_sig_count).map(|_| None))
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                println!(
                "WARNING: argument debug info count invalid(Too many). Expected {}, got {}. fn name:{}",
                arg_sig_count,
                arg_debug_count,
                v1.call_site().name()
                );
                arg_names.truncate(arg_sig_count)
            }
        }
        assert_eq!(arg_names.len(), v1.call_site().signature().inputs().len());
        MethodDef::new(acceess, class, name, sig, kind, implementation, arg_names)
    }

    pub fn access(&self) -> &Access {
        &self.access
    }

    pub fn arg_names(&self) -> &[Option<StringIdx>] {
        &self.arg_names
    }

    pub(crate) fn iter_locals<'a>(
        &'a self,
        asm: &'a Assembly,
    ) -> impl Iterator<Item = &'a (Option<StringIdx>, TypeIdx)> {
        match self.resolved_implementation(asm) {
            MethodImpl::MethodBody { blocks: _, locals } => locals.iter(),
            MethodImpl::Extern { .. } => [].iter(),
            MethodImpl::AliasFor(_) => panic!(),
            MethodImpl::Missing => [].iter(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum MethodImpl {
    MethodBody {
        blocks: Vec<BasicBlock>,
        locals: Vec<(Option<StringIdx>, TypeIdx)>,
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
                        .map(|root_id| asm.get_root(root_id).clone())
                        .unwrap_or(CILRoot::VoidRet)
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
                    locals: locals.to_vec(),
                }
            }
            (MethodImpl::MethodBody { .. }, MethodImpl::Extern { .. }) => {
                panic!("Unmergable method impl.")
            }
            (MethodImpl::MethodBody { .. }, MethodImpl::AliasFor(_)) => {
                panic!("Unmergable method impl.")
            }
            (MethodImpl::MethodBody { blocks, locals }, MethodImpl::Missing) => {
                MethodImpl::MethodBody {
                    blocks: blocks.to_vec(),
                    locals: locals.to_vec(),
                }
            }
            (MethodImpl::Extern { .. }, MethodImpl::MethodBody { .. }) => {
                panic!("Unmergable method impl.")
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
                panic!("Unmergable method impl.")
            }
            (
                MethodImpl::Extern {
                    lib,
                    preserve_errno,
                },
                MethodImpl::Missing,
            ) => MethodImpl::Extern {
                lib: *lib,
                preserve_errno: *preserve_errno,
            },
            (MethodImpl::AliasFor(_), MethodImpl::MethodBody { .. })
            | (MethodImpl::AliasFor(_), MethodImpl::Extern { .. }) => {
                panic!("Unmergable method impl.")
            }
            (MethodImpl::AliasFor(a), MethodImpl::AliasFor(b)) => {
                assert_eq!(a, b);
                self.clone()
            }
            (MethodImpl::AliasFor(alias), MethodImpl::Missing) => MethodImpl::AliasFor(*alias),
            (MethodImpl::Missing, MethodImpl::MethodBody { blocks, locals }) => {
                MethodImpl::MethodBody {
                    blocks: blocks.to_vec(),
                    locals: locals.to_vec(),
                }
            }
            (
                MethodImpl::Missing,
                MethodImpl::Extern {
                    lib,
                    preserve_errno,
                },
            ) => MethodImpl::Extern {
                lib: *lib,
                preserve_errno: *preserve_errno,
            },
            (MethodImpl::Missing, MethodImpl::AliasFor(alias)) => MethodImpl::AliasFor(*alias),
            (MethodImpl::Missing, MethodImpl::Missing) => MethodImpl::Missing,
        };
        *self = tmp;
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
