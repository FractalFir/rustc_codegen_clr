use crate::{
    base_ir::CallSite, statement::CodegenCtx, types::Type, Assembly, BaseIR, FunctionSignature,
    IString,
};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        interpret::ConstValue, Body, Constant, ConstantKind, Local, LocalDecl, Operand, Place,
        Statement, Terminator, TerminatorKind,
    },
    ty::{Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum MethodAttribute {
    EntryPoint,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CLRMethod {
    ops: Vec<BaseIR>,
    locals: Vec<Type>,
    sig: FunctionSignature,
    name: IString,
    curr_bb: u32,
    attrs: Vec<MethodAttribute>,
    //bbs:
}
#[derive(Debug, Clone)]
pub(crate) enum LocalPlacement {
    Arg(u32),
    Var(u32),
}
impl CLRMethod {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn sig(&self) -> &FunctionSignature {
        &self.sig
    }
    pub(crate) fn locals(&self) -> &[Type] {
        &self.locals
    }
    pub(crate) fn ops(&self) -> &[BaseIR] {
        &self.ops
    }
    pub(crate) fn from_raw(
        ops: &[BaseIR],
        locals: &[Type],
        name: &str,
        sig: FunctionSignature,
    ) -> Self {
        //TODO: calculate value of curr_bb.
        let curr_bb = 0;
        Self {
            ops: ops.into(),
            locals: locals.into(),
            name: name.into(),
            sig,
            curr_bb,
            attrs: Vec::new(),
        }
    }
    pub(crate) fn has_attribute(&self, attr: &MethodAttribute) -> bool {
        self.attrs.iter().any(|mattr| *attr == *mattr)
    }
    pub(crate) fn add_attribute(&mut self, attr: MethodAttribute) {
        self.attrs.push(attr)
    }
    pub(crate) fn get_type_of_local(&self, local: u32) -> &Type {
        match self.local_id_placement(local) {
            LocalPlacement::Arg(index) => &self.sig.inputs[index as usize],
            LocalPlacement::Var(index) => &self.locals[index as usize],
        }
    }
    pub(crate) fn remove_void_locals(&mut self) {
        let void_locals: Box<[_]> = self
            .locals
            .iter()
            .enumerate()
            .filter(|(_index, vartype)| **vartype == Type::Void)
            .map(|(index, _vartype)| index)
            .collect();
        self.ops
            .iter_mut()
            .for_each(|op| op.remove_void_local(&void_locals));
        //TODO: remove void locals propely
        self.locals.iter_mut().for_each(|ltype| {
            if *ltype == Type::Void {
                *ltype = Type::I8;
            }
        });
    }
    fn count_rws(&self, local: u32) -> (usize, usize) {
        let (mut read_count, mut write_count) = (0, 0);
        for op in &self.ops {
            if let BaseIR::LDLoc(curr_local) = op {
                if *curr_local == local {
                    read_count += 1;
                };
            } else if let BaseIR::STLoc(curr_local) = op {
                if *curr_local == local {
                    write_count += 1;
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
        self.ops.retain(|op| *op != BaseIR::Nop);
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
    pub(crate) fn add_locals<'ctx>(
        &mut self,
        locals: &IndexVec<Local, LocalDecl<'ctx>>,
        tyctx: TyCtxt<'ctx>,
    ) {
        let mut new_locals: Vec<Type> = Vec::with_capacity(locals.len());
        for (local_id, local) in locals.iter().enumerate() {
            let placement = self.local_id_placement(local_id as u32);
            if let LocalPlacement::Var(_) = placement {
                new_locals.push(Type::from_ty(&local.ty, &tyctx));
            }
        }
        self.locals = new_locals;
        //todo!();
    }
    pub(crate) fn new(sig: FunctionSignature, name: &str) -> Self {
        Self {
            locals: Vec::new(),
            sig,
            name: name.into(),
            ops: Vec::with_capacity(0x100),
            curr_bb: 0,
            attrs: Vec::new(),
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
        let signature = FunctionSignature::from_poly_sig(fn_type.fn_sig(*tyctx), *tyctx)
            .expect("Can't get the function signature");
        let function_name = format!("{symbol}").into();
        let codegen_ctx = CodegenCtx::new(self, asm, body, *tyctx);
        let mut call = Vec::new();
        for arg in args {
            call.extend(crate::statement::handle_operand(arg, &codegen_ctx));
        }
        let is_void = signature.output.is_void();
        call.push(BaseIR::Call(Box::new(CallSite {
            owner: None,
            name: function_name,
            signature,
            is_static: true,
        })));
        // Hande
        if is_void {
            call
        } else {
            crate::codegen::place::place_setter_ops(destination, &codegen_ctx, call)
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
                self.ops.push(BaseIR::LDConstI32(i32::from(*expected)));
                self.ops.push(BaseIR::BEq {
                    target: (*target).into(),
                });
                self.ops
                    .push(BaseIR::LDConstString(format!("{msg:?}").into()));
                //  ctor_fn: "void [System.Runtime](string)".to_owned(),
                self.ops.push(BaseIR::NewObj(Box::new(CallSite {
                    owner: Some(Type::ExternType {
                        asm: "System.Runtime".into(),
                        name: "System.Exception".into(),
                    }),
                    name: ".ctor".into(),
                    is_static: false,
                    signature: FunctionSignature::new(
                        &[Type::ExternType {
                            asm: "System.Runtime".into(),
                            name: "System.String".into(),
                        }],
                        &Type::ExternType {
                            asm: "System.Runtime".into(),
                            name: "System.Exception".into(),
                        },
                    ),
                })));
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
            //TerminatorKind::Resume => todo!("Can't handle terminator kind resume!"),
            //TerminatorKind::Terminate => todo!("Can't handle terminator kind Terminate!"),
            //TerminatorKind::Unreachable => todo!("Can't handle terminator kind Unreachable!"),
            TerminatorKind::Drop { .. } => {
                //TODO: stop ignoreing drops!
                todo!("Can't handle terminator kind Drop!")
            }
            TerminatorKind::Yield { .. } => todo!("Can't handle terminator kind Yield!"),
            _ => todo!("Unknown terminator type {terminator:?}"),
        }
    }
}
