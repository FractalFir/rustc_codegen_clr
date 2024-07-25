use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::MethodKind,
    Access, Assembly, BasicBlock, ClassDefIdx, ClassRefIdx, FnSig, SigIdx, StringIdx, Type,
    TypeIdx,
};
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct MethodRef {
    class: ClassRefIdx,
    name: StringIdx,
    sig: SigIdx,
    kind: MethodKind,
    generics: Box<[Type]>,
}

impl MethodRef {
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
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MethodRefIdx(BiMapIndex);
impl IntoBiMapIndex for MethodRefIdx {
    fn from_hash(val: BiMapIndex) -> Self {
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
        let sig = asm.sig_idx(sig);
        let acceess = match v1.access() {
            crate::access_modifier::AccessModifer::Private => Access::Private,
            crate::access_modifier::AccessModifer::Public => Access::Public,
            crate::access_modifier::AccessModifer::ModulePublic => Access::Extern,
        };
        let name = asm.alloc_string(v1.call_site().name());
        let kind = if v1.call_site().is_static() {
            MethodKind::Static
        } else {
            MethodKind::Instance
        };
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
                    asm.type_idx(tpe),
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
                eprintln!(
                    "WARNING: argument debug info count invalid(Too few). Expected {}, got {}. fn name:{}",
                    arg_debug_count,
                    arg_sig_count,
                    v1.call_site().name()
                );
                arg_names.extend((arg_sig_count..arg_debug_count).map(|_| None))
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                eprintln!(
                "WARNING: argument debug info count invalid(Too many). Expected {}, got {}. fn name:{}",
                arg_debug_count,
                arg_sig_count,
                v1.call_site().name()
                );
                arg_names.truncate(arg_sig_count)
            }
        }
        MethodDef::new(acceess, class, name, sig, kind, implementation, arg_names)
    }

    pub fn access(&self) -> &Access {
        &self.access
    }

    pub fn arg_names(&self) -> &[Option<StringIdx>] {
        &self.arg_names
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
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MethodDefIdx(pub MethodRefIdx);

impl std::ops::Deref for MethodDefIdx {
    type Target = MethodRefIdx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
