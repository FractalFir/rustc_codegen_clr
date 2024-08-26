use cilly::{
    v2::{
        asm::MissingMethodPatcher, cilnode::MethodKind, Assembly, CILNode, CILRoot, Int, MethodRef,
        SigIdx, Type,
    },
    IString,
};

/*
pub fn patch_all(asm: &mut cilly::asm::Assembly) {
    let _ = asm;
    //println!("Applying patches to the resulting assembly...");
    //hijack_arg_init(asm);
}
/// Fixes calls to `pthread_create`
pub fn override_pthread_create(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_create",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        Box::new(CallSite::builtin(
                            "pthread_create".into(),
                            FnSig::new(
                                &[
                                    ptr!(Type::Int(Int::ISize)),
                                    ptr!(Type::Void),
                                    Type::FnPtr(Box::new(FnSig::new(
                                        &[ptr!(Type::Void)],
                                        ptr!(Type::Void),
                                    ))),
                                    ptr!(Type::Void),
                                ],
                                Type::Int(Int::I32),
                            ),
                            true,
                        )),
                        [
                            CILNode::LDArg(0),
                            CILNode::LDArg(1),
                            CILNode::LDIndISize {
                                ptr: Box::new(CILNode::LDArgA(2)),
                            },
                            CILNode::LDArg(3),
                        ]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![
                Some("thread".into()),
                Some("attr".into()),
                Some("start_routine".into()),
                Some("arg".into()),
            ],
        ),
    );
}
/// Fixes calls to `pthread_attr_init`
pub fn override_pthread_attr_init(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_attr_init",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        Box::new(CallSite::builtin(
                            "pthread_attr_init".into(),
                            FnSig::new(&[ptr!(Type::Int(Int::ISize))], Type::Int(Int::I32)),
                            true,
                        )),
                        [CILNode::LDArg(0)]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("attr".into())],
        ),
    );
}
/// Fixes calls to `pthread_join`
pub fn override_pthread_join(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_join",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        Box::new(CallSite::builtin(
                            "pthread_join".into(),
                            FnSig::new(&[Type::Int(Int::ISize), ptr!(ptr!(Type::Void))], Type::Int(Int::I32)),
                            true,
                        )),
                        [CILNode::LDArg(0), CILNode::LDArg(1)]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("thread".into()), Some("res_ptr".into())],
        ),
    );
}
/// Fixes calls to `pthread_attr_setstacksize`
pub fn override_pthread_attr_setstacksize(
    patched: &mut FxHashMap<CallSite, Method>,
    call: &CallSite,
) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_attr_setstacksize",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        Box::new(CallSite::builtin(
                            "pthread_attr_setstacksize".into(),
                            FnSig::new(&[ptr!(Type::Int(Int::ISize).into()), Type::Int(Int::USize)], Type::Int(Int::I32)),
                            true,
                        )),
                        [CILNode::LDArg(0), CILNode::LDArg(1)]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("attr".into()), Some("size".into())],
        ),
    );
}
/// Fixes calls to `pthread_detach`
pub fn override_pthread_detach(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_detach",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        Box::new(CallSite::builtin(
                            "pthread_detach".into(),
                            FnSig::new(&[Type::Int(Int::ISize)], Type::Int(Int::I32)),
                            true,
                        )),
                        [CILNode::LDArg(0)]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("attr".into())],
        ),
    );
}

pub(crate) fn pthread_attr_destroy(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_attr_destroy",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
                0,
                None,
            )],
            vec![Some("attr".into())],
        ),
    );
}
pub(crate) fn pthread_self(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_self",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
                0,
                None,
            )],
            vec![Some("attr".into())],
        ),
    );
}
pub(crate) fn pthread_setname_np(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_setname_np",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
                0,
                None,
            )],
            vec![Some("attr".into())],
        ),
    );
}*/
pub fn call_alias(
    overrides: &mut MissingMethodPatcher,
    asm: &mut Assembly,
    name: impl Into<IString>,
    call: MethodRef,
) {
    overrides.insert(
        asm.alloc_string(name),
        Box::new(move |original, asm| {
            let method_ref = asm.alloc_methodref(call.clone());
            let inputs: Box<[_]> = asm.get_sig(call.sig()).inputs().into();
            let original_inputs: Box<[_]> =
                asm.get_sig(asm.get_mref(original).sig()).inputs().into();
            let args = inputs
                .iter()
                .zip(original_inputs.iter())
                .enumerate()
                .map(|(arg, (tpe, original_tpe))| {
                    if tpe == original_tpe {
                        asm.alloc_node(CILNode::LdArg(arg as u32))
                    } else {
                        match (tpe, original_tpe) {
                            (
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_),
                                Type::ClassRef(_),
                            ) => {
                                // Transmute to a pointer
                                let ptr_address = asm.alloc_node(CILNode::LdArgA(arg as u32));
                                let tpe = asm.alloc_type(*tpe);
                                asm.alloc_node(CILNode::LdInd {
                                    addr: ptr_address,
                                    tpe,
                                    volitale: false,
                                })
                            }
                            (
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize),
                                Type::Int(Int::U64),
                            ) => {
                                let arg = asm.alloc_node(CILNode::LdArg(arg as u32));

                                asm.alloc_node(CILNode::IntCast {
                                    input: arg,
                                    target: Int::USize,
                                    extend: cilly::v2::cilnode::ExtendKind::ZeroExtend,
                                })
                            }
                            (
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_),
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_),
                            ) | (Type::Int(Int::I64),Type::Int(Int::U64)) => asm.alloc_node(CILNode::LdArg(arg as u32)),
                            _ => todo!("can't auto convert {original_tpe:?} to {tpe:?} when autogenrating wrappers."),
                        }
                    }
                })
                .collect();
            if *asm.get_sig(call.sig()).output() == Type::Void {
                let call = asm.alloc_root(CILRoot::Call(Box::new((method_ref, args))));
                let ret = asm.alloc_root(CILRoot::VoidRet);
                cilly::v2::MethodImpl::MethodBody {
                    blocks: vec![cilly::v2::BasicBlock::new(vec![call, ret], 0, None)],
                    locals: vec![],
                }
            } else {
                let ret_value = asm.alloc_node(CILNode::Call(Box::new((method_ref, args))));
                let ret = asm.alloc_root(CILRoot::Ret(ret_value));
                cilly::v2::MethodImpl::MethodBody {
                    blocks: vec![cilly::v2::BasicBlock::new(vec![ret], 0, None)],
                    locals: vec![],
                }
            }
        }),
    );
}
pub fn builtin_call(
    overrides: &mut MissingMethodPatcher,
    asm: &mut Assembly,
    name: impl Into<IString> + Clone,
    sig: SigIdx,
) {
    let main = asm.main_module();
    let call = asm.new_methodref(*main, name.clone(), sig, MethodKind::Static, []);
    let call = asm.get_mref(call).clone();
    call_alias(overrides, asm, name, call)
}
