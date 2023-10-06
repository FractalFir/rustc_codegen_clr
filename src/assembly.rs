use crate::cil_op::{CILOp, CallSite};
use crate::method::Modifier;
use crate::{
    access_modifier::AccessModifer, codegen_error::CodegenError, function_sig::FnSig,
    method::Method, r#type::Type, type_def::TypeDef,
};
use rustc_middle::mir::{mono::MonoItem, Local, LocalDecl};
use rustc_middle::ty::{Instance, ParamEnv, TyCtxt};
use std::collections::HashSet;
use std::panic::UnwindSafe;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Assembly {
    types: HashSet<TypeDef>,
    functions: HashSet<Method>,
    entrypoint: Option<CallSite>,
}
impl Assembly {
    pub fn empty() -> Self {
        Self {
            types: HashSet::new(),
            functions: HashSet::new(),
            entrypoint: None,
        }
    }
    pub fn join(self, other: Self) -> Self {
        let types = self.types.union(&other.types).cloned().collect();
        let functions = self.functions.union(&other.functions).cloned().collect();
        let entrypoint = self.entrypoint.or(other.entrypoint);
        Self {
            types,
            functions,
            entrypoint,
        }
    }
    pub fn add_fn<'tcx>(
        &mut self,
        instance: Instance<'tcx>,
        tcx: TyCtxt<'tcx>,
        name: &str,
    ) -> Result<(), CodegenError> {
        if crate::utilis::is_function_magic(name) {
            return Ok(());
        }
        // Get the MIR if it exisits. Othervise, return early.
        if !tcx.is_mir_available(instance.def_id()) {
            println!("function {instance:?} has no MIR. Skippping.");
            return Ok(());
        }
        let mir = tcx.optimized_mir(instance.def_id());
        // TODO: check if this is OK. It seems to work for now, but there may be some edge cases.
        let param_env = ParamEnv::empty();
        // Check if function is public or not.
        let access_modifier = AccessModifer::from_visibility(tcx.visibility(instance.def_id()));
        // Handle the function signature
        let sig = FnSig::from_poly_sig(&instance.ty(tcx, param_env).fn_sig(tcx), tcx)?;
        // Get locals
        let locals = locals_from_mir(&mir.local_decls, tcx, sig.inputs().len(), &instance);
        // Create method prototype
        let mut method = Method::new(access_modifier, vec![Modifier::Static], sig, name, locals);
        let mut ops = Vec::new();
        let mut last_bb_id = 0;
        let blocks = &(*mir.basic_blocks);
        for block_data in blocks {
            ops.push(CILOp::Label(last_bb_id));
            last_bb_id += 1;
            for statement in &block_data.statements {
                ops.extend(crate::statement::handle_statement(
                    statement, mir, tcx, mir, instance,
                ));
                //ops.push(CILOp::Comment(format!("{statement:?}").into()));
                //println!("ops:{ops:?}\n\n");
            }
            match &block_data.terminator {
                Some(term) => ops.extend(crate::terminator::handle_terminator(
                    term, mir, tcx, mir, instance,
                )),
                None => (),
            }
        }
        method.set_ops(ops);
        for local in &mir.local_decls {
            self.add_type(local.ty, tcx);
        }
        self.functions.insert(method);
        Ok(())
        //todo!("Can't add function")
    }
    pub fn add_method(&mut self, method: Method) {
        self.functions.insert(method);
    }
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.iter()
    }
    pub fn types(&self) -> impl Iterator<Item = &TypeDef> {
        self.types.iter()
    }
    pub fn add_type<'ctx>(&mut self, ty: rustc_middle::ty::Ty<'ctx>, tyctx: TyCtxt<'ctx>) {
        for type_def in TypeDef::from_ty(ty, tyctx) {
            self.types.insert(type_def);
        }
    }
    pub fn opt(&mut self) {
        let functions: HashSet<_> = self
            .functions
            .iter()
            .map(|method| {
                let mut method = method.clone();
                crate::opt::opt_method(&mut method);
                method
            })
            .collect();
        self.functions = functions;
    }
    pub fn add_typedef<'ctx>(&mut self, type_def: TypeDef) {
        self.types.insert(type_def);
    }
    pub fn add_item<'tcx>(
        &mut self,
        item: MonoItem<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> Result<(), CodegenError> {
        match item {
            MonoItem::Fn(instance) => {
                //let instance = crate::utilis::monomorphize(&instance,tcx);
                let symbol_name = crate::utilis::function_name(item.symbol_name(tcx));

                self.add_fn(instance, tcx, &symbol_name)
                    .expect("Could not add function!");

                Ok(())
            }
            _ => todo!("Unsupported item:\"{item:?}\"!"),
        }
    }
    pub fn set_entrypoint(&mut self, entrypoint: CallSite) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        self.functions
            .insert(crate::entrypoint::wrapper(&entrypoint));
        self.entrypoint = Some(entrypoint);
    }
}
fn locals_from_mir<'ctx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'ctx>>,
    tyctx: TyCtxt<'ctx>,
    argc: usize,
    method_instance: &Instance<'ctx>,
) -> Vec<Type> {
    let mut local_types: Vec<Type> = Vec::with_capacity(locals.len());
    for (local_id, local) in locals.iter().enumerate() {
        if local_id == 0 || local_id > argc {
            let ty = crate::utilis::monomorphize(method_instance, local.ty, tyctx);
            local_types.push(Type::from_ty(ty, tyctx));
        }
    }
    local_types
}
