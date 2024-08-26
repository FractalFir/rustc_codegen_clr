use cilly::{
    access_modifier::AccessModifer,
    asm::Assembly,
    basic_block::BasicBlock,
    call,
    call_site::CallSite,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_u32, conv_u64, conv_usize, size_of,
    static_field_desc::StaticFieldDescriptor,
    v2::{ClassRef, Int},
    FnSig, Type,
};
macro_rules! monitor_enter {
    () => {{
        CILRoot::Call {
            site: Box::new(CallSite::new(
                Some(ClassRef::monitor()),
                "Enter".into(),
                FnSig::new(
                    &[Type::ClassRef(Box::new(ClassRef::object_type()))],
                    Type::Void,
                ),
                true,
            )),
            args: [CILNode::LDStaticField(Box::new(
                StaticFieldDescriptor::new(
                    None,
                    Type::ClassRef(Box::new(ClassRef::object_type())),
                    "GlobalAtomicLock".into(),
                ),
            ))]
            .into(),
        }
        .into()
    }};
}
macro_rules! monitor_exit {
    () => {{
        CILRoot::Call {
            site: Box::new(CallSite::new(
                Some(ClassRef::monitor()),
                "Exit".into(),
                FnSig::new(
                    &[Type::ClassRef(Box::new(ClassRef::object_type()))],
                    Type::Void,
                ),
                true,
            )),
            args: [CILNode::LDStaticField(Box::new(
                StaticFieldDescriptor::new(
                    None,
                    Type::ClassRef(Box::new(ClassRef::object_type())),
                    "GlobalAtomicLock".into(),
                ),
            ))]
            .into(),
        }
        .into()
    }};
}

