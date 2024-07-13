use crate::r#type::{TyCache, Type};
use cilly::cil_node::{CILNode, ValidationContext};
use rustc_middle::ty::{Instance, ParamEnv, TyCtxt};
pub struct MethodCompileCtx<'tcx, 'validator, 'type_cache> {
    tcx: TyCtxt<'tcx>,
    method: &'tcx rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
    validator: ValidationContext<'validator>,
    type_cache: &'type_cache mut TyCache,
}

impl<'tcx, 'validator, 'type_cache> MethodCompileCtx<'tcx, 'validator, 'type_cache> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        method: &'tcx rustc_middle::mir::Body<'tcx>,
        method_instance: Instance<'tcx>,
        validator: ValidationContext<'validator>,
        type_cache: &'type_cache mut TyCache,
    ) -> Self {
        Self {
            tcx,
            method,
            method_instance,
            validator,
            type_cache,
        }
    }
    pub fn slice_ty(&mut self, inner: rustc_middle::ty::Ty<'tcx>) -> Type {
        self.type_cache
            .slice_ty(inner, self.tcx, self.method_instance)
    }
    pub fn slice_ref_to(&mut self, inner: rustc_middle::ty::Ty<'tcx>) -> Type {
        self.type_cache
            .slice_ref_to(self.tcx, inner, self.method_instance)
    }
    /// Returns the type context this method is compiled in.
    #[must_use]
    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
    /// Returns the MIR body of this method is compiled.
    #[must_use]
    pub fn body(&self) -> &'tcx rustc_middle::mir::Body<'tcx> {
        self.method
    }
    #[must_use]
    /// Returns the Instance representing the current method
    pub fn instance(&self) -> Instance<'tcx> {
        self.method_instance
    }
    /// Returns a Type cache.
    pub fn type_cache(&mut self) -> &mut &'type_cache mut TyCache {
        &mut self.type_cache
    }
    #[must_use]
    pub fn validator(&self) -> ValidationContext<'validator> {
        self.validator
    }
    pub fn monomorphize<T: rustc_middle::ty::TypeFoldable<TyCtxt<'tcx>> + Clone>(
        &self,
        ty: T,
    ) -> T {
        self.instance()
            .instantiate_mir_and_normalize_erasing_regions(
                self.tcx(),
                ParamEnv::reveal_all(),
                rustc_middle::ty::EarlyBinder::bind(ty),
            )
    }
    pub fn assert_raw_pointer_type(&self, ptr: &CILNode, node_from: &impl std::fmt::Debug) {
        let ptr_tpe = match ptr.validate(self.validator(), None) {
            Ok(ptr_tpe) => ptr_tpe,
            Err(err) => {
                panic!("VALIDATION falied: {err}. ops create from {node_from:?} weren't valid.")
            }
        };
        match ptr_tpe{
            Type::USize | Type::ISize | Type::DelegatePtr(_) | Type::Ptr(_) => (),
            _=>panic!("VALIDATION failed. {ptr_tpe:?} is not a raw pointer type. It is the result of {ptr:?}, compiled from MIR item {node_from:?}")  
        }
    }
    pub fn assert_fat_pointer_type(&self, ptr: &CILNode, node_from: &impl std::fmt::Debug) {
        let ptr_tpe = match ptr.validate(self.validator(), None) {
            Ok(ptr_tpe) => ptr_tpe,
            Err(err) => {
                panic!("VALIDATION falied: {err}. ops create from {node_from:?} weren't valid.")
            }
        };
        match ptr_tpe{
            Type::DotnetType(tpe) if tpe.name_path().contains("FatPtr")=> (),
            _=>panic!("VALIDATION failed. {ptr_tpe:?} is not a raw pointer type. It is the result of {ptr:?}, compiled from MIR item {node_from:?}")  
        }
    }
    pub fn type_from_cache(&mut self, ty: rustc_middle::ty::Ty<'tcx>) -> Type {
        self.type_cache
            .type_from_cache(ty, self.tcx, self.method_instance)
    }
    #[must_use]
    pub fn layout_of(
        &self,
        ty: rustc_middle::ty::Ty<'tcx>,
    ) -> rustc_middle::ty::layout::TyAndLayout<'tcx> {
        let ty = self.monomorphize(ty);
        self.tcx
            .layout_of(rustc_middle::ty::ParamEnvAnd {
                param_env: ParamEnv::reveal_all(),
                value: ty,
            })
            .expect("Could not get type layout!")
    }
}
impl<'tcx> rustc_middle::ty::layout::HasTyCtxt<'tcx> for MethodCompileCtx<'tcx, '_, '_> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}
impl rustc_abi::HasDataLayout for MethodCompileCtx<'_, '_, '_> {
    fn data_layout(&self) -> &rustc_abi::TargetDataLayout {
        self.tcx.data_layout()
    }
}
impl<'tcx> rustc_middle::ty::layout::HasParamEnv<'tcx> for MethodCompileCtx<'tcx, '_, '_> {
    fn param_env(&self) -> ParamEnv<'tcx> {
        ParamEnv::reveal_all()
    }
}
