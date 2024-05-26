use crate::basic_block::BasicBlock;

use crate::method::MethodType;
use crate::{
    access_modifier::AccessModifer, add_method_from_trees, assembly::Assembly, method::Method,
    r#type::Type,
};
use cilly::call_site::CallSite;
use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use cilly::fn_sig::FnSig;
use cilly::DotnetTypeRef;
use cilly::{call, conv_usize, ldc_u64, lt_un};
use rustc_middle::ty::TyCtxt;
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
        fn $name(asm: &mut $crate::assembly::Assembly) {
            let method = $crate::method::Method::new(
                $crate::access_modifier::AccessModifer::Private,
                $crate::method::MethodType::Static,
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
        fn $name(asm: &mut $crate::assembly::Assembly) {
            let mut method = $crate::method::Method::new(
                $crate::access_modifier::AccessModifer::Private,
                $crate::method::MethodType::MethodType::Static,
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
    let c_void = crate::r#type::c_void(tyctx);
    asm.add_typedef(crate::r#type::TypeDef::new(
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
    asm.add_typedef(crate::r#type::TypeDef::nameonly("Unresolved"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("RustVoid"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("Foreign"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("RustStr"));
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
