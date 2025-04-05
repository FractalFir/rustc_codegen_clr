use crate::{
    asm::MissingMethodPatcher, cilroot::BranchCond, Assembly, BasicBlock, CILNode, CILRoot, Int,
    MethodImpl,
};

fn generate_select(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, int: Int) {
    let name = format!("select_{}", int.name());
    let name = asm.alloc_string(name);
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldarg_2 = asm.alloc_node(CILNode::LdArg(2));
        let arg2_true = asm.alloc_root(CILRoot::Branch(Box::new((
            1,
            0,
            Some(BranchCond::True(ldarg_2)),
        ))));
        let ret_0 = asm.alloc_root(CILRoot::Ret(ldarg_0));
        let ret_1 = asm.alloc_root(CILRoot::Ret(ldarg_1));
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(vec![arg2_true, ret_1], 0, None),
                BasicBlock::new(vec![ret_0], 1, None),
            ],
            locals: vec![],
        }
    };

    patcher.insert(name, Box::new(generator));
}
pub fn generate_int_selects(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let ints = [
        Int::U8,
        Int::I8,
        Int::U16,
        Int::I16,
        Int::U32,
        Int::I32,
        Int::U64,
        Int::I64,
        Int::USize,
        Int::ISize,
        Int::I128,
        Int::U128,
    ];
    for int in ints {
        generate_select(asm, patcher, int);
    }
}
