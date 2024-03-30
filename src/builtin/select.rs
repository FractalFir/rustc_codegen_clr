use crate::{
    add_method_from_trees,
    assembly::Assembly,
    basic_block::BasicBlock,
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    r#type::Type,
};

macro_rules! select {
    ($tpe:ident, $name:ident) => {
        add_method_from_trees!(
            $name,
            &[Type::$tpe, Type::$tpe, Type::Bool],
            &Type::$tpe,
            vec![
                BasicBlock::new(
                    vec![
                        CILRoot::BTrue {
                            target: 1,
                            sub_target: 0,
                            ops: CILNode::LDArg(2),
                        }
                        .into(),
                        CILRoot::Ret {
                            tree: CILNode::LDArg(1)
                        }
                        .into()
                    ],
                    0,
                    None
                ),
                BasicBlock::new(
                    vec![CILRoot::Ret {
                        tree: CILNode::LDArg(0)
                    }
                    .into()],
                    1,
                    None
                ),
            ]
        );
    };
}
select!(U128, select_u128);
select!(USize, select_usize);
select!(U64, select_u64);
select!(U32, select_u32);
select!(U16, select_u16);
select!(U8, select_u8);
pub fn selects(asm: &mut Assembly) {
    select_u128(asm);
    select_usize(asm);
    select_u64(asm);
    select_u32(asm);
    select_u16(asm);
    select_u8(asm);
}
