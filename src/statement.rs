use crate::{
    projection::{projection_adress, projection_get, projection_set},
    Assembly, BaseIR, CLRMethod, LocalPlacement, VariableType,
};
use rustc_index::IndexVec;
use rustc_middle::mir::NullOp;
use rustc_middle::mir::UnOp;
use rustc_middle::mir::{
    interpret::ConstValue, interpret::Scalar, AggregateKind, BinOp, Body, CastKind, Constant,
    ConstantKind, Operand, Place, Rvalue, Statement, StatementKind,
};
use rustc_middle::ty::{Const, ParamEnv, TyCtxt};
use rustc_target::abi::FieldIdx;
#[macro_export]
macro_rules! sign_cast {
    ($var:ident,$src:ty,$dest:ty) => {
        (<$dest>::from_ne_bytes(($var as $src).to_ne_bytes()))
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
    pub(crate) fn get_local_type(&self, local: u32) -> &VariableType {
        self.clr_method.get_type_of_local(local)
    }
    pub(crate) fn place_get_ops(&self, place: &Place<'tctx>) -> Vec<BaseIR> {
        projection_get(place, self.get_local_type(place.local.into()), self)
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
    pub(crate) fn place_set_ops(&self, place: &Place<'tctx>) -> (Vec<BaseIR>, Vec<BaseIR>) {
        //Empty projection, just set the local
        projection_set(place, self.get_local_type(place.local.into()), self)
    }
}
fn load_const_scalar(scalar: Scalar, scalar_type: VariableType) -> Vec<BaseIR> {
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int
            .try_to_uint(scalar.size())
            .expect("IMPOSSIBLE. Size of scalar was not equal to itself."),
        Scalar::Ptr(_, _) => todo!("Can't handle scalar pointers yet!"),
    };
    match scalar_type {
        VariableType::I8 => vec![BaseIR::LDConstI32(sign_cast!(scalar_u128, u8, i8) as i32)],
        VariableType::U8 => vec![BaseIR::LDConstI32(scalar_u128 as u8 as i32)],
        VariableType::I16 => vec![BaseIR::LDConstI32(sign_cast!(scalar_u128, u16, i16) as i32)],
        VariableType::U16 => vec![BaseIR::LDConstI32(scalar_u128 as i32)],
        VariableType::I32 => vec![BaseIR::LDConstI32(sign_cast!(scalar_u128, u32, i32))],
        VariableType::U32 => vec![BaseIR::LDConstI32(scalar_u128 as i32)],
        VariableType::F32 => vec![BaseIR::LDConstF32(f32::from_bits(scalar_u128 as u32))],
        VariableType::Bool => vec![BaseIR::LDConstI32((scalar_u128 != 0) as u8 as i32)],
        VariableType::I64 => vec![BaseIR::LDConstI64(sign_cast!(scalar_u128, u64, i64))],
        VariableType::U64 => vec![BaseIR::LDConstI64(scalar_u128 as i64)],
        VariableType::USize => vec![BaseIR::LDConstI64(scalar_u128 as i64)],
        VariableType::ISize => vec![BaseIR::LDConstI64(scalar_u128 as i64)],
        VariableType::Enum(_) => vec![BaseIR::LDConstI32((scalar_u128 != 0) as u8 as i32)],
        _ => todo!("can't yet handle a scalar of type {scalar_type:?}!"),
    }
}
fn load_const_value(const_val: ConstValue, const_ty: VariableType) -> Vec<BaseIR> {
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
            load_const_value(value, VariableType::from_ty(const_ty, *codegen_ctx.tyctx()))
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
        Operand::Copy(place) => codegen_ctx.place_get_ops(place),
        Operand::Move(place) => codegen_ctx.place_get_ops(place),
        Operand::Constant(const_val) => handle_constant(const_val.as_ref(), codegen_ctx),
    }
}
fn handle_binop<'ctx>(
    binop: BinOp,
    a: &Operand<'ctx>,
    b: &Operand<'ctx>,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    let mut ops = Vec::new();
    ops.extend(handle_operand(a, codegen_ctx));
    ops.extend(handle_operand(b, codegen_ctx));
    match binop {
        BinOp::Add | BinOp::AddUnchecked => ops.push(BaseIR::Add),
        BinOp::Sub | BinOp::SubUnchecked => ops.push(BaseIR::Sub),
        BinOp::Mul | BinOp::MulUnchecked => ops.push(BaseIR::Mul),
        BinOp::Shl | BinOp::ShlUnchecked => ops.push(BaseIR::Shl),
        BinOp::Shr | BinOp::ShrUnchecked => ops.push(BaseIR::Shr),
        BinOp::Eq => ops.push(BaseIR::Eq),
        BinOp::Ne => ops.extend([BaseIR::Eq, BaseIR::LDConstI32(0), BaseIR::Eq]),
        BinOp::Gt => ops.push(BaseIR::Gt),
        BinOp::Lt => ops.push(BaseIR::Lt),
        BinOp::Ge => ops.push(BaseIR::Ge),
        BinOp::Le => ops.push(BaseIR::Le),
        BinOp::Rem => ops.push(BaseIR::Rem),
        BinOp::BitXor => ops.push(BaseIR::Xor),
        BinOp::BitOr => ops.push(BaseIR::Or),
        BinOp::BitAnd => ops.push(BaseIR::And),
        BinOp::Div => ops.push(BaseIR::Div),
        BinOp::Offset => todo!("Can't yet handle the pointer offset operator!"),
    };
    ops
}
fn handle_convert(src: &VariableType, dest: &VariableType) -> Vec<BaseIR> {
    crate::codegen::convert::convert_as(src, dest)
}
fn handle_agregate<'tyctx>(
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &Place<'tyctx>,
    aggregate: &AggregateKind<'tyctx>,
    fields: &IndexVec<FieldIdx, Operand<'tyctx>>,
) -> Vec<BaseIR> {
    match aggregate {
        AggregateKind::Array(element_type) => {
            let agregate_adress = codegen_ctx.place_adress_ops(target_location);
            let mut agregate_construction = Vec::new();
            let element = VariableType::from_ty(*element_type, *codegen_ctx.tyctx());
            let arr_name = VariableType::Array {
                element: Box::new(element.clone()),
                length: fields.len(),
            }
            .arg_name();
            if crate::ALWAYS_INIT_STRUCTS {
                agregate_construction.extend(agregate_adress.clone());
                agregate_construction.push(BaseIR::InitObj(arr_name.clone()));
            }
            for (index, operand) in fields.iter().enumerate() {
                agregate_construction.extend(agregate_adress.clone());
                agregate_construction.push(BaseIR::LDConstI32(index as i32));
                agregate_construction.push(BaseIR::ConvI);
                agregate_construction.extend(handle_operand(operand, codegen_ctx));
                agregate_construction.push(BaseIR::Call {
                    sig: crate::FunctionSignature::new(
                        &[VariableType::ISize, element.clone()],
                        &VariableType::Void,
                    ),
                    function_name: format!("{arr_name}::set_Item").into(),
                });
            }
            agregate_construction.extend(codegen_ctx.place_get_ops(target_location));
            agregate_construction
            //todo!("Can't yet create aggreate arrays with element type {element_type:?}")
        }
        AggregateKind::Adt(def_id, _varaint, subst, _uta, _field_idx) => {
            let agregate_adress = codegen_ctx.place_adress_ops(target_location);
            let mut agregate_construction = Vec::new();
            let param_env = ParamEnv::empty();
            let adt_type = VariableType::from_ty(
                rustc_middle::ty::Instance::resolve(
                    *codegen_ctx.tyctx(),
                    param_env,
                    *def_id,
                    subst,
                )
                .expect("Can't get type!")
                .expect("Can't get type!")
                .ty(*codegen_ctx.tyctx(), param_env),
                *codegen_ctx.tyctx(),
            );
            match adt_type {
                VariableType::Struct(name) => {
                    if crate::ALWAYS_INIT_STRUCTS {
                        agregate_construction.extend(agregate_adress.clone());
                        agregate_construction.push(BaseIR::InitObj(name.clone()));
                    }
                    for (field_idx, field) in fields.iter().enumerate() {
                        agregate_construction.extend(agregate_adress.clone());
                        agregate_construction.extend(handle_operand(field, codegen_ctx));
                        agregate_construction.extend(
                            codegen_ctx
                                .asm()
                                .get_field_setter(field_idx, &name)
                                .expect("Can't get field!"),
                        );
                    }
                    agregate_construction.extend(codegen_ctx.place_get_ops(target_location));
                    agregate_construction
                }
                VariableType::Enum(enum_name) => vec![BaseIR::DebugComment(enum_name)],
                _ => todo!("Unhandled adt type {adt_type:?}"),
            }
        }
        _ => todo!("Can't handle agregates of type {aggregate:?} yet!"),
    }
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
                &VariableType::from_ty(
                    operand.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),
                    *codegen_ctx.tyctx(),
                ),
                &VariableType::from_ty(*target, *codegen_ctx.tyctx()),
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
        Rvalue::Aggregate(aggregate, fields) => {
            handle_agregate(codegen_ctx, target_location, aggregate.as_ref(), fields)
        }
        Rvalue::Repeat(operand, ammount) => {
            let ammount = const_to_usize(ammount, *codegen_ctx.tyctx());
            let array_adress = codegen_ctx.place_adress_ops(target_location);

            let mut array_init = Vec::new();
            let element = VariableType::from_ty(
                operand.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),
                *codegen_ctx.tyctx(),
            );
            let arr_name = VariableType::Array {
                element: Box::new(element.clone()),
                length: ammount,
            }
            .arg_name();
            if crate::ALWAYS_INIT_STRUCTS {
                array_init.extend(array_adress.clone());
                array_init.push(BaseIR::InitObj(arr_name.clone()));
            }
            //TODO: handle large array sizes using loops!
            for index in 0..ammount {
                array_init.extend(array_adress.clone());
                array_init.push(BaseIR::LDConstI64(index as i64));
                array_init.push(BaseIR::ConvI);
                array_init.extend(handle_operand(operand, codegen_ctx));
                array_init.push(BaseIR::Call {
                    sig: crate::FunctionSignature::new(
                        &[VariableType::ISize, element.clone()],
                        &VariableType::Void,
                    ),
                    function_name: format!("{arr_name}::set_Item").into(),
                });
            }
            array_init.extend(codegen_ctx.place_get_ops(target_location));
            array_init
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
            let ty = VariableType::from_ty(
                palce.ty(codegen_ctx.body(), *codegen_ctx.tyctx()).ty,
                *codegen_ctx.tyctx(),
            );
            vec![ty.sizeof_op()]
        }
        Rvalue::CheckedBinaryOp(_, _) => todo!("Can't yet preform checked binary operations"),
        Rvalue::UnaryOp(unary, operand) => {
            let mut ops = handle_operand(operand, codegen_ctx);
            match unary {
                UnOp::Not => ops.push(BaseIR::Not),
                _ => todo!("Can't yet preform unary ops of type {unary:?}!"),
            }
            ops
        }
        Rvalue::NullaryOp(null_op, op_type) => {
            let op_type = VariableType::from_ty(*op_type, *codegen_ctx.tyctx());
            match null_op {
                NullOp::SizeOf => vec![op_type.sizeof_op()],
                //TODO: actualy calcualte algiment!
                NullOp::AlignOf => vec![BaseIR::LDConstI32(8)],
                _ => todo!("Unsuiported nullary op {null_op:?}"),
            }
            //todo!("Can't yet preform nulray ops!")
        }
        Rvalue::CopyForDeref(place) => codegen_ctx.place_get_ops(place),
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
            let rvalue_ops = handle_rvalue(&rvalue, &codegen_ctx, &place); // RValue::from_rvalue(rvalue, body, tyctx, self, asm,place.local.into());
            let (adress_calc, set_op) = codegen_ctx.place_set_ops(&place);
            let mut final_ops = adress_calc;
            final_ops.extend(rvalue_ops);
            final_ops.extend(set_op);
            final_ops
        }
        StatementKind::StorageLive(_local) => Vec::new(), //TODO: maybe use lifetime info to better guide CLR IL generation?
        StatementKind::StorageDead(_local) => Vec::new(), //TODO: maybe use lifetime info to better guide CLR IL generation?
        _ => todo!("Unhanded statement:{:?}", statement.kind),
    }
}
