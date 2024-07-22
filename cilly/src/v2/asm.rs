use super::{
    bimap::BiMap,
    cilnode::{BinOp, UnOp},
    CILNode, CILRoot, ClassIdx, ClassRef, Const, FieldDesc, FieldIdx, FnSig, MethodRef,
    MethodRefIdx, NodeIdx, RootIdx, SigIdx, StaticFieldDesc, StaticFieldIdx, StringIdx, Type,
    TypeIdx,
};
use crate::IString;
#[derive(Default)]
pub struct Assembly {
    strings: BiMap<StringIdx, IString>,
    types: BiMap<TypeIdx, Type>,
    class_refs: BiMap<ClassIdx, ClassRef>,
    nodes: BiMap<NodeIdx, CILNode>,
    roots: BiMap<RootIdx, CILRoot>,
    sigs: BiMap<SigIdx, FnSig>,
    method_refs: BiMap<MethodRefIdx, MethodRef>,
    fields: BiMap<FieldIdx, FieldDesc>,
    statics: BiMap<StaticFieldIdx, StaticFieldDesc>,
}
impl Assembly {
    pub fn alloc_string(&mut self, string: impl Into<IString>) -> StringIdx {
        self.strings.alloc(string.into())
    }
    pub fn sig(&mut self, input: impl Into<Box<[Type]>>, output: impl Into<Type>) -> SigIdx {
        self.sigs.alloc(FnSig::new(input.into(), output.into()))
    }
    pub fn nptr(&mut self, inner: Type) -> Type {
        Type::Ptr(self.types.alloc(inner))
    }
    pub fn nref(&mut self, inner: Type) -> Type {
        Type::Ref(self.types.alloc(inner))
    }
    pub fn type_from_id(&self, idx: TypeIdx) -> &Type {
        self.types.get(idx)
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
        CILNode::Const(Const::PlatformString(self.strings.alloc(msg.into())))
    }
    pub fn strct(&mut self, name: IString) -> ClassIdx {
        let class = ClassRef::new(self.strings.alloc(name), None, true, vec![].into());
        self.class_refs.alloc(class)
    }

    pub(crate) fn node_idx(&mut self, node: CILNode) -> NodeIdx {
        self.nodes.alloc(node)
    }

    pub(crate) fn class_idx(&mut self, cref: ClassRef) -> ClassIdx {
        self.class_refs.alloc(cref)
    }

    pub(crate) fn sig_idx(&mut self, sig: FnSig) -> SigIdx {
        self.sigs.alloc(sig)
    }

    pub(crate) fn methodref_idx(&mut self, method_ref: MethodRef) -> MethodRefIdx {
        self.method_refs.alloc(method_ref)
    }

    pub(crate) fn alloc_root(&mut self, val: CILRoot) -> RootIdx {
        self.roots.alloc(val)
    }

    pub(crate) fn type_idx(&mut self, tpe: Type) -> TypeIdx {
        self.types.alloc(tpe)
    }

    pub(crate) fn get_node(&self, key: NodeIdx) -> &CILNode {
        self.nodes.get(key)
    }

    pub(crate) fn field_idx(&mut self, field: FieldDesc) -> FieldIdx {
        self.fields.alloc(field)
    }

    pub(crate) fn sfld_idx(&mut self, sfld: StaticFieldDesc) -> StaticFieldIdx {
        self.statics.alloc(sfld)
    }
}
