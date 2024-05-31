use crate::add_method_from_trees;
use cilly::{
    access_modifier::AccessModifer,
    asm::Assembly,
    basic_block::BasicBlock,
    call,
    call_site::CallSite,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_usize,
    field_desc::FieldDescriptor,
    fn_sig::FnSig,
    ld_field, ldc_u64, lt_un,
    method::{Method, MethodType},
    r#type::Type,
    type_def::TypeDef,
    DotnetTypeRef,
};
use rustc_middle::ty::TyCtxt;
mod atomic;
mod casts;
mod select;
const MAX_ALLOC_SIZE: u64 = u32::MAX as u64;
add_method_from_trees!(
    bounds_check,
    &[Type::USize, Type::USize],
    Type::USize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: lt_un!(CILNode::LDArg(0), CILNode::LDArg(1))
                }
                .into(),
                CILRoot::Call {
                    site: CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[DotnetTypeRef::string_type().into()], Type::Void),
                        true
                    ),
                    args: Box::new([CILNode::LdStr("Bounds check failed. idx:".into())])
                }
                .into(),
                CILRoot::Call {
                    site: CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[Type::U64], Type::Void),
                        true
                    ),
                    args: Box::new([conv_usize!(CILNode::LDArg(0))])
                }
                .into(),
                CILRoot::Call {
                    site: CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[DotnetTypeRef::string_type().into()], Type::Void),
                        true
                    ),
                    args: Box::new([CILNode::LdStr("len:".into())])
                }
                .into(),
                CILRoot::Call {
                    site: CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "WriteLine".into(),
                        FnSig::new(&[Type::U64], Type::Void),
                        true
                    ),
                    args: Box::new([conv_usize!(CILNode::LDArg(1))])
                }
                .into(),
                CILRoot::Throw(CILNode::NewObj {
                    site: CallSite::boxed(
                        Some(
                            DotnetTypeRef::new(
                                Some("System.Runtime"),
                                "System.IndexOutOfRangeException"
                            )
                            .with_valuetype(false)
                        ),
                        ".ctor".into(),
                        FnSig::new(&[], Type::Void),
                        true
                    ),
                    args: [].into()
                })
                .into(),
            ],
            0,
            None
        ),
        BasicBlock::new(
            vec![CILRoot::Ret {
                tree: CILNode::LDArg(0),
            }
            .into()],
            1,
            None
        ),
    ],
    vec![Some("idx".into()), Some("bound".into())]
);

#[macro_export]
macro_rules! add_method_from_trees {
    ($name:ident,$input:expr,$output:expr,$trees:expr,$args:expr) => {
        fn $name(asm: &mut cilly::asm::Assembly) {
            let method = cilly::method::Method::new(
                AccessModifer::MoudlePublic,
                cilly::method::MethodType::Static,
                cilly::fn_sig::FnSig::new($input, $output),
                stringify!($name),
                vec![],
                $trees,
                $args,
            );
            asm.add_method(method);
        }
    };
    ($name:ident,$input:expr,$output:expr,$trees:expr,$locals:expr,$args:expr) => {
        fn $name(asm: &mut cilly::asm::Assembly) {
            let mut method = cilly::method::Method::new(
                AccessModifer::MoudlePublic,
                cilly::method::MethodType::Static,
                cilly::fn_sig::FnSig::new($input, $output),
                stringify!($name),
                $locals.into(),
                $trees,
                $args,
            );
            asm.add_method(method);
        }
    };
}

