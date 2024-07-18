#![allow(clippy::cast_precision_loss)]

use crate::add_method_from_trees;
use cilly::{
    access_modifier::AccessModifer, asm::Assembly, basic_block::BasicBlock, call,
    call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_f32, conv_f64, conv_f_un,
    conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8,
    conv_usize, eq, fn_sig::FnSig, gt, ldc_i32, ldc_i64, ldc_u32, ldc_u64, lt, or, DotnetTypeRef,
    Type,
};
use ordered_float::OrderedFloat;
add_method_from_trees!(
    cast_i32_to_u64,
    &[Type::I32],
    Type::U64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: lt!(CILNode::LDArg(0), CILNode::LdcI32(0))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(CILNode::LDArg(0))
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u64!(or!(
                    conv_i64!(CILNode::LDArg(0)),
                    conv_i64!(ldc_i32!(i32::MIN))
                ))
            }
            .into()],
            1,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_u8,
    &[Type::F32],
    Type::U8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU8(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(u8::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU8(u8::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(u8::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU8(u8::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u8!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_u16,
    &[Type::F32],
    Type::U16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU16(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(u16::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU16(u16::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(u16::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU16(u16::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u16!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_u32,
    &[Type::F32],
    Type::U32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_u32!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(u32::MAX as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(u32::MIN as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u32!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_u64,
    &[Type::F32],
    Type::U64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_u64!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(u64::MAX as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MAX))
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(u64::MIN as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MIN))
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u64!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_usize,
    &[Type::F32],
    Type::USize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(ldc_u32!(0))
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        conv_f32!(conv_f_un!(call!(
                            CallSite::new(
                                Some(DotnetTypeRef::usize_type()),
                                "get_MaxValue".into(),
                                FnSig::new(&[], Type::USize),
                                true
                            ),
                            []
                        )))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: call!(
                        CallSite::new(
                            Some(DotnetTypeRef::usize_type()),
                            "get_MaxValue".into(),
                            FnSig::new(&[], Type::USize),
                            true
                        ),
                        []
                    )
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(CILNode::LDArg(0), CILNode::LdcF32(OrderedFloat(0 as f32)))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(ldc_u64!(0))
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_usize!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_usize,
    &[Type::F64],
    Type::USize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(ldc_u64!(0))
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        conv_f64!(conv_f_un!(call!(
                            CallSite::new(
                                Some(DotnetTypeRef::usize_type()),
                                "get_MaxValue".into(),
                                FnSig::new(&[], Type::USize),
                                true
                            ),
                            []
                        )))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: call!(
                        CallSite::new(
                            Some(DotnetTypeRef::usize_type()),
                            "get_MaxValue".into(),
                            FnSig::new(&[], Type::USize),
                            true
                        ),
                        []
                    )
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(CILNode::LDArg(0), CILNode::LdcF64(OrderedFloat(0.0)))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_usize!(ldc_u64!(0))
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_usize!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_isize,
    &[Type::F64],
    Type::ISize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_isize!(ldc_i32!(0))
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        conv_f64!(call!(
                            CallSite::new(
                                Some(DotnetTypeRef::isize_type()),
                                "get_MaxValue".into(),
                                FnSig::new(&[], Type::ISize),
                                true
                            ),
                            []
                        ))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: call!(
                        CallSite::new(
                            Some(DotnetTypeRef::isize_type()),
                            "get_MaxValue".into(),
                            FnSig::new(&[], Type::ISize),
                            true
                        ),
                        []
                    )
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        conv_f64!(call!(
                            CallSite::new(
                                Some(DotnetTypeRef::isize_type()),
                                "get_MinValue".into(),
                                FnSig::new(&[], Type::ISize),
                                true
                            ),
                            []
                        ))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: call!(
                        CallSite::new(
                            Some(DotnetTypeRef::isize_type()),
                            "get_MinValue".into(),
                            FnSig::new(&[], Type::ISize),
                            true
                        ),
                        []
                    )
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_isize!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_isize,
    &[Type::F32],
    Type::ISize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_isize!(ldc_i32!(0))
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        conv_f32!(call!(
                            CallSite::new(
                                Some(DotnetTypeRef::isize_type()),
                                "get_MaxValue".into(),
                                FnSig::new(&[], Type::ISize),
                                true
                            ),
                            []
                        ))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: call!(
                        CallSite::new(
                            Some(DotnetTypeRef::isize_type()),
                            "get_MaxValue".into(),
                            FnSig::new(&[], Type::ISize),
                            true
                        ),
                        []
                    )
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        conv_f32!(call!(
                            CallSite::new(
                                Some(DotnetTypeRef::isize_type()),
                                "get_MinValue".into(),
                                FnSig::new(&[], Type::ISize),
                                true
                            ),
                            []
                        ))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: call!(
                        CallSite::new(
                            Some(DotnetTypeRef::isize_type()),
                            "get_MinValue".into(),
                            FnSig::new(&[], Type::ISize),
                            true
                        ),
                        []
                    )
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_isize!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_i8,
    &[Type::F32],
    Type::I8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI8(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(i8::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI8(i8::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(i8::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI8(i8::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i8!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_i16,
    &[Type::F32],
    Type::I16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI16(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(i16::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI16(i16::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(f32::from(i16::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI16(i16::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i16!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_i32,
    &[Type::F32],
    Type::I32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_i32!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(i32::MAX as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(i32::MIN as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i32!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f32_i64,
    &[Type::F32],
    Type::I64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::single(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F32], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_i64!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(i64::MAX as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF32(OrderedFloat(i64::MIN as f32))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i64!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_u8,
    &[Type::F64],
    Type::U8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU8(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(u8::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU8(u8::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(u8::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU8(u8::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u8!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_u16,
    &[Type::F64],
    Type::U16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU16(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(u16::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU16(u16::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(u16::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcU16(u16::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u16!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_u32,
    &[Type::F64],
    Type::U32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_u32!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(u32::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(u32::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u32!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_u64,
    &[Type::F64],
    Type::U64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_u64!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(u64::MAX as f64))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MAX))
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(u64::MIN as f64))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MIN))
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u64!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_i8,
    &[Type::F64],
    Type::I8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI8(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(i8::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI8(i8::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(i8::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI8(i8::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i8!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_i16,
    &[Type::F64],
    Type::I16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI16(0)
                }
                .into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(i16::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI16(i16::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(i16::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LdcI16(i16::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i16!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_i32,
    &[Type::F64],
    Type::I32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_i32!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(i32::MAX)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(f64::from(i32::MIN)))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i32!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
add_method_from_trees!(
    cast_f64_i64,
    &[Type::F64],
    Type::I64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: eq!(
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::double(),
                                "IsNaN".into(),
                                FnSig::new(&[Type::F64], Type::Bool),
                                true
                            ),
                            [CILNode::LDArg(0)]
                        ),
                        CILNode::LdFalse
                    )
                }
                .into(),
                CILRoot::Ret { tree: ldc_i64!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 2,
                    sub_target: 0,
                    cond: lt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(i64::MAX as f64))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MAX)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 3,
                    sub_target: 0,
                    cond: gt!(
                        CILNode::LDArg(0),
                        CILNode::LdcF64(OrderedFloat(i64::MIN as f64))
                    )
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MIN)
                }
                .into()
            ],
            2,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i64!(CILNode::LDArg(0))
            }
            .into()],
            3,
            None
        ),
    ],
    vec![Some("cast_from".into())]
);
pub fn casts(asm: &mut Assembly) {
    cast_f32_u8(asm);
    cast_f32_u16(asm);
    cast_f32_u32(asm);
    cast_f32_u64(asm);
    cast_f32_i8(asm);
    cast_f32_i16(asm);
    cast_f32_i32(asm);
    cast_f32_i64(asm);
    cast_f64_u8(asm);
    cast_f64_u16(asm);
    cast_f64_u32(asm);
    cast_f64_u64(asm);
    cast_f64_i8(asm);
    cast_f64_i16(asm);
    cast_f64_i32(asm);
    cast_f64_i64(asm);
    cast_f32_usize(asm);
    cast_f64_usize(asm);
    cast_f32_isize(asm);
    cast_f64_isize(asm);
    // Int casts
    cast_i32_to_u64(asm);
}
