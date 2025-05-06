use crate::constant::static_ty;
use cilly::{
    Access, Const, FnSig, Int, Interned, IntoAsmIndex, MethodDef, MethodDefIdx, MethodRef,
    StaticFieldDesc, Type,
    basic_block::BasicBlock,
    call,
    cil_node::V1Node,
    cil_root::V1Root,
    cil_tree::CILTree,
    cilnode::MethodKind,
    method::{Method, MethodType},
    utilis::encode,
    v2::CILNode,
};
use rustc_codegen_clr_call::CallInfo;
pub use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_ctx::function_name;
use rustc_codegen_clr_type::{GetTypeExt, align_of, r#type::fixed_array};
use rustc_middle::{
    mir::interpret::{AllocId, Allocation, GlobalAlloc},
    ty::{Instance, List, TypingEnv},
};
use rustc_span::def_id::DefId;

pub fn add_static(def_id: DefId, ctx: &mut MethodCompileCtx<'_, '_>) -> Interned<CILNode> {
    let main_module_id = ctx.main_module();
    let alloc = ctx.tcx().eval_static_initializer(def_id).unwrap();
    let attrs = ctx.tcx().codegen_fn_attrs(def_id);

    let thread_local = attrs
        .flags
        .contains(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags::THREAD_LOCAL);
    let align = alloc.0.align.bytes().max(1);
    let ty = static_ty(def_id, ctx.tcx());
    let tpe = ctx.type_from_cache(ty);
    assert_eq!(align, align_of(ty, ctx.tcx()));
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
    let ptr = ctx.alloc_node(CILNode::LdStaticFieldAdress(sfld));
    let ptr = ctx.cast_ptr(ptr, Int::U8);
    let ptr = ptr;
    let initialzer = allocation_initializer_method(&alloc.0, &symbol, ctx, ptr, true);
    let root = ctx.alloc_root(cilly::CILRoot::call(*initialzer, []));

    if thread_local {
        ctx.add_tcctor(&[root]);
    } else {
        ctx.add_cctor(&[root]);
    }

    ptr
}
fn alloc_default_type(alloc_id: u64, ctx: &mut MethodCompileCtx<'_, '_>) -> Type {
    let alloc = match ctx
        .tcx()
        .global_alloc(AllocId(alloc_id.try_into().expect("0 alloc id?")))
    {
        GlobalAlloc::Memory(alloc) => alloc,
        GlobalAlloc::Static(def_id) => return ctx.type_from_cache(static_ty(def_id, ctx.tcx())),
        GlobalAlloc::VTable(..) => {
            todo!()
        }
        GlobalAlloc::Function { .. } => {
            todo!()
        }
    };
    let tpe = match alloc.0.0.align.bytes() {
        ..1 => Int::U8,
        ..2 => Int::U16,
        ..4 => Int::U32,
        ..8 => Int::U64,
        _ => {
            ctx.tcx().dcx().span_warn(
                ctx.span(),
                format!(
                    "Alloc of align {} required, but that can't be guranteed!",
                    alloc.0.0.align.bytes()
                ),
            );
            Int::U64
        }
    };
    let arr_size = alloc.0.len() as u64;
    if arr_size == 0 {
        return Type::Void;
    }
    let size = tpe.size().unwrap_or(8) as u64;
    let tpe = fixed_array(
        ctx,
        Type::Int(tpe),
        arr_size.div_ceil(size),
        arr_size.next_multiple_of(size),
        tpe.size().unwrap_or(8) as u64,
    );
    Type::ClassRef(tpe)
}
/// Adds a static field and initialized for allocation represented by `alloc_id`.
pub fn add_allocation(
    alloc_id: u64,
    ctx: &mut MethodCompileCtx<'_, '_>,
    tpe: Interned<Type>,
) -> Interned<CILNode> {
    let uint8_ptr = ctx.nptr(Type::Int(Int::U8));
    let main_module_id = ctx.main_module();
    let const_allocation = match ctx
        .tcx()
        .global_alloc(AllocId(alloc_id.try_into().expect("0 alloc id?")))
    {
        GlobalAlloc::Memory(alloc) => alloc,
        GlobalAlloc::Static(def_id) => return add_static(def_id, ctx),
        GlobalAlloc::VTable(..) => {
            //TODO: handle VTables
            let field_desc = ctx.add_static(
                uint8_ptr,
                format!("v_{alloc_id:x}"),
                false,
                main_module_id,
                None,
                false,
            );
            return (ctx.load_static(field_desc));
        }
        GlobalAlloc::Function { .. } => {
            //TODO: handle constant functions
            let alloc_fld = format!("f_{alloc_id:x}");
            let field_desc =
                ctx.add_static(uint8_ptr, alloc_fld, false, main_module_id, None, false);

            return (ctx.load_static(field_desc));
            //todo!("Function/Vtable allocation.");
        }
    };

    let const_allocation = const_allocation.inner();

    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let align = const_allocation.align.bytes().max(1);
    if const_allocation.len() == 0 {
        return (ctx.alloc_node(Const::USize(align)));
    }
    // Check if const literal can be used
    if const_allocation.provenance().ptrs().is_empty() && align <= 1 {
        return (ctx.bytebuffer(bytes, Int::U8));
    }
    // Alloc ids are *not* unique across all crates. Adding the hash here ensures we don't overwrite allocations during linking
    // TODO:consider using something better here / making the hashes stable.
    let byte_hash = calculate_hash(&bytes);
    match (align, bytes.len()) {
        _ => {
            let alloc_name = format!(
                "al_{}_{}_{}_{}",
                encode(alloc_id),
                encode(byte_hash),
                encode(tpe.inner().into()),
                const_allocation.len()
            );
            let name = ctx.alloc_string(alloc_name.clone());
            let field_desc = StaticFieldDesc::new(*ctx.main_module(), name, ctx[tpe]);
            // Currently, all static fields are in one module. Consider spliting them up.

            let main_module = ctx.class_mut(main_module_id);

            if main_module.has_static_field(name, field_desc.tpe()) {
                return ctx.static_addr(field_desc).into();
            }
            let tpe = ctx[tpe].clone();
            ctx.add_static(tpe, &*alloc_name, false, main_module_id, None, false);

            let ptr = ctx.static_addr(field_desc);
            let ptr = ctx.cast_ptr(ptr, Int::U8);

            let initialzer: MethodDefIdx =
                allocation_initializer_method(const_allocation, &alloc_name, ctx, ptr.into(), true);

            // Calls the static initialzer, and sets the static field to the returned pointer.
            let root = ctx.alloc_root(cilly::CILRoot::call(*initialzer, []));
            ctx.add_cctor(&[root]);

            ctx.static_addr(field_desc)
        }
    }
}
pub fn add_const_value(asm: &mut cilly::Assembly, bytes: u128) -> StaticFieldDesc {
    let uint8_ptr = Type::Int(Int::U128);
    let main_module_id = asm.main_module();
    let alloc_fld = format!("a_{bytes:x}");
    let alloc_fld_name = asm.alloc_string(alloc_fld.clone());

    let field_desc = StaticFieldDesc::new(*asm.main_module(), alloc_fld_name, Type::Int(Int::U128));

    let main_module = asm.class_mut(main_module_id);
    if main_module.has_static_field(alloc_fld_name, field_desc.tpe()) {
        return field_desc;
    }
    asm.add_static(uint8_ptr, alloc_fld, false, main_module_id, None, false);
    let cst = V1Node::const_u128(bytes, asm);

    let field = asm.alloc_sfld(field_desc);
    let val = cilly::CILNode::from_v1(&cst, asm);
    let val = asm.alloc_node(val);
    let set = asm.alloc_root(cilly::CILRoot::SetStaticField { field, val });

    asm.add_cctor(&[set]);

    field_desc
}
fn alloc_buff(align: u64, asm: &mut cilly::Assembly, len: usize) -> V1Node {
    if align > 8 {
        let aligned_alloc = MethodRef::aligned_alloc(asm);
        Box::new(call!(
            asm.alloc_methodref(aligned_alloc),
            [
                V1Node::V2(asm.alloc_node(Const::USize(len as u64))),
                V1Node::V2(asm.alloc_node(Const::USize(align)))
            ]
        ))
        .cast_ptr(asm.nptr(Type::Int(Int::U8)))
    } else {
        let alloc = MethodRef::alloc(asm);
        Box::new(call!(
            asm.alloc_methodref(alloc),
            [V1Node::V2(asm.alloc_node(Const::ISize(
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
    ptr: Interned<CILNode>,
    void_ret: bool,
) -> MethodDefIdx {
    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let ptrs = const_allocation.provenance().ptrs();
    let mut trees: Vec<CILTree> = Vec::new();

    //trees.push(CILRoot::debug(&format!("Preparing to initialize allocation with size {}",bytes.len())).into());
    trees.push(
        V1Root::STLoc {
            local: 0,
            tree: ptr.into(),
        }
        .into(),
    );
    trees.push(
        V1Root::CpBlk {
            dst: Box::new(V1Node::LDLoc(0)),
            src: Box::new(V1Node::V2(ctx.bytebuffer(bytes, Int::U8))),
            len: Box::new(V1Node::V2(ctx.alloc_node(Const::USize(bytes.len() as u64)))),
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
                    V1Root::STIndISize(
                        (V1Node::LDLoc(0)
                            + V1Node::V2(ctx.alloc_node(Const::USize(offset.into()))))
                        .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                        V1Node::LDFtn(ctx.alloc_methodref(mref)).cast_ptr(Type::Int(Int::USize)),
                    )
                    .into(),
                );
            } else {
                let tpe = alloc_default_type(prov.alloc_id().0.into(), ctx);
                let tpe = ctx.alloc_type(tpe);
                let ptr_alloc = add_allocation(prov.alloc_id().0.into(), ctx, tpe);

                trees.push(
                    V1Root::STIndISize(
                        (V1Node::LDLoc(0)
                            + V1Node::V2(ctx.alloc_node(Const::USize(offset.into()))))
                        .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                        Into::<V1Node>::into(ptr_alloc).cast_ptr(Type::Int(Int::USize)),
                    )
                    .into(),
                );
            }
        }
    }
    //trees.push(CILRoot::debug(&format!("Finished initializing an allocation with size {}",bytes.len())).into());
    if void_ret {
        trees.push(V1Root::VoidRet.into());
    } else {
        trees.push(
            V1Root::Ret {
                tree: V1Node::LDLoc(0),
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
