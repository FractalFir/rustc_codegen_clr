use cilly::{
    access_modifier::AccessModifer, asm::Assembly, basic_block::BasicBlock, call,
    call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_u32, conv_u64, conv_usize,
    size_of, DotnetTypeRef, FnSig, Type,
};

crate::add_method_from_trees!(
    interlocked_add_usize,
    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
    Type::USize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BEq {
                    target: 1,
                    sub_target: 0,
                    a: size_of!(Type::USize),
                    b: size_of!(Type::U32)
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(call!(
                        CallSite::new(
                            Some(DotnetTypeRef::interlocked()),
                            "Add".into(),
                            FnSig::new(
                                &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                                Type::U64
                            ),
                            true
                        ),
                        [CILNode::LDArg(0), conv_u64!(CILNode::LDArg(1))]
                    ))
                }
                .into(),
            ],
            0,
            None
        ),
        // sizeof::<usize>() == sizeof::<u32>()
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_usize!(call!(
                    CallSite::new(
                        Some(DotnetTypeRef::interlocked()),
                        "Add".into(),
                        FnSig::new(
                            &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                            Type::U32
                        ),
                        true
                    ),
                    [CILNode::LDArg(0), conv_u32!(CILNode::LDArg(1))]
                ))
            }
            .into(),],
            1,
            None
        )
    ],
    vec![Some("addr".into()), Some("addend".into())]
);
pub fn atomics(asm: &mut Assembly) {
    interlocked_add_usize(asm);
}
