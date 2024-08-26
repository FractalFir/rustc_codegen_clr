use cilly::{call_site::CallSite, cil_root::CILRoot, fn_sig::FnSig, v2::Int, Type};
use rustc_middle::ty::TyCtxt;

/// Returns the call site refering to the function malloc.
#[must_use]
pub fn malloc(ctx: TyCtxt) -> CallSite {
    CallSite::new(
        None,
        "malloc".into(),
        FnSig::new(
            &[Type::Int(Int::USize)],
            Type::Ptr(crate::r#type::c_void(ctx).into()),
        ),
        true,
    )
}
pub(crate) fn span_source_info(tcx: TyCtxt, span: rustc_span::Span) -> CILRoot {
    let (file, lstart, cstart, lend, mut cend) = tcx.sess.source_map().span_to_location_info(span);
    let file = file.map_or(String::new(), |file| {
        file.name
            .display(rustc_span::FileNameDisplayPreference::Local)
            .to_string()
    });
    if cstart >= cend {
        cend = cstart + 1;
    }
    CILRoot::source_info(
        &file,
        (lstart as u64)..(lend as u64),
        (cstart as u64)..(cend as u64),
    )
}
