use cilly::{
    access_modifier,
    basic_block::BasicBlock,
    call,
    call_site::CallSite,
    cil_node::CILNode,
    cil_root::CILRoot,
    ldc_i32,
    method::{Method, MethodType},
    ptr, FnSig, Type,
};
use fxhash::FxHashMap;

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
                                    ptr!(Type::ISize),
                                    ptr!(Type::Void),
                                    Type::DelegatePtr(Box::new(FnSig::new(
                                        &[ptr!(Type::Void)],
                                        ptr!(Type::Void),
                                    ))),
                                    ptr!(Type::Void),
                                ],
                                Type::I32,
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
                            FnSig::new(&[ptr!(Type::ISize)], Type::I32),
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
                            FnSig::new(&[Type::ISize, ptr!(ptr!(Type::Void))], Type::I32),
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
                            FnSig::new(&[ptr!(Type::ISize.into()), Type::USize], Type::I32),
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
                            FnSig::new(&[Type::ISize], Type::I32),
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
