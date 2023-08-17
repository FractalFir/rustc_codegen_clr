use crate::{clr_method::LocalPlacement, BaseIR, CLRMethod,VariableType};
use rustc_middle::mir::Rvalue as CompilerRValue; 
use rustc_middle::mir::interpret::{Scalar,ConstValue};
use rustc_middle::{
    mir::{Body, Place,AggregateKind,CastKind,BinOp,Operand,ConstantKind},
    ty::TyCtxt,
};
use rustc_middle::mir::ProjectionElem;
macro_rules! sign_cast {
    ($var:ident,$src:ty,$dest:ty) => {
        (<$dest>::from_ne_bytes(($var as $src).to_ne_bytes()))
    };
}
enum AsigmentValuePosition {
    BeforeAdress,
    AfterAdress,
}
pub(crate) struct AsigmentTarget {
    adress_calc: Vec<BaseIR>,
    value_pos: AsigmentValuePosition,
    set_ops: Vec<BaseIR>,
}
impl AsigmentTarget {
    pub(crate) fn from_placement(place: Place, clr_method: &CLRMethod) -> Self {
        let mut new = Self {
            adress_calc: Vec::new(),
            set_ops: Vec::new(),
            value_pos: AsigmentValuePosition::BeforeAdress,
        };
        if place.projection.is_empty() {
            let local: u32 = place.local.into();
            new.set_ops
                .push(match clr_method.local_id_placement(local) {
                    LocalPlacement::Arg(arg_id) => BaseIR::STArg(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::STLoc(var_id),
                });
        } else {
            let local: u32 = place.local.into();
            new.adress_calc
                .push(match clr_method.local_id_placement(local) {
                    LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
                });
            for modifier in &place.projection[..(place.projection.len() - 1)]{
                todo!("Can't handle assignments with more than one level of indirection")
            }
            let last = place.projection[(place.projection.len() - 1)];
            match last{
                ProjectionElem::Deref => {
                    //TODO: handle the type
                    new.value_pos = AsigmentValuePosition::AfterAdress;
                    new.set_ops.push(BaseIR::STIInd(4));
                }
                _=> todo!("Can't handle ProjectionElements of type {last:?}!"),
            }
        }
        new
    }
    pub(crate) fn finalize_with_ops(self, ops: &[BaseIR], clr_method: &mut CLRMethod) {
        match self.value_pos{
            AsigmentValuePosition::BeforeAdress => {
                clr_method.extend_ops(ops);
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(&self.set_ops);
            },
            AsigmentValuePosition::AfterAdress => {
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(ops);
                clr_method.extend_ops(&self.set_ops);
            },
        }
    }
    pub(crate) fn finalize(self, rvalue: RValue, clr_method: &mut CLRMethod) {
        match self.value_pos{
            AsigmentValuePosition::BeforeAdress => {
                clr_method.extend_ops(rvalue.get_ops());
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(&self.set_ops);
            },
            AsigmentValuePosition::AfterAdress => {
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(rvalue.get_ops());
                clr_method.extend_ops(&self.set_ops);
            },
        }
    }
}
pub(crate) struct RValue {
    ops:Vec<BaseIR>,
}
impl RValue {
    pub(crate) fn get_ops(&self)->&[BaseIR]{
        &self.ops
    }
    pub(crate) fn from_rvalue<'ctx>(
        rvalue: &CompilerRValue<'ctx>,
        body: &Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        clr_method:&CLRMethod,
    ) -> Self {
        let mut new = Self {ops:Vec::new()};
        new.process_rvalue(rvalue, body, tyctx,clr_method);
        new
    }
    fn process_binop(&mut self,binop: BinOp, a: &Operand, b: &Operand,clr_method:&CLRMethod) {
        self.process_operand(a,clr_method);
        self.process_operand(b,clr_method);
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
    fn read_pointer_of_type(&mut self, element_type:&VariableType){
        match element_type{
            VariableType::I32 => self.ops.push(BaseIR::LDIndIn(std::mem::size_of::<i32>() as u8)),
            VariableType::Ref(_) | VariableType::RefMut(_) =>self.ops.push(BaseIR::LDIndI),
            _=>todo!("Can't derference pointer of type {element_type:?}"),
        }
    }
    fn process_rvalue<'ctx>(
        &mut self,
        rvalue: &CompilerRValue<'ctx>,
        body: &Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        clr_method:&CLRMethod,
    ) {
        match rvalue {
            CompilerRValue::Use(operand) => self.process_operand(operand,clr_method),
            CompilerRValue::BinaryOp(binop, operands) => {
                let (a, b): (_, _) = (&operands.0, &operands.1);
               self.process_binop(*binop,a,b,clr_method);
            }
            CompilerRValue::Cast(
                CastKind::IntToInt
                | CastKind::FloatToFloat
                | CastKind::FloatToInt
                | CastKind::IntToFloat,
                operand,
                target,
            ) => {
                self.process_operand(operand,clr_method);
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
                    _ => todo!(
                        "Can't yet handle the aggregate of kind {kind:?} and operands:{operands:?}"
                    ),
                }
            }
            CompilerRValue::Repeat(_, _) => todo!("Can't yet initialize arrays!"),
            CompilerRValue::CopyForDeref(place) =>{
                if place.projection.len() == 1{
                    self.load(place.local.into(),clr_method);
                    self.read_pointer_of_type(&clr_method.get_type_of_local(place.local.into()).get_pointed_type().expect("Tried to deference a value that was neither a pointer nor a reference"));
                }
                else {
                    panic!("Can't handle non-trivial deference's yet, projection:{projection:?}!",projection = place.projection);
                }
            }
            _ => todo!("Can't yet handle a rvalue of type {rvalue:?}"),
        }
    }
     fn process_constant(&mut self, constant: ConstantKind,clr_method:&CLRMethod) {
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
    fn process_operand(&mut self, operand: &Operand,clr_method:&CLRMethod) {
        match operand {
            Operand::Copy(place) => self.load(place.local.into(),clr_method),
            //TODO:Do moves need to be treated any diffrently forom copies in the context of CLR?
            Operand::Move(place) => self.load(place.local.into(),clr_method),
            Operand::Constant(const_val) => {
                self.process_constant(const_val.literal,clr_method);
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
    fn load(&mut self, local: u32,clr_method:&CLRMethod) {
        self.ops.push(match clr_method.local_id_placement(local) {
            LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
        })
    }
}
