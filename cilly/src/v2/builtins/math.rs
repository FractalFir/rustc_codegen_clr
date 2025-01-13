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
pub fn sinhf(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("sinhf");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let sinh = Float::F32.math1(arg, asm, "Sinh");
        let ret = asm.alloc_root(CILRoot::Ret(sinh));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn sinh(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("sinh");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let sinh = Float::F64.math1(arg, asm, "Sinh");
        let ret = asm.alloc_root(CILRoot::Ret(sinh));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn coshf(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("coshf");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let cosh = Float::F32.math1(arg, asm, "Cosh");
        let ret = asm.alloc_root(CILRoot::Ret(cosh));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn cosh(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("cosh");
    let generator = move |_, asm: &mut Assembly| {
        let arg = asm.alloc_node(CILNode::LdArg(0));
        let cosh = Float::F64.math1(arg, asm, "Cosh");
        let ret = asm.alloc_root(CILRoot::Ret(cosh));
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
    sinhf(asm, patcher);
    sinh(asm, patcher);
    coshf(asm, patcher);
    cosh(asm, patcher);

}
pub fn bitreverse(asm: &mut Assembly, patcher: &mut MissingMethodPatcher){
    bitreverse_u32(asm, patcher);
    bitreverse_u64(asm, patcher);
    bitreverse_u128(asm, patcher);
}

fn bitreverse_u32(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("bitreverse_u32");
    let generator = move |_, asm: &mut Assembly| {
        let curr = asm.alloc_node(CILNode::LdLoc(0));
        let mut shift = 16;
        let arg0 = asm.alloc_node(CILNode::LdArg(0));
        let mut trees = vec![asm.alloc_root(CILRoot::StLoc(0, arg0))];
        let mut i = 0;
        let masks = [
            0b11111111111111110000000000000000,
            0b11111111000000001111111100000000,
            0b11110000111100001111000011110000,
            0b11001100110011001100110011001100,
            0b10101010101010101010101010101010,
        ];
        while shift > 0 {
            let mask = asm.alloc_node(Const::U32(masks[i]));
            let inv_mask = asm.alloc_node(Const::U32(!masks[i]));
            let masked = asm.alloc_node(CILNode::BinOp(curr, mask, BinOp::And));
            let inv_masked = asm.alloc_node(CILNode::BinOp(curr, inv_mask, BinOp::And));
            let shift_ammount = asm.alloc_node(Const::I32(shift));
            let masked_shifted =
                asm.alloc_node(CILNode::BinOp(masked, shift_ammount, BinOp::ShrUn));
            let inv_masked_shifted =
                asm.alloc_node(CILNode::BinOp(inv_masked, shift_ammount, BinOp::Shl));
            let curr_val = asm.alloc_node(CILNode::BinOp(
                masked_shifted,
                inv_masked_shifted,
                BinOp::Or,
            ));
            trees.push(asm.alloc_root(CILRoot::StLoc(0, curr_val)));
            i += 1;
            shift /= 2;
        }
        trees.push(asm.alloc_root(CILRoot::Ret(curr)));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(trees, 0, None)],
            locals: vec![(None, asm.alloc_type(Type::Int(Int::U32)))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn bitreverse_u64(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("bitreverse_u64");
    let generator = move |_, asm: &mut Assembly| {
        let curr = asm.alloc_node(CILNode::LdLoc(0));
        let mut shift = 32;
        let arg0 = asm.alloc_node(CILNode::LdArg(0));
        let mut trees = vec![asm.alloc_root(CILRoot::StLoc(0, arg0))];
        let mut i = 0;
        let masks = [
            0b1111111111111111111111111111111100000000000000000000000000000000,
            0b1111111111111111000000000000000011111111111111110000000000000000,
            0b1111111100000000111111110000000011111111000000001111111100000000,
            0b1111000011110000111100001111000011110000111100001111000011110000,
            0b1100110011001100110011001100110011001100110011001100110011001100,
            0b1010101010101010101010101010101010101010101010101010101010101010,
        ];
        while shift > 0 {
            let mask = asm.alloc_node(Const::U64(masks[i]));
            let inv_mask = asm.alloc_node(Const::U64(!masks[i]));
            let masked = asm.alloc_node(CILNode::BinOp(curr, mask, BinOp::And));
            let inv_masked = asm.alloc_node(CILNode::BinOp(curr, inv_mask, BinOp::And));
            let shift_ammount = asm.alloc_node(Const::I32(shift));
            let masked_shifted =
                asm.alloc_node(CILNode::BinOp(masked, shift_ammount, BinOp::ShrUn));
            let inv_masked_shifted =
                asm.alloc_node(CILNode::BinOp(inv_masked, shift_ammount, BinOp::Shl));
            let curr_val = asm.alloc_node(CILNode::BinOp(
                masked_shifted,
                inv_masked_shifted,
                BinOp::Or,
            ));
            trees.push(asm.alloc_root(CILRoot::StLoc(0, curr_val)));
            i += 1;
            shift /= 2;
        }
        trees.push(asm.alloc_root(CILRoot::Ret(curr)));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(trees, 0, None)],
            locals: vec![(None, asm.alloc_type(Type::Int(Int::U64)))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn bitreverse_u128(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("bitreverse_u128");
    let generator = move |_, asm: &mut Assembly| {
        let u128_class = ClassRef::uint_128(asm);
        let u128_class = asm[u128_class].clone();
        let u128_ctor = u128_class.ctor(&[Type::Int(Int::U64), Type::Int(Int::U64)], asm);

        let mut shift = 64;
        //let op_add = asm.alloc_string("op_Addition");
        let op_and = asm.alloc_string("op_BitwiseAnd");
        let and = u128_class.static_mref(
            &[Type::Int(Int::U128), Type::Int(Int::U128)],
            Type::Int(Int::U128),
            op_and,
            asm,
        );
        let op_or = asm.alloc_string("op_BitwiseOr");
        let or = u128_class.static_mref(
            &[Type::Int(Int::U128), Type::Int(Int::U128)],
            Type::Int(Int::U128),
            op_or,
            asm,
        );
        let op_lshift = asm.alloc_string("op_LeftShift");
        let lshift = u128_class.static_mref(
            &[Type::Int(Int::U128), Type::Int(Int::I32)],
            Type::Int(Int::U128),
            op_lshift,
            asm,
        );
        let op_rshift = asm.alloc_string("op_RightShift");
        let rshift = u128_class.static_mref(
            &[Type::Int(Int::U128), Type::Int(Int::I32)],
            Type::Int(Int::U128),
            op_rshift,
            asm,
        );
        let curr = asm.alloc_node(CILNode::LdLoc(0));
        let arg0 = asm.alloc_node(CILNode::LdArg(0));
        let mut trees = vec![asm.alloc_root(CILRoot::StLoc(0, arg0))];
        let mut i = 0;
        let masks = [
            0b11111111111111111111111111111111111111111111111111111111111111110000000000000000000000000000000000000000000000000000000000000000,
            0b11111111111111111111111111111111000000000000000000000000000000001111111111111111111111111111111100000000000000000000000000000000,
            0b11111111111111110000000000000000111111111111111100000000000000001111111111111111000000000000000011111111111111110000000000000000,
            0b11111111000000001111111100000000111111110000000011111111000000001111111100000000111111110000000011111111000000001111111100000000,
            0b11110000111100001111000011110000111100001111000011110000111100001111000011110000111100001111000011110000111100001111000011110000,
            0b11001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100,
            0b10101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010_u128,
        ];
        while shift > 0 {
            let curr_mask = masks[i];
            let mask = asm.alloc_node(Const::U128(curr_mask));
            let curr_mask = !masks[i];
            let inv_mask = asm.alloc_node(Const::U128(curr_mask));
            let masked = asm.alloc_node(CILNode::Call(Box::new((and, [curr, mask].into()))));
            let inv_masked =
                asm.alloc_node(CILNode::Call(Box::new((and, [curr, inv_mask].into()))));
            let shift_ammount = asm.alloc_node(Const::I32(shift));
            let masked_shifted = asm.alloc_node(CILNode::Call(Box::new((
                rshift,
                [masked, shift_ammount].into(),
            ))));
            let inv_masked_shifted = asm.alloc_node(CILNode::Call(Box::new((
                lshift,
                [inv_masked, shift_ammount].into(),
            ))));

            let curr_val = asm.alloc_node(CILNode::Call(Box::new((
                or,
                [masked_shifted, inv_masked_shifted].into(),
            ))));
            trees.push(asm.alloc_root(CILRoot::StLoc(0, curr_val)));
            i += 1;
            shift /= 2;
        }
        trees.push(asm.alloc_root(CILRoot::Ret(curr)));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(trees, 0, None)],
            locals: vec![(None, asm.alloc_type(Type::Int(Int::U128)))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
