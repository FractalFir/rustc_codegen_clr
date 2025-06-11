#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_abi;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_middle;
use cilly::cil_node::V1Node;
use cilly::cil_root::V1Root;
use cilly::{ClassRef, Float, Int, Interned};
use cilly::{Const, Type};
use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_codegen_clr_type::utilis::pointer_to_is_fat;

use rustc_middle::mir::Place;

mod address;
mod body;
mod get;
mod set;
pub use address::*;
pub use body::*;
pub use get::*;
use rustc_middle::ty::{FloatTy, IntTy, Ty, TyKind, UintTy};
pub use set::*;

fn slice_head<T>(slice: &[T]) -> (&T, &[T]) {
    assert!(!slice.is_empty());
    let last = &slice[slice.len() - 1];
    (last, &slice[..(slice.len() - 1)])
}
fn pointed_type(ty: PlaceTy) -> Ty {
    if let PlaceTy::Ty(ty) = ty {
        if let TyKind::Ref(_region, inner, _mut) = ty.kind() {
            *inner
        } else if let TyKind::RawPtr(ty, _) = ty.kind() {
            *ty
        } else {
            panic!("{ty:?} is not a pointer type!");
        }
    } else {
        panic!("Can't dereference enum variant!");
    }
}
fn body_ty_is_by_address<'tcx>(last_ty: Ty<'tcx>, ctx: &mut MethodCompileCtx<'tcx, '_>) -> bool {
    match *last_ty.kind() {
        // True for non-0 tuples
        TyKind::Tuple(elements) => !elements.is_empty(),

        //TODO: check if slices are handled propely
        TyKind::Adt(_, _)
        | TyKind::Closure(_, _)
        | TyKind::Coroutine(_, _)
        | TyKind::Array(_, _)
        | TyKind::Slice(_)
        | TyKind::Str => true,

        TyKind::Int(_) | TyKind::Float(_) | TyKind::Uint(_) | TyKind::Bool | TyKind::Char => false,
        TyKind::Ref(_, ty, _) | TyKind::RawPtr(ty, _) => {
            pointer_to_is_fat(ty, ctx.tcx(), ctx.instance())
        }
        _ => todo!(
            "TODO: body_ty_is_by_address does not support type {last_ty:?} kind:{kind:?}",
            kind = last_ty.kind()
        ),
    }
}

/// Given a type `derefed_type`, it retuns a set of instructions to get a value behind a pointer to `derefed_type`.
pub fn deref_op<'tcx>(
    derefed_type: PlaceTy<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ptr: Interned<cilly::v2::CILNode>,
) -> Interned<cilly::v2::CILNode> {
    let ptr = Box::new(ptr);
    let res = if let PlaceTy::Ty(derefed_type) = derefed_type {
        let derefed_type = ctx.type_from_cache(derefed_type);
        ctx.load(*ptr, derefed_type)
    } else {
        todo!("Can't dereference enum variants!")
    };
    res
}

