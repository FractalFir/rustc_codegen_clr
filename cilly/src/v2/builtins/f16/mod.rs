use crate::{
    v2::{
        asm::MissingMethodPatcher, Assembly, BasicBlock, BinOp, CILNode, CILRoot, Float, MethodImpl,
    },
    Type,
};
fn op_direct(
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
    lhs: Float,
    _rhs: Float,
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
    lhs_type: Float,
    rhs_type: Float,
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
            &[Type::Float(lhs_type), Type::Float(rhs_type)],
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
pub fn generate_f16_ops(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, direct: bool) {
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
    const CMPS: [BinOp; 3] = [BinOp::Lt, BinOp::Gt, BinOp::Eq];
    let ints = [Float::F16];
    for op in OPS {
        for float in ints {
            if direct {
                op_direct(asm, patcher, float, float, op);
            } else {
                op_indirect(asm, patcher, float, float, op, Type::Float(float));
            }
        }
    }

    for op in CMPS {
        for float in ints {
            if direct {
                op_direct(asm, patcher, float, float, op);
            } else {
                op_indirect(asm, patcher, float, float, op, Type::Bool);
            }
        }
    }
}
