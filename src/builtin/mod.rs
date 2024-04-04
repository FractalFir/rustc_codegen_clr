use crate::basic_block::BasicBlock;
use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;
use crate::method::MethodType;
use crate::r#type::DotnetTypeRef;
use crate::{
    access_modifier::AccessModifer, add_method_from_trees, assembly::Assembly, cil::CallSite,
    function_sig::FnSig, method::Method, r#type::Type,
};
use crate::{add, call, ldc_u64, lt_un};
use rustc_middle::ty::TyCtxt;
mod casts;
mod select;
add_method_from_trees!(
    bounds_check,
    &[Type::USize, Type::USize],
    &Type::USize,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: lt_un!(CILNode::LDArg(0), CILNode::LDArg(1))
                }
                .into(),
                CILRoot::Throw(CILNode::NewObj {
                    site: CallSite::boxed(
                        Some(DotnetTypeRef::new(
                            Some("System.Runtime"),
                            "System.IndexOutOfRangeException"
                        )),
                        ".ctor".into(),
                        FnSig::new(&[], &Type::Void),
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
    ]
);

#[macro_export]
macro_rules! add_method_from_trees {
    ($name:ident,$input:expr,$output:expr,$trees:expr) => {
        fn $name(asm: &mut $crate::assembly::Assembly) {
            let method = $crate::method::Method::new(
                $crate::access_modifier::AccessModifer::Private,
                crate::method::MethodType::Static,
                crate::function_sig::FnSig::new($input, $output),
                stringify!($name),
                vec![],
                $trees,
            );
            asm.add_method(method);
        }
    };
    ($name:ident,$input:expr,$output:expr,$trees:expr,$locals:expr) => {
        fn $name(asm: &mut crate::assembly::Assembly) {
            let mut method = crate::method::Method::new(
                crate::access_modifier::AccessModifer::Private,
                crate::method::MethodType::MethodType::Static,
                crate::function_sig::FnSig::new($input, $output),
                stringify!($name),
                $locals.into(),
                $trees,
            );
            asm.add_method(method);
        }
    };
}

/// Inserts a small subset of libc and some standard types into an assembly.
pub fn insert_ffi_functions(asm: &mut Assembly, tyctx: TyCtxt) {
    bounds_check(asm);
    let c_void = Type::c_void(tyctx);
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
    /*asm.add_method(Method::new(AccessModifer::Public,MethodType::Static,FnSig::new(&[Type::U64,Type::U64],&Type::U128),"new_u128",vec![],vec![
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
    let marshal = Some(marshal);
  
   

    let mut native_mem = DotnetTypeRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.NativeMemory",
    );
    native_mem.set_valuetype(false);
    let native_mem = Some(native_mem);
    let mut __rust_alloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::U8.into())),
        "__rust_alloc",
        vec![],
        if *crate::config::CHECK_ALLOCATIONS {
            todo!("Can't check allocations yet");
        } else {
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: call!(
                        CallSite::alloc(),
                        [CILNode::LDArg(0), CILNode::LDArg(1)]
                    ),
                }
                .into()],
                0,
                None,
            )]
        },
    );

    asm.add_method(__rust_alloc);
    let mut __rust_dealloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(
            &[Type::Ptr(Type::U8.into()), Type::USize, Type::USize],
            &Type::Void,
        ),
        "__rust_dealloc",
        vec![],
        vec![BasicBlock::new(
            vec![
                CILRoot::Call {
                    site: CallSite::new(
                        native_mem.clone(),
                        "AlignedFree".into(),
                        FnSig::new(&[Type::Ptr(Type::Void.into())], &Type::Void),
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
    );
    puts(asm);
    asm.add_method(__rust_dealloc);
    let free = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::Ptr(c_void.clone().into())], &Type::Void),
        "free",
        vec![],
        vec![BasicBlock::new(
            vec![
                CILRoot::Call {
                    site: CallSite::new(
                        marshal.clone(),
                        "FreeHGlobal".into(),
                        FnSig::new(&[Type::ISize], &Type::Void),
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
    );

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
    &Type::Void,
    vec![
        BasicBlock::new(
            vec![
                CILRoot::BTrue {
                    target: 1,
                    sub_target: 0,
                    ops: CILNode::LDIndI8 {
                        ptr: CILNode::LDArg(0).into()
                    }
                }
                .into(),
                CILRoot::Call {
                    site: CallSite::new(
                        Some(DotnetTypeRef::console()),
                        "Write".into(),
                        FnSig::new(&[Type::DotnetChar], &Type::Void),
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
                    tree: add!(CILNode::LDArg(0), ldc_u64!(1))
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
    ]
);

/*
add_method!(
    unlikely,
    &[Type::Bool],
    &Type::Bool,
    [CILOp::LDArg(0), CILOp::Ret,]
);
//System.Environment.Exit(a_ExitCode)
add_method!(abort, &[], &Type::Void, CILOp::throw_msg("Called abort!"));
pub fn add_ptr_offset_from_unsigned(asm: &mut Assembly) {
    let ptr_offset_from_unsigned_calls: Box<[_]> = asm
        .call_sites()
        .filter(|call_site| {
            call_site.signature().inputs().len() == 2
                && call_site.name() == "ptr_offset_from_unsigned"
        })
        .cloned()
        .collect();
    for call in ptr_offset_from_unsigned_calls.iter() {
        let rtype: &Type = &call.inputs()[0];
        let mut ptr_offset_from_unsigned = Method::new_empty(
            AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "raw_eq",
            vec![],
        );
        ptr_offset_from_unsigned.set_ops(match rtype {
            Type::Ptr(inner) => {
                vec![
                    CILOp::LDArg(0),
                    CILOp::LDArg(1),
                    CILOp::Sub,
                    CILOp::Div,
                    CILOp::SizeOf(inner.clone()),
                    CILOp::Ret,
                ]
            }
            Type::DotnetType(type_ref) => {
                if type_ref.is_valuetype() && type_ref.name_path().contains("PtrComponents") {
                    todo!();
                    /*
                    vec![
                        CILOp::LDArg(0),
                        CILOp::LDArg(1),
                        CILOp::Sub,
                        CILOp::Div,
                        CILOp::SizeOf(inner.clone()),
                        CILOp::Ret,
                    ]*/
                } else {
                    continue;
                }
            }
            _ => continue,
        });
        asm.add_method(ptr_offset_from_unsigned);
    }
}
*/
