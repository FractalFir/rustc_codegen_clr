#![feature(rustc_private)]
extern crate rustc_abi;
extern crate rustc_middle;
extern crate rustc_driver;
use cilly::v2::Assembly;
use rustc_middle::ty::layout::HasTypingEnv;
use rustc_middle::ty::{Instance, PseudoCanonicalInput, TyCtxt};
pub struct MethodCompileCtx<'tcx, 'asm> {
    tcx: TyCtxt<'tcx>,
    method: Option<&'tcx rustc_middle::mir::Body<'tcx>>,
    method_instance: Instance<'tcx>,
    asm: &'asm mut Assembly,
}

impl std::ops::DerefMut for MethodCompileCtx<'_, '_> {
    #[allow(clippy::mut_mut)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asm
    }
}

impl<'asm> std::ops::Deref for MethodCompileCtx<'_, 'asm> {
    type Target = &'asm mut Assembly;

    fn deref(&self) -> &Self::Target {
        &self.asm
    }
}

impl<'tcx, 'asm> MethodCompileCtx<'tcx, 'asm> {
    #[must_use]
    /// Creates a [`MethodCompileCtx`] with a certain MIR body.
    pub fn with_body<'a: 'asm>(&'a mut self, body: &'tcx rustc_middle::mir::Body<'tcx>) -> Self {
        assert!(
            self.method.is_none(),
            "ERROR: attempt to change the body of a method compilation context"
        );
        Self {
            tcx: self.tcx,
            method: Some(body),
            method_instance: self.method_instance,
            asm: self.asm,
        }
    }
    pub fn new(
        tcx: TyCtxt<'tcx>,
        method: Option<&'tcx rustc_middle::mir::Body<'tcx>>,
        method_instance: Instance<'tcx>,
        asm: &'asm mut Assembly,
    ) -> Self {
        Self {
            tcx,
            method,
            method_instance,
            asm,
        }
    }
    pub fn tcx_and_asm(&mut self) -> (TyCtxt<'tcx>, &mut Assembly) {
        (self.tcx, self.asm)
    }
    /// Returns the type context this method is compiled in.
    #[must_use]
    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
    /// Returns the MIR body of this method is compiled.
    #[must_use]
    pub fn body(&self) -> &'tcx rustc_middle::mir::Body<'tcx> {
        self.method.unwrap()
    }
    #[must_use]
    /// Returns the Instance representing the current method
    pub fn instance(&self) -> Instance<'tcx> {
        self.method_instance
    }
    pub fn monomorphize<T: rustc_middle::ty::TypeFoldable<TyCtxt<'tcx>> + Clone>(
        &self,
        ty: T,
    ) -> T {
        self.instance()
            .instantiate_mir_and_normalize_erasing_regions(
                self.tcx(),
                rustc_middle::ty::TypingEnv::fully_monomorphized(),
                rustc_middle::ty::EarlyBinder::bind(ty),
            )
    }
    
    #[must_use]
    pub fn layout_of(
        &self,
        ty: rustc_middle::ty::Ty<'tcx>,
    ) -> rustc_middle::ty::layout::TyAndLayout<'tcx> {
        let ty = self.monomorphize(ty);
        self.tcx
            .layout_of(PseudoCanonicalInput {
                typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
                value: ty,
            })
            .expect("Could not get type layout!")
    }

    pub fn asm_mut<'s: 'a, 'a>(&'s mut self) -> &'a mut Assembly {
        self.asm
    }
    #[must_use]
    pub fn asm<'s: 'a, 'a>(&'s self) -> &'a Assembly {
        self.asm
    }
}
impl<'tcx> rustc_middle::ty::layout::HasTyCtxt<'tcx> for MethodCompileCtx<'tcx, '_> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}
impl rustc_abi::HasDataLayout for MethodCompileCtx<'_, '_> {
    fn data_layout(&self) -> &rustc_abi::TargetDataLayout {
        self.tcx.data_layout()
    }
}
impl<'tcx> HasTypingEnv<'tcx> for MethodCompileCtx<'tcx, '_> {
    fn typing_env(&self) -> rustc_middle::ty::TypingEnv<'tcx> {
        rustc_middle::ty::TypingEnv::fully_monomorphized()
    }
}
