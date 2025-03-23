use rustc_abi::VariantIdx;
use rustc_codegen_clr_type::r#type::escape_field_name;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::interpret::AllocId,
    ty::{ConstKind, GenericArg, Instance, List, PseudoCanonicalInput, Ty, TyCtxt, TyKind},
};

pub mod adt;
pub const CTOR_FN_NAME: &str = "rustc_clr_interop_managed_ctor";
pub const MANAGED_CALL_FN_NAME: &str = "rustc_clr_interop_managed_call";
pub const MANAGED_CALL_VIRT_FN_NAME: &str = "rustc_clr_interop_managed_call_virt";
pub const MANAGED_LD_LEN: &str = "rustc_clr_interop_managed_ld_len";
pub const MANAGED_LD_NULL: &str = "rustc_clr_interop_managed_ld_null";
pub const MANAGED_CHECKED_CAST: &str = "rustc_clr_interop_managed_checked_cast";
pub const MANAGED_IS_INST: &str = "rustc_clr_interop_managed_is_inst";
pub const MANAGED_LD_ELEM_REF: &str = "rustc_clr_interop_managed_ld_elem_ref";
pub fn is_function_magic(name: &str) -> bool {
    name.contains(CTOR_FN_NAME) || name.contains(MANAGED_CALL_FN_NAME)
}

// WARNING: this is *wrong*: For some reason, `Instance::try_resolve` should not operate on structs(why?), and this just silences the newly introduced warning.
pub fn instance_try_resolve<'tcx>(
    adt: DefId,
    tcx: TyCtxt<'tcx>,
    gargs: &'tcx List<GenericArg<'tcx>>,
) -> Instance<'tcx> {
    tcx.resolve_instance_raw(PseudoCanonicalInput {
        typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
        value: (adt, gargs),
    })
    .unwrap()
    .unwrap()
}

/// Gets the name of a field with index `idx`
pub fn field_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let field_def = adt_def
                .all_fields()
                .nth(idx as usize)
                .expect("Field index out of range.");
            escape_field_name(&field_def.name.to_string()).into()
        }
        TyKind::Tuple(_) => format!("Item{}", idx + 1).into(),
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}
/// Gets the name of a enum variant with index `idx`
pub fn variant_name(ty: Ty, idx: u32) -> crate::IString {
    match ty.kind() {
        TyKind::Adt(adt_def, _subst) => {
            let variant_def = &adt_def.variants()[VariantIdx::from_u32(idx)];
            variant_def.name.to_string().into()
        }
        _ => todo!("Can't yet get fields of typr {ty:?}"),
    }
}

/// Converts a generic argument to a boolean, and panics if it could not.
pub fn garag_to_bool<'tcx>(garg: GenericArg<'tcx>, _ctx: TyCtxt<'tcx>) -> bool {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");

    let kind = usize_const.kind();
    match kind {
        ConstKind::Value(val) => {
            let scalar = val
                .valtree
                .try_to_scalar_int()
                .expect("String const did not contain valid scalar!");
            let ty = val.ty;
            assert!(
                ty.is_bool(),
                "Generic argument was not a bool type! ty:{ty:?}"
            );
            scalar.to_uint(scalar.size()) != 0
        }
        _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
    }
}
/// This function returns the size of a type at the compile time. This should be used ONLY for handling constants. It currently assumes a 64 bit env
pub fn compiletime_sizeof<'tcx>(ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> u64 {
    let layout = tcx
        .layout_of(PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;
    layout.size.bytes()
}
/// Ensures that a type is morphic.
#[macro_export]
macro_rules! assert_morphic {
    ($ty:ident) => {
        let ty_kind = $ty.kind();
        debug_assert!(
            !matches!(ty_kind, TyKind::Alias(_, _)),
            "ERROR: NON MORPHIC TYPE(ALIAS TYPE) {ty:?} WHERE MORPHIC TYPE EXPECTED!",
            ty = $ty
        );
        debug_assert!(
            !matches!(ty_kind, TyKind::Param(_)),
            "ERROR: NON MORPHIC TYPE(GENERIC PARAM TYPE) {ty:?} WHERE MORPHIC TYPE EXPECTED!",
            ty = $ty
        );
    };
}

pub(crate) fn alloc_id_to_u64(alloc_id: AllocId) -> u64 {
    alloc_id.0.into()
}
pub fn is_fn_intrinsic<'tcx>(fn_ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    match fn_ty.kind() {
        TyKind::FnDef(did, _) => tcx.is_intrinsic(*did, tcx.item_name(*did)),
        TyKind::Closure(_, _) => false,
        _ => todo!("Can't get signature of {fn_ty}"),
    }
}
pub fn align_of<'tcx>(ty: rustc_middle::ty::Ty<'tcx>, tcx: TyCtxt<'tcx>) -> u64 {
    let layout = tcx
        .layout_of(PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: ty,
        })
        .expect("Can't get layout of a type.")
        .layout;

    let align = layout.align.abi;
    align.bytes()
}
