use crate::IString;
use cilly::{
    basic_block::BasicBlock,
    call,
    cil_node::CILNode,
    cil_root::CILRoot,
    cil_tree::CILTree,
    cilnode::MethodKind,
    method::{Method, MethodType},
    utilis::encode,
    Access, Assembly, Const, FnSig, Int, IntoAsmIndex, MethodDef, MethodDefIdx, MethodRef,
    StaticFieldDesc, Type,
};
use rustc_codegen_clr_call::CallInfo;
use rustc_codegen_clr_ctx::function_name;
pub use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_middle::{
    mir::interpret::{AllocId, Allocation, GlobalAlloc},
    ty::{Instance, List, Ty, TyCtxt, TypingEnv},
};
use rustc_span::def_id::DefId;
pub fn static_ty<'tcx>(def_id: DefId, tcx: TyCtxt<'tcx>) -> Ty<'tcx> {
    tcx.type_of(def_id).instantiate_identity()
}
pub fn add_static(def_id: DefId, ctx: &mut MethodCompileCtx<'_, '_>) -> CILNode {
    let main_module_id = ctx.main_module();
    let alloc = ctx.tcx().eval_static_initializer(def_id).unwrap();
    let attrs = ctx.tcx().codegen_fn_attrs(def_id);

    let thread_local = attrs
        .flags
        .contains(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags::THREAD_LOCAL);
    let align = alloc.0.align.bytes().max(1);
    let ty = static_ty(def_id, ctx.tcx());
    let tpe = ctx.type_from_cache(ty);
    assert_eq!(align, crate::utilis::align_of(ty, ctx.tcx()));
    assert!(ty.is_sized(ctx.tcx(), TypingEnv::fully_monomorphized()));
    let symbol: String = ctx
        .tcx()
        .symbol_name(Instance::new(def_id, List::empty()))
        .to_string();
    let sfld = ctx.add_static(
        tpe,
        symbol.clone(),
        thread_local,
        main_module_id,
        None,
        false,
    );
    let ptr = ctx.alloc_node(cilly::v2::CILNode::LdStaticFieldAdress(sfld));
    let ptr = ctx.cast_ptr(ptr, Int::U8);
    let ptr = CILNode::V2(ptr);
    let initialzer = allocation_initializer_method(&alloc.0, &symbol, ctx, ptr.clone(), true);
    let root = ctx.alloc_root(cilly::CILRoot::call(*initialzer, []));

    if thread_local {
        ctx.add_tcctor(&[root]);
    } else {
        ctx.add_cctor(&[root]);
    }

    ptr
}
/// Adds a static field and initialized for allocation represented by `alloc_id`.
pub fn add_allocation(alloc_id: u64, ctx: &mut MethodCompileCtx<'_, '_>) -> CILNode {
    let uint8_ptr = ctx.nptr(Type::Int(Int::U8));
    let main_module_id = ctx.main_module();
    let (thread_local, const_allocation) = match ctx
        .tcx()
        .global_alloc(AllocId(alloc_id.try_into().expect("0 alloc id?")))
    {
        GlobalAlloc::Memory(alloc) => (false, alloc),
        GlobalAlloc::Static(def_id) => return add_static(def_id, ctx),
        GlobalAlloc::VTable(..) => {
            //TODO: handle VTables
            let alloc_fld: IString = format!("al_{alloc_id:x}").into();

            let field_desc = StaticFieldDesc::new(
                *ctx.main_module(),
                ctx.alloc_string(alloc_fld.clone()),
                uint8_ptr,
            );
            ctx.add_static(uint8_ptr, alloc_fld, false, main_module_id, None, false);
            return CILNode::LDStaticField(Box::new(field_desc));
        }
        GlobalAlloc::Function { .. } => {
            //TODO: handle constant functions
            let alloc_fld: IString = format!("al_{alloc_id:x}").into();
            let field_desc = StaticFieldDesc::new(
                *ctx.main_module(),
                ctx.alloc_string(alloc_fld.clone()),
                uint8_ptr,
            );
            ctx.add_static(uint8_ptr, alloc_fld, false, main_module_id, None, false);

            return CILNode::LDStaticField(Box::new(field_desc));
            //todo!("Function/Vtable allocation.");
        }
    };

    let const_allocation = const_allocation.inner();

    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let align = const_allocation.align.bytes().max(1);
    if const_allocation.len() == 0 {
        return CILNode::V2(ctx.alloc_node(Const::USize(align)));
    }
    // Alloc ids are *not* unique across all crates. Adding the hash here ensures we don't overwrite allocations during linking
    // TODO:consider using something better here / making the hashes stable.
    let byte_hash = calculate_hash(&bytes);
    match (align, bytes.len()) {
        // Assumes this constant is not a pointer.
        (0..=1, len @ 1) | (0..=2, len @ 1..=2) | (0..=4, len @ 1..=4) | (0..=8, len @ 1..=16)
            if len <= ctx.max_static_size() =>
        {
            let alloc_name: IString = format!(
                "s_{}_{}_{}_{thread_local}",
                encode(alloc_id),
                encode(byte_hash),
                const_allocation.len()
            )
            .into();
            let name = ctx.alloc_string(alloc_name.clone());
            let tpe: Int = Int::from_size_sign(
                u8::try_from(bytes.len()).unwrap().next_power_of_two(),
                false,
            );
            let field_desc = StaticFieldDesc::new(*ctx.main_module(), name, cilly::Type::Int(tpe));
            // Currently, all static fields are in one module. Consider spliting them up.
            let main_module = ctx.class_mut(main_module_id);
            if main_module.has_static_field(name, field_desc.tpe()) {
                return CILNode::AddressOfStaticField(Box::new(field_desc));
            }
            let cst: Const = tpe.from_bytes(bytes);
            let field = ctx.alloc_sfld(field_desc);

            let val = ctx.alloc_node(cst);
            let mut roots = if thread_local || len > 8 {
                ctx.add_static(
                    cilly::Type::Int(tpe),
                    &*alloc_name,
                    thread_local,
                    main_module_id,
                    None,
                    false,
                );
                vec![ctx.alloc_root(cilly::CILRoot::SetStaticField { field, val })]
            } else {
                ctx.add_static(
                    cilly::Type::Int(tpe),
                    &*alloc_name,
                    thread_local,
                    main_module_id,
                    Some(cst),
                    false,
                );
                vec![]
            };

            let addr = CILNode::AddressOfStaticField(Box::new(field_desc));
            for (offset, prov) in const_allocation.provenance().ptrs().iter() {
                let offset = u32::try_from(offset.bytes_usize()).unwrap();
                // Check if this allocation is a function
                let reloc_target_alloc = ctx.tcx().global_alloc(prov.alloc_id());
                if let GlobalAlloc::Function {
                    instance: finstance,
                } = reloc_target_alloc
                {
                    // If it is a function, patch its pointer up.

                    let call_info = CallInfo::sig_from_instance_(finstance, ctx);
                    let function_name = function_name(ctx.tcx().symbol_name(finstance));
                    let mref = MethodRef::new(
                        *ctx.main_module(),
                        ctx.alloc_string(function_name),
                        ctx.alloc_sig(call_info.sig().clone()),
                        MethodKind::Static,
                        vec![].into(),
                    );
                    let st_ind = &CILRoot::STIndISize(
                        (addr.clone() + CILNode::V2(ctx.alloc_node(Const::USize(offset.into()))))
                            .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                        CILNode::LDFtn(ctx.alloc_methodref(mref)).cast_ptr(Type::Int(Int::USize)),
                    );
                    let root = cilly::CILRoot::from_v1(st_ind, ctx);
                    roots.push(ctx.alloc_root(root));
                } else {
                    let ptr_alloc = add_allocation(prov.alloc_id().0.into(), ctx);
                    let root = cilly::CILRoot::from_v1(
                        &CILRoot::STIndISize(
                            (addr.clone()
                                + CILNode::V2(ctx.alloc_node(Const::USize(offset.into()))))
                            .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                            ptr_alloc.cast_ptr(Type::Int(Int::USize)),
                        ),
                        ctx,
                    );
                    roots.push(ctx.alloc_root(root));
                }
            }
            if thread_local {
                ctx.add_tcctor(&roots);
            } else {
                ctx.add_cctor(&roots);
            }
            CILNode::AddressOfStaticField(Box::new(field_desc))
        }
        _ => {
            let tl = if thread_local { "t" } else { "g" };
            let alloc_name: IString = format!(
                "al_{}_{}_{}_{tl}",
                encode(alloc_id),
                encode(byte_hash),
                const_allocation.len()
            )
            .into();
            let name = ctx.alloc_string(alloc_name.clone());
            let field_desc =
                StaticFieldDesc::new(*ctx.main_module(), name, ctx.nptr(Type::Int(Int::U8)));
            // Currently, all static fields are in one module. Consider spliting them up.
            let main_module = ctx.class_mut(main_module_id);
            if main_module.has_static_field(name, field_desc.tpe()) {
                return CILNode::LDStaticField(Box::new(field_desc));
            }
            ctx.add_static(
                uint8_ptr,
                &*alloc_name,
                thread_local,
                main_module_id,
                None,
                false,
            );
            let align: u64 = const_allocation.align.bytes().max(1);
            let ptr = alloc_buff(align, ctx, const_allocation.len());
            let initialzer: MethodDefIdx =
                allocation_initializer_method(const_allocation, &alloc_name, ctx, ptr, false);

            // Calls the static initialzer, and sets the static field to the returned pointer.
            let val = ctx.alloc_node(cilly::CILNode::call(*initialzer, []));

            let field = ctx.alloc_sfld(field_desc);
            let root = ctx.alloc_root(cilly::CILRoot::SetStaticField { field, val });
            if thread_local {
                ctx.add_tcctor(&[root]);
            } else {
                ctx.add_cctor(&[root]);
            }

            CILNode::LDStaticField(Box::new(field_desc))
        }
    }
}
pub fn add_const_value(asm: &mut cilly::Assembly, bytes: u128) -> StaticFieldDesc {
    let uint8_ptr = Type::Int(Int::U128);
    let main_module_id = asm.main_module();
    let alloc_fld: IString = format!("a_{bytes:x}").into();
    let alloc_fld_name = asm.alloc_string(alloc_fld.clone());

    let field_desc = StaticFieldDesc::new(*asm.main_module(), alloc_fld_name, Type::Int(Int::U128));

    let main_module = asm.class_mut(main_module_id);
    if main_module.has_static_field(alloc_fld_name, field_desc.tpe()) {
        return field_desc;
    }
    asm.add_static(uint8_ptr, alloc_fld, false, main_module_id, None, false);
    let cst = CILNode::const_u128(bytes, asm);

    let field = asm.alloc_sfld(field_desc);
    let val = cilly::CILNode::from_v1(&cst, asm);
    let val = asm.alloc_node(val);
    let set = asm.alloc_root(cilly::CILRoot::SetStaticField { field, val });

    asm.add_cctor(&[set]);

    field_desc
}
fn alloc_buff(align: u64, asm: &mut cilly::Assembly, len: usize) -> CILNode {
    if align > 8 {
        let aligned_alloc = MethodRef::aligned_alloc(asm);
        Box::new(call!(
            asm.alloc_methodref(aligned_alloc),
            [
                CILNode::V2(asm.alloc_node(Const::USize(len as u64))),
                CILNode::V2(asm.alloc_node(Const::USize(align)))
            ]
        ))
        .cast_ptr(asm.nptr(Type::Int(Int::U8)))
    } else {
        let alloc = MethodRef::alloc(asm);
        Box::new(call!(
            asm.alloc_methodref(alloc),
            [CILNode::V2(asm.alloc_node(Const::ISize(
                i64::try_from(len as u64).expect("Static alloc too big")
            )))]
        ))
        .cast_ptr(asm.nptr(Type::Int(Int::U8)))
    }
}
fn allocation_initializer_method(
    const_allocation: &Allocation,
    name: &str,
    ctx: &mut MethodCompileCtx<'_, '_>,
    ptr: CILNode,
    void_ret: bool,
) -> MethodDefIdx {
    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let ptrs = const_allocation.provenance().ptrs();
    let mut trees: Vec<CILTree> = Vec::new();

    //trees.push(CILRoot::debug(&format!("Preparing to initialize allocation with size {}",bytes.len())).into());
    trees.push(
        CILRoot::STLoc {
            local: 0,
            tree: ptr,
        }
        .into(),
    );
    trees.push(
        CILRoot::CpBlk {
            dst: Box::new(CILNode::LDLoc(0)),
            src: Box::new(CILNode::V2(ctx.bytebuffer(bytes, Int::U8))),
            len: Box::new(CILNode::V2(
                ctx.alloc_node(Const::USize(bytes.len() as u64)),
            )),
        }
        .into(),
    );

    if !ptrs.is_empty() {
        for (offset, prov) in ptrs.iter() {
            let offset = u32::try_from(offset.bytes_usize()).unwrap();
            // Check if this allocation is a function
            let reloc_target_alloc = ctx.tcx().global_alloc(prov.alloc_id());
            if let GlobalAlloc::Function {
                instance: finstance,
            } = reloc_target_alloc
            {
                // If it is a function, patch its pointer up.
                let mut ctx = MethodCompileCtx::new(ctx.tcx(), None, finstance, ctx);
                let call_info = CallInfo::sig_from_instance_(finstance, &mut ctx);
                let function_name = function_name(ctx.tcx().symbol_name(finstance));
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string(function_name),
                    ctx.alloc_sig(call_info.sig().clone()),
                    MethodKind::Static,
                    vec![].into(),
                );
                trees.push(
                    CILRoot::STIndISize(
                        (CILNode::LDLoc(0)
                            + CILNode::V2(ctx.alloc_node(Const::USize(offset.into()))))
                        .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                        CILNode::LDFtn(ctx.alloc_methodref(mref)).cast_ptr(Type::Int(Int::USize)),
                    )
                    .into(),
                );
            } else {
                let ptr_alloc = add_allocation(prov.alloc_id().0.into(), ctx);

                trees.push(
                    CILRoot::STIndISize(
                        (CILNode::LDLoc(0)
                            + CILNode::V2(ctx.alloc_node(Const::USize(offset.into()))))
                        .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                        ptr_alloc.cast_ptr(Type::Int(Int::USize)),
                    )
                    .into(),
                );
            }
        }
    }
    //trees.push(CILRoot::debug(&format!("Finished initializing an allocation with size {}",bytes.len())).into());
    if void_ret {
        trees.push(CILRoot::VoidRet.into());
    } else {
        trees.push(
            CILRoot::Ret {
                tree: CILNode::LDLoc(0),
            }
            .into(),
        );
    }
    let uint8_ptr = ctx.nptr(Type::Int(Int::U8));
    let ret = if void_ret { Type::Void } else { uint8_ptr };
    let uint8_ptr_idx = ctx.alloc_type(uint8_ptr);
    let init_method = Method::new(
        Access::Private,
        MethodType::Static,
        FnSig::new([], ret),
        &format!("init_{name}"),
        vec![(Some("alloc_ptr".into_idx(ctx)), uint8_ptr_idx)],
        vec![BasicBlock::new(trees, 0, None)],
        vec![],
        ctx,
    );
    let main_module_id = ctx.main_module();
    let init_method = MethodDef::from_v1(&init_method, ctx, main_module_id);
    ctx.new_method(init_method)
}
fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::{DefaultHasher, Hasher};
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