crate::add_method_from_trees!(
    interlocked_add_usize,
    &[
        Type::Ref(Box::new(Type::Int(Int::USize))),
        Type::Int(Int::USize)
    ],
    Type::Int(Int::USize),
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BEq {
                    target: 1,
                    sub_target: 0,
                    a: Box::new(size_of!(Type::Int(Int::USize))),
                    b: Box::new(size_of!(Type::Int(Int::U32)))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(call!(
                        CallSite::new(
                            Some(ClassRef::interlocked()),
                            "Add".into(),
                            FnSig::new(
                                &[
                                    Type::Ref(Box::new(Type::Int(Int::U64))),
                                    Type::Int(Int::U64)
                                ],
                                Type::Int(Int::U64)
                            ),
                            true
                        ),
                        [
                            CILNode::LDArg(0).cast_ptr(Type::Ref(Box::new(Type::Int(Int::U64)))),
                            conv_u64!(CILNode::LDArg(1))
                        ]
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
                        Some(ClassRef::interlocked()),
                        "Add".into(),
                        FnSig::new(
                            &[
                                Type::Ref(Box::new(Type::Int(Int::U32))),
                                Type::Int(Int::U32)
                            ],
                            Type::Int(Int::U32)
                        ),
                        true
                    ),
                    [
                        CILNode::LDArg(0).cast_ptr(Type::Ref(Box::new(Type::Int(Int::U32)))),
                        conv_u32!(CILNode::LDArg(1))
                    ]
                ))
            }
            .into(),],
            1,
            None
        )
    ],
    vec![Some("addr".into()), Some("addend".into())]
);
crate::add_method_from_trees!(
    interlocked_or_usize,
    &[
        Type::Ref(Box::new(Type::Int(Int::USize))),
        Type::Int(Int::USize)
    ],
    Type::Int(Int::USize),
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BEq {
                    target: 1,
                    sub_target: 0,
                    a: Box::new(size_of!(Type::Int(Int::USize))),
                    b: Box::new(size_of!(Type::Int(Int::U32)))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(call!(
                        CallSite::new(
                            Some(ClassRef::interlocked()),
                            "Or".into(),
                            FnSig::new(
                                &[
                                    Type::Ref(Box::new(Type::Int(Int::U64))),
                                    Type::Int(Int::U64)
                                ],
                                Type::Int(Int::U64)
                            ),
                            true
                        ),
                        [
                            CILNode::LDArg(0).cast_ptr(Type::Ref(Box::new(Type::Int(Int::U64)))),
                            conv_u64!(CILNode::LDArg(1))
                        ]
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
                        Some(ClassRef::interlocked()),
                        "Or".into(),
                        FnSig::new(
                            &[
                                Type::Ref(Box::new(Type::Int(Int::U32))),
                                Type::Int(Int::U32)
                            ],
                            Type::Int(Int::U32)
                        ),
                        true
                    ),
                    [
                        CILNode::LDArg(0).cast_ptr(Type::Ref(Box::new(Type::Int(Int::U32)))),
                        conv_u32!(CILNode::LDArg(1))
                    ]
                ))
            }
            .into(),],
            1,
            None
        )
    ],
    vec![Some("addr".into()), Some("orend".into())]
);
crate::add_method_from_trees!(
    interlocked_and_usize,
    &[
        Type::Ref(Box::new(Type::Int(Int::USize))),
        Type::Int(Int::USize)
    ],
    Type::Int(Int::USize),
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BEq {
                    target: 1,
                    sub_target: 0,
                    a: Box::new(size_of!(Type::Int(Int::USize))),
                    b: Box::new(size_of!(Type::Int(Int::U32)))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(call!(
                        CallSite::new(
                            Some(ClassRef::interlocked()),
                            "And".into(),
                            FnSig::new(
                                &[
                                    Type::Ref(Box::new(Type::Int(Int::U64))),
                                    Type::Int(Int::U64)
                                ],
                                Type::Int(Int::U64)
                            ),
                            true
                        ),
                        [
                            CILNode::LDArg(0).cast_ptr(Type::Ref(Box::new(Type::Int(Int::U64)))),
                            conv_u64!(CILNode::LDArg(1))
                        ]
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
                        Some(ClassRef::interlocked()),
                        "And".into(),
                        FnSig::new(
                            &[
                                Type::Ref(Box::new(Type::Int(Int::U32))),
                                Type::Int(Int::U32)
                            ],
                            Type::Int(Int::U32)
                        ),
                        true
                    ),
                    [
                        CILNode::LDArg(0).cast_ptr(Type::Ref(Box::new(Type::Int(Int::U32)))),
                        conv_u32!(CILNode::LDArg(1))
                    ]
                ))
            }
            .into(),],
            1,
            None
        )
    ],
    vec![Some("addr".into()), Some("andend".into())]
);
pub fn atomics(asm: &mut Assembly) {
    interlocked_add_usize(asm);
    interlocked_or_usize(asm);
    interlocked_and_usize(asm);
    interlocked_emulate_xchng_byte(asm);
}
crate::add_method_from_trees!(
    interlocked_emulate_xchng_byte,
    &[Type::Ref(Box::new(Type::Int(Int::U8))), Type::Int(Int::U8)],
    Type::Int(Int::U8),
    vec![
        BasicBlock::new(
            vec![
                monitor_enter!(),
                CILRoot::GoTo {
                    target: 1,
                    sub_target: 0
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::STLoc {
                    tree: CILNode::LDIndU8 {
                        ptr: Box::new(CILNode::LDArg(0))
                    },
                    local: 0,
                }
                .into(),
                CILRoot::STIndI8(CILNode::LDArg(0), CILNode::LDArg(1)).into(),
                CILRoot::GoTo {
                    target: 1,
                    sub_target: 2
                }
                .into(),
                CILRoot::JumpingPad {
                    source: 1,
                    target: 2
                }
                .into(),
            ],
            1,
            Some(cilly::basic_block::Handler::Blocks(vec![BasicBlock::new(
                vec![monitor_exit!(), CILRoot::ReThrow.into()],
                3,
                None
            )]))
        ),
        BasicBlock::new(
            vec![
                monitor_exit!(),
                CILRoot::Ret {
                    tree: CILNode::LDLoc(0)
                }
                .into()
            ],
            2,
            None
        ),
    ],
    vec![(Some("val".into()), Type::Int(Int::U8))],
    vec![Some("addr".into()), Some("new_val".into())]
);
#[test]
fn test_interlocked_emu() {
    let mut asm = Assembly::empty();
    interlocked_emulate_xchng_byte(&mut asm);
}
