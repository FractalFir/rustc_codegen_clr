use crate::{
    v2::{
        asm::MissingMethodPatcher, Assembly, BasicBlock, BinOp, CILNode, CILRoot, Int, MethodImpl,
    },
    Type,
};

pub fn op_direct(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, int: Int, op: BinOp) {
    let name = asm.alloc_string(format!("{op}_{int}", op = op.name(), int = int.name()));
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
pub fn op_indirect(
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
    int: Int,
    op: BinOp,
    ret_type: Type,
) {
    let name = asm.alloc_string(format!("{op}_{int}", op = op.name(), int = int.name()));
    let generator = move |_, asm: &mut Assembly| {
        let lhs = asm.alloc_node(CILNode::LdArg(0));
        let rhs = asm.alloc_node(CILNode::LdArg(1));
        let class = int.class(asm);
        let class = asm[class].clone();
        let call_op = class.static_mref(
            &[Type::Int(int), Type::Int(int)],
            ret_type,
            asm.alloc_string(op.dotnet_name()),
            asm,
        );
        let call = asm.alloc_node(CILNode::Call(Box::new((call_op, [lhs, rhs].into()))));
        let ret = asm.alloc_root(CILRoot::Ret(call));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn generate_int128_ops(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, direct: bool) {
    const OPS: [BinOp; 10] = [
        BinOp::Add,
        BinOp::Sub,
        BinOp::Mul,
        BinOp::Or,
        BinOp::XOr,
        BinOp::And,
        BinOp::Rem,
        BinOp::Shl,
        BinOp::Shr,
        BinOp::Div,
    ];
    const CMPS: [BinOp; 3] = [BinOp::Lt, BinOp::Gt, BinOp::Eq];
    let ints = [Int::U128, Int::I128];
    for op in OPS {
        for int in ints {
            if direct {
                op_direct(asm, patcher, int, op);
            } else {
                op_indirect(asm, patcher, int, op, Type::Int(int));
            }
        }
    }
    for op in CMPS {
        for int in ints {
            if direct {
                op_direct(asm, patcher, int, op);
            } else {
                op_indirect(asm, patcher, int, op, Type::Bool);
            }
        }
    }
}
