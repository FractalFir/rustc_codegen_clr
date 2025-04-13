use cilly::{
    call, cil_node::CILNode, cilnode::MethodKind, eq, gt, gt_un, ld_field, lt, lt_un, Assembly,
    ClassRef, FieldDesc, Float, Int, Interned, MethodRef, Type,
};
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::{adt::field_descrptor, r#type::get_type, utilis::is_fat_ptr};
use rustc_middle::ty::{FloatTy, IntTy, Ty, TyKind, UintTy};

pub fn ne_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    operand_a: CILNode,
    operand_b: CILNode,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    //vec![eq_unchecked(ty_a), CILOp::LdcI32(0), CILOp::Eq]
    eq!(
        eq_unchecked(ty_a, operand_a, operand_b, ctx),
        CILNode::V2(ctx.alloc_node(false))
    )
}
pub fn eq_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    operand_a: CILNode,
    operand_b: CILNode,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => {
                let main_module = *ctx.main_module();
                let mref = MethodRef::new(
                    main_module,
                    ctx.alloc_string("eq_u128"),
                    ctx.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => {
                let main_module = *ctx.main_module();
                let mref = MethodRef::new(
                    main_module,
                    ctx.alloc_string("eq_i128"),
                    ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            eq!(operand_a, operand_b)
        }
        TyKind::RawPtr(_, _) | TyKind::FnPtr(_, _) => {
            if is_fat_ptr(ty_a, ctx.tcx(), ctx.instance()) {
                let tpe = get_type(ty_a, ctx).as_class_ref().unwrap();
                let f0 = Interned::data_ptr(ctx, tpe);
                let f0 = eq!(
                    ld_field!(operand_a.clone(), f0),
                    ld_field!(operand_b.clone(), f0)
                );
                let f1 = Interned::metadata(ctx, tpe);
                let f1 = eq!(ld_field!(operand_a, f1), ld_field!(operand_b, f1));
                cilly::and!(f0, f1)
            } else {
                eq!(operand_a, operand_b)
            }
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("__eqtf2"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("eq_f16"),
                ctx.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
pub fn lt_unchecked(
    ty_a: Ty<'_>,
    operand_a: CILNode,
    operand_b: CILNode,
    asm: &mut Assembly,
) -> CILNode {
    //return CILOp::Lt;
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => {
                let mref = MethodRef::new(
                    ClassRef::uint_128(asm),
                    asm.alloc_string("op_LessThan"),
                    asm.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => lt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => {
                let mref = MethodRef::new(
                    ClassRef::int_128(asm),
                    asm.alloc_string("op_LessThan"),
                    asm.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => lt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            lt!(operand_a, operand_b)
        }
        TyKind::RawPtr(_, _) | TyKind::FnPtr(_, _) => lt_un!(operand_a, operand_b),
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("__lttf2"),
                asm.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("lt_f16"),
                asm.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
pub fn gt_unchecked(
    ty_a: Ty<'_>,
    operand_a: CILNode,
    operand_b: CILNode,
    asm: &mut Assembly,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => {
                let mref = MethodRef::new(
                    ClassRef::uint_128(asm),
                    asm.alloc_string("op_GreaterThan"),
                    asm.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => gt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => {
                let mref = MethodRef::new(
                    ClassRef::int_128(asm),
                    asm.alloc_string("op_GreaterThan"),
                    asm.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => gt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            gt!(operand_a, operand_b)
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("__gttf2"),
                asm.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("gt_f16"),
                asm.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::RawPtr(_, _) => gt_un!(operand_a, operand_b),
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
