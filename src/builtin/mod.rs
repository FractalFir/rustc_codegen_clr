use crate::add_method_from_trees;
use cilly::{
    access_modifier::AccessModifer,
    asm::Assembly,
    basic_block::{BasicBlock, Handler},
    call,
    call_site::CallSite,
    call_virt,
    cil_node::{CILNode, CallOpArgs},
    cil_root::CILRoot,
    conv_isize, conv_u64, conv_usize,
    field_desc::FieldDescriptor,
    fn_sig::FnSig,
    ld_field, ldc_i32, ldc_u32, ldc_u64, lt_un,
    method::{Method, MethodType},
    ptr,
    r#type::Type,
    shl, shr, size_of, source_info,
    static_field_desc::StaticFieldDescriptor,
    type_def::ClassDef,
    utilis::escape_class_name,
    ClassRef,
};
use rustc_middle::ty::TyCtxt;
mod atomic;
mod casts;
mod select;
const MAX_ALLOC_SIZE: u64 = u32::MAX as u64;
add_method_from_trees!(
    swap_at_generic,
    &[ptr!(Type::Void), ptr!(Type::Void), Type::Int(Int::USize)],
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
    vec![(Some("loc_buff".into()), ptr!(Type::Void))],
    vec![
        Some("buf1".into()),
        Some("buf2".into()),
        Some("size".into())
    ]
);
add_method_from_trees!(
    bounds_check,
    &[Type::Int(Int::USize), Type::Int(Int::USize)],
    Type::Int(Int::USize),
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
                        ClassRef::console(),
                        "Write".into(),
                        FnSig::new(&[ClassRef::string_type().into()], Type::Void),
                        true
                    )),
                    args: Box::new([CILNode::LdStr("Bounds check failed. idx:".into())])
                }
                .into(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        ClassRef::console(),
                        "Write".into(),
                        FnSig::new(&[Type::Int(Int::U64)], Type::Void),
                        true
                    )),
                    args: Box::new([conv_u64!(CILNode::LDArg(0))])
                }
                .into(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        ClassRef::console(),
                        "Write".into(),
                        FnSig::new(&[ClassRef::string_type().into()], Type::Void),
                        true
                    )),
                    args: Box::new([CILNode::LdStr("len:".into())])
                }
                .into(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        ClassRef::console(),
                        "WriteLine".into(),
                        FnSig::new(&[Type::Int(Int::U64)], Type::Void),
                        true
                    )),
                    args: Box::new([conv_u64!(CILNode::LDArg(1))])
                }
                .into(),
                CILRoot::Throw(CILNode::NewObj(Box::new(CallOpArgs {
                    site: CallSite::boxed(
                        Some(
                            ClassRef::new(
                                Some("System.Runtime"),
                                "System.IndexOutOfRangeException"
                            )
                            .with_valuetype(false)
                        ),
                        ".ctor".into(),
                        FnSig::new(
                            &[Type::ClassRef(Box::new(
                                ClassRef::new(
                                    Some("System.Runtime"),
                                    "System.IndexOutOfRangeException"
                                )
                                .with_valuetype(false)
                            ))],
                            Type::Void
                        ),
                        false
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
        #[allow(non_snake_case)]
        fn $name(asm: &mut cilly::asm::Assembly) {
            let method = cilly::method::Method::new(
                AccessModifer::Public,
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
                AccessModifer::Public,
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
    bitreverse_u128(asm);
    bitreverse_u64(asm);
    bitreverse_u32(asm);
    let c_void = crate::r#type::c_void(tcx);

    asm.add_typedef(ClassDef::new(
        AccessModifer::Public,
        escape_class_name(c_void.as_class_ref().unwrap().name_path()).into(),
        vec![],
        vec![],
        vec![],
        None,
        0,
        None,
        None,
    ));
    asm.add_typedef(ClassDef::nameonly("Unresolved"));
    asm.add_typedef(ClassDef::nameonly("RustVoid"));
    asm.add_typedef(ClassDef::nameonly("Foreign"));
    asm.add_typedef(ClassDef::nameonly("RustStr"));
    /*asm.add_method(Method::new(AccessModifer::Public,MethodType::Static,FnSig::new(&[Type::Int(Int::U64),Type::Int(Int::U64)],Type::Int(Int::U128)),"new_u128",vec![],vec![
        BasicBlock::new(vec![CILRoot::Ret{ tree: todo!() }.into()],0,None),
    ]));*/
    //rust_slice(asm);

    casts::casts(asm);
    select::selects(asm);
    let mut marshal = ClassRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.Marshal",
    );
    marshal.set_valuetype(false);
    let mut native_mem = ClassRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.NativeMemory",
    );
    native_mem.set_valuetype(false);
    let mut __rust_alloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(
            &[Type::Int(Int::USize), Type::Int(Int::USize)],
            ptr!(Type::Int(Int::U8)),
        ),
        "__rust_alloc",
        vec![(None, ptr!(Type::Int(Int::U8)))],
        vec![
            BasicBlock::new(
                vec![
                    CILRoot::STLoc {
                        local: 0,
                        tree: call!(
                            CallSite::aligned_alloc(),
                            [CILNode::LDArg(0), CILNode::LDArg(1)]
                        )
                        .cast_ptr(ptr!(Type::Int(Int::U8))),
                    }
                    .into(),
                    CILRoot::GoTo {
                        target: 0,
                        sub_target: 2,
                    }
                    .into(),
                    CILRoot::JumpingPad {
                        source: 0,
                        target: 2,
                    }
                    .into(),
                ],
                0,
                Some(Handler::Blocks(vec![BasicBlock::new(
                    vec![
                        CILRoot::STLoc {
                            local: 0,
                            tree: conv_usize!(ldc_i32!(0)).cast_ptr(ptr!(Type::Int(Int::U8))),
                        }
                        .into(),
                        CILRoot::GoTo {
                            target: 0,
                            sub_target: 2,
                        }
                        .into(),
                        CILRoot::JumpingPad {
                            source: 0,
                            target: 2,
                        }
                        .into(),
                    ],
                    1,
                    None,
                )])),
            ),
            BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: CILNode::LDLoc(0),
                }
                .into()],
                2,
                None,
            ),
        ],
        vec![Some("size".into()), Some("align".into())],
    );

    asm.add_method(__rust_alloc);
    let mut __rust_alloc_zeroed = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(
            &[Type::Int(Int::USize), Type::Int(Int::USize)],
            ptr!(Type::Int(Int::U8)),
        ),
        "__rust_alloc_zeroed",
        vec![(Some("alloc_ptr".into()), ptr!(Type::Int(Int::U8)))],
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
                            tree: call!(
                                CallSite::aligned_alloc(),
                                [CILNode::LDArg(0), CILNode::LDArg(1)]
                            ),
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
                    tree: call!(
                        CallSite::aligned_alloc(),
                        [CILNode::LDArg(0), CILNode::LDArg(1)]
                    )
                    .cast_ptr(ptr!(Type::Int(Int::U8))),
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
            &[
                ptr!(Type::Int(Int::U8)),
                Type::Int(Int::USize),
                Type::Int(Int::USize),
            ],
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
                    args: [CILNode::LDArg(0).cast_ptr(ptr!(Type::Void))].into(),
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
                        FnSig::new(&[Type::Int(Int::ISize)], Type::Void),
                        true,
                    )),
                    args: [CILNode::LDArg(0).cast_ptr(Type::Int(Int::ISize))].into(),
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
                ptr!(Type::Int(Int::U8)),
                Type::Int(Int::USize),
                Type::Int(Int::USize),
                Type::Int(Int::USize),
            ],
            ptr!(Type::Int(Int::U8)),
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
                            [
                                CILNode::LDArg(0).cast_ptr(ptr!(Type::Void)),
                                CILNode::LDArg(3),
                                CILNode::LDArg(2)
                            ]
                        )
                        .cast_ptr(ptr!(Type::Int(Int::U8))),
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
                        [
                            CILNode::LDArg(0).cast_ptr(ptr!(Type::Void)),
                            CILNode::LDArg(3),
                            CILNode::LDArg(2)
                        ]
                    )
                    .cast_ptr(ptr!(Type::Int(Int::U8))),
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

    asm.add_static(
        Type::ClassRef(Box::new(ClassRef::dictionary(
            Type::Int(Int::I32),
            Type::Int(Int::ISize),
        ))),
        "thread_results",
        false,
    );
    asm.add_initialzer(CILRoot::SetStaticField {
        descr: Box::new(StaticFieldDescriptor::new(
            None,
            Type::ClassRef(Box::new(ClassRef::dictionary(
                Type::Int(Int::I32),
                Type::Int(Int::ISize),
            ))),
            "thread_results".into(),
        )),
        value: CILNode::NewObj(Box::new(CallOpArgs {
            site: Box::new(CallSite::new_extern(
                ClassRef::dictionary(Type::Int(Int::I32), Type::Int(Int::ISize)),
                ".ctor".into(),
                FnSig::new(
                    [Type::ClassRef(Box::new(ClassRef::dictionary(
                        Type::Int(Int::I32),
                        Type::Int(Int::ISize),
                    )))],
                    Type::Void,
                ),
                false,
            )),
            args: [].into(),
        })),
    });
    pthread_create(asm);
    pthread_attr_init(asm);
    pthread_attr_destroy(asm);
    pthread_attr_setstacksize(asm);
    pthread_detach(asm);
    pthread_join(asm);
    pthread_self(asm);
    _Unwind_RaiseException(asm);
    pthread_setname_np(asm);
    __cxa_thread_atexit_impl(asm);
    llvm_x86_sse2_pause(asm);
    let rust_exception = ClassDef::new(
        AccessModifer::Public,
        "RustException".into(),
        vec![],
        vec![("data_pointer".into(), Type::Int(Int::USize))],
        vec![Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            FnSig::new(
                &[
                    Type::ClassRef(Box::new(
                        ClassRef::new::<&str, _>(None, "RustException").with_valuetype(false),
                    )),
                    Type::Int(Int::USize),
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
                            ClassRef::new::<&str, _>(None, "RustException").with_valuetype(false),
                            Type::Int(Int::USize),
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
        Some(ClassRef::exception()),
        None,
    );
    asm.add_typedef(rust_exception);
    let start = Method::new(
        AccessModifer::Public,
        MethodType::Instance,
        FnSig::new(&[Type::ClassRef(Box::new(unmanaged_start()))], Type::Void),
        "Start",
        vec![(Some("res".into()), Type::Int(Int::ISize))],
        vec![BasicBlock::new(
            vec![
                CILRoot::Call {
                    site: Box::new(CallSite::new(
                        None,
                        ".tcctor".into(),
                        FnSig::new(&[], Type::Void),
                        true,
                    )),
                    args: [].into(),
                }
                .into(),
                source_info!(),
                CILRoot::STLoc {
                    local: 0,
                    tree: CILNode::CallI(Box::new((
                        (FnSig::new(&[ptr!(Type::Void)], ptr!(Type::Void))),
                        (ld_field!(
                            CILNode::LDArg(0),
                            FieldDescriptor::new(
                                unmanaged_start(),
                                Type::FnPtr(Box::new(FnSig::new(
                                    &[ptr!(Type::Void)],
                                    ptr!(Type::Void),
                                ))),
                                "start_fn".into(),
                            )
                        )),
                        [ld_field!(
                            CILNode::LDArg(0),
                            FieldDescriptor::new(
                                unmanaged_start(),
                                ptr!(Type::Void),
                                "data".into(),
                            )
                        )]
                        .into(),
                    )))
                    .cast_ptr(Type::Int(Int::ISize)),
                }
                .into(),
                source_info!(),
                CILRoot::Call {
                    site: Box::new(CallSite::new_extern(
                        ClassRef::dictionary(Type::Int(Int::I32), Type::Int(Int::ISize)),
                        "set_Item".into(),
                        FnSig::new(
                            [
                                Type::ClassRef(Box::new(ClassRef::dictionary(
                                    Type::Int(Int::I32),
                                    Type::Int(Int::ISize),
                                ))),
                                Type::GenericArg(0),
                                Type::GenericArg(1),
                            ],
                            Type::Void,
                        ),
                        false,
                    )),
                    args: Box::new([
                        CILNode::LDStaticField(Box::new(StaticFieldDescriptor::new(
                            None,
                            Type::ClassRef(Box::new(ClassRef::dictionary(
                                Type::Int(Int::I32),
                                Type::Int(Int::ISize),
                            ))),
                            "thread_results".into(),
                        ))),
                        CILNode::CallVirt(Box::new(CallOpArgs {
                            site: Box::new(CallSite::new_extern(
                                ClassRef::thread(),
                                "get_ManagedThreadId".into(),
                                FnSig::new(
                                    [Type::ClassRef(Box::new(ClassRef::thread()))],
                                    Type::Int(Int::I32),
                                ),
                                false,
                            )),
                            args: [call!(
                                CallSite::new_extern(
                                    ClassRef::thread(),
                                    "get_CurrentThread".into(),
                                    FnSig::new([], Type::ClassRef(Box::new(ClassRef::thread())),),
                                    true,
                                ),
                                []
                            )]
                            .into(),
                        })),
                        CILNode::LDLoc(0),
                    ]),
                }
                .into(),
                source_info!(),
                CILRoot::VoidRet.into(),
            ],
            0,
            None,
        )],
        vec![Some("this".into())],
    );
    let unmanaged_start = ClassDef::new(
        AccessModifer::Public,
        "UnmanagedThreadStart".into(),
        vec![],
        vec![
            (
                "start_fn".into(),
                Type::FnPtr(Box::new(FnSig::new(&[ptr!(Type::Void)], ptr!(Type::Void)))),
            ),
            ("data".into(), ptr!(Type::Void)),
        ],
        vec![
            Method::new(
                AccessModifer::Public,
                MethodType::Instance,
                FnSig::new(
                    &[
                        Type::ClassRef(Box::new(unmanaged_start())),
                        Type::FnPtr(Box::new(FnSig::new(&[ptr!(Type::Void)], ptr!(Type::Void)))),
                        ptr!(Type::Void),
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
                                Type::FnPtr(Box::new(FnSig::new(
                                    &[ptr!(Type::Void)],
                                    ptr!(Type::Void),
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
                                ptr!(Type::Void),
                                "data".into(),
                            )),
                        }
                        .into(),
                        CILRoot::VoidRet.into(),
                    ],
                    0,
                    None,
                )],
                vec![
                    Some("this".into()),
                    Some("start_routine".into()),
                    Some("data".into()),
                ],
            ),
            start,
        ],
        None,
        0,
        Some(ClassRef::object_type()),
        None,
    );
    asm.add_typedef(unmanaged_start);
}
add_method_from_trees!(
    pthread_self,
    &[],
    Type::Int(Int::ISize),
    vec![BasicBlock::new(
        vec![CILRoot::Ret {
            tree: conv_isize!(ldc_i32!(0))
        }
        .into()],
        0,
        None
    )],
    vec![],
    vec![]
);
add_method_from_trees!(
    pthread_setname_np,
    &[Type::Int(Int::U64), ptr!(Type::Int(Int::I8))],
    Type::Int(Int::I32),
    vec![BasicBlock::new(
        vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
        0,
        None
    )],
    vec![],
    vec![Some("thread".into()), Some("name".into())]
);
add_method_from_trees!(
    _Unwind_RaiseException,
    &[ptr!(Type::Void)],
    Type::Void,
    vec![BasicBlock::new(
        vec![CILRoot::Throw(CILNode::NewObj(Box::new(CallOpArgs {
            args: Box::new([conv_usize!(CILNode::LDArg(0))]),
            site: Box::new(CallSite::new(
                Some(ClassRef::new::<&str, _>(None, "RustException").with_valuetype(false),),
                ".ctor".into(),
                FnSig::new(
                    &[
                        Type::ClassRef(Box::new(
                            ClassRef::new::<&str, _>(None, "RustException").with_valuetype(false),
                        )),
                        Type::Int(Int::USize),
                    ],
                    Type::Void,
                ),
                false,
            )),
        })))
        .into()],
        0,
        None,
    )],
    vec![],
    vec![Some("ptr".into())]
);
add_method_from_trees!(
    pthread_create,
    &[
        ptr!(Type::Int(Int::ISize)),
        ptr!(Type::Void),
        Type::FnPtr(Box::new(FnSig::new(&[ptr!(Type::Void)], ptr!(Type::Void)))),
        ptr!(Type::Void)
    ],
    Type::Int(Int::I32),
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
                                            Type::ClassRef(Box::new(unmanaged_start())),
                                            Type::FnPtr(Box::new(FnSig::new(
                                                &[ptr!(Type::Void)],
                                                ptr!(Type::Void)
                                            ))),
                                            ptr!(Type::Void)
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
                                    &[Type::ClassRef(Box::new(unmanaged_start()))],
                                    Type::Void
                                ),
                                false
                            )))
                        ]
                        .into(),
                        site: Box::new(CallSite::new(
                            Some(ClassRef::thread_start()),
                            ".ctor".into(),
                            FnSig::new(
                                &[
                                    Type::ClassRef(Box::new(ClassRef::thread_start())),
                                    Type::ClassRef(Box::new(ClassRef::object_type())),
                                    Type::Int(Int::ISize)
                                ],
                                Type::Void
                            ),
                            false
                        )),
                    }))]
                    .into(),
                    site: Box::new(CallSite::new(
                        Some(ClassRef::thread()),
                        ".ctor".into(),
                        FnSig::new(
                            &[
                                Type::ClassRef(Box::new(ClassRef::thread())),
                                Type::ClassRef(Box::new(ClassRef::thread_start())),
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
                    Some(ClassRef::thread()),
                    "Start".into(),
                    FnSig::new(&[Type::ClassRef(Box::new(ClassRef::thread()))], Type::Void),
                    false
                )),
                args: [CILNode::LDLoc(0)].into(),
            }
            .into(),
            CILRoot::STIndISize(
                CILNode::LDArg(0),
                CILNode::managed_ref_to_handle(CILNode::LDLoc(0))
            )
            .into(),
            CILRoot::Ret { tree: ldc_i32!(0) }.into(),
        ],
        0,
        None
    ),],
    vec![(
        Some("thread_handle".into()),
        Type::ClassRef(Box::new(ClassRef::thread()))
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
    &[ptr!(Type::Int(Int::ISize)),],
    Type::Int(Int::I32),
    vec![BasicBlock::new(
        vec![
            CILRoot::InitBlk {
                dst: Box::new(CILNode::LDArg(0)),
                val: Box::new(ldc_u32!(0)),
                count: Box::new(size_of!(Type::Int(Int::USize)) * ldc_i32!(3))
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
    &[ptr!(Type::Int(Int::ISize)), Type::Int(Int::USize)],
    Type::Int(Int::I32),
    vec![BasicBlock::new(
        vec![
            CILRoot::STIndISize(
                CILNode::LDArg(0) + conv_usize!(size_of!(Type::Int(Int::USize)) * ldc_i32!(2)),
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
// However, we still have to free the GC handle.
add_method_from_trees!(
    pthread_detach,
    &[Type::Int(Int::ISize)],
    Type::Int(Int::I32),
    vec![BasicBlock::new(
        vec![
            CILRoot::STLoc {
                local: 0,
                tree: call!(
                    CallSite::new_extern(
                        ClassRef::gc_handle(),
                        "FromIntPtr".into(),
                        FnSig::new(
                            [Type::Int(Int::ISize)],
                            Type::ClassRef(Box::new(ClassRef::gc_handle()))
                        ),
                        true
                    ),
                    [CILNode::LDArg(0)]
                )
            }
            .into(),
            CILRoot::Call {
                site: Box::new(CallSite::new_extern(
                    ClassRef::gc_handle(),
                    "Free".into(),
                    FnSig::new(
                        [Type::Ref(Box::new(Type::ClassRef(Box::new(
                            ClassRef::gc_handle()
                        ))))],
                        Type::Void
                    ),
                    false
                )),
                args: [CILNode::LDLocA(0)].into()
            }
            .into(),
            CILRoot::Ret { tree: ldc_i32!(0) }.into()
        ],
        0,
        None
    )],
    vec![(
        Some("gc_habdle".into()),
        Type::ClassRef(Box::new(ClassRef::gc_handle()))
    )],
    vec![Some("thread".into())]
);
add_method_from_trees!(
    pthread_attr_destroy,
    &[ptr!(Type::Int(Int::ISize)),],
    Type::Int(Int::I32),
    vec![BasicBlock::new(
        vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
        0,
        None
    )],
    vec![],
    vec![Some("thread_attr".into()),]
);
fn unmanaged_start() -> ClassRef {
    ClassRef::new::<&str, _>(None, "UnmanagedThreadStart").with_valuetype(false)
}
// TODO: Can't yet register thread-local deconstructors.
add_method_from_trees!(
    __cxa_thread_atexit_impl,
    [
        Type::FnPtr(Box::new(FnSig::new([ptr!(Type::Void)], Type::Void))),
        ptr!(Type::Void),
        ptr!(Type::Void)
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
        AccessModifer::Public,
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
        Type::FnPtr(Box::new(FnSig::new(
            &[ptr!(Type::Int(Int::U8))],
            Type::Void
        ))),
        ptr!(Type::Int(Int::U8)),
        Type::FnPtr(Box::new(FnSig::new(
            &[ptr!(Type::Int(Int::U8)), ptr!(Type::Int(Int::U8))],
            Type::Void
        ))),
    ],
    Type::Int(Int::I32),
    vec![
        BasicBlock::new(
            vec![
                CILRoot::CallI {
                    sig: Box::new(FnSig::new(&[ptr!(Type::Int(Int::U8))], Type::Void)),
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
                                ClassRef::new::<&str, _>(None, "RustException")
                                    .with_valuetype(false)
                            ))),
                        }
                        .into(),
                        CILRoot::STLoc {
                            local: 0,
                            tree: ld_field!(
                                CILNode::CheckedCast(Box::new((
                                    CILNode::LDLoc(1),
                                    ClassRef::new::<&str, _>(None, "RustException")
                                        .with_valuetype(false)
                                ))),
                                FieldDescriptor::new(
                                    ClassRef::new::<&str, _>(None, "RustException")
                                        .with_valuetype(false),
                                    Type::Int(Int::USize),
                                    "data_pointer".into()
                                )
                            ),
                        }
                        .into(),
                        CILRoot::CallI {
                            sig: Box::new(FnSig::new(
                                &[ptr!(Type::Int(Int::U8)), ptr!(Type::Int(Int::U8))],
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
                        CILRoot::Call {
                            site: Box::new(CallSite::new_extern(
                                ClassRef::console(),
                                "WriteLine".into(),
                                FnSig::new(
                                    &[Type::ClassRef(Box::new(ClassRef::object_type()))],
                                    Type::Void
                                ),
                                true
                            )),
                            args: Box::new([CILNode::LDLoc(1)])
                        }
                        .into(),
                        CILRoot::CallI {
                            sig: Box::new(FnSig::new(
                                &[ptr!(Type::Int(Int::U8)), ptr!(Type::Int(Int::U8))],
                                Type::Void
                            )),
                            fn_ptr: Box::new(CILNode::LDArg(2)),
                            args: Box::new([
                                CILNode::LDArg(1),
                                call!(
                                    CallSite::builtin(
                                        "exception_to_native".into(),
                                        FnSig::new(
                                            vec![Type::ClassRef(Box::new(
                                                ClassRef::object_type()
                                            )),],
                                            ptr!(Type::Int(Int::U8))
                                        ),
                                        true
                                    ),
                                    [CILNode::LDLoc(1)]
                                )
                            ])
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
        (Some(crate::DATA_PTR.into()), Type::Int(Int::USize)),
        (
            Some("exception".into()),
            Type::ClassRef(Box::new(ClassRef::exception())),
        ),
        (
            Some("tmp_buff".into()),
            Type::ClassRef(Box::new(ClassRef::array(&Type::Int(Int::USize), 4)))
        ),
    ],
    vec![
        Some("try_fn".into()),
        Some("data".into()),
        Some("catch_fn".into()),
    ]
);
add_method_from_trees!(
    pthread_join,
    [Type::Int(Int::ISize), ptr!(ptr!(Type::Void))],
    Type::Int(Int::I32),
    vec![
        BasicBlock::new(
            vec![
                source_info!(),
                CILRoot::STLoc {
                    local: 0,
                    tree: CILNode::LDArg(0).gc_handle_to_obj(ClassRef::thread()),
                }
                .into(),
                source_info!(),
                CILRoot::CallVirt {
                    site: Box::new(CallSite::new_extern(
                        ClassRef::thread(),
                        "Join".into(),
                        FnSig::new([Type::ClassRef(Box::new(ClassRef::thread()))], Type::Void),
                        false
                    )),
                    args: Box::new([CILNode::LDLoc(0)])
                }
                .into(),
                // If the result pointer is null, do not check the result.
                CILRoot::BFalse {
                    target: 1,
                    sub_target: 0,
                    cond: CILNode::LDArg(1)
                }
                .into(),
                source_info!(),
                CILRoot::STIndPtr(
                    CILNode::LDArg(1),
                    call_virt!(
                        CallSite::new_extern(
                            ClassRef::dictionary(Type::Int(Int::I32), Type::Int(Int::ISize)),
                            "get_Item".into(),
                            FnSig::new(
                                [
                                    Type::ClassRef(Box::new(ClassRef::dictionary(
                                        Type::Int(Int::I32),
                                        Type::Int(Int::ISize)
                                    ))),
                                    Type::GenericArg(0)
                                ],
                                Type::GenericArg(1)
                            ),
                            false
                        ),
                        [
                            CILNode::LDStaticField(Box::new(StaticFieldDescriptor::new(
                                None,
                                Type::ClassRef(Box::new(ClassRef::dictionary(
                                    Type::Int(Int::I32),
                                    Type::Int(Int::ISize)
                                )),),
                                "thread_results".into()
                            ))),
                            call_virt!(
                                CallSite::new_extern(
                                    ClassRef::thread(),
                                    "get_ManagedThreadId".into(),
                                    FnSig::new(
                                        [Type::ClassRef(Box::new(ClassRef::thread()))],
                                        Type::Int(Int::I32)
                                    ),
                                    false
                                ),
                                [CILNode::LDLoc(0)]
                            )
                        ]
                    )
                    .cast_ptr(ptr!(ptr!(Type::Void))),
                    Box::new(ptr!(Type::Void))
                )
                .into(),
                source_info!(),
                CILRoot::Ret { tree: ldc_i32!(0) }.into()
            ],
            0,
            None
        ),
        BasicBlock::new(vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()], 1, None)
    ],
    vec![(
        Some("thread".into()),
        Type::ClassRef(Box::new(ClassRef::thread()))
    )],
    vec![Some("thread_handle".into()), Some("result_ptr".into())]
);

fn shr_u128(value: CILNode, shift: CILNode) -> CILNode {
    call!(
        CallSite::boxed(
            ClassRef::uint_128(asm).into(),
            "op_RightShift".into(),
            FnSig::new(
                &[Type::Int(Int::U128), Type::Int(Int::I32)],
                Type::Int(Int::U128)
            ),
            true,
        ),
        [value, shift]
    )
}
fn or_u128(lhs: CILNode, rhs: CILNode) -> CILNode {
    call!(
        CallSite::boxed(
            ClassRef::uint_128(asm).into(),
            "op_BitwiseOr".into(),
            FnSig::new(
                &[Type::Int(Int::U128), Type::Int(Int::U128)],
                Type::Int(Int::U128)
            ),
            true,
        ),
        [lhs, rhs]
    )
}
fn and_u128(lhs: CILNode, rhs: CILNode) -> CILNode {
    call!(
        CallSite::boxed(
            ClassRef::uint_128(asm).into(),
            "op_BitwiseAnd".into(),
            FnSig::new(
                &[Type::Int(Int::U128), Type::Int(Int::U128)],
                Type::Int(Int::U128)
            ),
            true,
        ),
        [lhs, rhs]
    )
}
fn shl_u128(value: CILNode, shift: CILNode) -> CILNode {
    call!(
        CallSite::boxed(
            ClassRef::uint_128(asm).into(),
            "op_LeftShift".into(),
            FnSig::new(
                &[Type::Int(Int::U128), Type::Int(Int::I32)],
                Type::Int(Int::U128)
            ),
            true,
        ),
        [value, shift]
    )
}

add_method_from_trees!(
    bitreverse_u128,
    &[Type::Int(Int::U128)],
    Type::Int(Int::U128),
    vec![BasicBlock::new(
        vec![
            CILRoot::STLoc {
                local: 0,
                tree: or_u128(
                    and_u128(
                        shr_u128(CILNode::LDArg(0), ldc_i32!(1)),
                        CILNode::const_u128(0x5555_5555_5555_5555_5555_5555_5555_5555_u128),
                    ),
                    shl_u128(
                        and_u128(
                            CILNode::LDArg(0),
                            CILNode::const_u128(0x5555_5555_5555_5555_5555_5555_5555_5555_u128),
                        ),
                        ldc_i32!(1),
                    ),
                ),
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: or_u128(
                    and_u128(
                        shr_u128(CILNode::LDLoc(0), ldc_i32!(2)),
                        CILNode::const_u128(0x3333_3333_3333_3333_3333_3333_3333_3333_u128),
                    ),
                    shl_u128(
                        and_u128(
                            CILNode::LDLoc(0),
                            CILNode::const_u128(0x3333_3333_3333_3333_3333_3333_3333_3333_u128),
                        ),
                        ldc_i32!(2),
                    ),
                ),
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: or_u128(
                    and_u128(
                        shr_u128(CILNode::LDLoc(0), ldc_i32!(4)),
                        CILNode::const_u128(0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_u128),
                    ),
                    shl_u128(
                        and_u128(
                            CILNode::LDLoc(0),
                            CILNode::const_u128(0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_u128),
                        ),
                        ldc_i32!(4),
                    ),
                ),
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: or_u128(
                    and_u128(
                        shr_u128(CILNode::LDLoc(0), ldc_i32!(8)),
                        CILNode::const_u128(0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF_u128),
                    ),
                    shl_u128(
                        and_u128(
                            CILNode::LDLoc(0),
                            CILNode::const_u128(0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF_u128),
                        ),
                        ldc_i32!(8),
                    ),
                ),
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: or_u128(
                    and_u128(
                        shr_u128(CILNode::LDLoc(0), ldc_i32!(16)),
                        CILNode::const_u128(0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF_u128),
                    ),
                    shl_u128(
                        and_u128(
                            CILNode::LDLoc(0),
                            CILNode::const_u128(0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF_u128),
                        ),
                        ldc_i32!(16),
                    ),
                ),
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: or_u128(
                    and_u128(
                        shr_u128(CILNode::LDLoc(0), ldc_i32!(32)),
                        CILNode::const_u128(0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF_u128),
                    ),
                    shl_u128(
                        and_u128(
                            CILNode::LDLoc(0),
                            CILNode::const_u128(0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF_u128),
                        ),
                        ldc_i32!(32),
                    ),
                ),
            }
            .into(),
            CILRoot::Ret {
                tree: or_u128(
                    shr_u128(CILNode::LDLoc(0), ldc_i32!(64)),
                    shl_u128(CILNode::LDLoc(0), ldc_i32!(64)),
                ),
            }
            .into(),
        ],
        0,
        None
    )],
    vec![(Some("n".into()), Type::Int(Int::U128))],
    vec![Some("input".into()),]
);
add_method_from_trees!(
    bitreverse_u64,
    &[Type::Int(Int::U64)],
    Type::Int(Int::U64),
    vec![BasicBlock::new(
        vec![
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDArg(0), ldc_i32!(1)) & ldc_u64!(0x5555_5555_5555_5555_u64)
                    | shl!(
                        (CILNode::LDArg(0) & ldc_u64!(0x5555_5555_5555_5555_u64)),
                        ldc_i32!(1)
                    )
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(2)) & ldc_u64!(0x3333_3333_3333_3333_u64)
                    | shl!(
                        (CILNode::LDLoc(0) & ldc_u64!(0x3333_3333_3333_3333_u64)),
                        ldc_i32!(2)
                    )
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(4)) & ldc_u64!(0x0F0F_0F0F_0F0F_0F0F_u64)
                    | shl!(
                        (CILNode::LDLoc(0) & ldc_u64!(0x0F0F_0F0F_0F0F_0F0F_u64)),
                        ldc_i32!(4)
                    )
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(8)) & ldc_u64!(0x00FF_00FF_00FF_00FF_u64)
                    | shl!(
                        (CILNode::LDLoc(0) & ldc_u64!(0x00FF_00FF_00FF_00FF_u64)),
                        ldc_i32!(8)
                    )
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(16)) & ldc_u64!(0x0000_FFFF_0000_FFFF_u64)
                    | shl!(
                        (CILNode::LDLoc(0) & ldc_u64!(0x0000_FFFF_0000_FFFF_u64)),
                        ldc_i32!(16)
                    )
            }
            .into(),
            CILRoot::Ret {
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(32)) & ldc_u64!(0x0000_0000_FFFF_FFFF_u64)
                    | shl!(
                        (CILNode::LDLoc(0) & ldc_u64!(0x0000_0000_FFFF_FFFF_u64)),
                        ldc_i32!(32)
                    )
            }
            .into()
        ],
        0,
        None
    )],
    vec![(Some("n".into()), Type::Int(Int::U64))],
    vec![Some("input".into()),]
);
add_method_from_trees!(
    bitreverse_u32,
    &[Type::Int(Int::U32)],
    Type::Int(Int::U32),
    vec![BasicBlock::new(
        vec![
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDArg(0), ldc_i32!(1)) & ldc_u32!(0x5555_5555_u32)
                    | shl!((CILNode::LDArg(0) & ldc_u32!(0x5555_5555_u32)), ldc_i32!(1))
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(2)) & ldc_u32!(0x3333_3333_u32)
                    | shl!((CILNode::LDLoc(0) & ldc_u32!(0x3333_3333_u32)), ldc_i32!(2))
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(4)) & ldc_u32!(0x0F0F_0F0F_u32)
                    | shl!((CILNode::LDLoc(0) & ldc_u32!(0x0F0F_0F0F_u32)), ldc_i32!(4))
            }
            .into(),
            CILRoot::STLoc {
                local: 0,
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(8)) & ldc_u32!(0x00FF_00FF_u32)
                    | shl!((CILNode::LDLoc(0) & ldc_u32!(0x00FF_00FF_u32)), ldc_i32!(8))
            }
            .into(),
            CILRoot::Ret {
                tree: shr!(CILNode::LDLoc(0), ldc_i32!(16)) & ldc_u32!(0x0000_FFFF_u32)
                    | shl!(
                        (CILNode::LDLoc(0) & ldc_u32!(0x0000_FFFF_u32)),
                        ldc_i32!(16)
                    )
            }
            .into()
        ],
        0,
        None
    )],
    vec![(Some("n".into()), Type::Int(Int::U32))],
    vec![Some("input".into()),]
);
