use cilly::cil_node::CILNode;
use cilly::v2::cilnode::MethodKind;
use cilly::v2::{Assembly, ClassRef, Float, Int, MethodRef};
use cilly::Type;

use cilly::{
    call, conv_f32, conv_f64, conv_f_un, conv_i16, conv_i32, conv_i64, conv_i8, conv_isize,
    conv_u16, conv_u32, conv_u64, conv_u8, conv_usize,
};
/// Casts from intiger type `src` to target `target`
pub fn int_to_int(src: Type, target: Type, operand: CILNode, asm: &mut Assembly) -> CILNode {
    if src == target {
        return operand;
    }
    match (&src, &target) {
        // Unsinged casts are special
        (
            Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize),
            Type::Int(Int::ISize),
        ) => {
            conv_isize!(conv_usize!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I64)) => {
            conv_i64!(conv_u64!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I32)) => {
            conv_i32!(conv_u32!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I16)) => {
            conv_i16!(conv_u16!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I8)) => {
            conv_i8!(conv_u8!(operand))
        }
        //
        (Type::Int(Int::ISize), Type::Int(Int::I128)) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([Type::Int(Int::ISize)], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (Type::Int(Int::U32), Type::Int(Int::I128)) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([Type::Int(Int::U32)], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (Type::Int(Int::ISize), Type::Int(Int::U128)) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([Type::Int(Int::I64)], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [conv_i64!(operand)])
        }
        (Type::Bool, Type::Int(Int::U128)) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([Type::Int(Int::I32)], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [conv_i32!(operand)])
        }
        (Type::Bool, Type::Int(Int::I128)) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([Type::Int(Int::I32)], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [conv_i32!(operand)])
        }
        // Fixes sign casts
        (Type::Int(Int::I64 | Int::I32 | Int::I16 | Int::I8), Type::Int(Int::USize)) => {
            CILNode::SignExtendToUSize(operand.into())
        }
        (Type::Int(Int::I64 | Int::I32 | Int::I16 | Int::I8), Type::Int(Int::U64)) => {
            CILNode::SignExtendToU64(operand.into())
        }
        // i128 bit casts
        (Type::Int(Int::U128), Type::Int(Int::I128))
        | (Type::Int(Int::I8 | Int::I16 | Int::I32 | Int::I64), Type::Int(Int::U128)) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (_, Type::Int(Int::I128)) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (Type::Int(Int::I128), Type::Int(Int::U128)) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (_, Type::Int(Int::U128)) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (Type::Int(Int::I128), _) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        (Type::Int(Int::U128), _) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        //todo!("Casting to 128 bit intiegers is not supported!"),
        _ => to_int(target, operand),
    }
}
/// Returns CIL ops required to convert type src to target
pub fn float_to_int(src: Type, target: Type, operand: CILNode, asm: &mut Assembly) -> CILNode {
    match target {
        Type::Int(Int::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        Type::Int(Int::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Explicit"),
                asm.sig([src], target),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand])
        }
        Type::Int(Int::U8) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_u8"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::U8)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_u8"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::U8)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::U16) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_u16"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::U16)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_u16"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::U16)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::U32) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_u32"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::U32)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_u32"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::U32)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::U64) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_u64"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::U64)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_u64"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::U64)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::USize) => match src {
            //TODO: Check why this caused
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_usize"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::USize)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }

            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_usize"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::USize)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            //_=> to_int(target, operand),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::ISize) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_isize"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::ISize)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_isize"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::ISize)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I8) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_i8"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::I8)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_i8"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::I8)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I16) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_i16"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::I16)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_i16"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::I16)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I32) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_i32"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::I32)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_i32"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::I32)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I64) => match src {
            Type::Float(Float::F32) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f32_i64"),
                    asm.sig([Type::Float(Float::F32)], Type::Int(Int::I64)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            Type::Float(Float::F64) => {
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string("cast_f64_i64"),
                    asm.sig([Type::Float(Float::F64)], Type::Int(Int::I64)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand])
            }
            _ => panic!("Non-float type!"),
        },
        _ => to_int(target, operand),
    }

    //call uint64 [System.Runtime]System.Int128::op_Explicit(valuetype [System.Runtime]System.Int128)
    //
}
/// Returns CIL ops required to convert to intiger of type `target`
fn to_int(target: Type, operand: CILNode) -> CILNode {
    match target {
        Type::Int(Int::I8) => conv_i8!(operand),
        Type::Int(Int::U8) => conv_u8!(operand),
        Type::Int(Int::I16) => conv_i16!(operand),
        Type::Int(Int::U16) => conv_u16!(operand),
        Type::Int(Int::U32) => conv_u32!(operand),
        Type::Int(Int::I32) => conv_i32!(operand),
        Type::Int(Int::I64) => conv_i64!(operand),
        Type::Int(Int::U64) => conv_u64!(operand),
        Type::Int(Int::ISize) => conv_isize!(operand),
        Type::Int(Int::USize) => conv_usize!(operand),
        Type::Ptr(tpe) => conv_usize!(operand).cast_ptr(Type::Ptr(tpe)),
        _ => todo!("Can't cast to {target:?} yet!"),
    }
}
/// Returns CIL ops required to casts from intiger type `src` to `target` MOVE TO CILLY
pub fn int_to_float(src: Type, target: Type, parrent: CILNode, asm: &mut Assembly) -> CILNode {
    if matches!(src, Type::Int(Int::I128)) {
        let mref = MethodRef::new(
            ClassRef::int_128(asm).into(),
            asm.alloc_string("op_Explicit"),
            asm.sig([src], target),
            MethodKind::Static,
            vec![].into(),
        );
        call!(asm.alloc_methodref(mref), [parrent])
        //todo!("Casting from 128 bit intiegers is not supported!")
    } else if matches!(src, Type::Int(Int::U128)) {
        let mref = MethodRef::new(
            ClassRef::uint_128(asm).into(),
            asm.alloc_string("op_Explicit"),
            asm.sig([src], target),
            MethodKind::Static,
            vec![].into(),
        );
        call!(asm.alloc_methodref(mref), [parrent])
    } else if matches!(target, Type::Int(Int::I128 | Int::U128)) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        match (&src, &target) {
            (Type::Int(Int::U32 | Int::U64), Type::Float(Float::F32)) => {
                conv_f32!(conv_f_un!(parrent))
            }
            (Type::Int(Int::USize), Type::Float(Float::F32)) => {
                conv_f32!(conv_f_un!(conv_u64!(parrent)))
            }
            (_, Type::Float(Float::F32)) => conv_f32!(parrent),
            (Type::Int(Int::U32 | Int::U64), Type::Float(Float::F64)) => {
                conv_f64!(conv_f_un!(parrent))
            }
            (Type::Int(Int::USize), Type::Float(Float::F64)) => {
                conv_f64!(conv_f_un!(conv_u64!(parrent)))
            }
            (_, Type::Float(Float::F64)) => conv_f64!(parrent),
            _ => todo!("Can't  cast {src:?} to {target:?} yet!"),
        }
    }
}
