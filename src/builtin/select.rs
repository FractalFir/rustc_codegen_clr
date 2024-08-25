use crate::add_method_from_trees;
use cilly::{
    access_modifier::AccessModifer, asm::Assembly, basic_block::BasicBlock, cil_node::CILNode,
    cil_root::CILRoot, Type,
};
macro_rules! select {
    ($tpe:ident, $name:ident) => {
        add_method_from_trees!(
            $name,
            &[Type::$tpe, Type::$tpe, Type::Bool],
            Type::$tpe,
            vec![
                BasicBlock::new(
                    vec![
                        CILRoot::BTrue {
                            target: 1,
                            sub_target: 0,
                            cond: CILNode::LDArg(2),
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
            ],
            vec![
                Some("if_true".into()),
                Some("if_false".into()),
                Some("cond".into())
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
select!(I128, select_i128);
select!(ISize, select_isize);
select!(I64, select_i64);
select!(I32, select_i32);
select!(I16, select_i16);
select!(I8, select_i8);
pub fn selects(asm: &mut Assembly) {
    select_u128(asm);
    select_usize(asm);
    select_u64(asm);
    select_u32(asm);
    select_u16(asm);
    select_u8(asm);
    select_i128(asm);
    select_isize(asm);
    select_i64(asm);
    select_i32(asm);
    select_i16(asm);
    select_i8(asm);
}
