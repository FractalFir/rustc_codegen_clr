use super::{bimap::HashWrapper, cilnode::MethodKind, ClassIdx, SigIdx, StringIdx, Type};
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct MethodRef {
    class: Option<ClassIdx>,
    name: StringIdx,
    sig: SigIdx,
    kind: MethodKind,
    generics: Box<[Type]>,
}

impl MethodRef {
    pub fn new(
        class: Option<ClassIdx>,
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

    pub fn class(&self) -> Option<ClassIdx> {
        self.class
    }

    pub fn name(&self) -> StringIdx {
        self.name
    }

    pub fn sig(&self) -> SigIdx {
        self.sig
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
pub struct MethodRefIdx(u64);
impl HashWrapper for MethodRefIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
