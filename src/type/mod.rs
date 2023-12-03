/// A representation of a primitve type or a reference.
pub mod r#type;
/// Contains a reperesentation of a non-primitve .NET type(class,struct)
pub mod type_def;

pub mod tycache;

pub use r#type::*;
use rustc_middle::ty::{GenericArg, IntTy, Ty, TyCtxt, TyKind, UintTy};
pub use tycache::*;
pub use type_def::*;
/* 
pub fn mangle_ty<'tyctx>(ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> std::borrow::Cow<'static, str> {
    match ty.kind() {
        TyKind::Bool => "B".into(),
        TyKind::Char => "C".into(),
        TyKind::Slice(inner) => format!("Sl{}", mangle_ty(*inner, tyctx)).into(),
        TyKind::Str => "Str".into(),
        TyKind::Uint(uint) => match uint {
            UintTy::U8 => "U8",
            UintTy::U16 => "U16",
            UintTy::U32 => "U32",
            UintTy::U64 => "U64",
            UintTy::U128 => "U128",
            UintTy::Usize => "USize",
        }
        .into(),
        TyKind::Int(int) => match int {
            IntTy::I8 => "I8",
            IntTy::I16 => "I16",
            IntTy::I32 => "I32",
            IntTy::I64 => "I64",
            IntTy::I128 => "I128",
            IntTy::Isize => "ISize",
        }
        .into(),
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
                "V".into()
            } else {
                format!(
                    "T{element_count}{elements}",
                    element_count = elements.len(),
                    elements = elements
                        .iter()
                        .map(|ele| mangle_ty(ele, tyctx).to_string())
                        .collect::<String>()
                )
                .into()
            }
        }
        TyKind::Array(element, length) => {
            let length = crate::utilis::try_resolve_const_size(length).unwrap();
            format!("A{length}{element}", element = mangle_ty(*element, tyctx)).into()
        }
        TyKind::Ref(_region, inner, _mut) => format!("Ref{}", mangle_ty(*inner, tyctx)).into(),
        TyKind::Adt(def, subst) => {
            let name = crate::utilis::adt_name(def, tyctx, subst);
            if is_name_magic(name.as_ref()) {
                todo!("Can't yet handle interop-related generic types.");
            }
            mangle_susbt(&name, subst, tyctx)
        }
        _ => todo!("Can't yet mangle type {ty:?}"),
    }
}
fn mangle_elem<'tyctx>(
    elem: &GenericArg<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
) -> std::borrow::Cow<'static, str> {
    if let Some(tpe) = elem.as_type() {
        return mangle_ty(tpe, tyctx);
    }
    if let Some(_tpe) = elem.as_const() {
        todo!("Can't mangle consts!");
    }
    "".into()
}
fn mangle_susbt<'tyctx>(
    name: &str,
    subst: &[GenericArg<'tyctx>],
    tyctx: TyCtxt<'tyctx>,
) -> std::borrow::Cow<'static, str> {
    let mut result: String = name.into();
    for generic in subst {
        result.push_str(&mangle_elem(generic, tyctx));
    }
    result.into()
}*/