/// Inserts a small subset of libc and some standard types into an assembly.
pub fn insert_ffi_functions(asm: &mut Assembly, tyctx: TyCtxt) {
    bounds_check(asm);
    atomic::atomics(asm);
    let c_void = crate::r#type::c_void(tyctx);
    asm.add_typedef(TypeDef::new(
        AccessModifer::Public,
        c_void.as_dotnet().unwrap().name_path().into(),
        vec![],
        vec![],
        vec![],
        None,
        0,
        None,
        None,
    ));
    asm.add_typedef(TypeDef::nameonly("Unresolved"));
    asm.add_typedef(TypeDef::nameonly("RustVoid"));
    asm.add_typedef(TypeDef::nameonly("Foreign"));
    asm.add_typedef(TypeDef::nameonly("RustStr"));
    /*asm.add_method(Method::new(AccessModifer::Public,MethodType::Static,FnSig::new(&[Type::U64,Type::U64],Type::U128),"new_u128",vec![],vec![
        BasicBlock::new(vec![CILRoot::Ret{ tree: todo!() }.into()],0,None),
    ]));*/
    //rust_slice(asm);

    casts::casts(asm);
    select::selects(asm);
    //malloc(asm);
    let mut marshal = DotnetTypeRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.Marshal",
    );
    marshal.set_valuetype(false);
    let mut native_mem = DotnetTypeRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.NativeMemory",
    );
    native_mem.set_valuetype(false);
    let mut __rust_alloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::USize, Type::USize], Type::Ptr(Type::U8.into())),
        "__rust_alloc",
        vec![],
        if *crate::config::CHECK_ALLOCATIONS {
            vec![
                BasicBlock::new(
                    vec![CILRoot::BTrue {
                        target: 2,
                        sub_target: 0,
                        cond: lt_un!(conv_usize!(ldc_u64!(MAX_ALLOC_SIZE)), CILNode::LDArg(0)),
                    }
                    .into()],
                    0,
                    None,
                ),
                BasicBlock::new(
                    vec![CILRoot::Ret {
                        tree: call!(CallSite::alloc(), [CILNode::LDArg(0), CILNode::LDArg(1)]),
                    }
                    .into()],
                    1,
                    None,
                ),
                BasicBlock::new(
                    vec![
                        CILRoot::throw(&format!("Max alloc size of {MAX_ALLOC_SIZE} exceeded."))
                            .into(),
                    ],
                    2,
                    None,
                ),
            ]
        } else {
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(CallSite::alloc(), [CILNode::LDArg(0), CILNode::LDArg(1)]),
                }
                .into()],
                0,
                None,
            )]
        },
        vec![Some("size".into()), Some("align".into())],
    );

    asm.add_method(__rust_alloc);
    let mut __rust_alloc_zeroed = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::USize, Type::USize], Type::Ptr(Type::U8.into())),
        "__rust_alloc_zeroed",
        vec![(Some("alloc_ptr".into()), Type::Ptr(Box::new(Type::U8)))],
        if *crate::config::CHECK_ALLOCATIONS {
            vec![
                BasicBlock::new(
                    vec![CILRoot::BTrue {
                        target: 2,
                        sub_target: 0,
                        cond: lt_un!(conv_usize!(ldc_u64!(MAX_ALLOC_SIZE)), CILNode::LDArg(0)),
                    }
                    .into()],
                    0,
                    None,
                ),
                BasicBlock::new(
                    vec![
                        CILRoot::STLoc {
                            local: 0,
                            tree: call!(CallSite::alloc(), [CILNode::LDArg(0), CILNode::LDArg(1)]),
                        }
                        .into(),
                        CILRoot::InitBlk {
                            dst: CILNode::LDLoc(0),
                            val: CILNode::LdcU32(0),
                            count: CILNode::LDArg(0),
                        }
                        .into(),
                        CILRoot::Ret {
                            tree: CILNode::LDLoc(0),
                        }
                        .into(),
                    ],
                    1,
                    None,
                ),
                BasicBlock::new(
                    vec![
                        CILRoot::throw(&format!("Max alloc size of {MAX_ALLOC_SIZE} exceeded."))
                            .into(),
                    ],
                    2,
                    None,
                ),
            ]
        } else {
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(CallSite::alloc(), [CILNode::LDArg(0), CILNode::LDArg(1)]),
                }
                .into()],
                0,
                None,
            )]
        },
        vec![Some("size".into()), Some("align".into())],
    );

    asm.add_method(__rust_alloc_zeroed);
    let mut __rust_dealloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(
            &[Type::Ptr(Type::U8.into()), Type::USize, Type::USize],
            Type::Void,
        ),
        "__rust_dealloc",
        vec![],
        vec![BasicBlock::new(
            vec![
                CILRoot::Call {
                    site: CallSite::new_extern(
                        native_mem,
                        "AlignedFree".into(),
                        FnSig::new(&[Type::Ptr(Type::Void.into())], Type::Void),
                        true,
                    ),
                    args: [CILNode::LDArg(0)].into(),
                }
                .into(),
                CILRoot::VoidRet.into(),
            ],
            0,
            None,
        )],
        vec![
            Some("ptr".into()),
            Some("align".into()),
            Some("size".into()),
        ],
    );
    puts(asm);
    asm.add_method(__rust_dealloc);
    let free = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::Ptr(c_void.clone().into())], Type::Void),
        "free",
        vec![],
        vec![BasicBlock::new(
            vec![
                CILRoot::Call {
                    site: CallSite::new_extern(
                        marshal,
                        "FreeHGlobal".into(),
                        FnSig::new(&[Type::ISize], Type::Void),
                        true,
                    ),
                    args: [CILNode::LDArg(0)].into(),
                }
                .into(),
                CILRoot::VoidRet.into(),
            ],
            0,
            None,
        )],
        vec![Some("ptr".into())],
    );
    let mut __rust_realloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(
            &[
                Type::Ptr(Type::U8.into()),
                Type::USize,
                Type::USize,
                Type::USize,
            ],
            Type::Ptr(Type::U8.into()),
        ),
        "__rust_realloc",
        vec![],
        if *crate::config::CHECK_ALLOCATIONS {
            vec![
                BasicBlock::new(
                    vec![CILRoot::BTrue {
                        target: 2,
                        sub_target: 0,
                        cond: lt_un!(conv_usize!(ldc_u64!(MAX_ALLOC_SIZE)), CILNode::LDArg(0)),
                    }
                    .into()],
                    0,
                    None,
                ),
                BasicBlock::new(
                    vec![CILRoot::Ret {
                        tree: call!(
                            CallSite::realloc(),
                            [CILNode::LDArg(0), CILNode::LDArg(3), CILNode::LDArg(2)]
                        ),
                    }
                    .into()],
                    1,
                    None,
                ),
                BasicBlock::new(
                    vec![
                        CILRoot::throw(&format!("Max alloc size of {MAX_ALLOC_SIZE} exceeded."))
                            .into(),
                    ],
                    2,
                    None,
                ),
            ]
        } else {
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        CallSite::realloc(),
                        [CILNode::LDArg(0), CILNode::LDArg(3), CILNode::LDArg(2)]
                    ),
                }
                .into()],
                0,
                None,
            )]
        },
        vec![
            Some("ptr".into()),
            Some("old_size".into()),
            Some("new_size".into()),
            Some("align".into()),
        ],
    );

    asm.add_method(__rust_realloc);
    asm.add_method(free);
    //TODO: add volatile prefix to volatile loads

    // asm.add_method(volatile_load);
    //atomics::add_atomics(asm);
    //ctpop::add_ctpop(asm);
    // exact_div::add_exact_div(asm);
    //memcmp::add_memcmp(asm);
    //memcmp::add_raw_eq(asm);
    //add_ptr_offset_from_unsigned(asm);
    //caller_location::add_caller_location(asm,tyctx,&mut TyCache::empty());
    pthread_create(asm);
    let unmanaged_start = TypeDef::new(
        AccessModifer::MoudlePublic,
        "UnmanagedThreadStart".into(),
        vec![],
        vec![
            (
                "start_fn".into(),
                Type::DelegatePtr(Box::new(FnSig::new(
                    &[Type::Ptr(Box::new(Type::Void))],
                    Type::Void,
                ))),
            ),
            ("data".into(), Type::Ptr(Box::new(Type::Void))),
        ],
        vec![
            Method::new(
                AccessModifer::MoudlePublic,
                MethodType::Instance,
                FnSig::new(
                    &[
                        Type::DotnetType(Box::new(unmanaged_start())),
                        Type::DelegatePtr(Box::new(FnSig::new(
                            &[Type::Ptr(Box::new(Type::Void))],
                            Type::Void,
                        ))),
                        Type::Ptr(Box::new(Type::Void)),
                    ],
                    Type::Void,
                ),
                ".ctor".into(),
                vec![],
                vec![BasicBlock::new(
                    vec![
                        CILRoot::SetField {
                            addr: CILNode::LDArg(0),
                            value: CILNode::LDArg(1),
                            desc: FieldDescriptor::new(
                                unmanaged_start(),
                                Type::DelegatePtr(Box::new(FnSig::new(
                                    &[Type::Ptr(Box::new(Type::Void))],
                                    Type::Void,
                                ))),
                                "start_fn".into(),
                            ),
                        }
                        .into(),
                        CILRoot::SetField {
                            addr: CILNode::LDArg(0),
                            value: CILNode::LDArg(2),
                            desc: FieldDescriptor::new(
                                unmanaged_start(),
                                Type::Ptr(Box::new(Type::Void)),
                                "data".into(),
                            ),
                        }
                        .into(),
                        CILRoot::VoidRet.into(),
                    ],
                    0,
                    None,
                )],
                vec![Some("start_routine".into()), Some("data".into())],
            ),
            Method::new(
                AccessModifer::MoudlePublic,
                MethodType::Instance,
                FnSig::new(&[Type::DotnetType(Box::new(unmanaged_start()))], Type::Void),
                "Start",
                vec![],
                vec![BasicBlock::new(
                    vec![
                        CILRoot::CallI {
                            sig: FnSig::new(&[Type::Ptr(Box::new(Type::Void))], Type::Void),
                            fn_ptr: ld_field!(
                                CILNode::LDArg(0),
                                FieldDescriptor::new(
                                    unmanaged_start(),
                                    Type::DelegatePtr(Box::new(FnSig::new(
                                        &[Type::Ptr(Box::new(Type::Void))],
                                        Type::Void,
                                    ))),
                                    "start_fn".into(),
                                )
                            ),
                            args: [ld_field!(
                                CILNode::LDArg(0),
                                FieldDescriptor::new(
                                    unmanaged_start(),
                                    Type::Ptr(Box::new(Type::Void)),
                                    "data".into(),
                                )
                            )]
                            .into(),
                        }
                        .into(),
                        CILRoot::VoidRet.into(),
                    ],
                    0,
                    None,
                )],
                vec![],
            ),
        ],
        None,
        0,
        Some(DotnetTypeRef::object_type()),
        None,
    );
    asm.add_typedef(unmanaged_start);
}

