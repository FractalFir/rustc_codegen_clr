use crate::v2::{cilnode::MethodKind, Assembly, CILNode, ClassRef, Int, MethodRef, NodeIdx, Type};

pub fn int_max(asm: &mut Assembly, lhs: NodeIdx, rhs: NodeIdx, int: Int) -> NodeIdx {
    let math = ClassRef::math(asm);
    let math = asm.alloc_class_ref(math);
    let max = asm.alloc_string("Max");
    let sig = asm.sig([Type::Int(int), Type::Int(int)], Type::Int(int));
    let mref = asm.alloc_methodref(MethodRef::new(
        math,
        max,
        sig,
        MethodKind::Static,
        vec![].into(),
    ));
    asm.alloc_node(CILNode::Call(Box::new((mref, Box::new([lhs, rhs])))))
}

pub fn int_min(asm: &mut Assembly, lhs: NodeIdx, rhs: NodeIdx, int: Int) -> NodeIdx {
    let math = ClassRef::math(asm);
    let math = asm.alloc_class_ref(math);
    let max = asm.alloc_string("Min");
    let sig = asm.sig([Type::Int(int), Type::Int(int)], Type::Int(int));
    let mref = asm.alloc_methodref(MethodRef::new(
        math,
        max,
        sig,
        MethodKind::Static,
        vec![].into(),
    ));
    asm.alloc_node(CILNode::Call(Box::new((mref, Box::new([lhs, rhs])))))
}
