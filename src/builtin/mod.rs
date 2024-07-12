use crate::add_method_from_trees;
use cilly::{
    access_modifier::AccessModifer,
    asm::Assembly,
    asm_exporter::escape_class_name,
    basic_block::{BasicBlock, Handler},
    call,
    call_site::CallSite,
    cil_node::{CILNode, CallOpArgs},
    cil_root::CILRoot,
    conv_u64, conv_usize,
    field_desc::FieldDescriptor,
    fn_sig::FnSig,
    ld_field, ldc_i32, ldc_u32, ldc_u64, lt_un,
    method::{Method, MethodType},
    ptr,
    r#type::Type,
    size_of,
    type_def::TypeDef,
    DotnetTypeRef,
};
use rustc_middle::ty::TyCtxt;
mod atomic;
mod casts;
mod select;
const MAX_ALLOC_SIZE: u64 = u32::MAX as u64;
add_method_from_trees!(
    swap_at_generic,
    &[
        Type::Ptr(Box::new(Type::Void)),
        Type::Ptr(Box::new(Type::Void)),
        Type::USize
    ],
    Type::Void,
    vec![BasicBlock::new(
        vec![
            // Alloc tmp buffer
            CILRoot::STLoc {
                local: 0,
                tree: CILNode::LocAlloc {
                    size: Box::new(CILNode::LDArg(2))
                }
            }
            .into(),
            // Blit loc1 into buffer
            CILRoot::CpBlk {
                dst: Box::new(CILNode::LDLoc(0)),
                src: Box::new(CILNode::LDArg(0)),
                len: Box::new(CILNode::LDArg(2))
            }
            .into(),
            // Blit loc2 into loc1
            CILRoot::CpBlk {
                dst: Box::new(CILNode::LDArg(0)),
                src: Box::new(CILNode::LDArg(1)),
                len: Box::new(CILNode::LDArg(2))
            }
            .into(),
            // Blit buffer into loc2
            CILRoot::CpBlk {
                dst: Box::new(CILNode::LDArg(1)),
                src: Box::new(CILNode::LDLoc(0)),
                len: Box::new(CILNode::LDArg(2))
            }
            .into(),
            CILRoot::VoidRet.into(),
        ],
        0,
        None
    )],
    vec![(Some("loc_buff".into()), Type::Ptr(Box::new(Type::Void)))],
    vec![
        Some("buf1".into()),
        Some("buf2".into()),
        Some("size".into())
    ]
);
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
                    site: Box::new(CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[DotnetTypeRef::string_type().into()], Type::Void),
                        true
                    )),
                    args: Box::new([CILNode::LdStr("Bounds check failed. idx:".into())])
                }
                .into(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[Type::U64], Type::Void),
                        true
                    )),
                    args: Box::new([conv_u64!(CILNode::LDArg(0))])
                }
                .into(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[DotnetTypeRef::string_type().into()], Type::Void),
                        true
                    )),
                    args: Box::new([CILNode::LdStr("len:".into())])
                }
                .into(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "WriteLine".into(),
                        FnSig::new(&[Type::U64], Type::Void),
                        true
                    )),
                    args: Box::new([conv_u64!(CILNode::LDArg(1))])
                }
                .into(),
                CILRoot::Throw(CILNode::NewObj(Box::new(CallOpArgs {
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
                })))
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
                AccessModifer::ModulePublic,
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
            let method = cilly::method::Method::new(
                AccessModifer::ModulePublic,
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
pub fn insert_ffi_functions(asm: &mut Assembly, tcx: TyCtxt) {
    catch_unwind(asm);
    swap_at_generic(asm);
    bounds_check(asm);
    atomic::atomics(asm);
    let c_void = crate::r#type::c_void(tcx);

    asm.add_typedef(TypeDef::new(
        AccessModifer::Public,
        escape_class_name(c_void.as_dotnet().unwrap().name_path()).into(),
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
                    tree: call!(CallSite::alloc(), [CILNode::LDArg(0), CILNode::LDArg(1)])
                        .cast_ptr(ptr!(Type::U8)),
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
                            dst: Box::new(CILNode::LDLoc(0)),
                            val: Box::new(CILNode::LdcU32(0)),
                            count: Box::new(CILNode::LDArg(0)),
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
                    tree: call!(CallSite::alloc(), [CILNode::LDArg(0), CILNode::LDArg(1)])
                        .cast_ptr(ptr!(Type::U8)),
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
                    site: Box::new(CallSite::new_extern(
                        native_mem,
                        "AlignedFree".into(),
                        FnSig::new(&[Type::Ptr(Type::Void.into())], Type::Void),
                        true,
                    )),
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
                    site: Box::new(CallSite::new_extern(
                        marshal,
                        "FreeHGlobal".into(),
                        FnSig::new(&[Type::ISize], Type::Void),
                        true,
                    )),
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
                        cond: lt_un!(conv_usize!(ldc_u64!(MAX_ALLOC_SIZE)), CILNode::LDArg(2)),
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
    //caller_location::add_caller_location(asm,tcx,&mut TyCache::empty());
    pthread_create(asm);
    pthread_attr_init(asm);
    pthread_attr_destroy(asm);
    pthread_attr_setstacksize(asm);
    pthread_detach(asm);
    __cxa_thread_atexit_impl(asm);
    llvm_x86_sse2_pause(asm);
    let rust_exception = TypeDef::new(
        AccessModifer::ModulePublic,
        "RustException".into(),
        vec![],
        vec![("data_pointer".into(), Type::USize)],
        vec![Method::new(
            AccessModifer::ModulePublic,
            MethodType::Instance,
            FnSig::new(
                &[
                    Type::DotnetType(Box::new(
                        DotnetTypeRef::new::<&str, _>(None, "RustException").with_valuetype(false),
                    )),
                    Type::USize,
                ],
                Type::Void,
            ),
            ".ctor",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::SetField {
                        addr: Box::new(CILNode::LDArg(0)),
                        value: Box::new(CILNode::LDArg(1)),
                        desc: Box::new(FieldDescriptor::new(
                            DotnetTypeRef::new::<&str, _>(None, "RustException")
                                .with_valuetype(false),
                            Type::USize,
                            "data_pointer".into(),
                        )),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
            vec![Some("data_pointer".into())],
        )],
        None,
        0,
        Some(DotnetTypeRef::exception()),
        None,
    );
    asm.add_typedef(rust_exception);
    let unmanaged_start = TypeDef::new(
        AccessModifer::ModulePublic,
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
                AccessModifer::ModulePublic,
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
                ".ctor",
                vec![],
                vec![BasicBlock::new(
                    vec![
                        CILRoot::SetField {
                            addr: Box::new(CILNode::LDArg(0)),
                            value: Box::new(CILNode::LDArg(1)),
                            desc: Box::new(FieldDescriptor::new(
                                unmanaged_start(),
                                Type::DelegatePtr(Box::new(FnSig::new(
                                    &[Type::Ptr(Box::new(Type::Void))],
                                    Type::Void,
                                ))),
                                "start_fn".into(),
                            )),
                        }
                        .into(),
                        CILRoot::SetField {
                            addr: Box::new(CILNode::LDArg(0)),
                            value: Box::new(CILNode::LDArg(2)),
                            desc: Box::new(FieldDescriptor::new(
                                unmanaged_start(),
                                Type::Ptr(Box::new(Type::Void)),
                                "data".into(),
                            )),
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
                AccessModifer::ModulePublic,
                MethodType::Instance,
                FnSig::new(&[Type::DotnetType(Box::new(unmanaged_start()))], Type::Void),
                "Start",
                vec![],
                vec![BasicBlock::new(
                    vec![
                        CILRoot::CallI {
                            sig: Box::new(FnSig::new(
                                &[Type::Ptr(Box::new(Type::Void))],
                                Type::Void,
                            )),
                            fn_ptr: Box::new(ld_field!(
                                CILNode::LDArg(0),
                                FieldDescriptor::new(
                                    unmanaged_start(),
                                    Type::DelegatePtr(Box::new(FnSig::new(
                                        &[Type::Ptr(Box::new(Type::Void))],
                                        Type::Void,
                                    ))),
                                    "start_fn".into(),
                                )
                            )),
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
                    site: Box::new(CallSite::new_extern(
                        DotnetTypeRef::console(),
                        "Write".into(),
                        FnSig::new(&[Type::DotnetChar], Type::Void),
                        true
                    )),
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
                tree: CILNode::NewObj(Box::new(CallOpArgs {
                    args: [CILNode::NewObj(Box::new(CallOpArgs {
                        args: [
                            CILNode::NewObj(Box::new(CallOpArgs {
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
                            })),
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
                    }))]
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
                }))
            }
            .into(),
            CILRoot::CallVirt {
                site: Box::new(CallSite::new(
                    Some(DotnetTypeRef::thread()),
                    "Start".into(),
                    FnSig::new(
                        &[Type::DotnetType(Box::new(DotnetTypeRef::thread()))],
                        Type::Void
                    ),
                    false
                )),
                args: [CILNode::LDLoc(0)].into(),
            }
            .into(),
            CILRoot::STIndI64(
                CILNode::LDArg(0),
                conv_u64!(call!(
                    CallSite::new(
                        Some(DotnetTypeRef::thread()),
                        "get_ManagedThreadId".into(),
                        FnSig::new(
                            &[Type::DotnetType(Box::new(DotnetTypeRef::thread()))],
                            Type::I32
                        ),
                        false
                    ),
                    [CILNode::LDLoc(0)]
                ))
            )
            .into(),
            CILRoot::Ret { tree: ldc_i32!(0) }.into(),
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
add_method_from_trees!(
    pthread_attr_init,
    &[Type::Ptr(Box::new(Type::ISize)),],
    Type::I32,
    vec![BasicBlock::new(
        vec![
            CILRoot::InitBlk {
                dst: Box::new(CILNode::LDArg(0)),
                val: Box::new(ldc_u32!(0)),
                count: Box::new(size_of!(Type::USize) * ldc_i32!(3))
            }
            .into(),
            CILRoot::Ret { tree: ldc_i32!(0) }.into()
        ],
        0,
        None
    )],
    vec![],
    vec![Some("thread_attr".into()),]
);
add_method_from_trees!(
    pthread_attr_setstacksize,
    &[Type::Ptr(Box::new(Type::ISize)), Type::USize],
    Type::I32,
    vec![BasicBlock::new(
        vec![
            CILRoot::STIndISize(
                CILNode::LDArg(0) + size_of!(Type::USize) * ldc_i32!(2),
                CILNode::LDArg(1)
            )
            .into(),
            CILRoot::Ret { tree: ldc_i32!(0) }.into()
        ],
        0,
        None
    )],
    vec![],
    vec![Some("thread_attr".into()), Some("size".into())]
);

// detaching a thread in .NET does nothing(since the runtime manages everyting) - so this is safe.
add_method_from_trees!(
    pthread_detach,
    &[Type::ISize],
    Type::I32,
    vec![BasicBlock::new(
        vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
        0,
        None
    )],
    vec![],
    vec![Some("thread_attr".into()), Some("size".into())]
);
add_method_from_trees!(
    pthread_attr_destroy,
    &[Type::Ptr(Box::new(Type::ISize)),],
    Type::I32,
    vec![BasicBlock::new(
        vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
        0,
        None
    )],
    vec![],
    vec![Some("thread_attr".into()),]
);
fn unmanaged_start() -> DotnetTypeRef {
    DotnetTypeRef::new::<&str, _>(None, "UnmanagedThreadStart").with_valuetype(false)
}
// TODO: Can't yet register thread-local deconstructors.
add_method_from_trees!(
    __cxa_thread_atexit_impl,
    [
        Type::DelegatePtr(Box::new(FnSig::new(
            [Type::Ptr(Box::new(Type::Void))],
            Type::Void
        ))),
        Type::Ptr(Box::new(Type::Void)),
        Type::Ptr(Box::new(Type::Void))
    ],
    Type::Void,
    vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
    vec![],
    vec![
        Some("dtor".into()),
        Some("obj".into()),
        Some("dso_handle".into())
    ]
);
// TODO: this instruction waits for a small ammount of time. Implementing it could improve performance.
fn llvm_x86_sse2_pause(asm: &mut cilly::asm::Assembly) {
    let method = cilly::method::Method::new(
        AccessModifer::ModulePublic,
        cilly::method::MethodType::Static,
        cilly::fn_sig::FnSig::new([], Type::Void),
        "llvm.x86.sse2.pause",
        vec![],
        vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
        vec![],
    );
    asm.add_method(method);
}
add_method_from_trees!(
    catch_unwind,
    &[
        Type::DelegatePtr(Box::new(FnSig::new(&[ptr!(Type::U8)], Type::Void))),
        ptr!(Type::U8),
        Type::DelegatePtr(Box::new(FnSig::new(
            &[ptr!(Type::U8), ptr!(Type::U8)],
            Type::Void
        ))),
    ],
    Type::I32,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::CallI {
                    sig: Box::new(FnSig::new(&[ptr!(Type::U8)], Type::Void)),
                    fn_ptr: Box::new(CILNode::LDArg(0)),
                    args: Box::new([CILNode::LDArg(1)])
                }
                .into(),
                CILRoot::JumpingPad {
                    source: 0,
                    target: 3
                }
                .into()
            ],
            0,
            Some(Handler::Blocks(vec![
                BasicBlock::new(
                    vec![
                        CILRoot::STLoc {
                            local: 1,
                            tree: CILNode::GetException,
                        }
                        .into(),
                        CILRoot::BFalse {
                            target: 0,
                            sub_target: 4,
                            cond: CILNode::IsInst(Box::new((
                                CILNode::LDLoc(1),
                                DotnetTypeRef::new::<&str, _>(None, "RustException")
                                    .with_valuetype(false)
                            ))),
                        }
                        .into(),
                        CILRoot::STLoc {
                            local: 0,
                            tree: ld_field!(
                                CILNode::CheckedCast(Box::new((
                                    CILNode::LDLoc(1),
                                    DotnetTypeRef::new::<&str, _>(None, "RustException")
                                        .with_valuetype(false)
                                ))),
                                FieldDescriptor::new(
                                    DotnetTypeRef::new::<&str, _>(None, "RustException")
                                        .with_valuetype(false),
                                    Type::USize,
                                    "data_pointer".into()
                                )
                            ),
                        }
                        .into(),
                        CILRoot::CallI {
                            sig: Box::new(FnSig::new(
                                &[ptr!(Type::U8), ptr!(Type::U8)],
                                Type::Void
                            )),
                            fn_ptr: Box::new(CILNode::LDArg(2)),
                            args: Box::new([CILNode::LDArg(1), CILNode::LDLoc(0)])
                        }
                        .into(),
                        CILRoot::GoTo {
                            target: 0,
                            sub_target: 2
                        }
                        .into()
                    ],
                    1,
                    None
                ),
                BasicBlock::new(
                    vec![
                        CILRoot::debug("Can't yet cacth .NET exceptions.").into(),
                        CILRoot::Call {
                            site: Box::new(CallSite::new_extern(
                                DotnetTypeRef::console(),
                                "WriteLine".into(),
                                FnSig::new(
                                    &[Type::DotnetType(Box::new(DotnetTypeRef::object_type()))],
                                    Type::Void
                                ),
                                true
                            )),
                            args: Box::new([CILNode::LDLoc(1)])
                        }
                        .into(),
                        CILRoot::CallI {
                            sig: Box::new(FnSig::new(
                                &[ptr!(Type::U8), ptr!(Type::U8)],
                                Type::Void
                            )),
                            fn_ptr: Box::new(CILNode::LDArg(2)),
                            args: Box::new([CILNode::LDArg(1), conv_usize!(ldc_u32!(0))])
                        }
                        .into(),
                        CILRoot::JumpingPad {
                            source: 0,
                            target: 2
                        }
                        .into()
                    ],
                    4,
                    None
                )
            ]))
        ),
        BasicBlock::new(vec![CILRoot::Ret { tree: ldc_i32!(1) }.into()], 2, None),
        BasicBlock::new(vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()], 3, None)
    ],
    vec![
        (Some("data_ptr".into()), Type::USize),
        (
            Some("exception".into()),
            Type::DotnetType(Box::new(DotnetTypeRef::exception())),
        )
    ],
    vec![
        Some("try_fn".into()),
        Some("data".into()),
        Some("catch_fn".into()),
    ]
);
