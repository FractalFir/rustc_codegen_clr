use crate::{
    bimap::{BiMap, HashWrapper},
    IString,
};
#[derive(Default)]
struct Assembly {
    strings: BiMap<StringIdx, IString>,
    types: BiMap<TypeIdx, Type>,
    class_refs: BiMap<ClassIdx, Class>,
    nodes: BiMap<NodeIdx, CILNode>,
}
impl Assembly {
    pub fn nptr(&mut self, inner: Type) -> Type {
        Type::Ptr(self.types.alloc(inner))
    }
    pub fn nref(&mut self, inner: Type) -> Type {
        Type::Ref(self.types.alloc(inner))
    }
    pub fn type_from_id(&self, idx: TypeIdx) -> &Type {
        self.types.get(idx)
    }
    pub fn add(&mut self, lhs: CILNode, rhs: CILNode) -> CILNode {
        let lhs = self.nodes.alloc(lhs);
        let rhs = self.nodes.alloc(rhs);
        CILNode::Add(lhs, rhs)
    }
    pub fn ldstr(&mut self, msg: impl Into<IString>) -> CILNode {
        CILNode::LdStr(self.strings.alloc(msg.into()))
    }
    pub fn strct(&mut self, name: IString) -> ClassIdx {
        let class = Class {
            name: self.strings.alloc(name),
            asm: None,
            is_valuetype: true,
            generics: vec![],
        };
        self.class_refs.alloc(class)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
struct TypeIdx(u64);
impl HashWrapper for TypeIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
struct StringIdx(u64);
impl HashWrapper for StringIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
struct ClassIdx(u64);
impl HashWrapper for ClassIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Type {
    Ptr(TypeIdx),
    Ref(TypeIdx),
    Int(Int),
    ClassRef(ClassIdx),
    Float(Float),
    PlatformChar,
    PlarformGeneric,
    Bool,
    PlatformArray { elem: TypeIdx, dim: usize },
}
impl Type {
    pub fn deref<'a, 'b: 'a>(&'a self, asm: &'b Assembly) -> &Self {
        match self {
            Type::Ptr(inner) | Type::Ref(inner) => (asm.type_from_id(inner.clone())),
            _ => panic!(),
        }
    }
}
impl From<ClassIdx> for Type {
    fn from(val: ClassIdx) -> Self {
        Type::ClassRef(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Int {
    U8,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Float {
    U8,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Class {
    name: StringIdx,
    asm: Option<ClassIdx>,
    is_valuetype: bool,
    generics: Vec<Type>,
}
#[test]
fn types() {
    let mut asm = Assembly::default();
    let tpe = asm.nptr(Type::Int(Int::U8));
    let tpe = asm.nref(tpe);
    assert_eq!(*tpe.deref(&asm).deref(&asm), Type::Int(Int::U8));
}
#[test]
pub fn nodes() {
    let mut asm = Assembly::default();
    let add = asm.add(CILNode::LdcI32(2), CILNode::LdcI32(1));
    let msg = asm.ldstr("Hi!");
}
#[test]
fn no_collision() {
    let mut asm = Assembly::default();
    let mut curr = CILNode::LdcI32(1);
    for _ in 0..100_000 {
        curr = std::hint::black_box(asm.add(curr.clone(), curr));
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum CILNode {
    LdcI32(i32),
    Add(NodeIdx, NodeIdx),
    LdStr(StringIdx),
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
struct NodeIdx(u64);
impl HashWrapper for NodeIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
