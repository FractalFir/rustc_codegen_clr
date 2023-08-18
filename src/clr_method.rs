use crate::assigment_target::AsigmentTarget;
use crate::{rvalue::RValue, BaseIR, FunctionSignature, IString, VariableType};
use rustc_index::IndexVec;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::Constant;
use rustc_middle::mir::Place;

use rustc_middle::mir::{Body, Local, LocalDecl};
use rustc_middle::{
    mir::{
        interpret::ConstValue, ConstantKind, Operand, Statement, StatementKind, Terminator,
        TerminatorKind,
    },
    ty::{Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
use serde::{Deserialize, Serialize};
macro_rules! sign_cast {
    ($var:ident,$src:ty,$dest:ty) => {
        (<$dest>::from_ne_bytes(($var as $src).to_ne_bytes()))
    };
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CLRMethod {
    ops: Vec<BaseIR>,
    locals: Vec<VariableType>,
    sig: FunctionSignature,
    name: IString,
    curr_bb: u32,
    //bbs:
}
pub(crate) enum LocalPlacement {
    Arg(u32),
    Var(u32),
}
impl CLRMethod {
    pub(crate) fn get_type_of_local(&self, local: u32) -> &VariableType {
        match self.local_id_placement(local) {
            LocalPlacement::Arg(index) => &self.sig.inputs[index as usize],
            LocalPlacement::Var(index) => &self.locals[index as usize],
        }
    }
    pub(crate) fn extend_ops(&mut self, ops: &[BaseIR]) {
        self.ops.extend(ops.iter().map(|ref_op| ref_op.clone()))
    }
    fn count_rws(&self, local: u32) -> (usize, usize) {
        let (mut read_count, mut write_count) = (0, 0);
        for op in &self.ops {
            if let BaseIR::LDLoc(curr_local) = op {
                if *curr_local == local {
                    read_count += 1
                };
            } else if let BaseIR::STLoc(curr_local) = op {
                if *curr_local == local {
                    write_count += 1
                };
            }
        }
        (read_count, write_count)
    }
    fn remove_useless_local_wr_combo(&mut self) {
        for index in 0..(self.ops.len() - 1) {
            let next_index = index + 1;
            if let BaseIR::STLoc(wloc) = self.ops[index] {
                if let BaseIR::LDLoc(rloc) = self.ops[next_index] {
                    if wloc != rloc {
                        continue;
                    }
                    let (reads, writes) = self.count_rws(wloc);
                    //TODO: use a better method to determine if STLoc && LDLoc combo has no side effects and can be removed
                    if reads != 1 || writes != 1 {
                        continue;
                    }
                    self.ops[index] = BaseIR::Nop;
                    self.ops[next_index] = BaseIR::Nop;
                }
            }
        }
    }
    fn prune_nops(&mut self) {
        self.ops = self
            .ops
            .iter()
            .filter(|op| **op != BaseIR::Nop)
            .cloned()
            .collect();
    }
    pub(crate) fn opt(&mut self) {
        self.remove_useless_local_wr_combo();
        self.prune_nops();
    }
    pub fn begin_bb(&mut self) {
        self.ops.push(BaseIR::BBLabel {
            bb_id: self.curr_bb,
        });
        self.curr_bb += 1;
    }
    fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn add_locals(&mut self, locals: &IndexVec<Local, LocalDecl>) {
        let mut new_locals: Vec<VariableType> = Vec::with_capacity(locals.len());
        for (local_id, local) in locals.iter().enumerate() {
            let placement = self.local_id_placement(local_id as u32);
            if let LocalPlacement::Var(_) = placement {
                new_locals.push(VariableType::from_ty(local.ty));
            }
        }
        self.locals = new_locals;
        //todo!();
    }

    pub(crate) fn locals_init(&self) -> IString {
        if self.locals.is_empty() {
            return "".into();
        }
        let mut locals = String::new();
        let mut locals_iter = self.locals.iter().enumerate();
        match locals_iter.next() {
            Some((index, first)) => locals.push_str(&format!(
                "\n\t\t[{index}] {loc_type}",
                loc_type = first.il_name()
            )),
            None => (),
        }
        for (index, local) in locals_iter {
            locals.push_str(&format!(
                ",\n\t\t[{index}] {loc_type}",
                loc_type = local.il_name()
            ))
        }
        format!("\t.locals init({locals}\n\t)").into()
    }
    pub(crate) fn into_il_ir(&self) -> String {
        let output = self.sig.output().il_name();
        let mut arg_list = String::new();
        let mut arg_iter = self.sig.inputs.iter();
        match arg_iter.next() {
            Some(start) => arg_list.push_str(&start.il_name()),
            None => (),
        }
        for arg in arg_iter {
            arg_list.push(',');
            arg_list.push_str(&arg.il_name());
        }
        let mut ops_ir = String::new();
        for op in &self.ops {
            ops_ir.push_str(&op.clr_ir());
        }
        format!(
            ".method public static {output} {name}({arg_list}){{\n{locals_init}\n{ops_ir}}}\n",
            name = self.name(),
            locals_init = self.locals_init()
        )
    }
    fn argc(&self) -> u32 {
        self.sig.inputs().len() as u32
    }
    fn has_return(&self) -> bool {
        true
    }

    pub(crate) fn new(sig: FunctionSignature, name: &str) -> Self {
        Self {
            locals: Vec::new(),
            sig,
            name: name.into(),
            ops: Vec::with_capacity(0x100),
            curr_bb: 0,
        }
    }
    fn var_live(&mut self, _local: u32) {
        //TODO: use variable lifetimes!
    }
    fn var_dead(&mut self, _local: u32) {
        //TODO: use variable lifetimes!
    }
    pub(crate) fn local_id_placement(&self, local: u32) -> LocalPlacement {
        // I assume local 0 is supposed to be the return value. TODO: check if this is always the case.
        if self.has_return() {
            if local == 0 {
                LocalPlacement::Var(0)
            } else if local - 1 < self.argc() {
                LocalPlacement::Arg(local - 1)
            } else {
                LocalPlacement::Var(local - self.argc())
            }
        } else {
            panic!("Can't handle void functions yet. Diagnose MIR to determine what happens to the return var(0)!");
        }
    }
    fn load(&mut self, local: u32) {
        self.ops.push(match self.local_id_placement(local) {
            LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
        })
    }
    fn store(&mut self, local: u32) {
        self.ops.push(match self.local_id_placement(local) {
            LocalPlacement::Arg(arg_id) => BaseIR::STArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::STLoc(var_id),
        })
    }
    fn process_constant(&mut self, constant: ConstantKind) {
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
    fn process_operand(&mut self, operand: &Operand) {
        match operand {
            Operand::Copy(place) => self.load(place.local.into()),
            //TODO:Do moves need to be treated any diffrently forom copies in the context of CLR?
            Operand::Move(place) => self.load(place.local.into()),
            Operand::Constant(const_val) => {
                self.process_constant(const_val.literal);
            }
        }
    }
    pub(crate) fn add_statement<'ctx>(
        &mut self,
        statement: &Statement<'ctx>,
        body: &Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
    ) {
        println!("statement:{statement:?}");
        match &statement.kind {
            StatementKind::Assign(asign_box) => {
                let (place, rvalue) = (asign_box.0, &asign_box.1);
                let rvalue = RValue::from_rvalue(rvalue, body, tyctx, self);
                AsigmentTarget::from_placement(place, &self).finalize(rvalue, self);
            }
            StatementKind::StorageLive(local) => {
                self.var_live((*local).into());
            }
            StatementKind::StorageDead(local) => {
                self.var_dead((*local).into());
            }
            _ => todo!("Unhanded statement:{:?}", statement.kind),
        }
    }
    pub(crate) fn load_constant_primitive(&mut self, var_type: &VariableType, value: u128) {
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
    pub(crate) fn call<'ctx>(
        &mut self,
        fn_type: &Ty<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        args: &[Operand],
        destination: &Place,
    ) {
        let instance = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind() {
            let env = ParamEnv::empty();
            let instance = Instance::resolve(*tyctx, env, *def_id, subst_ref)
                .expect("Error: could not resolve a call target due to an external error!")
                .expect("Error: could not resolve a call target!");
            instance
        } else {
            panic!("Trying to call a type which is not a function!");
        };
        let symbol = tyctx.symbol_name(instance);
        let sig = FunctionSignature::from_poly_sig(fn_type.fn_sig(*tyctx))
            .expect("Can't get the function signature");
        let function_name = format!("{symbol}").into();
        for arg in args {
            self.process_operand(arg);
        }
        // Hande
        if sig.output.is_void() {
            self.ops.push(BaseIR::CallStatic { function_name, sig });
        } else {
            let assigement = AsigmentTarget::from_placement(*destination, &self);
            assigement.finalize_with_ops(&[BaseIR::CallStatic { function_name, sig }], self);
        }
    }
    pub(crate) fn add_terminator<'ctx>(
        &mut self,
        terminator: &Terminator<'ctx>,
        body: &Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
    ) {
        match &terminator.kind {
            TerminatorKind::Return => {
                if self.has_return() {
                    self.load(0);
                    self.ops.push(BaseIR::Return);
                } else {
                    todo!("Can't yet return from a void method!");
                }
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                for (value, target) in targets.iter() {
                    self.process_operand(discr);
                    self.load_constant_primitive(
                        &VariableType::from_ty(discr.ty(body, *tyctx)),
                        value,
                    );
                    self.ops.push(BaseIR::BEq {
                        target: target.into(),
                    });
                }
                self.ops.push(BaseIR::GoTo {
                    target: (*targets.all_targets().last().unwrap()).into(),
                });
            }
            TerminatorKind::Goto { target } => {
                self.ops.push(BaseIR::GoTo {
                    target: (*target).into(),
                });
            }
            TerminatorKind::Assert {
                cond,
                expected,
                msg,
                target,
                unwind: _,
            } => {
                self.process_operand(cond);
                self.load_constant_primitive(&VariableType::Bool, if *expected { 1 } else { 0 });
                self.ops.push(BaseIR::BEq {
                    target: (*target).into(),
                });
                self.ops.push(BaseIR::LDConstString(format!("{msg:?}")));
                self.ops.push(BaseIR::NewObj {
                    ctor_fn: "void [System.Runtime]System.Exception::.ctor(string)".to_owned(),
                });
                self.ops.push(BaseIR::Throw);
                //todo!()
                //TODO: handle assertions!
            }
            TerminatorKind::Call {
                func,
                args,
                destination,
                target: _,
                unwind: _,
                fn_span: _,
                call_source: _,
            } => {
                //let fn_sig = FunctionSignature::from_poly_sig(func);
                match func {
                    Operand::Constant(fn_const) => {
                        let Constant {
                            span: _,
                            user_ty: _,
                            literal,
                        } = **fn_const;
                        if let ConstantKind::Val(ConstValue::ZeroSized, fn_ty) = literal {
                            assert!(
                                fn_ty.is_fn(),
                                "literal{literal:?} in call is not a function type!"
                            );
                            self.call(&fn_ty, tyctx, args, destination);
                        } else {
                            panic!("Invalid function literal!");
                        }
                    }
                    _ => panic!("func must be const!"),
                }
            }
            _ => todo!("Unknown terminator type {terminator:?}"),
        }
    }
}
