use crate::{statement::CodegenCtx, Assembly, BaseIR, FunctionSignature, IString, VariableType};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        interpret::ConstValue, Body, Constant, ConstantKind, Local, LocalDecl, Operand, Place,
        Statement, Terminator, TerminatorKind,
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
    pub(crate) fn remove_void_locals(&mut self){
        let void_locals:Box<[_]> = self.locals.iter().enumerate().filter(|(index,vartype)|**vartype == VariableType::Void).map(|(index,vartype)|index).collect();
        self.ops.iter_mut().for_each(|op|op.remove_void_local(&void_locals));
        //TODO: remove void locals propely
        self.locals.iter_mut().for_each(|ltype|if *ltype == VariableType::Void{*ltype = VariableType::I8});
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
    pub(crate) fn add_locals<'ctx>(
        &mut self,
        locals: &IndexVec<Local, LocalDecl<'ctx>>,
        tyctx: TyCtxt<'ctx>,
    ) {
        let mut new_locals: Vec<VariableType> = Vec::with_capacity(locals.len());
        for (local_id, local) in locals.iter().enumerate() {
            let placement = self.local_id_placement(local_id as u32);
            if let LocalPlacement::Var(_) = placement {
                new_locals.push(VariableType::from_ty(local.ty, tyctx));
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
        if crate::ALWAYS_INIT_LOCALS {
            format!("\t.locals init({locals}\n\t)").into()
        } else {
            format!("\t.locals ({locals}\n\t)").into()
        }
    }
    pub(crate) fn into_il_ir(&self) -> String {
        let output = self.sig.output().il_name();
        let mut arg_list = String::new();
        let mut arg_iter = self.sig.inputs.iter();
        match arg_iter.next() {
            Some(start) => arg_list.push_str(&start.arg_name()),
            None => (),
        }
        for arg in arg_iter {
            arg_list.push(',');
            arg_list.push_str(&arg.arg_name());
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
    pub(crate) fn new(sig: FunctionSignature, name: &str) -> Self {
        let name = if name.contains("main"){
            "main"
        }else{name};
        Self {
            locals: Vec::new(),
            sig,
            name: name.into(),
            ops: Vec::with_capacity(0x100),
            curr_bb: 0,
        }
    }
    pub(crate) fn local_id_placement(&self, local: u32) -> LocalPlacement {
        // I assume local 0 is supposed to be the return value. TODO: check if this is always the case.
        let argc = self.sig.inputs().len() as u32;
        if local == 0 {
            LocalPlacement::Var(0)
        } else if local - 1 < argc {
            LocalPlacement::Arg(local - 1)
        } else {
            LocalPlacement::Var(local - argc)
        }
    }
    pub(crate) fn add_statement<'ctx>(
        &mut self,
        statement: &Statement<'ctx>,
        body: &'ctx Body<'ctx>,
        tyctx: TyCtxt<'ctx>,
        asm: &Assembly,
    ) {
        if cfg!(debug_assertions) {
            println!("statement:{statement:?}");
            self.ops
                .push(BaseIR::DebugComment(format!("{statement:?}").into()));
        }
        self.ops.extend(crate::statement::handle_statement(
            statement, self, asm, body, tyctx,
        ));
    }
    pub(crate) fn call<'ctx>(
        &mut self,
        fn_type: &Ty<'ctx>,
        body: &'ctx Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        args: &[Operand<'ctx>],
        destination: &Place<'ctx>,
        asm: &Assembly,
    ) -> Vec<BaseIR> {
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
        let sig = FunctionSignature::from_poly_sig(fn_type.fn_sig(*tyctx), *tyctx)
            .expect("Can't get the function signature");
        let function_name = format!("{symbol}").into();
        let codegen_ctx = CodegenCtx::new(self, asm, body, *tyctx);
        let mut call = Vec::new();
        for arg in args {
            call.extend(crate::statement::handle_operand(arg, &codegen_ctx));
        }
        let is_void = sig.output.is_void();
        call.push(BaseIR::CallStatic { function_name, sig });
        // Hande
        if is_void {
            call
        } else {
            let (mut addr_calc, set_ops) = crate::projection::projection_set(
                destination,
                codegen_ctx.get_local_type(destination.local.into()),
                &codegen_ctx,
            );
            //let assigement = AsigmentTarget::from_placement(*destination, &self, asm);
            addr_calc.extend(call);
            addr_calc.extend(set_ops);
            addr_calc
        }
    }
    pub(crate) fn add_terminator<'ctx>(
        &mut self,
        terminator: &Terminator<'ctx>,
        body: &'ctx Body<'ctx>,
        tyctx: &TyCtxt<'ctx>,
        asm: &Assembly,
    ) {
        match &terminator.kind {
            TerminatorKind::Return => {
                self.ops.push(BaseIR::LDLoc(0));
                self.ops.push(BaseIR::Return);
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                for (value, target) in targets.iter() {
                    self.ops.extend(crate::statement::handle_operand(
                        discr,
                        &CodegenCtx::new(self, asm, body, *tyctx),
                    ));
                    self.ops.push(BaseIR::LDConstI64(value as i64));
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
                self.ops.extend(crate::statement::handle_operand(
                    cond,
                    &CodegenCtx::new(self, asm, body, *tyctx),
                ));
                self.ops
                    .push(BaseIR::LDConstI32(if *expected { 1 } else { 0 } as i32));
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
                            let call_ops = self.call(&fn_ty, body, tyctx, args, destination, asm);
                            self.ops.extend(call_ops);
                        } else {
                            panic!("Invalid function literal!");
                        }
                    }
                    _ => panic!("func must be const!"),
                }
            }
            TerminatorKind::Resume => todo!("Can't handle terminator kind resume!"),
            TerminatorKind::Terminate => todo!("Can't handle terminator kind Terminate!"),
            TerminatorKind::Unreachable => todo!("Can't handle terminator kind Unreachable!"),
            TerminatorKind::Drop { .. } => {
                //TODO: stop ignoreing drops!
                //todo!("Can't handle terminator kind Drop!")
            },
            TerminatorKind::Yield { .. } => todo!("Can't handle terminator kind Yield!"),
            _ => todo!("Unknown terminator type {terminator:?}"),
        }
    }
}
