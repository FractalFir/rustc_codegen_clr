use std::fmt::Debug;

use crate::method::Method;
use crate::static_field_desc::StaticFieldDescriptor;
use crate::v2::{ClassRef, FnSig, Int};
use crate::{
    asm::Assembly, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, eq, lt, size_of,
};
use crate::{call, call_virt, conv_i32, conv_usize, ldc_i32, ldc_u32, mul, Type};
pub fn argc_argv_init_method(asm: &mut Assembly) -> CallSite {
    use std::num::NonZeroU8;
    let init_cs = CallSite::new(
        None,
        "argc_argv_init".into(),
        FnSig::new(Box::new([]), Type::Void),
        true,
    );
    if asm.contains_fn(&init_cs) {
        return init_cs;
    }
    let mut init_method = Method::new(
        crate::access_modifier::AccessModifer::Extern,
        crate::method::MethodType::Static,
        FnSig::new(Box::new([]), Type::Void),
        "argc_argv_init",
        vec![],
        vec![],
        vec![],
    );

    // Allocate the variables necesarry for initializng args.
    let argc =
        u32::try_from(init_method.add_local(Type::Int(Int::I32), Some("argc".into()))).unwrap();
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let argv =
        u32::try_from(init_method.add_local(asm.nptr(uint8_ptr), Some("argv".into()))).unwrap();
    let managed_args = u32::try_from(init_method.add_local(
        Type::PlatformArray {
            elem: asm.alloc_type(Type::PlatformString),
            dims: NonZeroU8::new(1).unwrap(),
        },
        Some("managed_args".into()),
    ))
    .unwrap();
    let arg_idx =
        u32::try_from(init_method.add_local(Type::Int(Int::I32), Some("arg_idx".into()))).unwrap();
    // Get managed args
    let margs_init = CILRoot::STLoc {
        local: managed_args,
        tree: call!(
            CallSite::new_extern(
                ClassRef::enviroment(asm),
                "GetCommandLineArgs".into(),
                FnSig::new(
                    Box::new([]),
                    Type::PlatformArray {
                        elem: asm.alloc_type(Type::PlatformString),
                        dims: NonZeroU8::new(1).unwrap()
                    }
                ),
                true
            ),
            []
        ),
    };
    // Calculate argc
    let argc_init = CILRoot::STLoc {
        local: argc,
        tree: conv_i32!(CILNode::LDLen {
            arr: CILNode::LDLoc(managed_args).into()
        }),
    };

    // Alloc argv
    let tree = call!(
        CallSite::aligned_alloc(asm),
        [
            mul!(
                conv_usize!(CILNode::LDLoc(argc)),
                conv_usize!(size_of!(Type::Int(Int::USize)))
            ),
            conv_usize!(ldc_u32!(8))
        ]
    )
    .cast_ptr(asm.nptr(uint8_ptr));
    let argv_alloc = CILRoot::STLoc { local: argv, tree };
    // Create the block which allocates argv and calculates argc.
    let start_bb = init_method.new_bb();
    let mut blocks = init_method.blocks_mut();

    // Fill up the start block
    let start_block = &mut blocks[start_bb as usize];
    let status = StaticFieldDescriptor::new(None, Type::Bool, "argv_argc_init_status".into());
    start_block.trees_mut().push(
        CILRoot::BTrue {
            target: start_bb + 3,
            sub_target: 0,
            cond: CILNode::LDStaticField(Box::new(status.clone())),
        }
        .into(),
    );
    start_block.trees_mut().push(margs_init.into());
    start_block.trees_mut().push(argc_init.into());
    start_block.trees_mut().push(argv_alloc.into());
    // Init arg_idx to 0
    start_block.trees_mut().push(
        CILRoot::STLoc {
            local: arg_idx,
            tree: ldc_i32!(0),
        }
        .into(),
    );
    start_block.trees_mut().push(
        CILRoot::GoTo {
            target: start_bb + 1,
            sub_target: 0,
        }
        .into(),
    );
    drop(blocks);
    // Set-up the arg convertion loop
    let loop_bb = init_method.new_bb();
    let mut blocks = init_method.blocks_mut();
    let loop_block = &mut blocks[loop_bb as usize];
    // Load nth argument
    let arg_nth = CILNode::LDElelemRef {
        arr: CILNode::LDLoc(managed_args).into(),
        idx: CILNode::LDLoc(arg_idx).into(),
    };
    // Convert the nth managed argument to UTF16
    let uarg = mstring_to_utf8ptr(arg_nth, asm);
    // Store the converted arg at idx+1
    loop_block.trees_mut().push(
        CILRoot::STIndPtr(
            CILNode::LDLoc(argv)
                + conv_usize!(size_of!(Type::Int(Int::ISize)) * CILNode::LDLoc(arg_idx)),
            uarg,
            Box::new(Type::Int(Int::U8)),
        )
        .into(),
    );
    // Incr the arg_idx
    loop_block.trees_mut().push(
        CILRoot::STLoc {
            local: arg_idx,
            tree: CILNode::LDLoc(arg_idx) + ldc_i32!(1),
        }
        .into(),
    );
    //If no args left, jump to exit
    loop_block.trees_mut().push(
        CILRoot::BTrue {
            target: loop_bb + 1,
            sub_target: 0,
            cond: eq!(
                lt!(
                    CILNode::LDLoc(arg_idx),
                    conv_i32!(CILNode::LDLen {
                        arr: CILNode::LDLoc(managed_args).into()
                    })
                ),
                CILNode::LdFalse
            ),
        }
        .into(),
    );
    //If some args left, jump back to loop head!
    loop_block.trees_mut().push(
        CILRoot::GoTo {
            target: loop_bb,
            sub_target: 0,
        }
        .into(),
    );
    drop(blocks);
    // first block after the loop, sets the relevant statics.
    let loop_end_bb = init_method.new_bb();
    let mut blocks = init_method.blocks_mut();
    let loop_end_block = &mut blocks[loop_end_bb as usize];
    let argv_static = StaticFieldDescriptor::new(None, asm.nptr(uint8_ptr), "argv".into());
    loop_end_block.trees_mut().push(
        CILRoot::SetStaticField {
            descr: Box::new(argv_static),
            value: CILNode::LDLoc(argv),
        }
        .into(),
    );
    let argc_static = StaticFieldDescriptor::new(None, Type::Int(Int::I32), "argc".into());
    loop_end_block.trees_mut().push(
        CILRoot::SetStaticField {
            descr: Box::new(argc_static),
            value: CILNode::LDLoc(argc),
        }
        .into(),
    );
    loop_end_block.trees_mut().push(
        CILRoot::GoTo {
            target: loop_end_bb + 1,
            sub_target: 0,
        }
        .into(),
    );
    loop_end_block.trees_mut().push(
        CILRoot::SetStaticField {
            descr: Box::new(status),
            value: CILNode::LdTrue,
        }
        .into(),
    );
    drop(blocks);
    // Final block, just returns.
    let final_bb = init_method.new_bb();
    let mut blocks = init_method.blocks_mut();
    let final_block = &mut blocks[final_bb as usize];
    final_block.trees_mut().push(CILRoot::VoidRet.into());
    drop(blocks);
    asm.add_method(init_method);
    asm.add_static(Type::Bool, "argv_argc_init_status", false);
    let uint8_ptr_ptr = asm.nptr(uint8_ptr);
    asm.add_static(uint8_ptr_ptr, "argv", false);
    asm.add_static(Type::Int(Int::I32), "argc", false);
    init_cs
}
pub fn mstring_to_utf8ptr(mstring: CILNode, asm: &mut Assembly) -> CILNode {
    call!(
        CallSite::new_extern(
            ClassRef::marshal(asm),
            "StringToCoTaskMemUTF8".into(),
            FnSig::new(Box::new([Type::PlatformString]), Type::Int(Int::ISize)),
            true
        ),
        [mstring]
    )
    .cast_ptr(asm.nptr(Type::Int(Int::U8)))
}
pub fn get_environ(asm: &mut Assembly) -> CallSite {
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let uint8_ptr_ptr = asm.nptr(uint8_ptr);
    let init_cs = CallSite::new(
        None,
        "get_environ".into(),
        FnSig::new(Box::new([]), uint8_ptr_ptr),
        true,
    );
    if asm.contains_fn(&init_cs) {
        return init_cs;
    }

    let mut get_environ = Method::new(
        crate::access_modifier::AccessModifer::Extern,
        crate::method::MethodType::Static,
        FnSig::new(Box::new([]), uint8_ptr_ptr),
        "get_environ",
        vec![],
        vec![],
        vec![],
    );
    let dictionary_local = get_environ.add_local(
        Type::ClassRef(ClassRef::i_dictionary(asm)),
        Some("dict".into()),
    ) as u32;
    let envc = get_environ.add_local(Type::Int(Int::I32), Some("envc".into())) as u32;
    let arr_ptr = get_environ.add_local(uint8_ptr_ptr, Some("arr_ptr".into())) as u32;
    let idx = get_environ.add_local(Type::Int(Int::I32), Some("idx".into())) as u32;
    let iter_local = get_environ.add_local(
        Type::ClassRef(ClassRef::dictionary_iterator(asm)),
        Some("iter".into()),
    ) as u32;
    let keyval_tpe = ClassRef::dictionary_entry(asm);
    let keyval = get_environ.add_local(Type::ClassRef(keyval_tpe), Some("keyval".into())) as u32;
    let encoded_keyval = get_environ.add_local(
        Type::ClassRef(ClassRef::string(asm)),
        Some("encoded_keyval".into()),
    ) as u32;
    let first_check_bb = get_environ.new_bb();
    let init_bb = get_environ.new_bb();
    let loop_body_bb = get_environ.new_bb();
    let loop_end_bb = get_environ.new_bb();
    let ret_bb = get_environ.new_bb();
    let mut blocks = get_environ.blocks_mut();
    let first_check = &mut blocks[first_check_bb as usize];
    first_check.trees_mut().push(
        CILRoot::BNe {
            target: ret_bb,
            sub_target: 0,
            a: Box::new(CILNode::LDStaticField(Box::new(
                StaticFieldDescriptor::new(None, uint8_ptr_ptr, "environ".into()),
            ))),
            b: Box::new(conv_usize!(ldc_u32!(0)).cast_ptr(uint8_ptr_ptr)),
        }
        .into(),
    );
    first_check.trees_mut().push(
        CILRoot::GoTo {
            target: init_bb,
            sub_target: 0,
        }
        .into(),
    );
    let init = &mut blocks[init_bb as usize];
    init.trees_mut().push(
        CILRoot::STLoc {
            local: dictionary_local,
            tree: call!(
                CallSite::new(
                    Some(ClassRef::enviroment(asm)),
                    "GetEnvironmentVariables".into(),
                    FnSig::new(Box::new([]), Type::ClassRef(ClassRef::i_dictionary(asm))),
                    true
                ),
                []
            ),
        }
        .into(),
    );
    init.trees_mut().push(
        CILRoot::STLoc {
            local: envc,
            tree: call_virt!(
                CallSite::new(
                    Some(ClassRef::i_collection(asm)),
                    "get_Count".into(),
                    FnSig::new(
                        Box::new([Type::ClassRef(ClassRef::i_dictionary(asm))]),
                        Type::Int(Int::I32)
                    ),
                    false
                ),
                [CILNode::LDLoc(dictionary_local)]
            ),
        }
        .into(),
    );
    let element_count = CILNode::LDLoc(envc) + ldc_i32!(1);
    let arr_size = conv_usize!(element_count) * conv_usize!(size_of!(uint8_ptr_ptr));
    let arr_align = conv_usize!(size_of!(uint8_ptr_ptr));
    init.trees_mut().push(
        CILRoot::STLoc {
            local: arr_ptr,
            tree: call!(CallSite::aligned_alloc(asm), [arr_size, arr_align])
                .cast_ptr(uint8_ptr_ptr),
        }
        .into(),
    );
    init.trees_mut().push(
        CILRoot::STLoc {
            local: idx,
            tree: ldc_i32!(0),
        }
        .into(),
    );
    init.trees_mut().push(
        CILRoot::STLoc {
            local: iter_local,
            tree: call_virt!(
                CallSite::new(
                    Some(ClassRef::i_dictionary(asm)),
                    "GetEnumerator".into(),
                    FnSig::new(
                        Box::new([Type::ClassRef(ClassRef::i_dictionary(asm))]),
                        Type::ClassRef(ClassRef::dictionary_iterator(asm))
                    ),
                    false
                ),
                [CILNode::LDLoc(dictionary_local)]
            ),
        }
        .into(),
    );

    init.trees_mut().push(
        CILRoot::GoTo {
            target: loop_body_bb,
            sub_target: 0,
        }
        .into(),
    );
    let ret = &mut blocks[ret_bb as usize];
    ret.trees_mut().push(
        CILRoot::Ret {
            tree: CILNode::LDStaticField(Box::new(StaticFieldDescriptor::new(
                None,
                uint8_ptr_ptr,
                "environ".into(),
            ))),
        }
        .into(),
    );
    let loop_body = &mut blocks[loop_body_bb as usize];
    loop_body.trees_mut().push(
        CILRoot::BFalse {
            target: loop_end_bb,
            sub_target: 0,
            cond: call_virt!(
                CallSite::new_extern(
                    ClassRef::i_enumerator(asm),
                    "MoveNext".into(),
                    FnSig::new(
                        Box::new([Type::ClassRef(ClassRef::dictionary_iterator(asm))]),
                        Type::Bool,
                    ),
                    false
                ),
                [CILNode::LDLoc(iter_local)]
            ),
        }
        .into(),
    );
    loop_body.trees_mut().push(
        CILRoot::STLoc {
            local: keyval,
            tree: CILNode::UnboxAny(
                Box::new(call_virt!(
                    CallSite::new_extern(
                        ClassRef::i_enumerator(asm),
                        "get_Current".into(),
                        FnSig::new(
                            Box::new([Type::ClassRef(ClassRef::dictionary_iterator(asm))]),
                            Type::PlatformObject,
                        ),
                        false
                    ),
                    [CILNode::LDLoc(iter_local)]
                )),
                Box::new(Type::ClassRef(keyval_tpe)),
            ),
        }
        .into(),
    );
    let key = call!(
        CallSite::new_extern(
            keyval_tpe,
            "get_Key".into(),
            FnSig::new(
                Box::new([asm.nref(Type::ClassRef(keyval_tpe))]),
                Type::PlatformObject
            ),
            false,
        ),
        [CILNode::LDLocA(keyval)]
    );
    let value = call!(
        CallSite::new_extern(
            keyval_tpe,
            "get_Value".into(),
            FnSig::new(
                Box::new([asm.nref(Type::ClassRef(keyval_tpe))]),
                Type::PlatformObject
            ),
            false,
        ),
        [CILNode::LDLocA(keyval)]
    );
    loop_body.trees_mut().push(
        CILRoot::STLoc {
            local: encoded_keyval,
            tree: call!(
                CallSite::new_extern(
                    ClassRef::string(asm),
                    "Concat".into(),
                    FnSig::new(
                        Box::new([
                            Type::PlatformObject,
                            Type::PlatformObject,
                            Type::PlatformObject
                        ]),
                        Type::PlatformString
                    ),
                    true
                ),
                [key, CILNode::LdStr("=".into()), value]
            ),
        }
        .into(),
    );
    let utf8_kval = mstring_to_utf8ptr(CILNode::LDLoc(encoded_keyval), asm);
    loop_body.trees_mut().push(
        CILRoot::STIndPtr(
            CILNode::LDLoc(arr_ptr) + conv_usize!(CILNode::LDLoc(idx) * size_of!(uint8_ptr_ptr)),
            utf8_kval,
            Box::new(Type::Int(Int::U8)),
        )
        .into(),
    );
    loop_body.trees_mut().push(
        CILRoot::STLoc {
            local: idx,
            tree: CILNode::LDLoc(idx) + ldc_i32!(1),
        }
        .into(),
    );

    loop_body.trees_mut().push(
        CILRoot::GoTo {
            target: loop_body_bb,
            sub_target: 0,
        }
        .into(),
    );
    let loop_end = &mut blocks[loop_end_bb as usize];
    let null_ptr = conv_usize!(ldc_u32!(0)).cast_ptr(uint8_ptr);
    loop_end.trees_mut().push(
        CILRoot::STIndPtr(
            CILNode::LDLoc(arr_ptr) + conv_usize!(CILNode::LDLoc(envc) * size_of!(uint8_ptr_ptr)),
            null_ptr,
            Box::new(Type::Int(Int::U8)),
        )
        .into(),
    );
    loop_end.trees_mut().push(
        CILRoot::SetStaticField {
            descr: Box::new(StaticFieldDescriptor::new(
                None,
                uint8_ptr_ptr,
                "environ".into(),
            )),
            value: CILNode::LDLoc(arr_ptr),
        }
        .into(),
    );
    drop(blocks);
    asm.add_method(get_environ);

    asm.add_static(uint8_ptr_ptr, "environ", true);
    init_cs
}
static CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'r', 's', 't', 'u', 'w', 'v', 'x', 'y', 'z', 'A', 'B', 'C',
    'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'R', 'S', 'T', 'U', 'W', 'V',
    'X', 'Y', 'Z', '_',
];
pub fn encode(mut int: u64) -> String {
    let mut res = String::new();
    while int != 0 {
        let curr = int % (CHARS.len() as u64);
        res.push(CHARS[curr as usize]);
        int /= CHARS.len() as u64;
    }
    res
}
/// Checks if all elements in a slice are truly unquie.
#[track_caller]
pub fn assert_unique<T: std::hash::Hash + PartialEq + Eq>(val: &[T], msg: impl Debug) {
    let mut set = std::collections::HashSet::new();
    set.extend(val.iter());
    assert_eq!(set.len(), val.len(), "{msg:?}");
}
#[must_use]
pub fn escape_class_name(name: &str) -> String {
    name.replace("::", ".")
        .replace("..", ".")
        .replace('$', "_dsig_")
        .replace('<', "_lt_")
        .replace('\'', "_ap_")
        .replace(' ', "_spc_")
        .replace('>', "_gt_")
        .replace('(', "_lpar_")
        .replace(')', "_rpar")
        .replace('{', "_lbra_")
        .replace('}', "_rbra")
        .replace('[', "_lsbra_")
        .replace(']', "_rsbra_")
        .replace('+', "_pls_")
        .replace('-', "_hyp_")
        .replace(',', "_com_")
        .replace('*', "_ptr_")
        .replace('#', "_hsh_")
        .replace('&', "_ref_")
        .replace(';', "_scol_")
        .replace('!', "_excl_")
        .replace('\"', "_qt_")
}
#[test]
fn argv() {
    let mut asm = Assembly::empty();
    argc_argv_init_method(&mut asm);
}

#[test]
fn environ() {
    let mut asm = Assembly::empty();
    get_environ(&mut asm);
}
#[test]
fn test_escape_name() {
    assert_eq!(escape_class_name("SomeFunnyType"), "SomeFunnyType");
    assert_eq!(
        escape_class_name("MyNamespace::SomeFunnyType"),
        "MyNamespace.SomeFunnyType"
    );
    assert_eq!(
        escape_class_name("MyNamespace..SomeFunnyType"),
        "MyNamespace.SomeFunnyType"
    );
    assert_eq!(
        escape_class_name("SomeFunnyType<[Inner]>"),
        "SomeFunnyType_lt__lsbra_Inner_rsbra__gt_"
    );
}
