use crate::codegen::place::{place_getter_ops, place_setter_ops};
use crate::{
    projection::projection_adress, types::Type, Assembly, BaseIR, CLRMethod, LocalPlacement,
};
use rustc_middle::mir::NullOp;
use rustc_middle::mir::{
    interpret::ConstValue, interpret::Scalar, BinOp, Body, CastKind, Constant,
    ConstantKind, Operand, Place, Rvalue, Statement, StatementKind,
};
use rustc_middle::ty::{Const, ParamEnv, TyCtxt};
#[macro_export]
macro_rules! sign_cast {
    ($var:ident,$src:ty,$dest:ty) => {
        <$dest>::from_ne_bytes(($var as $src).to_ne_bytes())
    };
}
pub(crate) struct CodegenCtx<'tctx, 'local_ctx> {
    clr_method: &'local_ctx CLRMethod,
    asm: &'local_ctx Assembly,
    body: &'tctx Body<'tctx>,
    tyctx: TyCtxt<'tctx>,
}
impl<'tctx, 'local_ctx> CodegenCtx<'tctx, 'local_ctx> {
    pub(crate) fn new(
        clr_method: &'local_ctx CLRMethod,
        asm: &'local_ctx Assembly,
        body: &'tctx Body<'tctx>,
        tyctx: TyCtxt<'tctx>,
    ) -> Self {
        Self {
            clr_method,
            asm,
            body,
            tyctx,
        }
    }
    pub(crate) fn local_placement(&self, local: u32) -> LocalPlacement {
        self.meth().local_id_placement(local)
    }
    pub(crate) fn asm(&self) -> &Assembly {
        self.asm
    }
    pub(crate) fn meth(&self) -> &CLRMethod {
        self.clr_method
    }
    pub(crate) fn body(&self) -> &'tctx Body<'tctx> {
        self.body
    }
    pub(crate) fn tyctx(&self) -> &TyCtxt<'tctx> {
        &self.tyctx
    }
    pub(crate) fn get_local_type(&self, local: u32) -> &Type {
        self.clr_method.get_type_of_local(local)
    }
    pub(crate) fn place_adress_ops(&self, place: &Place<'tctx>) -> Vec<BaseIR> {
        if place.projection.is_empty() {
            vec![
                match self.clr_method.local_id_placement(place.local.into()) {
                    LocalPlacement::Arg(arg_id) => BaseIR::LDArgA(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::LDLocA(var_id),
                },
            ]
        } else {
            projection_adress(place, self.get_local_type(place.local.into()), self)
        }
    }
}
fn load_const_scalar(scalar: Scalar, scalar_type: Type) -> Vec<BaseIR> {
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int
            .try_to_uint(scalar.size())
            .expect("IMPOSSIBLE. Size of scalar was not equal to itself."),
        Scalar::Ptr(_, _) => todo!("Can't handle scalar pointers yet!"),
    };
    match scalar_type {
        Type::I8 => vec![BaseIR::LDConstI32(i32::from(sign_cast!(
            scalar_u128,
            u8,
            i8
        )))],
        Type::U8 => vec![BaseIR::LDConstI32(i32::from(scalar_u128 as u8))],
        Type::I16 => vec![BaseIR::LDConstI32(i32::from(sign_cast!(
            scalar_u128,
            u16,
            i16
        )))],
        Type::U16 => vec![BaseIR::LDConstI32(scalar_u128 as i32)],
        Type::I32 => vec![BaseIR::LDConstI32(sign_cast!(scalar_u128, u32, i32))],
        Type::U32 => vec![BaseIR::LDConstI32(scalar_u128 as i32)],
        Type::F32 => vec![BaseIR::LDConstF32(f32::from_bits(scalar_u128 as u32))],
        Type::Bool => vec![BaseIR::LDConstI32(i32::from(u8::from(scalar_u128 != 0)))],
        Type::I64 => vec![BaseIR::LDConstI64(sign_cast!(scalar_u128, u64, i64))],
        Type::U64 => vec![BaseIR::LDConstI64(scalar_u128 as i64)],
        Type::USize => vec![BaseIR::LDConstI64(scalar_u128 as i64)],
        Type::ISize => vec![BaseIR::LDConstI64(scalar_u128 as i64)],
        Type::Struct { name, fields } => todo!(),
        //Type::Enum(_) => vec![BaseIR::LDConstI32((scalar_u128 != 0) as u8 as i32)],
        _ => todo!("can't yet handle a scalar of type {scalar_type:?}!"),
    }
}
fn load_const_value(const_val: ConstValue, const_ty: Type) -> Vec<BaseIR> {
    match const_val {
        ConstValue::Scalar(scalar) => load_const_scalar(scalar, const_ty),
        ConstValue::ZeroSized => vec![BaseIR::DebugComment("ZeroSized!".into())],
        _ => todo!("Unhandled const value {const_val:?}"),
    }
}
fn handle_constant<'ctx>(
    constant: &Constant<'ctx>,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    let const_kind = constant.literal;
    match const_kind {
        ConstantKind::Val(value, const_ty) => {
            load_const_value(value, Type::from_ty(&const_ty, codegen_ctx.tyctx()))
        }
        ConstantKind::Unevaluated(a, b) => {
            vec![BaseIR::DebugComment(format!("{a:?} {b:?}").into())]
        }
        _ => todo!("Unhanded const kind {const_kind:?}!"),
    }
}
pub(crate) fn handle_operand<'ctx>(
    operand: &Operand<'ctx>,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    match operand {
        Operand::Copy(place) => place_getter_ops(place, codegen_ctx),
        Operand::Move(place) => place_getter_ops(place, codegen_ctx),
        Operand::Constant(const_val) => handle_constant(const_val.as_ref(), codegen_ctx),
    }
}
fn handle_binop<'ctx>(
    binop: BinOp,
    a: &Operand<'ctx>,
    b: &Operand<'ctx>,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    let a_type = Type::from_ty(
        &a.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),
        codegen_ctx.tyctx(),
    );
    let b_type = Type::from_ty(
        &a.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),
        codegen_ctx.tyctx(),
    );
    crate::codegen::arthmetics::binop_unchecked(
        binop,
        (handle_operand(a, codegen_ctx), a_type),
        (handle_operand(b, codegen_ctx), b_type),
    )
}
fn handle_convert(src: &Type, dest: &Type) -> Vec<BaseIR> {
    crate::codegen::convert::convert_as(src, dest)
}
fn const_to_usize<'tyctx>(constant: &Const<'tyctx>, tyctx: TyCtxt<'tyctx>) -> usize {
    //TODO: handle constant conversion better.
    constant.eval_target_usize(tyctx, ParamEnv::empty()) as usize
}
fn handle_rvalue<'tyctx>(
    rvalue: &Rvalue<'tyctx>,
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &Place<'tyctx>,
) -> Vec<BaseIR> {
    match rvalue {
        Rvalue::BinaryOp(binop, operands) => {
            handle_binop(*binop, &operands.0, &operands.1, codegen_ctx)
        }
        Rvalue::Use(operand) => handle_operand(operand, codegen_ctx),
        Rvalue::Cast(
            CastKind::IntToInt
            | CastKind::FloatToFloat
            | CastKind::FloatToInt
            | CastKind::IntToFloat,
            operand,
            target,
        ) => {
            let conversion = handle_convert(
                &Type::from_ty(
                    &operand.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),
                    codegen_ctx.tyctx(),
                ),
                &Type::from_ty(target, codegen_ctx.tyctx()),
            );
            let operand = handle_operand(operand, codegen_ctx);
            let mut final_ops = operand;
            final_ops.extend(conversion);
            final_ops
        }
        Rvalue::Cast(CastKind::PtrToPtr, operand, target) => {
            let mut ops = handle_operand(operand, codegen_ctx);
            ops.push(BaseIR::ConvI);
            ops
        }
        Rvalue::Aggregate(aggregate, fields) => crate::codegen::aggregate::handle_agregate(codegen_ctx, target_location, aggregate.as_ref(), fields),
        Rvalue::Repeat(operand, ammount) => {
            todo!();
        }
        Rvalue::ThreadLocalRef(_) => todo!("Can't handle thread local data yet!"),
        Rvalue::Ref(_region, _kind, place) => {
            codegen_ctx.place_adress_ops(place)
            //todo!("Can't create referneces yet!")
        }
        Rvalue::AddressOf(_mutability, place) => {
            codegen_ctx.place_adress_ops(place)
            //todo!("Can't get adress of things yet!"),
        }
        Rvalue::Len(palce) => {
            let ty = Type::from_ty(
                &palce.ty(codegen_ctx.body(), *codegen_ctx.tyctx()).ty,
                codegen_ctx.tyctx(),
            );
            crate::codegen::sizeof_ops(&ty)
        }
        Rvalue::CheckedBinaryOp(_, _) => todo!("Can't yet preform checked binary operations"),
        Rvalue::UnaryOp(unary, operand) => {
            let tpe = Type::from_ty(
                &operand.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),
                codegen_ctx.tyctx(),
            );
            crate::codegen::arthmetics::unop_unchecked(
                *unary,
                (handle_operand(operand, codegen_ctx), tpe),
            )
        }
        Rvalue::NullaryOp(null_op, op_type) => {
            let op_type = Type::from_ty(op_type, codegen_ctx.tyctx());
            match null_op {
                NullOp::SizeOf => crate::codegen::sizeof_ops(&op_type),
                NullOp::AlignOf => vec![crate::codegen::align_of(op_type)],
                _ => todo!("Unsuiported nullary op {null_op:?}"),
            }
        }
        Rvalue::CopyForDeref(place) => place_getter_ops(place, codegen_ctx),
        Rvalue::Discriminant(_) => todo!("Can't yet compute discriminat types!"),
        Rvalue::ShallowInitBox(_, _) => todo!("Can't yet shalowly initalize a box!"),
        Rvalue::Cast(CastKind::PointerCoercion(ptr), operand, _) => {
            handle_operand(operand, codegen_ctx)
        }
        Rvalue::Cast(cast_kind, _, _) => todo!("Can't yet handle casts of type {cast_kind:?}"),
    }
}
pub(crate) fn handle_statement<'tctx, 'local_ctx>(
    statement: &Statement<'tctx>,
    clr_method: &'local_ctx CLRMethod,
    asm: &'local_ctx Assembly,
    body: &'tctx Body<'tctx>,
    tyctx: TyCtxt<'tctx>,
) -> Vec<BaseIR> {
    let codegen_ctx = CodegenCtx::new(clr_method, asm, body, tyctx);
    match &statement.kind {
        StatementKind::Assign(asign_box) => {
            let (place, rvalue) = (asign_box.0, &asign_box.1);
            let rvalue_ops = handle_rvalue(rvalue, &codegen_ctx, &place); // RValue::from_rvalue(rvalue, body, tyctx, self, asm,place.local.into());

            place_setter_ops(&place, &codegen_ctx, rvalue_ops)
        }
        StatementKind::StorageLive(_local) => Vec::new(), //TODO: maybe use lifetime info to better guide CLR IL generation?
        StatementKind::StorageDead(_local) => Vec::new(), //TODO: maybe use lifetime info to better guide CLR IL generation?
        _ => todo!("Unhanded statement:{:?}", statement.kind),
    }
}