add_method_from_trees!(
    puts,
    &[Type::Ptr(Box::new(Type::U8))],
    Type::Void,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    cond: CILNode::LDIndI8 {
                        ptr: CILNode::LDArg(0).into()
                    }
                }
                .into(),
                CILRoot::Call {
                    site: CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[Type::DotnetChar], Type::Void),
                        true
                    ),
                    args: [CILNode::LDIndI8 {
                        ptr: CILNode::LDArg(0).into()
                    }]
                    .into()
                }
                .into(),
                CILRoot::STArg {
                    arg: 0,
                    tree: CILNode::LDArg(0) + ldc_u64!(1)
                }
                .into(),
                CILRoot::GoTo {
                    target: 0,
                    sub_target: 0
                }
                .into(),
            ],
            0,
            None,
        ),
        BasicBlock::new(vec![CILRoot::VoidRet.into(),], 1, None,)
    ],
    vec![Some("str".into())]
);
add_method_from_trees!(
    pthread_create,
    &[
        Type::Ptr(Box::new(Type::ISize)),
        Type::Ptr(Box::new(Type::Void)),
        Type::DelegatePtr(Box::new(FnSig::new(
            &[Type::Ptr(Box::new(Type::Void))],
            Type::Void
        ))),
        Type::Ptr(Box::new(Type::Void))
    ],
    Type::I32,
    vec![BasicBlock::new(
        vec![
            CILRoot::STLoc {
                local: 0,
                tree: CILNode::NewObj {
                    args: [CILNode::NewObj {
                        args: [
                            CILNode::NewObj {
                                site: Box::new(CallSite::new(
                                    Some(unmanaged_start()),
                                    ".ctor".into(),
                                    FnSig::new(
                                        &[
                                            Type::DotnetType(Box::new(unmanaged_start())),
                                            Type::DelegatePtr(Box::new(FnSig::new(
                                                &[Type::Ptr(Box::new(Type::Void))],
                                                Type::Void
                                            ))),
                                            Type::Ptr(Box::new(Type::Void))
                                        ],
                                        Type::Void
                                    ),
                                    false
                                )),
                                args: [CILNode::LDArg(2), CILNode::LDArg(3),].into()
                            },
                            CILNode::LDFtn(Box::new(CallSite::new(
                                Some(unmanaged_start()),
                                "Start".into(),
                                FnSig::new(
                                    &[Type::DotnetType(Box::new(unmanaged_start()))],
                                    Type::Void
                                ),
                                false
                            )))
                        ]
                        .into(),
                        site: Box::new(CallSite::new(
                            Some(DotnetTypeRef::thread_start()),
                            ".ctor".into(),
                            FnSig::new(
                                &[
                                    Type::DotnetType(Box::new(DotnetTypeRef::thread_start())),
                                    Type::DotnetType(Box::new(DotnetTypeRef::object_type())),
                                    Type::ISize
                                ],
                                Type::Void
                            ),
                            false
                        )),
                    }]
                    .into(),
                    site: Box::new(CallSite::new(
                        Some(DotnetTypeRef::thread()),
                        ".ctor".into(),
                        FnSig::new(
                            &[
                                Type::DotnetType(Box::new(DotnetTypeRef::thread())),
                                Type::DotnetType(Box::new(DotnetTypeRef::thread_start())),
                            ],
                            Type::Void
                        ),
                        false
                    )),
                }
            }
            .into(),
            CILRoot::CallVirt {
                site: CallSite::new(
                    Some(DotnetTypeRef::thread()),
                    "Start".into(),
                    FnSig::new(
                        &[Type::DotnetType(Box::new(DotnetTypeRef::thread()))],
                        Type::Void
                    ),
                    false
                ),
                args: [CILNode::LDLoc(0)].into(),
            }
            .into(),
            CILRoot::STIndISize(
                CILNode::LDArg(0),
                CILNode::Call {
                    args: [CILNode::Call {
                        site: Box::new(CallSite::new(
                            Some(DotnetTypeRef::gc_handle()),
                            "Alloc".into(),
                            FnSig::new(
                                &[Type::DotnetType(Box::new(DotnetTypeRef::object_type()))],
                                Type::DotnetType(Box::new(DotnetTypeRef::gc_handle()))
                            ),
                            true
                        )),
                        args: Box::new([CILNode::LDLoc(0)])
                    }]
                    .into(),
                    site: Box::new(CallSite::new(
                        Some(DotnetTypeRef::gc_handle()),
                        "op_Explicit".into(),
                        FnSig::new(
                            &[Type::DotnetType(Box::new(DotnetTypeRef::gc_handle()))],
                            Type::ISize
                        ),
                        true
                    ))
                }
            )
            .into(),
            CILRoot::Ret { tree: ldc_u64!(0) }.into(),
        ],
        0,
        None
    )],
    vec![(
        Some("thread_handle".into()),
        Type::DotnetType(Box::new(DotnetTypeRef::thread()))
    )],
    vec![
        Some("thread".into()),
        Some("attr".into()),
        Some("start_routine".into()),
        Some("arg".into()),
    ]
);
/*
 "pthread_attr_init",
    "pthread_attr_destroy",
    "pthread_attr_setstacksize",
    "pthread_create",
    "pthread_detach",
*/
fn unmanaged_start() -> DotnetTypeRef {
    DotnetTypeRef::new::<&str, _>(None, "UnmanagedThreadStart").with_valuetype(false)
}
