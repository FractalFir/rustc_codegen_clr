#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_symbol_mangling;
pub mod adt;
pub mod r#type;
pub mod utilis;

use crate::r#type::get_type;
use cilly::Type;
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_middle::ty::Ty;
pub trait GetTypeExt<'tcx> {
    fn type_from_cache(&mut self, ty: Ty<'tcx>) -> Type;
}
impl<'tcx> GetTypeExt<'tcx> for MethodCompileCtx<'tcx, '_> {
    fn type_from_cache(&mut self, ty: Ty<'tcx>) -> Type {
        get_type(ty, self)
    }
}
