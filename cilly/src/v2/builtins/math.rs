use crate::v2::{
    asm::MissingMethodPatcher,
    cilnode::MethodKind,
    hashable::{HashableF32, HashableF64},
    Assembly, BasicBlock, BinOp, CILNode, CILRoot, ClassRef, Const, Float, Int, MethodImpl,
    MethodRef, NodeIdx, Type,
};

pub fn int_max(asm: &mut Assembly, lhs: NodeIdx, rhs: NodeIdx, int: Int) -> NodeIdx {
    let math = ClassRef::math(asm);
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
pub fn ldexpf(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("ldexpf");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let exp = asm.alloc_node(CILNode::LdArg(1));
        let exp = asm.alloc_node(CILNode::FloatCast {
            input: exp,
            target: Float::F32,
            is_signed: true,
        });
        let two = asm.alloc_node(Const::F32(HashableF32(2.0)));
        let pow = Float::F32.pow(two, exp, asm);
        let res_val = asm.alloc_node(CILNode::BinOp(arg, pow, BinOp::Mul));
        let ret = asm.alloc_root(CILRoot::Ret(res_val));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn ldexp(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("ldexp");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let exp = asm.alloc_node(CILNode::LdArg(1));
        let exp = asm.alloc_node(CILNode::FloatCast {
            input: exp,
            target: Float::F64,
            is_signed: true,
        });
        let two = asm.alloc_node(Const::F64(HashableF64(2.0)));
        let pow = Float::F64.pow(two, exp, asm);
        let res_val = asm.alloc_node(CILNode::BinOp(arg, pow, BinOp::Mul));
        let ret = asm.alloc_root(CILRoot::Ret(res_val));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
/*
pub fn bitreverse_u128(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("bitreverse_u128");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let loc0 = asm.alloc_node(CILNode::LdLoc(0));
        let ret = asm.alloc_root(CILRoot::Ret(res_val));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![(None, asm.alloc_type(Type::Int(Int::I128)))],
        }
    };
    patcher.insert(name, Box::new(generator));
} */
pub fn math(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    ldexp(asm, patcher);
    ldexpf(asm, patcher);
}
