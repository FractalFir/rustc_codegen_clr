use crate::{
    call_info::CallInfo,
    interop::AssemblyRef,
    operand::operand_address,
    utilis::{garg_to_string, CTOR_FN_NAME, MANAGED_CALL_FN_NAME, MANAGED_CALL_VIRT_FN_NAME},
};
use cilly::{
    call, call_virt, cil_node::CILNode, cil_root::CILRoot, conv_usize, ld_field, ldc_u32, size_of,
};
use cilly::{call_site::CallSite, field_desc::FieldDescriptor, fn_sig::FnSig, DotnetTypeRef, Type};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{GenericArg, Instance, InstanceDef, ParamEnv, Ty, TyCtxt, TyKind},
};
use rustc_span::source_map::Spanned;
fn argc_from_fn_name(function_name: &str, prefix: &str) -> u32 {
    let argc_start = function_name.find(prefix).unwrap() + (prefix.len());
    let argc_end = argc_start + function_name[argc_start..].find('_').unwrap();
    let argument_count = &function_name[argc_start..argc_end];
    argument_count.parse::<u32>().unwrap()
}
/// Calls a non-virtual managed function(used for interop)
fn call_managed<'tyctx>(
    tyctx: TyCtxt<'tyctx>,
    subst_ref: &[GenericArg<'tyctx>],
    function_name: &str,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    method: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    fn_instance: Instance<'tyctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> CILRoot {
    let argument_count = argc_from_fn_name(function_name, MANAGED_CALL_FN_NAME);
    //FIXME: figure out the proper argc.
    //assert!(subst_ref.len() as u32 == argc + 3 || subst_ref.len() as u32 == argc + 4);
    assert!(args.len() == argument_count as usize);
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], tyctx);
    let asm = asm.name();
    let class_name = garg_to_string(subst_ref[1], tyctx);
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], tyctx);
    let managed_fn_name = garg_to_string(subst_ref[3], tyctx);
    let mut tpe = DotnetTypeRef::new(asm, class_name);
    tpe.set_valuetype(is_valuetype);
    //eprintln!("tpe:{tpe:?}");
    let signature = crate::function_sig::sig_from_instance_(fn_instance, tyctx, type_cache)
        .expect("Can't get the function signature");

    if argument_count == 0 {
        let ret = crate::r#type::Type::Void;
        let call_site = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name.into(),
            FnSig::new(&[], ret),
            true,
        );
        if *signature.output() == crate::r#type::Type::Void {
            CILRoot::Call {
                site: call_site,
                args: [].into(),
            }
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call!(call_site, []),
                method,
                method_instance,
                type_cache,
            )
        }
    } else {
        let is_static = crate::utilis::garag_to_bool(subst_ref[4], tyctx);

        let mut call_args = Vec::new();
        for arg in args {
            call_args.push(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
        }
        let call = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name.into(),
            signature.clone(),
            is_static,
        );
        if *signature.output() == crate::r#type::Type::Void {
            CILRoot::Call {
                site: call,
                args: call_args.into(),
            }
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call!(call, call_args),
                method,
                method_instance,
                type_cache,
            )
        }
    }
}
/// Calls a virtual managed function(used for interop)
fn callvirt_managed<'tyctx>(
    tyctx: TyCtxt<'tyctx>,
    subst_ref: &[GenericArg<'tyctx>],
    function_name: &str,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    method: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    fn_instance: Instance<'tyctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> CILRoot {
    let argument_count = argc_from_fn_name(function_name, MANAGED_CALL_VIRT_FN_NAME);
    //assert!(subst_ref.len() as u32 == argc + 3 || subst_ref.len() as u32 == argc + 4);
    assert!(
        u32::try_from(args.len()).expect("More than 2^32 function arguments.") == argument_count
    );
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], tyctx);
    let asm = asm.name();
    let class_name = garg_to_string(subst_ref[1], tyctx);
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], tyctx);

    let managed_fn_garg = &subst_ref[3];
    let managed_fn_garg = crate::utilis::monomorphize(&method_instance, *managed_fn_garg, tyctx);
    let managed_fn_name = garg_to_string(managed_fn_garg, tyctx);

    let mut tpe = DotnetTypeRef::new(asm, class_name);
    tpe.set_valuetype(is_valuetype);
    let signature = crate::function_sig::sig_from_instance_(fn_instance, tyctx, type_cache)
        .expect("Can't get the function signature");
    if argument_count == 0 {
        let ret = crate::r#type::Type::Void;
        let call = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name.into(),
            FnSig::new(&[], ret),
            true,
        );
        if *signature.output() == crate::r#type::Type::Void {
            CILRoot::CallVirt {
                site: call,
                args: [].into(),
            }
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call_virt!(call, []),
                method,
                method_instance,
                type_cache,
            )
        }
    } else {
        let is_static = crate::utilis::garag_to_bool(subst_ref[4], tyctx);

        let mut call_args = Vec::new();
        for arg in args {
            call_args.push(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
        }
        let call = CallSite::new(
            Some(tpe.clone()),
            managed_fn_name.into(),
            signature.clone(),
            is_static,
        );
        if *signature.output() == crate::r#type::Type::Void {
            CILRoot::CallVirt {
                site: call,
                args: call_args.into(),
            }
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call_virt!(call, call_args),
                method,
                method_instance,
                type_cache,
            )
        }
    }
}
/// Creates a new managed object, and places a reference to it in destination
fn call_ctor<'tyctx>(
    tyctx: TyCtxt<'tyctx>,
    subst_ref: &[GenericArg<'tyctx>],
    function_name: &str,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    method: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> CILRoot {
    let argument_count = argc_from_fn_name(function_name, CTOR_FN_NAME);
    // Check that there are enough function path and argument specifers
    assert!(subst_ref.len() == argument_count as usize + 3);
    // Check that a proper number of arguments is used
    assert!(args.len() == argument_count as usize);
    // Get the name of the assembly the constructed object resides in
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], tyctx);
    let asm = asm.name();
    // Get the name of the constructed object
    let class_name = garg_to_string(subst_ref[1], tyctx);
    // Check if the costructed object is valuetype. TODO: this may be unnecesary. Are valuetpes constructed using newobj?
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], tyctx);
    let mut tpe = DotnetTypeRef::new(asm, class_name);
    tpe.set_valuetype(is_valuetype);
    // If no arguments, inputs don't have to be handled, so a simpler call handling is used.
    if argument_count == 0 {
        crate::place::place_set(
            destination,
            tyctx,
            CILNode::NewObj {
                site: CallSite::boxed(
                    Some(tpe.clone()),
                    ".ctor".into(),
                    FnSig::new(&[], cilly::r#type::Type::Void),
                    false,
                ),
                args: [].into(),
            },
            method,
            method_instance,
            type_cache,
        )
    } else {
        let mut inputs: Vec<_> = subst_ref[3..]
            .iter()
            .map(|ty| {
                let ty = crate::utilis::monomorphize(&method_instance, *ty, tyctx);
                type_cache.type_from_cache(
                    ty.as_type()
                        .expect("Expceted generic type but got something that was not a type!"),
                    tyctx,
                    method_instance,
                )
            })
            .collect();
        inputs.insert(0, tpe.clone().into());
        let sig = FnSig::new(inputs, cilly::Type::Void);
        let mut call = Vec::new();
        for arg in args {
            call.push(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
        }

        crate::place::place_set(
            destination,
            tyctx,
            CILNode::NewObj {
                site: CallSite::boxed(Some(tpe.clone()), ".ctor".into(), sig, false),
                args: call.into(),
            },
            method,
            method_instance,
            type_cache,
        )
    }
}
pub fn call_closure<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    sig: FnSig,
    body: &'tyctx Body<'tyctx>,
    function_name: &str,
    method_instance: Instance<'tyctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> CILRoot {
    let last_arg = args
        .last()
        .expect("Closure must be called with at least 2 arguments(closure + arg tuple)");

    let other_args = &args[..args.len() - 1];
    let mut call_args = Vec::new();
    for arg in other_args {
        call_args.push(crate::operand::handle_operand(
            &arg.node,
            tyctx,
            body,
            method_instance,
            type_cache,
        ));
    }
    // "Rust call" is wierd, and not at all optimized for .NET. Passing all the arguments in a tuple is bad for performance and simplicty. Thus, unpacking this tuple and forcing "Rust call" to be
    // "normal" is far easier and better for performance.
    let last_arg_type =
        crate::utilis::monomorphize(&method_instance, last_arg.node.ty(body, tyctx), tyctx);
    match last_arg_type.kind() {
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
            } else {
                let tuple_type = type_cache.type_from_cache(last_arg_type, tyctx, method_instance);

                for (index, element) in elements.iter().enumerate() {
                    let element_type = type_cache.type_from_cache(element, tyctx, method_instance);
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
                        crate::operand::handle_operand(
                            &last_arg.node,
                            tyctx,
                            body,
                            method_instance,
                            type_cache
                        ),
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
    let is_void = matches!(sig.output(), crate::r#type::Type::Void);
    let call = CallSite::new(None, function_name.into(), sig, true);
    // Hande the call itself

    if is_void {
        CILRoot::Call {
            site: call,
            args: call_args.into(),
        }
    } else {
        crate::place::place_set(
            destination,
            tyctx,
            call!(call, call_args),
            body,
            method_instance,
            type_cache,
        )
    }
}
/// Calls `fn_type` with `args`, placing the return value in destination.
pub fn call<'tyctx>(
    fn_type: Ty<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut crate::r#type::TyCache,
    span: rustc_span::Span,
) -> CILRoot {
    let fn_type = crate::utilis::monomorphize(&method_instance, fn_type, tyctx);
    let (instance, subst_ref) = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind() {
        let subst = crate::utilis::monomorphize(&method_instance, *subst_ref, tyctx);
        let env = ParamEnv::reveal_all();
        let Some(instance) =
            Instance::resolve(tyctx, env, *def_id, subst).expect("Invalid function def")
        else {
            panic!("ERROR: Could not get function instance. fn type:{fn_type:?}")
        };

        (instance, subst)
    } else {
        todo!("Trying to call a type which is not a function definition!");
    };
    if let rustc_middle::ty::InstanceDef::Virtual(_def, fn_idx) = instance.def {
        assert!(!args.is_empty());
        let fat_ptr_ty =
            crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
        let fat_ptr_type = type_cache.type_from_cache(fat_ptr_ty, tyctx, method_instance);
        let fat_ptr_address =
            operand_address(&args[0].node, tyctx, body, method_instance, type_cache);
        let vtable_ptr = ld_field!(
            fat_ptr_address.clone(),
            FieldDescriptor::new(
                fat_ptr_type.as_dotnet().unwrap(),
                Type::USize,
                "metadata".into()
            )
        );

        let vtable_index =
            ldc_u32!(u32::try_from(fn_idx).expect("More tahn 2^32 functions in a vtable!"));
        let vtable_offset = conv_usize!(vtable_index * size_of!(Type::USize));
        // Get the address of the function ptr, and load it
        let fn_ptr = CILNode::LDIndISize {
            ptr: Box::new(vtable_ptr + vtable_offset),
        };
        // Get the addres of the object
        let obj_ptr = ld_field!(
            fat_ptr_address,
            FieldDescriptor::new(
                fat_ptr_type.as_dotnet().unwrap(),
                Type::Ptr(Type::Void.into()),
                "data_pointer".into()
            )
        );
        // Get the call info
        let call_info = CallInfo::sig_from_instance_(instance, tyctx, type_cache);
        let mut signature = call_info.sig().clone();
        signature.inputs_mut()[0] = Type::ISize;
        let mut call_args = [obj_ptr].to_vec();
        for arg in args.iter().skip(1) {
            call_args.push(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
        }
        assert_eq!(
            signature.inputs().len(),
            call_args.len(),
            "sig:{signature:?} call_args:{call_args:?}"
        );
        let is_ret_void = matches!(signature.output(), crate::r#type::Type::Void);
        return if is_ret_void {
            CILRoot::CallI {
                sig: signature,
                fn_ptr,
                args: call_args.into(),
            }
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                CILNode::CallI(Box::new((signature, fn_ptr, call_args.into()))),
                body,
                method_instance,
                type_cache,
            )
        };
    }
    let call_info = CallInfo::sig_from_instance_(instance, tyctx, type_cache);
    // SHOULD NOT BE MUTABLE BUT VARIADICS ARE FUCKING WIERD.
    let mut signature = call_info.sig().clone();

    let function_name = crate::utilis::function_name(tyctx.symbol_name(instance));
    if crate::utilis::is_fn_intrinsic(fn_type, tyctx) {
        return super::intrinsics::handle_intrinsic(
            &function_name,
            args,
            destination,
            tyctx,
            body,
            method_instance,
            instance,
            type_cache,
            signature,
            span,
        );
    }

    // Checks if function is "magic"
    if function_name.contains(CTOR_FN_NAME) {
        assert!(
            !call_info.split_last_tuple(),
            "Constructors may not use the `rust_call` calling convention!"
        );
        // Constructor
        return call_ctor(
            tyctx,
            subst_ref,
            &function_name,
            args,
            destination,
            body,
            method_instance,
            type_cache,
        );
    } else if function_name.contains(MANAGED_CALL_VIRT_FN_NAME) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed virtual calls may not use the `rust_call` calling convention!"
        );
        // Virtual (for interop)
        return callvirt_managed(
            tyctx,
            subst_ref,
            &function_name,
            args,
            destination,
            body,
            method_instance,
            instance,
            type_cache,
        );
    } else if function_name.contains(MANAGED_CALL_FN_NAME) {
        assert!(
            !call_info.split_last_tuple(),
            "Managed calls may not use the `rust_call` calling convention!"
        );
        // Not-Virtual (for interop)
        return call_managed(
            tyctx,
            subst_ref,
            &function_name,
            args,
            destination,
            body,
            method_instance,
            instance,
            type_cache,
        );
    }
    if call_info.split_last_tuple() {
        return call_closure(
            args,
            destination,
            tyctx,
            signature,
            body,
            &function_name,
            method_instance,
            type_cache,
        );
    }

    let mut call_args = Vec::new();
    for arg in args {
        let res_calc = crate::r#type::tycache::validity_check(
            crate::operand::handle_operand(&arg.node, tyctx, body, method_instance, type_cache),
            crate::utilis::monomorphize(&method_instance, arg.node.ty(body, tyctx), tyctx),
            type_cache,
            method_instance,
            tyctx,
        );
        call_args.push(res_calc);
    }
    if crate::function_sig::is_fn_variadic(fn_type, tyctx) {
        signature.set_inputs(
            args.iter()
                .map(|operand| {
                    type_cache.type_from_cache(
                        crate::utilis::monomorphize(
                            &method_instance,
                            operand.node.ty(body, tyctx),
                            tyctx,
                        ),
                        tyctx,
                        method_instance,
                    )
                })
                .collect(),
        );
    }
    if args.len() < signature.inputs().len() {
        let tpe: crate::r#type::Type = signature.inputs()[signature.inputs().len() - 1].clone();
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
    let is_void = matches!(signature.output(), crate::r#type::Type::Void);
    //rustc_middle::ty::print::with_no_trimmed_paths! {call.push(CILOp::Comment(format!("Calling {instance:?}").into()))};
    if let InstanceDef::DropGlue(_def, None) = instance.def {
        return CILRoot::Nop;
    };
    let call_site = CallSite::new(None, function_name, signature, true);
    // Hande
    if is_void {
        CILRoot::Call {
            site: call_site,
            args: call_args.into(),
        }
    } else {
        let res_calc = crate::r#type::tycache::validity_check(
            call!(call_site, call_args),
            crate::utilis::monomorphize(&method_instance, destination.ty(body, tyctx).ty, tyctx),
            type_cache,
            method_instance,
            tyctx,
        );
        crate::place::place_set(
            destination,
            tyctx,
            res_calc,
            body,
            method_instance,
            type_cache,
        )
    }
}
