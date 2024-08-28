use crate::v2::{
    asm::MissingMethodPatcher, cilnode::ExtendKind, Assembly, BasicBlock, CILNode, CILRoot, Float,
    Int, MethodImpl,
};
fn float_to_int(asm: &mut Assembly, int: Int, float: Float, patcher: &mut MissingMethodPatcher) {
    let name = format!("cast_{}_{}", float.name(), int.name());
    let name = asm.alloc_string(name);
    let generator = move |_, asm: &mut Assembly| {
        // Consts
        let imax = int.max(asm);
        let imax = asm.alloc_node(imax);
        let fmax = asm.alloc_node(CILNode::FloatCast {
            input: imax,
            target: float,
            is_signed: int.is_signed(),
        });
        let imin = int.min(asm);
        let imin = asm.alloc_node(imin);
        let fmin = asm.alloc_node(CILNode::FloatCast {
            input: imin,
            target: float,
            is_signed: int.is_signed(),
        });
        // Args
        let ld_arg_0 = asm.alloc_node(CILNode::LdArg(0));

        // If arg is smaller than max, pass. Else jump to block 1.
        let overflow = asm.alloc_root(CILRoot::Branch(Box::new((
            1,
            0,
            Some(crate::v2::cilroot::BranchCond::Gt(
                ld_arg_0,
                fmax,
                crate::v2::cilroot::CmpKind::Ordered,
            )),
        ))));
        // If arg is bigger than min, pass. Else jump to block 2.
        let underflow = asm.alloc_root(CILRoot::Branch(Box::new((
            2,
            0,
            Some(crate::v2::cilroot::BranchCond::Lt(
                ld_arg_0,
                fmin,
                crate::v2::cilroot::CmpKind::Ordered,
            )),
        ))));
        // Return the cast if in range.
        let cast = asm.alloc_node(CILNode::IntCast {
            input: ld_arg_0,
            target: int,
            extend: if int.is_signed() {
                ExtendKind::SignExtend
            } else {
                ExtendKind::ZeroExtend
            },
        });
        let return_cast = asm.alloc_root(CILRoot::Ret(cast));
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(vec![overflow, underflow, return_cast], 0, None),
                BasicBlock::new(vec![asm.alloc_root(CILRoot::Ret(imax))], 1, None),
                BasicBlock::new(vec![asm.alloc_root(CILRoot::Ret(imin))], 2, None),
            ],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn insert_casts(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let floats = [Float::F32, Float::F64];
    let ints = [
        Int::U8,
        Int::I8,
        Int::U16,
        Int::I16,
        Int::U32,
        Int::I32,
        Int::U64,
        Int::I64,
        Int::ISize,
        Int::USize,
    ];
    for int in ints {
        for float in floats {
            float_to_int(asm, int, float, patcher);
        }
    }
}
