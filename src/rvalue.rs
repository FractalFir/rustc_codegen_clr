use crate::{Assembly, BaseIR, CLRMethod, LocalPlacement, VariableType};
use rustc_middle::mir::interpret::{ConstValue, Scalar};
use rustc_middle::mir::Rvalue as CompilerRValue;
use rustc_middle::{
    mir::{AggregateKind, BinOp, Body, CastKind, ConstantKind, Operand},
    ty::TyCtxt,
};
macro_rules! sign_cast {
    ($var:ident,$src:ty,$dest:ty) => {
        (<$dest>::from_ne_bytes(($var as $src).to_ne_bytes()))
    };
}
pub(crate) struct RValue {
    ops: Vec<BaseIR>,
}
impl RValue {
    pub(crate) fn get_ops(&self) -> &[BaseIR] {
        &self.ops
    }
    pub(crate) fn from_rvalue<'ctx>(
        rvalue: &CompilerRValue<'ctx>,
        body: &Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        clr_method: &CLRMethod,
        asm: &Assembly,
        target_local:u32,
    ) -> Self {
        let mut new = Self { ops: Vec::new() };
        new.process_rvalue(rvalue, body, tyctx, clr_method, asm,target_local);
        new
    }
    fn process_binop(
        &mut self,
        binop: BinOp,
        a: &Operand,
        b: &Operand,
        clr_method: &CLRMethod,
        asm: &Assembly,
    ) {
        self.process_operand(a, clr_method, asm);
        self.process_operand(b, clr_method, asm);
        self.ops.push(match binop {
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
    }
    fn read_pointer_of_type(&mut self, element_type: &VariableType) {
        match element_type {
            VariableType::I32 => self
                .ops
                .push(BaseIR::LDIndIn(std::mem::size_of::<i32>() as u8)),
            VariableType::Ref(_) | VariableType::RefMut(_) => self.ops.push(BaseIR::LDIndI),
            _ => todo!("Can't derference pointer of type {element_type:?}"),
        }
    }
    fn process_rvalue<'ctx>(
        &mut self,
        rvalue: &CompilerRValue<'ctx>,
        body: &Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        clr_method: &CLRMethod,
        asm: &Assembly,
        target_local:u32,
    ) {
        println!("rvalue:{rvalue:?}");
        match rvalue {
            CompilerRValue::Use(operand) => {
                println!("Use");
                self.process_operand(operand, clr_method, asm)
            }
            CompilerRValue::BinaryOp(binop, operands) => {
                let (a, b): (_, _) = (&operands.0, &operands.1);
                self.process_binop(*binop, a, b, clr_method, asm);
            }
            CompilerRValue::Cast(
                CastKind::IntToInt
                | CastKind::FloatToFloat
                | CastKind::FloatToInt
                | CastKind::IntToFloat,
                operand,
                target,
            ) => {
                self.process_operand(operand, clr_method, asm);
                self.convert(
                    &VariableType::from_ty(operand.ty(body, *tyctx)),
                    &VariableType::from_ty(*target),
                );
            }
            CompilerRValue::Aggregate(kind, operands) => {
                match kind.as_ref() {
                    AggregateKind::Adt(def_id, variant_idx, subst_ref, utai, fidx) => {
                        //let (def_id,variant_idx,subst_ref,utai,fidx) = *adt;
                        panic!("def_id:{def_id:?},variant_idx:{variant_idx:?},subst_ref:{subst_ref:?},utai:{utai:?},fidx:{fidx:?}");
                    }
                    AggregateKind::Array(element_type)=>{
                        //TODO:handle projection!
                        self.load(target_local,clr_method);
                        let element = VariableType::from_ty(*element_type);
                        let arr_name = VariableType::Array{element:Box::new(element.clone()),length:operands.len()}.arg_name();
                        self.ops.push(BaseIR::InitObj(arr_name.clone()));
                        for (index,operand) in operands.iter().enumerate(){
                            self.load(target_local,clr_method);
                            self.ops.push(BaseIR::LDConstI32(index as i32));
                            self.process_operand(operand,clr_method,asm);
                            self.ops.push(BaseIR::CallStatic { sig:crate::FunctionSignature::new(&[element.clone(),VariableType::ISize],&VariableType::Void), function_name:format!("{arr_name}::set_Item").into()});
                        }
                        //todo!();
                    }
                    _ => todo!(
                        "Can't yet handle the aggregate of kind {kind:?} and operands:{operands:?}"
                    ),
                }
            }
            CompilerRValue::Repeat(_, _) => todo!("Can't yet initialize arrays!"),
            CompilerRValue::CopyForDeref(place) => {
                if place.projection.len() == 1 {
                    self.load(place.local.into(), clr_method);
                    self.read_pointer_of_type(&clr_method.get_type_of_local(place.local.into()).get_pointed_type().expect("Tried to deference a value that was neither a pointer nor a reference"));
                } else {
                    panic!(
                        "Can't handle non-trivial deference's yet, projection:{projection:?}!",
                        projection = place.projection
                    );
                }
            }
            _ => todo!("Can't yet handle a rvalue of type {rvalue:?}"),
        }
    }
    fn process_constant(&mut self, constant: ConstantKind, _clr_method: &CLRMethod) {
        match constant {
            ConstantKind::Val(value, r#type) => match value {
                ConstValue::Scalar(scalar) => {
                    let value: u128 = if let Scalar::Int(scalar) = scalar {
                        scalar
                            .try_to_uint(scalar.size())
                            .expect("IMPOSSIBLE. Size of scalar was not equal to itself.")
                    } else {
                        panic!("Can't support pointers quite yet!");
                    };
                    self.load_constant_primitive(&VariableType::from_ty(r#type), value);
                }
                _ => todo!("Unhanled constant value {value:?}"),
            },
            _ => todo!("Unhanled constant {constant:?}"),
        };
    }
    // Makes so the top of the stack is the value of RValue
    fn process_operand(&mut self, operand: &Operand, clr_method: &CLRMethod, asm: &Assembly) {
        match operand {
            Operand::Copy(place) => {
                self.load(place.local.into(), clr_method);
                //println!("Use");
                if place.projection.len() > 0 {
                    self.ops.extend(crate::projection::projection_get(
                        place.projection,
                        clr_method.get_type_of_local(place.local.into()),
                        clr_method,
                        asm,
                    ));
                }
            }
            //TODO:Do moves need to be treated any diffrently forom copies in the context of CLR?
            Operand::Move(place) => {
                self.load(place.local.into(), clr_method);
                if place.projection.len() > 0 {
                    self.ops.extend(crate::projection::projection_get(
                        place.projection,
                        clr_method.get_type_of_local(place.local.into()),
                        clr_method,
                        asm,
                    ));
                }
            }
            Operand::Constant(const_val) => {
                self.process_constant(const_val.literal, clr_method);
            }
        }
    }
    fn load_constant_primitive(&mut self, var_type: &VariableType, value: u128) {
        match var_type {
            VariableType::I8 => self.ops.push(BaseIR::LDConstI8(sign_cast!(value, u8, i8))),
            VariableType::I32 => self
                .ops
                .push(BaseIR::LDConstI32(sign_cast!(value, u32, i32))),
            VariableType::I64 => self
                .ops
                .push(BaseIR::LDConstI64(sign_cast!(value, u64, i64))),
            VariableType::I64 => self
                .ops
                .push(BaseIR::LDConstI64(sign_cast!(value, u64, i64))),
            VariableType::F32 => self
                .ops
                .push(BaseIR::LDConstF32(f32::from_bits(value as u32))),
            VariableType::Bool => self.ops.push(BaseIR::LDConstI8((value != 0) as u8 as i8)),
            _ => todo!("Can't yet load constant primitives of type {var_type:?}!"),
        }
    }
    fn convert(&mut self, src: &VariableType, dest: &VariableType) {
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
            ) => self.ops.push(BaseIR::ConvI32),
            (
                VariableType::F64
                | VariableType::I64
                | VariableType::U64
                | VariableType::ISize
                | VariableType::USize,
                VariableType::I32,
            ) => self.ops.push(BaseIR::ConvI32Checked),
            (VariableType::F32, VariableType::I8) => self.ops.push(BaseIR::ConvI8),
            (VariableType::I32, VariableType::F32) => self.ops.push(BaseIR::ConvF32),
            _ => todo!("Can't convert type {src:?} to {dest:?}"),
        }
    }
    fn load(&mut self, local: u32, clr_method: &CLRMethod) {
        self.ops.push(match clr_method.local_id_placement(local) {
            LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
        })
    }
}
