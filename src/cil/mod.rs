use cilly::{call_site::CallSite, cil_root::CILRoot, fn_sig::FnSig, Type};
use rustc_middle::ty::TyCtxt;

/// Returns the call site refering to the function malloc.
#[must_use]
pub fn malloc(ctx: TyCtxt) -> CallSite {
    CallSite::new(
        None,
        "malloc".into(),
        FnSig::new(&[Type::USize], Type::Ptr(crate::r#type::c_void(ctx).into())),
        true,
    )
}
pub(crate) fn span_source_info(tyctx: TyCtxt, span: rustc_span::Span) -> CILRoot {
    let (file, lstart, cstart, lend, mut cend) =
        tyctx.sess.source_map().span_to_location_info(span);
    let file = file
        .map(|file| {
            file.name
                .display(rustc_span::FileNameDisplayPreference::Local)
                .to_string()
        })
        .unwrap_or("".to_string());
    if cstart >= cend {
        cend = cstart + 1;
    }
    CILRoot::source_info(
        &file,
        (lstart as u64)..(lend as u64),
        (cstart as u64)..(cend as u64),
    )
}
