use cilly::cil_root::CILRoot;
use rustc_middle::ty::TyCtxt;

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