/// Returns the ops for getting the address of a given place.
pub fn place_address<'a>(place: &Place<'a>, ctx: &mut MethodCompileCtx<'a, '_>) -> V1Node {
    let place_ty = place.ty(ctx.body(), ctx.tcx());
    let place_ty = ctx.monomorphize(place_ty).ty;

    let layout = ctx.layout_of(place_ty);
    if layout.is_zst() {
        let place_type = ctx.type_from_cache(place_ty);
        return V1Node::V2(ctx.alloc_node(Const::USize(layout.align.abi.bytes())))
            .cast_ptr(ctx.nptr(place_type));
    }
    if place.projection.is_empty() {
        let loc_ty = ctx.monomorphize(ctx.body().local_decls[place.local].ty);
        if pointer_to_is_fat(loc_ty, ctx.tcx(), ctx.instance()) {
            V1Node::V2(local_get(place.local.as_usize(), ctx.body(), ctx))
        } else {
            V1Node::V2(local_address(place.local.as_usize(), ctx.body(), ctx))
        }
    } else {
        let (mut addr_calc, mut ty) = local_body(place.local.as_usize(), ctx);

        ty = ctx.monomorphize(ty);
        let mut ty = ty.into();

        let (head, body) = slice_head(place.projection);
        for elem in body {
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx, addr_calc.clone());
            ty = curr_ty.monomorphize(ctx);
            addr_calc = curr_ops;
        }
        let addr_calc = V1Node::V2(addr_calc);
        address::place_elem_address(head, ty, ctx, place_ty, addr_calc)
    }
}
/// Should be only used in certain builit-in features. For unsized types, returns the address of the fat pointer, not the address contained within it.
pub fn place_address_raw<'a>(place: &Place<'a>, ctx: &mut MethodCompileCtx<'a, '_>) -> V1Node {
    let place_ty = place.ty(ctx.body(), ctx.tcx());
    let place_ty = ctx.monomorphize(place_ty).ty;

    let layout = ctx.layout_of(place_ty);
    if layout.is_zst() {
        return V1Node::V2(ctx.alloc_node(Const::USize(layout.align.abi.bytes())));
    }
    if place.projection.is_empty() {
        V1Node::V2(local_address(place.local.as_usize(), ctx.body(), ctx))
    } else if place.projection.len() == 1
        && matches!(
            slice_head(place.projection).0,
            rustc_middle::mir::PlaceElem::Deref
        )
    {
        return V1Node::V2(local_address(place.local.as_usize(), ctx.body(), ctx));
    } else {
        let (mut addr_calc, mut ty) = local_body(place.local.as_usize(), ctx);

        ty = ctx.monomorphize(ty);
        let mut ty = ty.into();

        let (head, body) = slice_head(place.projection);
        for elem in body {
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx, addr_calc.clone());
            ty = curr_ty.monomorphize(ctx);
            addr_calc = curr_ops;
        }
        let addr_calc = V1Node::V2(addr_calc);
        address::place_elem_address(head, ty, ctx, place_ty, addr_calc)
    }
}
pub fn place_set<'tcx>(
    place: &Place<'tcx>,
    value_calc: V1Node,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Root {
    if place.projection.is_empty() {
        set::local_set(place.local.as_usize(), ctx.body(), value_calc)
    } else {
        let (mut addr_calc, ty) = local_body(place.local.as_usize(), ctx);

        let mut ty: PlaceTy = ty.into();
        ty = ty.monomorphize(ctx);

        let (head, body) = slice_head(place.projection);
        for elem in body {
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx, addr_calc);
            ty = curr_ty.monomorphize(ctx);
            addr_calc = curr_ops;
        }
        //
        ty = ty.monomorphize(ctx);
        let addr_calc = V1Node::V2(addr_calc);
        place_elem_set(head, ty, ctx, addr_calc, value_calc)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PlaceTy<'tcx> {
    Ty(Ty<'tcx>),
    EnumVariant(Ty<'tcx>, u32),
}
impl<'tcx> From<Ty<'tcx>> for PlaceTy<'tcx> {
    fn from(ty: Ty<'tcx>) -> Self {
        Self::Ty(ty)
    }
}
impl<'tcx> PlaceTy<'tcx> {
    pub fn monomorphize(&self, ctx: &mut MethodCompileCtx<'tcx, '_>) -> Self {
        match self {
            Self::Ty(inner) => Self::Ty(ctx.monomorphize(*inner)),
            Self::EnumVariant(enm, variant) => Self::EnumVariant(ctx.monomorphize(*enm), *variant),
        }
    }
    pub fn as_ty(&self) -> Option<Ty<'tcx>> {
        match self {
            Self::Ty(inner) => Some(*inner),
            Self::EnumVariant(..) => None,
        }
    }
    /// Returns the kind of the underlyting Ty.
    pub fn kind(&self) -> &TyKind<'tcx> {
        match self {
            Self::Ty(ty) => ty.kind(),
            //TODO: find a better way to get the emum variant!
            Self::EnumVariant(ty, _variant) => ty.kind(),
        }
    }
}
