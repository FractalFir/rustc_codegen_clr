use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::MethodKind,
    BasicBlock, ClassDefIdx, ClassRefIdx, FnSig, SigIdx, StringIdx, Type, TypeIdx,
};
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct MethodRef {
    class: Option<ClassRefIdx>,
    name: StringIdx,
    sig: SigIdx,
    kind: MethodKind,
    generics: Box<[Type]>,
}

impl MethodRef {
    pub fn new(
        class: Option<ClassRefIdx>,
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

    pub fn class(&self) -> Option<ClassRefIdx> {
        self.class
    }

    pub fn name(&self) -> StringIdx {
        self.name
    }

    pub fn sig(&self) -> SigIdx {
        self.sig
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
    class: ClassDefIdx,
    name: StringIdx,
    sig: SigIdx,
    kind: MethodKind,
    implementation: MethodImpl,
}

impl MethodDef {
    pub fn ref_to(&self) -> MethodRef {
        MethodRef::new(
            Some(*self.class()),
            self.name(),
            self.sig(),
            self.kind(),
            vec![].into(),
        )
    }
    pub fn new(
        class: ClassDefIdx,
        name: StringIdx,
        sig: SigIdx,
        kind: MethodKind,
        implementation: MethodImpl,
    ) -> Self {
        Self {
            class,
            name,
            sig,
            kind,
            implementation,
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
        MethodDef::new(class, name, sig, kind, implementation)
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
