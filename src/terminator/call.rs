use crate::{
    assembly::MethodCompileCtx,
    call_info::CallInfo,
    interop::AssemblyRef,
    operand::operand_address,
    utilis::{
        garg_to_string, CTOR_FN_NAME, MANAGED_CALL_FN_NAME, MANAGED_CALL_VIRT_FN_NAME,
        MANAGED_CHECKED_CAST, MANAGED_IS_INST, MANAGED_LD_ELEM_REF, MANAGED_LD_LEN,
        MANAGED_LD_NULL,
    },
};
use cilly::{
    call, call_virt,
    cil_node::{CILNode, CallOpArgs},
    cil_root::CILRoot,
    conv_usize, ld_field, ldc_i32, ptr, size_of,
};
use cilly::{call_site::CallSite, field_desc::FieldDescriptor, fn_sig::FnSig, DotnetTypeRef, Type};
use rustc_middle::ty::InstanceKind;
use rustc_middle::{
    mir::{Operand, Place},
    ty::{GenericArg, Instance, ParamEnv, Ty, TyKind},
};
use rustc_span::source_map::Spanned;
fn argc_from_fn_name(function_name: &str, prefix: &str) -> u32 {
    let argc_start = function_name.find(prefix).unwrap() + (prefix.len());
    let argc_end = argc_start + function_name[argc_start..].find('_').unwrap();
    let argument_count = &function_name[argc_start..argc_end];
    argument_count.parse::<u32>().unwrap()
}
/// Calls a non-virtual managed function(used for interop)
fn call_managed<'tcx>(
    subst_ref: &[GenericArg<'tcx>],
    function_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    fn_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> CILRoot {
    let argument_count = argc_from_fn_name(function_name, MANAGED_CALL_FN_NAME);
    //FIXME: figure out the proper argc.
    //assert!(subst_ref.len() as u32 == argc + 3 || subst_ref.len() as u32 == argc + 4);
    assert!(args.len() == argument_count as usize);
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], ctx.tcx());
    let asm = asm.name();
    let class_name = garg_to_string(subst_ref[1], ctx.tcx());
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], ctx.tcx());
    let managed_fn_name = garg_to_string(subst_ref[3], ctx.tcx());
    let mut tpe = DotnetTypeRef::new(asm, class_name);
    tpe.set_valuetype(is_valuetype);
    //eprintln!("tpe:{tpe:?}");
    let signature =
        crate::function_sig::sig_from_instance_(fn_instance, ctx.tcx(), ctx.type_cache())
            .expect("Can't get the function signature");

    if argument_count == 0 {
        let ret = cilly::Type::Void;
        let call_site = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name,
            FnSig::new(&[], ret),
            true,
        );
        if *signature.output() == cilly::Type::Void {
            CILRoot::Call {
                site: Box::new(call_site),
                args: [].into(),
            }
        } else {
            crate::place::place_set(destination, call!(call_site, []), ctx)
        }
    } else {
        let is_static = crate::utilis::garag_to_bool(subst_ref[4], ctx.tcx());

        let mut call_args = Vec::new();
        for arg in args {
            call_args.push(crate::operand::handle_operand(&arg.node, ctx));
        }
        let call = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name,
            signature.clone(),
            is_static,
        );
        if *signature.output() == cilly::Type::Void {
            CILRoot::Call {
                site: Box::new(call),
                args: call_args.into(),
            }
        } else {
            crate::place::place_set(destination, call!(call, call_args), ctx)
        }
    }
}
/// Calls a virtual managed function(used for interop)
fn callvirt_managed<'tcx>(
    subst_ref: &[GenericArg<'tcx>],
    function_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    fn_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> CILRoot {
    let argument_count = argc_from_fn_name(function_name, MANAGED_CALL_VIRT_FN_NAME);
    //assert!(subst_ref.len() as u32 == argc + 3 || subst_ref.len() as u32 == argc + 4);
    assert!(
        u32::try_from(args.len()).expect("More than 2^32 function arguments.") == argument_count
    );
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], ctx.tcx());
    let asm = asm.name();
    let class_name = garg_to_string(subst_ref[1], ctx.tcx());
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], ctx.tcx());

    let managed_fn_garg = &subst_ref[3];
    let managed_fn_garg = ctx.monomorphize(*managed_fn_garg);
    let managed_fn_name = garg_to_string(managed_fn_garg, ctx.tcx());

    let mut tpe = DotnetTypeRef::new(asm, class_name);
    tpe.set_valuetype(is_valuetype);
    let signature =
        crate::function_sig::sig_from_instance_(fn_instance, ctx.tcx(), ctx.type_cache())
            .expect("Can't get the function signature");
    if argument_count == 0 {
        let ret = cilly::Type::Void;
        let call = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name,
            FnSig::new(&[], ret),
            true,
        );
        if *signature.output() == cilly::Type::Void {
            CILRoot::CallVirt {
                site: Box::new(call),
                args: [].into(),
            }
        } else {
            crate::place::place_set(destination, call_virt!(call, []), ctx)
        }
    } else {
        let is_static = crate::utilis::garag_to_bool(subst_ref[4], ctx.tcx());

        let mut call_args = Vec::new();
        for arg in args {
            call_args.push(crate::operand::handle_operand(&arg.node, ctx));
        }
        let call = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name,
            signature.clone(),
            is_static,
        );
        if *signature.output() == cilly::Type::Void {
            CILRoot::CallVirt {
                site: Box::new(call),
                args: call_args.into(),
            }
        } else {
            crate::place::place_set(destination, call_virt!(call, call_args), ctx)
        }
    }
}
/// Creates a new managed object, and places a reference to it in destination
fn call_ctor<'tcx>(
    subst_ref: &[GenericArg<'tcx>],
    function_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> CILRoot {
    let argument_count = argc_from_fn_name(function_name, CTOR_FN_NAME);
    // Check that there are enough function path and argument specifers
    assert!(subst_ref.len() == argument_count as usize + 3);
    // Check that a proper number of arguments is used
    assert!(args.len() == argument_count as usize);
    // Get the name of the assembly the constructed object resides in
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], ctx.tcx());
    let asm = asm.name();
    // Get the name of the constructed object
    let class_name = garg_to_string(subst_ref[1], ctx.tcx());
    // Check if the costructed object is valuetype. TODO: this may be unnecesary. Are valuetpes constructed using newobj?
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], ctx.tcx());
    let mut tpe = DotnetTypeRef::new(asm, class_name);
    tpe.set_valuetype(is_valuetype);
    // If no arguments, inputs don't have to be handled, so a simpler call handling is used.
    if argument_count == 0 {
        crate::place::place_set(
            destination,
            CILNode::NewObj(Box::new(CallOpArgs {
                site: CallSite::boxed(
                    Some(tpe.clone()),
                    ".ctor".into(),
                    FnSig::new(
                        &[Type::DotnetType(tpe.clone().into())],
                        cilly::r#type::Type::Void,
                    ),
                    false,
                ),
                args: [].into(),
            })),
            ctx,
        )
    } else {
        let mut inputs: Vec<_> = subst_ref[3..]
            .iter()
            .map(|ty| {
                ctx.type_from_cache(
                    ctx.monomorphize(*ty)
                        .as_type()
                        .expect("Expceted generic type but got something that was not a type!"),
                )
            })
            .collect();
        inputs.insert(0, tpe.clone().into());
        let sig = FnSig::new(inputs, cilly::Type::Void);
        let mut call = Vec::new();
        for arg in args {
            call.push(crate::operand::handle_operand(&arg.node, ctx));
        }

        crate::place::place_set(
            destination,
            CILNode::NewObj(Box::new(CallOpArgs {
                site: CallSite::boxed(Some(tpe.clone()), ".ctor".into(), sig, false),
                args: call.into(),
            })),
            ctx,
        )
    }
}
pub fn call_closure<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    sig: FnSig,
    function_name: &str,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> CILRoot {
    let last_arg = args
        .last()
        .expect("Closure must be called with at least 2 arguments(closure + arg tuple)");

    let other_args = &args[..args.len() - 1];
    let mut call_args = Vec::new();
    for arg in other_args {
        call_args.push(crate::operand::handle_operand(&arg.node, ctx));
    }
    // "Rust call" is wierd, and not at all optimized for .NET. Passing all the arguments in a tuple is bad for performance and simplicty. Thus, unpacking this tuple and forcing "Rust call" to be
    // "normal" is far easier and better for performance.
    let last_arg_type = ctx.monomorphize(last_arg.node.ty(ctx.body(), ctx.tcx()));
    match last_arg_type.kind() {
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
            } else {
                let tuple_type = ctx.type_from_cache(last_arg_type);

                for (index, element) in elements.iter().enumerate() {
                    let element_type = ctx.type_from_cache(element);
                    if element_type == Type::Void {
                        call_args.push(CILNode::TemporaryLocal(Box::new((
                            Type::Void,
                            [].into(),
                            CILNode::LoadTMPLocal,
                        ))));
                        continue;
                    }
                    let tuple_element_name = format!("Item{}", index + 1);
                    let field_descriptor = FieldDescriptor::boxed(
                        tuple_type.as_dotnet().expect("Invalid tuple type"),
                        element_type,
                        tuple_element_name.into(),
                    );

                    call_args.push(ld_field!(
                        crate::operand::handle_operand(&last_arg.node, ctx),
                        field_descriptor
                    ));
                }

                //todo!("Can't unbox tupels yet!")
            }
        }
        _ => panic!("Can't unbox type {last_arg_type:?}!"),
    }
    //panic!("Last arg:{last_arg:?}last_arg_type:{last_arg_type:?}");
    //assert_eq!(args.len(),signature.inputs().len(),"CALL SIGNATURE ARG COUNT MISMATCH!");
    let is_void = matches!(sig.output(), cilly::Type::Void);
    let call = CallSite::new(None, function_name.into(), sig, true);
    // Hande the call itself

    if is_void {
        CILRoot::Call {
            site: Box::new(call),
            args: call_args.into(),
        }
    } else {
        crate::place::place_set(destination, call!(call, call_args), ctx)
    }
}
/// Calls `fn_type` with `args`, placing the return value in destination.
pub fn call<'tcx>(
    fn_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    span: rustc_span::Span,
) -> CILRoot {
    let fn_type = ctx.monomorphize(fn_type);
    let (instance, subst_ref) = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind() {
        let subst = ctx.monomorphize(*subst_ref);
        let env = ParamEnv::reveal_all();
        let Some(instance) =
            Instance::try_resolve(ctx.tcx(), env, *def_id, subst).expect("Invalid function def")
        else {
            panic!("ERROR: Could not get function instance. fn type:{fn_type:?}")
        };

        (instance, subst)
    } else {
        todo!("Trying to call a type which is not a function definition!");
    };
    if let rustc_middle::ty::InstanceKind::Virtual(_def, fn_idx) = instance.def {
        assert!(!args.is_empty());

        let fat_ptr_address = operand_address(&args[0].node, ctx);

        let vtable_ptr = ld_field!(
            fat_ptr_address.clone(),
            FieldDescriptor::new(
                DotnetTypeRef::new::<&str, _>(None, "FatPtrDyn"),
                Type::USize,
                crate::METADATA.into()
            )
        );

        let vtable_index =
            ldc_i32!(i32::try_from(fn_idx).expect("More tahn 2^31 functions in a vtable!"));
        let vtable_offset = conv_usize!(vtable_index * size_of!(Type::USize));
        // Get the address of the function ptr, and load it
        let fn_ptr = CILNode::LDIndISize {
            ptr: Box::new((vtable_ptr + vtable_offset).cast_ptr(ptr!(Type::ISize))),
        };
        // Get the addres of the object
        let obj_ptr = ld_field!(
            fat_ptr_address,
            FieldDescriptor::new(
                DotnetTypeRef::new::<&str, _>(None, "FatPtrDyn"),
                Type::Ptr(Type::Void.into()),
                crate::DATA_PTR.into()
            )
        );
        // Get the call info
        let call_info = CallInfo::sig_from_instance_(instance, ctx.tcx(), ctx.type_cache());

        let mut signature = call_info.sig().clone();
        signature.inputs_mut()[0] = ptr!(Type::Void);
        let mut call_args = [obj_ptr].to_vec();
        if call_info.split_last_tuple() {
            let last_arg = args
                .last()
                .expect("Closure must be called with at least 2 arguments(closure + arg tuple)");

            let other_args = &args[..args.len() - 1];
            for arg in other_args.iter().skip(1) {
                call_args.push(crate::operand::handle_operand(&arg.node, ctx));
            }
            // "Rust call" is wierd, and not at all optimized for .NET. Passing all the arguments in a tuple is bad for performance and simplicty. Thus, unpacking this tuple and forcing "Rust call" to be
            // "normal" is far easier and better for performance.
            let last_arg_type = ctx.monomorphize(last_arg.node.ty(ctx.body(), ctx.tcx()));
            match last_arg_type.kind() {
                TyKind::Tuple(elements) => {
                    if elements.is_empty() {
                    } else {
                        let tuple_type = ctx.type_from_cache(last_arg_type);

                        for (index, element) in elements.iter().enumerate() {
                            let element_type = ctx.type_from_cache(element);
                            if element_type == Type::Void {
                                call_args.push(CILNode::TemporaryLocal(Box::new((
                                    Type::Void,
                                    [].into(),
                                    CILNode::LoadTMPLocal,
                                ))));
                                continue;
                            }
                            let tuple_element_name = format!("Item{}", index + 1);
                            let field_descriptor = FieldDescriptor::boxed(
                                tuple_type.as_dotnet().expect("Invalid tuple type"),
                                element_type,
                                tuple_element_name.into(),
                            );
                            call_args.push(ld_field!(
                                crate::operand::handle_operand(&last_arg.node, ctx),
                                field_descriptor
                            ));
                        }
                    }
                }
                _ => panic!("Can't unbox type {last_arg_type:?}!"),
            }
        } else {
            for arg in args.iter().skip(1) {
                call_args.push(crate::operand::handle_operand(&arg.node, ctx));
            }
        }
        assert_eq!(
            signature.inputs().len(),
            call_args.len(),
            "sig:{signature:?} call_args:{call_args:?}"
        );
        let is_ret_void = matches!(signature.output(), cilly::Type::Void);
        return if is_ret_void {
            CILRoot::CallI {
                sig: Box::new(signature),
                fn_ptr: Box::new(fn_ptr),
                args: call_args.into(),
            }
        } else {
            crate::place::place_set(
                destination,
                CILNode::CallI(Box::new((signature, fn_ptr, call_args.into()))),
                ctx,
            )
        };
    }
    let call_info = CallInfo::sig_from_instance_(instance, ctx.tcx(), ctx.type_cache());

    let function_name = crate::utilis::function_name(ctx.tcx().symbol_name(instance));
    if crate::utilis::is_fn_intrinsic(fn_type, ctx.tcx()) {
        return super::intrinsics::handle_intrinsic(
            &function_name,
            args,
            destination,
            instance,
            span,
            ctx,
        );
    }
    let mut signature = call_info.sig().clone();
    // Checks if function is "magic"
    if function_name.contains(CTOR_FN_NAME) {
        assert!(
            !call_info.split_last_tuple(),
            "Constructors may not use the `rust_call` calling convention!"
        );
        // Constructor
        return call_ctor(subst_ref, &function_name, args, destination, ctx);
    } else if function_name.contains(MANAGED_CALL_VIRT_FN_NAME) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed virtual calls may not use the `rust_call` calling convention!"
        );
        // Virtual (for interop)
        return callvirt_managed(subst_ref, &function_name, args, destination, instance, ctx);
    } else if function_name.contains(MANAGED_CALL_FN_NAME) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed calls may not use the `rust_call` calling convention!"
        );
        // Not-Virtual (for interop)
        return call_managed(subst_ref, &function_name, args, destination, instance, ctx);
    } else if function_name.contains(MANAGED_LD_LEN) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed calls may not use the `rust_call` calling convention!"
        );
        // Not-Virtual (for interop)
        return crate::place::place_set(
            destination,
            CILNode::LDLen {
                arr: Box::new(crate::operand::handle_operand(&args[0].node, ctx)),
            },
            ctx,
        );
    } else if function_name.contains(MANAGED_LD_NULL) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed calls may not use the `rust_call` calling convention!"
        );
        // Not-Virtual (for interop)
        let tpe = ctx.type_from_cache(subst_ref[0].as_type().unwrap());

        return crate::place::place_set(
            destination,
            CILNode::LdNull(tpe.as_dotnet().unwrap()),
            ctx,
        );
    } else if function_name.contains(MANAGED_CHECKED_CAST) {
        let tpe = ctx
            .type_from_cache(subst_ref[0].as_type().unwrap())
            .as_dotnet()
            .unwrap();
        let input = crate::operand::handle_operand(&args[0].node, ctx);
        // Not-Virtual (for interop)
        return crate::place::place_set(
            destination,
            CILNode::CheckedCast(Box::new((input, tpe))),
            ctx,
        );
    } else if function_name.contains(MANAGED_IS_INST) {
        let tpe = ctx
            .type_from_cache(subst_ref[0].as_type().unwrap())
            .as_dotnet()
            .unwrap();
        let input = crate::operand::handle_operand(&args[0].node, ctx);
        // Not-Virtual (for interop)
        return crate::place::place_set(destination, CILNode::IsInst(Box::new((input, tpe))), ctx);
    } else if function_name.contains(MANAGED_LD_ELEM_REF) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed calls may not use the `rust_call` calling convention!"
        );
        // Not-Virtual (for interop)
        return crate::place::place_set(
            destination,
            CILNode::LDElelemRef {
                arr: Box::new(crate::operand::handle_operand(&args[0].node, ctx)),
                idx: Box::new(crate::operand::handle_operand(&args[1].node, ctx)),
            },
            ctx,
        );
    }
    if call_info.split_last_tuple() {
        return call_closure(args, destination, signature, &function_name, ctx);
    }

    let mut call_args = Vec::new();
    for arg in args {
        let method_instance = ctx.instance();
        let tcx = ctx.tcx();
        let res_calc = crate::r#type::tycache::validity_check(
            crate::operand::handle_operand(&arg.node, ctx),
            ctx.monomorphize(arg.node.ty(ctx.body(), ctx.tcx())),
            ctx.type_cache(),
            method_instance,
            tcx,
        );
        call_args.push(res_calc);
    }
    if crate::function_sig::is_fn_variadic(fn_type, ctx.tcx()) {
        signature.set_inputs(
            args.iter()
                .map(|operand| {
                    ctx.type_from_cache(ctx.monomorphize(operand.node.ty(ctx.body(), ctx.tcx())))
                })
                .collect(),
        );
    }
    if args.len() < signature.inputs().len() {
        let tpe: cilly::Type = signature.inputs()[signature.inputs().len() - 1].clone();
        // let arg_len = args.len();
        //assert_eq!(args.len() + 1,signature.inputs().len(),"ERROR: mismatched argument count. \nsignature inputs:{:?} \narguments:{args:?}\narg_len:{arg_len}\n",signature.inputs());
        // assert_eq!(signature.inputs()[signature.inputs().len() - 1],tpe);
        //FIXME:This assembles a panic location from uninitialized memory. This WILL lead to bugs once unwinding is added. The fields `file`,`col`, and `line` should be set there.
        call_args.push(CILNode::TemporaryLocal(Box::new((
            tpe,
            [].into(),
            CILNode::LoadTMPLocal,
        ))));
        //panic!("Call with PanicLocation!");
    }
    //assert_eq!(args.len(),signature.inputs().len(),"CALL SIGNATURE ARG COUNT MISMATCH!");
    let is_void = matches!(signature.output(), cilly::Type::Void);
    //rustc_middle::ty::print::with_no_trimmed_paths! {call.push(CILOp::Comment(format!("Calling {instance:?}").into()))};
    if let InstanceKind::DropGlue(_def, None) = instance.def {
        return CILRoot::Nop;
    };
    let call_site = CallSite::new(None, function_name, signature, true);
    // Hande
    if is_void {
        CILRoot::Call {
            site: Box::new(call_site),
            args: call_args.into(),
        }
    } else {
        let method_instance = ctx.instance();
        let tcx = ctx.tcx();
        let res_calc = crate::r#type::tycache::validity_check(
            call!(call_site, call_args),
            ctx.monomorphize(destination.ty(ctx.body(), ctx.tcx()).ty),
            ctx.type_cache(),
            method_instance,
            tcx,
        );
        crate::place::place_set(destination, res_calc, ctx)
    }
}
