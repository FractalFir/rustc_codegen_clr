use crate::{types::Type, CLRMethod, FunctionSignature, IString};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{mono::MonoItem, Local, LocalDecl},
    ty::{Instance, ParamEnv, Ty, TyCtxt},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Serialize, Deserialize)]
pub(crate) struct Assembly {
    methods: Vec<CLRMethod>,
    name: IString,
    types: HashSet<Type>,
    size_t: u8,
}
impl Assembly {
    pub(crate) fn types(&self) -> impl Iterator<Item = &Type> {
        self.types.iter()
    }
    pub(crate) fn methods(&self) -> &[CLRMethod] {
        &self.methods
    }
    pub(crate) fn add_type<'ctx>(&mut self, ty: Ty<'ctx>, tyctx: &TyCtxt<'ctx>) {
        self.types.insert(Type::from_ty(&ty, tyctx));
    }
    pub(crate) fn add_types_from_locals<'ctx>(
        &mut self,
        locals: &IndexVec<Local, LocalDecl<'ctx>>,
        tyctx: &TyCtxt<'ctx>,
    ) {
        for local in locals {
            self.add_type(local.ty, tyctx);
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn new(name: &str) -> Self {
        let name: String = name.chars().take_while(|c| *c != '.').collect();
        let name = name.replace('-', "_");
        Self {
            methods: Vec::with_capacity(0x100),
            types: HashSet::with_capacity(0x100),
            name: name.into(),
            size_t: 8,
        }
    }
    pub(crate) fn add_fn<'tcx>(&mut self, instance: Instance<'tcx>, tcx: TyCtxt<'tcx>, name: &str) {
        // TODO: figure out: What should it be???
        let param_env = ParamEnv::empty();

        let def_id = instance.def_id();
        let mir = tcx.optimized_mir(def_id);
        let blocks = &(*mir.basic_blocks);
        let sig = instance.ty(tcx, param_env).fn_sig(tcx);
        let mut clr_method = CLRMethod::new(
            FunctionSignature::from_poly_sig(sig, tcx)
                .expect("Could not resolve the function signature"),
            name,
        );
        self.add_types_from_locals(&mir.local_decls, &tcx);
        clr_method.add_locals(&mir.local_decls, tcx);
        for block_data in blocks {
            clr_method.begin_bb();
            for statement in &block_data.statements {
                clr_method.add_statement(statement, mir, tcx, self);
            }
            match &block_data.terminator {
                Some(term) => clr_method.add_terminator(term, mir, &tcx, self),
                None => (),
            }
        }
        clr_method.remove_void_locals();
        clr_method.opt();
        //println!("clr_method:{clr_method:?}");
        //println!("instance:{instance:?}\n");
        //println!("types:{types:?}", types = self.types);
        self.methods.push(clr_method);
    }
    pub(crate) fn add_item<'tcx>(&mut self, item: MonoItem<'tcx>, tcx: TyCtxt<'tcx>) {
        println!("adding item:{}", item.symbol_name(tcx));

        match item {
            MonoItem::Fn(instance) => {
                self.add_fn(instance, tcx, &format!("{}", item.symbol_name(tcx)));
            }
            _ => todo!("Unsupported item:\"{item:?}\"!"),
        }
    }
    pub(crate) fn link(&mut self, other: Self) {
        //TODO: do linking.
        self.methods.extend_from_slice(&other.methods);
        self.types.extend(other.types);
    }
}
