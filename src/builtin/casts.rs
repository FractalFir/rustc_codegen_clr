use crate::{
    add_method_from_trees,
    assembly::Assembly,
    basic_block::BasicBlock,
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    conv_i16, conv_i32, conv_i64, conv_i8, conv_u16, conv_u32, conv_u64, conv_u8, gt, ldc_i32,
    ldc_i64, ldc_u32, ldc_u64, lt, or,
    r#type::Type,
};
add_method_from_trees!(
    select_usize,
    &[Type::USize, Type::USize, Type::Bool],
    &Type::USize,
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
add_method_from_trees!(
    cast_i32_to_u64,
    &[Type::I32],
    &Type::U64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcI32(0))
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
                tree: or!(conv_u64!(CILNode::LDArg(0)), conv_i64!(ldc_i32!(i32::MIN)))
            }
            .into()],
            1,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_u8,
    &[Type::F32],
    &Type::U8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(u8::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u8::MAX as u32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(u8::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u8::MIN as u32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u8!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_u16,
    &[Type::F32],
    &Type::U16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(u16::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u16::MAX as u32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(u16::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u16::MIN as u32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u16!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_u32,
    &[Type::F32],
    &Type::U32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(u32::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MAX)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(u32::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MIN)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u32!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_u64,
    &[Type::F32],
    &Type::U64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(u64::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MAX))
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(u64::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MIN))
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u64!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_i8,
    &[Type::F32],
    &Type::I8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(i8::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i8::MAX as i32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(i8::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i8::MIN as i32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i8!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_i16,
    &[Type::F32],
    &Type::I16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(i16::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i16::MAX as i32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(i16::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i16::MIN as i32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i16!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_i32,
    &[Type::F32],
    &Type::I32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(i32::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MAX)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(i32::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MIN)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i32!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f32_i64,
    &[Type::F32],
    &Type::I64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF32(i64::MAX as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MAX)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF32(i64::MIN as f32))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MIN)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i64!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_u8,
    &[Type::F64],
    &Type::U8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(u8::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u8::MAX as u32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(u8::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u8::MIN as u32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u8!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_u16,
    &[Type::F64],
    &Type::U16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(u16::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u16::MAX as u32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(u16::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u16::MIN as u32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u16!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_u32,
    &[Type::F64],
    &Type::U32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(u32::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MAX)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(u32::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_u32!(u32::MIN)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u32!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_u64,
    &[Type::F64],
    &Type::U64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(u64::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MAX))
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(u64::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: conv_u64!(ldc_u64!(u64::MIN))
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_u64!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_i8,
    &[Type::F64],
    &Type::I8,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(i8::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i8::MAX as i32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(i8::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i8::MIN as i32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i8!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_i16,
    &[Type::F64],
    &Type::I16,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(i16::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i16::MAX as i32)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(i16::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i16::MIN as i32)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i16!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_i32,
    &[Type::F64],
    &Type::I32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(i32::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MAX)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(i32::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i32!(i32::MIN)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i32!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
);
add_method_from_trees!(
    cast_f64_i64,
    &[Type::F64],
    &Type::I64,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt!(CILNode::LDArg(0), CILNode::LdcF64(i64::MAX as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MAX)
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
                    ops: gt!(CILNode::LDArg(0), CILNode::LdcF64(i64::MIN as f64))
                }
                .into(),
                CILRoot::Ret {
                    tree: ldc_i64!(i64::MIN)
                }
                .into()
            ],
            1,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: conv_i64!(CILNode::LDArg(0))
            }
            .into()],
            2,
            None
        ),
    ]
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
    // Int casts
    cast_i32_to_u64(asm);
    // Select
    select_usize(asm);
}
