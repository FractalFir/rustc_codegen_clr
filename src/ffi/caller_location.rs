/*
pub fn add_caller_location(asm: &mut Assembly, tyctx: TyCtxt, cache: &mut TyCache) {
    let panic_location = tyctx
        .get_lang_items(())
        .get(rustc_hir::lang_items::LangItem::PanicLocation)
        .expect("ERROR: could not find PanicLocation!");
    let adt_def = tyctx.adt_def(panic_location);
    let panic_location = Ty::new(
        tyctx,
        TyKind::Adt(adt_def, List::identity_for_item(tyctx, panic_location)),
    );
    let panic_location = cache.type_from_cache(panic_location, tyctx, None);
    let mut caller_location = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[], &panic_location),
        "caller_location",
        vec![(Some("panic_location".into()), panic_location.clone())],
    );
    caller_location.set_ops(vec![
        CILOp::LDLocA(0),
        CILOp::InitObj(panic_location.into()),
        CILOp::LDLoc(0),
        CILOp::Ret,
    ]);
    asm.add_method(caller_location);
}*/
