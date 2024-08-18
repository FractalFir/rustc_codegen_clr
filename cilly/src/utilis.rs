use std::fmt::Debug;

use fxhash::FxHashMap;

use crate::method::Method;
use crate::static_field_desc::StaticFieldDescriptor;
use crate::{
    asm::Assembly, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, eq, lt, size_of,
};
use crate::{
    call, call_virt, conv_i32, conv_usize, ldc_i32, ldc_u32, mul, ptr, DotnetTypeRef, FnSig, Type,
};
pub fn argc_argv_init_method(asm: &mut Assembly) -> CallSite {
    use std::num::NonZeroU8;
    let init_cs = CallSite::new(
        None,
        "argc_argv_init".into(),
        FnSig::new(&[], Type::Void),
        true,
    );
    if asm.contains_fn(&init_cs) {
        return init_cs;
    }
    let mut init_method = Method::new(
        crate::access_modifier::AccessModifer::Extern,
        crate::method::MethodType::Static,
        FnSig::new(&[], Type::Void),
        "argc_argv_init",
        vec![],
        vec![],
        vec![],
    );

    // Allocate the variables necesarry for initializng args.
    let argc = u32::try_from(init_method.add_local(Type::I32, Some("argc".into()))).unwrap();
    let argv =
        u32::try_from(init_method.add_local(ptr!(ptr!(Type::U8)), Some("argv".into()))).unwrap();
    let managed_args = u32::try_from(init_method.add_local(
        Type::ManagedArray {
            element: Box::new(DotnetTypeRef::string_type().into()),
            dims: NonZeroU8::new(1).unwrap(),
        },
        Some("managed_args".into()),
    ))
    .unwrap();
    let arg_idx = u32::try_from(init_method.add_local(Type::I32, Some("arg_idx".into()))).unwrap();
    // Get managed args
    let margs_init = CILRoot::STLoc {
        local: managed_args,
        tree: call!(
            CallSite::new_extern(
                DotnetTypeRef::enviroment(),
                "GetCommandLineArgs".into(),
                FnSig::new(
                    &[],
                    Type::ManagedArray {
                        element: Box::new(DotnetTypeRef::string_type().into()),
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
        CallSite::aligned_alloc(),
        [
            mul!(
                conv_usize!(CILNode::LDLoc(argc)),
                conv_usize!(size_of!(Type::USize))
            ),
            conv_usize!(ldc_u32!(8))
        ]
    )
    .cast_ptr(ptr!(ptr!(Type::U8)));
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
    let uarg = mstring_to_utf8ptr(arg_nth);
    // Store the converted arg at idx+1
    loop_block.trees_mut().push(
        CILRoot::STIndPtr(
            CILNode::LDLoc(argv) + conv_usize!(size_of!(Type::ISize) * CILNode::LDLoc(arg_idx)),
            uarg,
            Box::new(Type::U8),
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
    let argv_static = StaticFieldDescriptor::new(None, ptr!(ptr!(Type::U8)), "argv".into());
    loop_end_block.trees_mut().push(
        CILRoot::SetStaticField {
            descr: Box::new(argv_static),
            value: CILNode::LDLoc(argv),
        }
        .into(),
    );
    let argc_static = StaticFieldDescriptor::new(None, Type::I32, "argc".into());
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
    asm.add_static(ptr!(ptr!(Type::U8)), "argv", false);
    asm.add_static(Type::I32, "argc", false);
    init_cs
}
pub fn mstring_to_utf8ptr(mstring: CILNode) -> CILNode {
    call!(
        CallSite::new_extern(
            DotnetTypeRef::marshal(),
            "StringToCoTaskMemUTF8".into(),
            FnSig::new(&[DotnetTypeRef::string_type().into()], Type::ISize),
            true
        ),
        [mstring]
    )
    .cast_ptr(ptr!(Type::U8))
}
/*
 .method public hidebysig
        instance uint8** M () cil managed
    {
        // Method begins at RVA 0x2050
        // Code size 182 (0xb6)
        .maxstack 3
        .locals init (
            [0] int32 count,
            [1] uint8** ptr,
            [2] native uint num,
            [3] class [System.Runtime]System.Collections.IDictionaryEnumerator enumerator,
            [4] valuetype [System.Runtime]System.ValueTuple`2<string, string> valueTuple,
            [5] string s,
            [6] class [System.Runtime]System.IDisposable disposable
        )

        IL_0000: ldsfld uint8** C::environ
        IL_0005: ldc.i4.0
        IL_0006: conv.u
        IL_0007: beq.s IL_000f

        IL_0009: ldsfld uint8** C::environ
        IL_000e: ret

        IL_000f: call class [System.Runtime]System.Collections.IDictionary [System.Runtime]System.Environment::GetEnvironmentVariables()
        IL_0014: dup
        IL_0015: callvirt instance int32 [System.Runtime]System.Collections.ICollection::get_Count()
        IL_001a: stloc.0
        IL_001b: ldloc.0
        IL_001c: ldc.i4.1
        IL_001d: add
        IL_001e: sizeof uint8*
        IL_0024: mul
        IL_0025: conv.i
        IL_0026: sizeof uint8*
        IL_002c: conv.i
        IL_002d: call void* [System.Runtime.InteropServices]System.Runtime.InteropServices.NativeMemory::AlignedAlloc(native uint, native uint)
        IL_0032: stloc.1
        IL_0033: ldc.i4.0
        IL_0034: conv.i
        IL_0035: stloc.2
        IL_0036: callvirt instance class [System.Runtime]System.Collections.IDictionaryEnumerator [System.Runtime]System.Collections.IDictionary::GetEnumerator()
        IL_003b: stloc.3
        .try
        {
            // sequence point: hidden
            IL_003c: br.s IL_007e
            // loop start (head: IL_007e)
                IL_003e: ldloc.3

                IL_003f: callvirt instance object [System.Runtime]System.Collections.IEnumerator::get_Current()
                IL_0044: unbox.any valuetype [System.Runtime]System.ValueTuple`2<string, string>
                IL_0049: stloc.s 4
                IL_004b: ldloc.s 4
                IL_004d: ldfld !0 valuetype [System.Runtime]System.ValueTuple`2<string, string>::Item1
                IL_0052: ldstr "="
                IL_0057: ldloc.s 4
                IL_0059: ldfld !1 valuetype [System.Runtime]System.ValueTuple`2<string, string>::Item2
                IL_005e: call string [System.Runtime]System.String::Concat(string, string, string)
                IL_0063: stloc.s 5
                IL_0065: ldloc.1
                IL_0066: ldloc.2
                IL_0067: conv.u8
                IL_0068: sizeof uint8*
                IL_006e: conv.i8
                IL_006f: mul
                IL_0070: conv.u
                IL_0071: add
                IL_0072: ldloc.s 5
                IL_0074: call native int [System.Runtime.InteropServices]System.Runtime.InteropServices.Marshal::StringToCoTaskMemUTF8(string)
                IL_0079: stind.i
                IL_007a: ldloc.2
                IL_007b: ldc.i4.1
                IL_007c: add
                IL_007d: stloc.2

                IL_007e: ldloc.3
                IL_007f: callvirt instance bool [System.Runtime]System.Collections.IEnumerator::MoveNext()
                IL_0084: brtrue.s IL_003e
            // end loop

            IL_0086: leave.s IL_009c
        } // end .try
        finally
        {
            IL_0088: ldloc.3
            IL_0089: isinst [System.Runtime]System.IDisposable
            IL_008e: stloc.s 6
            IL_0090: ldloc.s 6
            IL_0092: brfalse.s IL_009b

            IL_0094: ldloc.s 6
            IL_0096: callvirt instance void [System.Runtime]System.IDisposable::Dispose()

            IL_009b: endfinally
        } // end handler

        IL_009c: ldloc.1
        IL_009d: ldloc.0
        IL_009e: conv.i
        IL_009f: sizeof uint8*
        IL_00a5: mul
        IL_00a6: add
        IL_00a7: ldc.i4.0
        IL_00a8: conv.u
        IL_00a9: stind.i
        IL_00aa: ldloc.1
        IL_00ab: stsfld uint8** C::environ
        IL_00b0: ldsfld uint8** C::environ
        IL_00b5: ret
    } // end of method C::M

*/
pub fn get_environ(asm: &mut Assembly) -> CallSite {
    let init_cs = CallSite::new(
        None,
        "get_environ".into(),
        FnSig::new(&[], ptr!(ptr!(Type::U8))),
        true,
    );
    if asm.contains_fn(&init_cs) {
        return init_cs;
    }
    let mut get_environ = Method::new(
        crate::access_modifier::AccessModifer::Extern,
        crate::method::MethodType::Static,
        FnSig::new(&[], ptr!(ptr!(Type::U8))),
        "get_environ",
        vec![],
        vec![],
        vec![],
    );
    let dictionary_local = get_environ.add_local(
        Type::DotnetType(Box::new(DotnetTypeRef::i_dictionary())),
        Some("dict".into()),
    ) as u32;
    let envc = get_environ.add_local(Type::I32, Some("envc".into())) as u32;
    let arr_ptr = get_environ.add_local(ptr!(ptr!(Type::U8)), Some("arr_ptr".into())) as u32;
    let idx = get_environ.add_local(Type::I32, Some("idx".into())) as u32;
    let iter_local = get_environ.add_local(
        Type::DotnetType(Box::new(DotnetTypeRef::dictionary_iterator())),
        Some("iter".into()),
    ) as u32;
    let keyval_tpe = DotnetTypeRef::dictionary_entry();
    let keyval = get_environ.add_local(
        Type::DotnetType(Box::new(keyval_tpe.clone())),
        Some("keyval".into()),
    ) as u32;
    let encoded_keyval = get_environ.add_local(
        Type::DotnetType(Box::new(DotnetTypeRef::string_type())),
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
        CILRoot::BTrue {
            target: ret_bb,
            sub_target: 0,
            cond: CILNode::LDStaticField(Box::new(StaticFieldDescriptor::new(
                None,
                Type::Bool,
                "environ_init_status".into(),
            ))),
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
                    Some(DotnetTypeRef::enviroment()),
                    "GetEnvironmentVariables".into(),
                    FnSig::new(
                        &[],
                        Type::DotnetType(Box::new(DotnetTypeRef::i_dictionary()))
                    ),
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
                    Some(DotnetTypeRef::collection()),
                    "get_Count".into(),
                    FnSig::new(
                        &[Type::DotnetType(Box::new(DotnetTypeRef::i_dictionary()))],
                        Type::I32
                    ),
                    false
                ),
                [CILNode::LDLoc(dictionary_local)]
            ),
        }
        .into(),
    );
    let element_count = CILNode::LDLoc(envc) + ldc_i32!(1);
    let arr_size = conv_usize!(element_count * size_of!(ptr!(ptr!(Type::U8))));
    let arr_align = conv_usize!(size_of!(ptr!(ptr!(Type::U8))));
    init.trees_mut().push(
        CILRoot::STLoc {
            local: arr_ptr,
            tree: call!(CallSite::aligned_alloc(), [arr_size, arr_align])
                .cast_ptr(ptr!(ptr!(Type::U8))),
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
                    Some(DotnetTypeRef::i_dictionary()),
                    "GetEnumerator".into(),
                    FnSig::new(
                        &[Type::DotnetType(Box::new(DotnetTypeRef::i_dictionary()))],
                        Type::DotnetType(Box::new(DotnetTypeRef::dictionary_iterator()))
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
                ptr!(ptr!(Type::U8)),
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
                    DotnetTypeRef::collection_iterator(),
                    "MoveNext".into(),
                    FnSig::new(
                        &[Type::DotnetType(Box::new(
                            DotnetTypeRef::dictionary_iterator()
                        ))],
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
                        DotnetTypeRef::collection_iterator(),
                        "get_Current".into(),
                        FnSig::new(
                            &[Type::DotnetType(Box::new(
                                DotnetTypeRef::dictionary_iterator()
                            ))],
                            Type::DotnetType(Box::new(DotnetTypeRef::object_type())),
                        ),
                        false
                    ),
                    [CILNode::LDLoc(iter_local)]
                )),
                Box::new(Type::DotnetType(Box::new(keyval_tpe.clone()))),
            ),
        }
        .into(),
    );
    let key = call!(
        CallSite::new_extern(
            keyval_tpe.clone(),
            "get_Key".into(),
            FnSig::new(
                [Type::ManagedReference(
                    Type::DotnetType(Box::new(keyval_tpe.clone())).into()
                )],
                Type::DotnetType(Box::new(DotnetTypeRef::object_type()))
            ),
            false,
        ),
        [CILNode::LDLocA(keyval)]
    );
    let value = call!(
        CallSite::new_extern(
            keyval_tpe.clone(),
            "get_Value".into(),
            FnSig::new(
                [Type::ManagedReference(
                    Type::DotnetType(Box::new(keyval_tpe.clone())).into()
                )],
                Type::DotnetType(Box::new(DotnetTypeRef::object_type()))
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
                    DotnetTypeRef::string_type(),
                    "Concat".into(),
                    FnSig::new(
                        &[
                            Type::DotnetType(Box::new(DotnetTypeRef::object_type())),
                            Type::DotnetType(Box::new(DotnetTypeRef::object_type())),
                            Type::DotnetType(Box::new(DotnetTypeRef::object_type()))
                        ],
                        Type::DotnetType(Box::new(DotnetTypeRef::string_type()))
                    ),
                    true
                ),
                [key, CILNode::LdStr("=".into()), value]
            ),
        }
        .into(),
    );
    let utf8_kval = mstring_to_utf8ptr(CILNode::LDLoc(encoded_keyval));
    loop_body.trees_mut().push(
        CILRoot::STIndPtr(
            CILNode::LDLoc(arr_ptr)
                + conv_usize!(CILNode::LDLoc(idx) * size_of!(ptr!(ptr!(Type::U8)))),
            utf8_kval,
            Box::new(Type::U8),
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
    let null_ptr = conv_usize!(ldc_u32!(0)).cast_ptr(ptr!(Type::U8));
    loop_end.trees_mut().push(
        CILRoot::STIndPtr(
            CILNode::LDLoc(arr_ptr)
                + conv_usize!(CILNode::LDLoc(envc) * size_of!(ptr!(ptr!(Type::U8)))),
            null_ptr,
            Box::new(Type::U8),
        )
        .into(),
    );
    drop(blocks);
    asm.add_method(get_environ);
    asm.add_static(Type::Bool, "environ_init_status", false);
    asm.add_static(ptr!(ptr!(Type::U8)), "environ", false);
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
#[test]
fn argv() {
    let mut asm = Assembly::empty();
    argc_argv_init_method(&mut asm);
}
pub trait MemoryUsage {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize;
}
pub trait MemoryUsageCounter {
    fn add_type(&mut self, tpe_name: &str, size: usize);
    fn add_field(&mut self, tpe_name: &str, field_name: &str, size: usize);
}
impl<K: MemoryUsage, V: MemoryUsage> MemoryUsage for FxHashMap<K, V> {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let values = self.values().map(|val| val.memory_usage(counter)).sum();
        counter.add_field(tpe_name, "values", values);
        total_size += values;
        let keys = self.values().map(|val| val.memory_usage(counter)).sum();
        total_size += keys;
        counter.add_field(tpe_name, "keys", keys);
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
impl<T: MemoryUsage> MemoryUsage for Vec<T> {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let elements = self.iter().map(|val| val.memory_usage(counter)).sum();
        counter.add_field(tpe_name, "elements", elements);
        total_size += elements;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}

impl<T: MemoryUsage> MemoryUsage for Option<T> {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let inner = match self.as_ref() {
            Some(inner) => inner.memory_usage(counter),
            None => 0,
        };
        counter.add_field(tpe_name, "inner", inner);
        total_size += inner;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}

impl<T: MemoryUsage> MemoryUsage for Box<T> {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let inner = self.as_ref().memory_usage(counter);
        counter.add_field(tpe_name, "inner", inner);
        total_size += inner;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
impl MemoryUsage for Box<str> {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let inner = self.as_ref().len();
        counter.add_field(tpe_name, "inner", inner);
        total_size += inner;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
impl MemoryUsage for bool {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let total_size = std::mem::size_of::<Self>();
        counter.add_type("bool", total_size);
        total_size
    }
}
impl MemoryUsage for u16 {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let total_size = std::mem::size_of::<Self>();
        counter.add_type("u16", total_size);
        total_size
    }
}
impl MemoryUsage for std::ops::Range<u64> {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let total_size = std::mem::size_of::<Self>();
        counter.add_type("Range<u64>", total_size);
        total_size
    }
}
impl<T: MemoryUsage> MemoryUsage for &T {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        (*self).memory_usage(counter)
    }
}
impl<A: MemoryUsage> MemoryUsage for (A,) {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let a = self.0.memory_usage(counter);
        counter.add_field(tpe_name, "0", a);
        total_size += a;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
impl<A: MemoryUsage, B: MemoryUsage> MemoryUsage for (A, B) {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let a = self.0.memory_usage(counter);
        counter.add_field(tpe_name, "0", a);
        total_size += a;
        let b = self.1.memory_usage(counter);
        counter.add_field(tpe_name, "1", b);
        total_size += b;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
impl<A: MemoryUsage, B: MemoryUsage, C: MemoryUsage> MemoryUsage for (A, B, C) {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let a = self.0.memory_usage(counter);
        counter.add_field(tpe_name, "0", a);
        total_size += a;
        let b = self.1.memory_usage(counter);
        counter.add_field(tpe_name, "1", b);
        total_size += b;
        let c = self.2.memory_usage(counter);
        counter.add_field(tpe_name, "2", c);
        total_size += c;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
impl<A: MemoryUsage, B: MemoryUsage, C: MemoryUsage, D: MemoryUsage> MemoryUsage for (A, B, C, D) {
    fn memory_usage(&self, counter: &mut impl MemoryUsageCounter) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        let tpe_name = std::any::type_name::<Self>();
        let a = self.0.memory_usage(counter);
        counter.add_field(tpe_name, "0", a);
        total_size += a;
        let b = self.1.memory_usage(counter);
        counter.add_field(tpe_name, "1", b);
        total_size += b;
        let c = self.2.memory_usage(counter);
        counter.add_field(tpe_name, "2", c);
        total_size += c;
        let d = self.3.memory_usage(counter);
        counter.add_field(tpe_name, "3", c);
        total_size += d;
        counter.add_type(tpe_name, total_size);
        total_size
    }
}
#[test]
fn environ() {
    let mut asm = Assembly::empty();

    get_environ(&mut asm);
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
