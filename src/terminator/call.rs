use crate::{
    call_info::CallInfo,
    cil::FieldDescriptor,
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    interop::AssemblyRef,
    r#type::DotnetTypeRef,
    utilis::garg_to_string,
    utilis::CTOR_FN_NAME,
    utilis::MANAGED_CALL_FN_NAME,
    utilis::MANAGED_CALL_VIRT_FN_NAME,
};
use rustc_middle::{
    mir::{Body, Operand, Place, SwitchTargets, Terminator, TerminatorKind},
    ty::{GenericArg, Instance, InstanceDef, ParamEnv, Ty, TyCtxt, TyKind},
};
use rustc_span::source_map::Spanned;
fn decode_interop_call<'tyctx>(
    function_name: &str,
    prefix: &str,
    subst_ref: &[GenericArg<'tyctx>],
    tyctx: TyCtxt<'tyctx>,
) -> CallSite {
    let argument_count = argc_from_fn_name(function_name, MANAGED_CALL_FN_NAME);
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], tyctx);
    let asm = asm.name();
    let class_name = garg_to_string(subst_ref[1], tyctx);
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], tyctx);
    let managed_fn_name = garg_to_string(subst_ref[3], tyctx);
    todo!();
}
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
) -> Vec<CILOp> {
    let argument_count = argc_from_fn_name(function_name, MANAGED_CALL_FN_NAME);
    //FIXME: figure out the proper argc.
    //assert!(subst_ref.len() as u32 == argc + 3 || subst_ref.len() as u32 == argc + 4);
    assert!(args.len() == argument_count as usize);
    let asm = AssemblyRef::decode_assembly_ref(subst_ref[0], tyctx);
    let asm = asm.name();
    let class_name = garg_to_string(subst_ref[1], tyctx);
    let is_valuetype = crate::utilis::garag_to_bool(subst_ref[2], tyctx);
    let managed_fn_name = garg_to_string(subst_ref[3], tyctx);
    let mut tpe = DotnetTypeRef::new(asm.as_deref(), &class_name);
    tpe.set_valuetype(is_valuetype);
    let signature = FnSig::sig_from_instance_(fn_instance, tyctx, type_cache)
        .expect("Can't get the function signature");

    if argument_count == 0 {
        let ret = crate::r#type::Type::Void;
        let call = vec![CILOp::Call(CallSite::boxed(
            Some(tpe.clone()),
            managed_fn_name.into(),
            FnSig::new(&[], &ret),
            true,
        ))];
        if *signature.output() == crate::r#type::Type::Void {
            call
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call,
                method,
                method_instance,
                type_cache,
            )
        }
    } else {
        let is_static = crate::utilis::garag_to_bool(subst_ref[4], tyctx);

        let mut call = Vec::new();
        for arg in args {
            call.extend(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
        }
        call.push(CILOp::Call(CallSite::boxed(
            Some(tpe.clone()),
            managed_fn_name.into(),
            signature.clone(),
            is_static,
        )));
        if *signature.output() == crate::r#type::Type::Void {
            call
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call,
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
) -> Vec<CILOp> {
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

    let mut tpe = DotnetTypeRef::new(asm.as_deref(), &class_name);
    tpe.set_valuetype(is_valuetype);
    let signature = FnSig::sig_from_instance_(fn_instance, tyctx, type_cache)
        .expect("Can't get the function signature");
    if argument_count == 0 {
        let ret = crate::r#type::Type::Void;
        let call = vec![CILOp::Call(CallSite::boxed(
            Some(tpe.clone()),
            managed_fn_name.into(),
            FnSig::new(&[], &ret),
            true,
        ))];
        if *signature.output() == crate::r#type::Type::Void {
            call
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call,
                method,
                method_instance,
                type_cache,
            )
        }
    } else {
        let is_static = crate::utilis::garag_to_bool(subst_ref[4], tyctx);

        let mut call = Vec::new();
        for arg in args {
            call.extend(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
        }
        call.push(CILOp::CallVirt(CallSite::boxed(
            Some(tpe.clone()),
            managed_fn_name.into(),
            signature.clone(),
            is_static,
        )));
        if *signature.output() == crate::r#type::Type::Void {
            call
        } else {
            crate::place::place_set(
                destination,
                tyctx,
                call,
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
) -> Vec<CILOp> {
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
    let mut tpe = DotnetTypeRef::new(asm.as_deref(), &class_name);
    tpe.set_valuetype(is_valuetype);
    // If no arguments, inputs don't have to be handled, so a simpler call handling is used.
    if argument_count == 0 {
        crate::place::place_set(
            destination,
            tyctx,
            vec![CILOp::NewObj(CallSite::boxed(
                Some(tpe.clone()),
                ".ctor".into(),
                FnSig::new(&[], &crate::r#type::Type::Void),
                false,
            ))],
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
                    Some(method_instance),
                )
            })
            .collect();
        inputs.insert(0, tpe.clone().into());
        let sig = FnSig::new(&inputs, &crate::r#type::Type::Void);
        let mut call = Vec::new();
        for arg in args {
            call.extend(crate::operand::handle_operand(
                &arg.node,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
        }
        call.push(CILOp::NewObj(CallSite::boxed(
            Some(tpe.clone()),
            ".ctor".into(),
            sig,
            false,
        )));
        crate::place::place_set(
            destination,
            tyctx,
            call,
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
) -> Vec<CILOp> {
    let mut call = Vec::new();
    let last_arg = args
        .get(args.len() - 1)
        .expect("Closure must be called with at least 2 arguments(closure + arg tuple)");
    let other_args = &args[..args.len() - 1];
    for arg in other_args {
        call.extend(crate::operand::handle_operand(
            &arg.node,
            tyctx,
            body,
            method_instance,
            type_cache,
        ));
    }
    let last_arg_type =
        crate::utilis::monomorphize(&method_instance, last_arg.node.ty(body, tyctx), tyctx);
    match last_arg_type.kind() {
        TyKind::Tuple(elements) => {
            if elements.is_empty() {
            } else {
                call.extend(crate::operand::handle_operand(
                    &last_arg.node,
                    tyctx,
                    body,
                    method_instance,
                    type_cache,
                ));
                let tuple_type =
                    type_cache.type_from_cache(last_arg_type, tyctx, Some(method_instance));
                call.push(CILOp::NewTMPLocal(tuple_type.clone().into()));
                call.push(CILOp::SetTMPLocal);
                for (index, element) in elements.iter().enumerate() {
                    call.push(CILOp::LoadAddresOfTMPLocal);
                    let element_type =
                        type_cache.type_from_cache(element, tyctx, Some(method_instance));
                    let tuple_element_name = format!("Item{}", index + 1);
                    let field_descriptor = FieldDescriptor::boxed(
                        tuple_type.as_dotnet().expect("Invalid tuple type"),
                        element_type,
                        tuple_element_name.into(),
                    );
                    call.push(CILOp::LDField(field_descriptor));
                }
                call.push(CILOp::FreeTMPLocal);
                //todo!("Can't unbox tupels yet!")
            }
        }
        _ => panic!("Can't unbox type {last_arg_type:?}!"),
    }
    //panic!("Last arg:{last_arg:?}last_arg_type:{last_arg_type:?}");
    //assert_eq!(args.len(),signature.inputs().len(),"CALL SIGNATURE ARG COUNT MISMATCH!");
    let is_void = matches!(sig.output(), crate::r#type::Type::Void);
    call.push(CILOp::Call(CallSite::boxed(
        None,
        function_name.into(),
        sig,
        true,
    )));
    // Hande
    if is_void {
        call
    } else {
        crate::place::place_set(destination, tyctx, call, body, method_instance, type_cache)
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
) -> Vec<CILOp> {
    let fn_type = crate::utilis::monomorphize(&method_instance, fn_type, tyctx);
    let (instance, subst_ref) = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind() {
        let subst = crate::utilis::monomorphize(&method_instance, *subst_ref, tyctx);
        let env = ParamEnv::reveal_all();
        let Some(instance) =
            Instance::resolve(tyctx, env, *def_id, subst).expect("Invalid function def")
        else {
            // I assume most functions wihc can't be resolved are empty drops.
            // There propably exists a better way to check if it REALLY is just an empty drop, but this is good enough for now.
            /*
            if rustc_middle::ty::print::with_no_trimmed_paths! {format!("{fn_type:?}")}
                .contains("drop")
            {
                rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!("Empty drop {fn_type:?}")};
                return vec![];
            }*/
            panic!("ERROR: Could not get function instance. fn type:{fn_type:?}")
        };

        (instance, subst)
    } else {
        todo!("Trying to call a type which is not a function definition!");
    };

    let call_info = CallInfo::sig_from_instance_(instance, tyctx, type_cache)
        .expect("Could not resolve function sig");

    let signature = call_info.sig().clone();

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
        );
    }
    if function_name.contains("map_or_else") {
        println!(
            "Calling function with name {function_name}. DefId:{:?}, Subst:{:?}",
            instance.def, instance.args
        );
        let sig = crate::utilis::monomorphize(&method_instance, fn_type.fn_sig(tyctx), tyctx);
        rustc_middle::ty::print::with_no_trimmed_paths! {println!("map_or_else_sig:{:?}",sig)};
        for input in call_info.sig().inputs() {
            println!("\t{input:?}");
        }
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

    let mut call = Vec::new();
    for arg in args {
        call.extend(crate::operand::handle_operand(
            &arg.node,
            tyctx,
            body,
            method_instance,
            type_cache,
        ));
    }

    if args.len() < signature.inputs().len() {
        let tpe: crate::r#type::Type = signature.inputs()[signature.inputs().len() - 1].clone();
        // let arg_len = args.len();
        //assert_eq!(args.len() + 1,signature.inputs().len(),"ERROR: mismatched argument count. \nsignature inputs:{:?} \narguments:{args:?}\narg_len:{arg_len}\n",signature.inputs());
        // assert_eq!(signature.inputs()[signature.inputs().len() - 1],tpe);
        //FIXME:This assembles a panic location from uninitialized memory. This WILL lead to bugs once unwinding is added. The fields `file`,`col`, and `line` should be set there.
        call.extend([
            CILOp::NewTMPLocal(Box::new(tpe)),
            CILOp::LoadTMPLocal,
            CILOp::FreeTMPLocal,
        ]);
        //panic!("Call with PanicLocation!");
    }
    //assert_eq!(args.len(),signature.inputs().len(),"CALL SIGNATURE ARG COUNT MISMATCH!");
    let is_void = matches!(signature.output(), crate::r#type::Type::Void);
    rustc_middle::ty::print::with_no_trimmed_paths! {call.push(CILOp::Comment(format!("Calling {instance:?}").into()))};
    match instance.def {
        InstanceDef::DropGlue(def, None) => return vec![],
        _ => (),
    }
    call.push(CILOp::Call(CallSite::boxed(
        None,
        function_name,
        signature,
        true,
    )));
    // Hande
    if is_void {
        call
    } else {
        crate::place::place_set(destination, tyctx, call, body, method_instance, type_cache)
    }
}
