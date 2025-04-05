use crate::{
    BranchCond, ClassRef, Const, Type,
    {asm::MissingMethodPatcher, Assembly, BasicBlock, BinOp, CILNode, CILRoot, Int, MethodImpl},
};

fn op_direct(
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
    lhs: Int,
    _rhs: Int,
    op: BinOp,
) {
    let name = asm.alloc_string(format!("{op}_{lhs}", op = op.name(), lhs = lhs.name()));
    let generator = move |_, asm: &mut Assembly| {
        let op = asm.biop(CILNode::LdArg(0), CILNode::LdArg(1), op);
        let op = asm.alloc_node(op);
        let ret = asm.alloc_root(CILRoot::Ret(op));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn op_indirect(
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
    lhs_type: Int,
    rhs_type: Int,
    op: BinOp,
    ret_type: Type,
) {
    let name = asm.alloc_string(format!(
        "{op}_{lhs_type}",
        op = op.name(),
        lhs_type = lhs_type.name()
    ));
    let generator = move |_, asm: &mut Assembly| {
        let lhs = asm.alloc_node(CILNode::LdArg(0));
        let rhs = asm.alloc_node(CILNode::LdArg(1));
        let class = lhs_type.class(asm);
        let class = asm[class].clone();
        let call_op = class.static_mref(
            &[Type::Int(lhs_type), Type::Int(rhs_type)],
            ret_type,
            asm.alloc_string(op.dotnet_name()),
            asm,
        );
        let call = asm.alloc_node(CILNode::call(call_op, [lhs, rhs]));
        let ret = asm.alloc_root(CILRoot::Ret(call));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn generate_int128_ops(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, direct: bool) {
    const OPS: [BinOp; 8] = [
        BinOp::Add,
        BinOp::Sub,
        BinOp::Mul,
        BinOp::Or,
        BinOp::XOr,
        BinOp::And,
        BinOp::Rem,
        BinOp::Div,
    ];
    const SHIFTS: [BinOp; 2] = [BinOp::Shl, BinOp::Shr];
    const CMPS: [BinOp; 3] = [BinOp::Lt, BinOp::Gt, BinOp::Eq];
    let ints = [Int::U128, Int::I128];
    for op in OPS {
        for int in ints {
            if direct {
                op_direct(asm, patcher, int, int, op);
            } else {
                op_indirect(asm, patcher, int, int, op, Type::Int(int));
            }
        }
    }
    for op in SHIFTS {
        for int in ints {
            if direct {
                op_direct(asm, patcher, int, Int::I32, op);
            } else {
                op_indirect(asm, patcher, int, Int::I32, op, Type::Int(int));
            }
        }
    }
    for op in CMPS {
        for int in ints {
            if direct {
                op_direct(asm, patcher, int, int, op);
            } else {
                op_indirect(asm, patcher, int, int, op, Type::Bool);
            }
        }
    }
}
pub fn i128_mul_ovf_check(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("i128_mul_ovf_check");
    let generator = move |_, asm: &mut Assembly| {
        let lhs = asm.alloc_node(CILNode::LdArg(0));
        let rhs = asm.alloc_node(CILNode::LdArg(1));
        let i128_class = ClassRef::int_128(asm);
        let get_zero = asm.alloc_string("get_Zero");
        let op_equality = asm.alloc_string("eq_i128");
        let op_mul = asm.alloc_string("mul_i128");
        let op_div = asm.alloc_string("div_i128");
        let i128_classref = asm[i128_class].clone();
        let main_module = *asm.main_module();
        let main_module = asm[main_module].clone();
        let const_zero = i128_classref.static_mref(&[], Type::Int(Int::I128), get_zero, asm);
        let const_zero = asm.alloc_node(CILNode::call(const_zero, []));
        let i128_eq = main_module.static_mref(
            &[Type::Int(Int::I128), Type::Int(Int::I128)],
            Type::Bool,
            op_equality,
            asm,
        );
        let i128_mul = main_module.static_mref(
            &[Type::Int(Int::I128), Type::Int(Int::I128)],
            Type::Int(Int::I128),
            op_mul,
            asm,
        );
        let i128_div = main_module.static_mref(
            &[Type::Int(Int::I128), Type::Int(Int::I128)],
            Type::Int(Int::I128),
            op_div,
            asm,
        );
        let rhs_zero = asm.alloc_node(CILNode::call(i128_eq, [rhs, const_zero]));
        let jmp_nz = asm.alloc_root(CILRoot::Branch(Box::new((
            0,
            1,
            Some(BranchCond::False(rhs_zero)),
        ))));
        let ret_false = asm.alloc_node(Const::Bool(false));
        let ret_false = asm.alloc_root(CILRoot::Ret(ret_false));
        let lhs_mul_rhs = asm.alloc_node(CILNode::call(i128_mul, [lhs, rhs]));
        let recomputed_rhs = asm.alloc_node(CILNode::call(i128_div, [lhs_mul_rhs, rhs]));
        let ovf = asm.alloc_node(CILNode::call(i128_eq, [recomputed_rhs, rhs]));
        let ret_ovf = asm.alloc_root(CILRoot::Ret(ovf));

        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(vec![jmp_nz, ret_false], 0, None),
                BasicBlock::new(vec![ret_ovf], 1, None),
            ],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
