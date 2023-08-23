use crate::{
    projection::{projection_adress, projection_get, projection_set},
    Assembly, BaseIR, CLRMethod, LocalPlacement, VariableType,
};
use rustc_index::IndexVec;
use rustc_middle::mir::{
    interpret::ConstValue, interpret::Scalar, AggregateKind, BinOp, Body, CastKind, Constant,
    ConstantKind, Operand, Place, Rvalue, Statement, StatementKind,
};
use rustc_middle::ty::{TyCtxt,ParamEnv,Const};
use rustc_target::abi::FieldIdx;
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
    pub(crate) fn place_get_ops(&self, place: &Place) -> Vec<BaseIR> {
        if place.projection.is_empty() {
            vec![
                match self.clr_method.local_id_placement(place.local.into()) {
                    LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
                },
            ]
        } else {
            projection_get(
                place.projection,
                self.get_local_type(place.local.into()),
                self
            )
        }
    }
    pub(crate) fn place_adress_ops(&self, place: &Place) -> Vec<BaseIR> {
        if place.projection.is_empty() {
            vec![
                match self.clr_method.local_id_placement(place.local.into()) {
                    LocalPlacement::Arg(arg_id) => BaseIR::LDArgA(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::LDLocA(var_id),
                },
            ]
        } else {
            projection_adress(
                place.projection,
                self.get_local_type(place.local.into()),
                self,
            )
        }
    }
    pub(crate) fn place_set_ops(&self, place: &Place) -> (Vec<BaseIR>, BaseIR) {
        //Empty projection, just set the local
        if place.projection.is_empty() {
            (
                Vec::new(),
                match self.clr_method.local_id_placement(place.local.into()) {
                    LocalPlacement::Arg(arg_id) => BaseIR::STArg(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::STLoc(var_id),
                },
            )
        } else {
            projection_set(
                place.projection,
                self.get_local_type(place.local.into()),
                self
            )
        }
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
        VariableType::I8 => vec![BaseIR::LDConstI8(sign_cast!(scalar_u128, u8, i8))],
        VariableType::U8 => vec![BaseIR::LDConstI32(scalar_u128 as u8 as i32)],
        VariableType::I16 => vec![BaseIR::LDConstI32(sign_cast!(scalar_u128, u16, i16) as i32)],
        VariableType::U16 => vec![BaseIR::LDConstI32(scalar_u128 as i32)],
        VariableType::I32 => vec![BaseIR::LDConstI32(sign_cast!(scalar_u128, u32, i32))],
        VariableType::F32 => vec![BaseIR::LDConstF32(f32::from_bits(scalar_u128 as u32))],
        VariableType::Bool => vec![BaseIR::LDConstI8((scalar_u128 != 0) as u8 as i8)],
        _ => todo!("can't yet handle a scalar of type {scalar_type:?}!"),
    }
}
fn load_const_value(const_val: ConstValue, const_ty: VariableType) -> Vec<BaseIR> {
    match const_val {
        ConstValue::Scalar(scalar) => load_const_scalar(scalar, const_ty),
        _ => todo!("Unhandled const value {const_val:?}"),
    }
}
fn handle_constant(constant: &Constant, codegen_ctx: &CodegenCtx) -> Vec<BaseIR> {
    let const_kind = constant.literal;
    match const_kind {
        ConstantKind::Val(value, const_ty) => {
            load_const_value(value, VariableType::from_ty(const_ty,*codegen_ctx.tyctx()))
        }
        _ => todo!("Unhanded const kind {const_kind:?}!"),
    }
}
pub(crate) fn handle_operand(operand: &Operand, codegen_ctx: &CodegenCtx) -> Vec<BaseIR> {
    match operand {
        Operand::Copy(place) => codegen_ctx.place_get_ops(place),
        Operand::Move(place) => codegen_ctx.place_get_ops(place),
        Operand::Constant(const_val) => handle_constant(const_val.as_ref(), codegen_ctx),
    }
}
fn handle_binop(binop: BinOp, a: &Operand, b: &Operand, codegen_ctx: &CodegenCtx) -> Vec<BaseIR> {
    let mut ops = Vec::new();
    ops.extend(handle_operand(a, codegen_ctx));
    ops.extend(handle_operand(b, codegen_ctx));
    ops.push(match binop {
        BinOp::Add => BaseIR::Add,
        BinOp::Sub => BaseIR::Sub,
        BinOp::Mul => BaseIR::Mul,
        BinOp::Shl => BaseIR::Shl,
        BinOp::Shr => BaseIR::Shr,
        BinOp::Eq => BaseIR::Eq,
        BinOp::Ne => BaseIR::NEq,
        BinOp::Gt => BaseIR::Gt,
        BinOp::Rem => BaseIR::Rem,
        BinOp::BitXor => BaseIR::Xor,
        BinOp::BitOr => BaseIR::Or,
        BinOp::BitAnd => BaseIR::And,
        BinOp::Div => BaseIR::Div,
        _ => todo!("Unknown binop:{binop:?}"),
    });
    ops
}
fn handle_convert(src: &VariableType, dest: &VariableType) -> Vec<BaseIR> {
    match (src, dest) {
        (
            VariableType::F32
            | VariableType::I8
            | VariableType::I16
            | VariableType::I32
            | VariableType::U8
            | VariableType::U16
            | VariableType::U32,
            VariableType::I32,
        ) => vec![BaseIR::ConvI32],
        (
            VariableType::F64
            | VariableType::I64
            | VariableType::U64
            | VariableType::ISize
            | VariableType::USize,
            VariableType::I32,
        ) => vec![BaseIR::ConvI32Checked],
        (VariableType::F32, VariableType::I8) => vec![BaseIR::ConvI8],
        (VariableType::I32, VariableType::F32) => vec![BaseIR::ConvF32],
        _ => todo!("Can't convert type {src:?} to {dest:?}"),
    }
}
fn handle_agregate<'tyctx>(
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &Place,
    aggregate: &AggregateKind<'tyctx>,
    fields: &IndexVec<FieldIdx, Operand<'tyctx>>,
) -> Vec<BaseIR> {
    match aggregate {
        AggregateKind::Array(element_type) => {
            let agregate_adress = codegen_ctx.place_adress_ops(target_location);
            let mut agregate_construction = Vec::new();
            let element = VariableType::from_ty(*element_type,*codegen_ctx.tyctx());
            let arr_name = VariableType::Array {
                element: Box::new(element.clone()),
                length: fields.len(),
            }
            .arg_name();
            if crate::ALWAYS_INIT_STRUCTS{
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
                        &[VariableType::ISize,element.clone()],
                        &VariableType::Void,
                    ),
                    function_name: format!("{arr_name}::set_Item").into(),
                });
            }
            agregate_construction.extend(codegen_ctx.place_get_ops(target_location));
            agregate_construction
            //todo!("Can't yet create aggreate arrays with element type {element_type:?}")
        }
        AggregateKind::Adt(def_id,_varaint,subst,_uta,_field_idx)=>{
            let agregate_adress = codegen_ctx.place_adress_ops(target_location);
            let mut agregate_construction = Vec::new();
            let param_env = ParamEnv::empty();
            let adt_type = VariableType::from_ty(rustc_middle::ty::Instance::resolve(*codegen_ctx.tyctx(),param_env,*def_id,subst).expect("Can't get type!").expect("Can't get type!").ty(*codegen_ctx.tyctx(),param_env),*codegen_ctx.tyctx());
            match adt_type{
                VariableType::Struct(name)=>{
                    if crate::ALWAYS_INIT_STRUCTS{
                        agregate_construction.extend(agregate_adress.clone());
                        agregate_construction.push(BaseIR::InitObj(name.clone()));
                    }
                    for (field_idx,field) in fields.iter().enumerate(){
                        agregate_construction.extend(agregate_adress.clone());
                        agregate_construction.extend(handle_operand(field, codegen_ctx));
                        agregate_construction.extend(codegen_ctx.asm().get_field_setter(field_idx,&name).expect("Can't get field!"));
                        
                    } 
                    agregate_construction.extend(codegen_ctx.place_get_ops(target_location));
                    agregate_construction 
                },
                _=>todo!("Unhandled adt type {adt_type:?}"),
            }
        },
        _ => todo!("Can't handle agregates of type {aggregate:?} yet!"),
    }
}
fn const_to_usize<'tyctx>(constant:&Const<'tyctx>,tyctx: TyCtxt<'tyctx>)->usize{
    //TODO: handle constant conversion better.
    constant.eval_target_usize(tyctx,ParamEnv::empty()) as usize
}
fn handle_rvalue<'tyctx>(
    rvalue: &Rvalue<'tyctx>,
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &Place,
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
                &VariableType::from_ty(operand.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),*codegen_ctx.tyctx()),
                &VariableType::from_ty(*target,*codegen_ctx.tyctx()),
            );
            let operand = handle_operand(operand, codegen_ctx);
            let mut final_ops = operand;
            final_ops.extend(conversion);
            final_ops
        }
        Rvalue::Aggregate(aggregate, fields) => {
            handle_agregate(codegen_ctx, target_location, aggregate.as_ref(), fields)
        }
        Rvalue::Repeat(operand, ammount) => {
            let ammount = const_to_usize(ammount,*codegen_ctx.tyctx());
            let array_adress = codegen_ctx.place_adress_ops(target_location);
            
            let mut array_init = Vec::new();
            let element = VariableType::from_ty(operand.ty(codegen_ctx.body(), *codegen_ctx.tyctx()),*codegen_ctx.tyctx());
            let arr_name = VariableType::Array {
                element: Box::new(element.clone()),
                length: ammount,
            }.arg_name();
            if crate::ALWAYS_INIT_STRUCTS{
                array_init.extend(array_adress.clone());
                array_init.push(BaseIR::InitObj(arr_name.clone()));
            }
            //TODO: handle large array sizes using loops!
            for index in 0..ammount{
                array_init.extend(array_adress.clone());
                array_init.push(BaseIR::LDConstI64(index as i64));
                array_init.push(BaseIR::ConvI);
                array_init.extend(handle_operand(operand, codegen_ctx));
                array_init.push(BaseIR::Call {
                    sig: crate::FunctionSignature::new(
                        &[VariableType::ISize,element.clone()],
                        &VariableType::Void,
                    ),
                    function_name: format!("{arr_name}::set_Item").into(),
                });
            }
            array_init.extend(codegen_ctx.place_get_ops(target_location));
            array_init
        }
        Rvalue::ThreadLocalRef(_) => todo!("Can't handle thread local data yet!"),
        Rvalue::Ref(_, _, _) => todo!("Can't create referneces yet!"),
        Rvalue::AddressOf(_, _) => todo!("Can't get adress of things yet!"),
        Rvalue::Len(palce) =>{
            let ty = VariableType::from_ty(palce.ty(codegen_ctx.body(),*codegen_ctx.tyctx()).ty,*codegen_ctx.tyctx());
            vec![ty.sizeof_op()]
        },
        Rvalue::CheckedBinaryOp(_, _) => todo!("Can't yet preform checked binary operations"),
        Rvalue::UnaryOp(_, _) => todo!("Can't yet preform unary ops!"),
        Rvalue::NullaryOp(_, _) => todo!("Can't yet preform nulray ops!"),
        Rvalue::CopyForDeref(place) => codegen_ctx.place_get_ops(place),
        Rvalue::Discriminant(_) => todo!("Can't yet compute discriminat types!"),
        Rvalue::ShallowInitBox(_, _) => todo!("Can't yet shalowly initalize a box!"),
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
            final_ops.push(set_op);
            final_ops
        }
        StatementKind::StorageLive(_local) => Vec::new(), //TODO: maybe use lifetime info to better guide CLR IL generation?
        StatementKind::StorageDead(_local) => Vec::new(), //TODO: maybe use lifetime info to better guide CLR IL generation?
        _ => todo!("Unhanded statement:{:?}", statement.kind),
    }
}
