use std::fmt::Debug;

use crate::bimap::Interned;
use crate::method::Method;

use crate::cilnode::{ExtendKind, MethodKind};
use crate::{call, call_virt, conv_usize, IntoAsmIndex, MethodDef, Type};
use crate::{cil_node::V1Node, cil_root::V1Root, Assembly};
use crate::{ClassRef, FnSig, Int, MethodRef, StaticFieldDesc};

pub fn argc_argv_init_method(asm: &mut Assembly) -> Interned<MethodRef> {
    let init_cs = MethodRef::new(
        *asm.main_module(),
        asm.alloc_string("argc_argv_init"),
        asm.sig([], Type::Void),
        MethodKind::Static,
        vec![].into(),
    );

    asm.alloc_methodref(init_cs)
}
pub fn mstring_to_utf8ptr(mstring: V1Node, asm: &mut Assembly) -> V1Node {
    let mref = MethodRef::new(
        ClassRef::marshal(asm),
        asm.alloc_string("StringToCoTaskMemUTF8"),
        asm.sig([Type::PlatformString], Type::Int(Int::ISize)),
        MethodKind::Static,
        vec![].into(),
    );
    call!(asm.alloc_methodref(mref), [mstring]).cast_ptr(asm.nptr(Type::Int(Int::U8)))
}

pub fn get_environ(asm: &mut Assembly) -> Interned<MethodRef> {
    let main_module = asm.main_module();
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let uint8_ptr_ptr = asm.nptr(uint8_ptr);
    let init_cs = MethodRef::new(
        *asm.main_module(),
        asm.alloc_string("get_environ"),
        asm.sig([], uint8_ptr_ptr),
        MethodKind::Static,
        vec![].into(),
    );
    let init_cs = asm.alloc_methodref(init_cs);
    if asm.method_def_from_ref(init_cs).is_some() {
        return init_cs;
    }
    let mut get_environ = Method::new(
        crate::Access::Extern,
        crate::method::MethodType::Static,
        FnSig::new([], uint8_ptr_ptr),
        "get_environ",
        vec![],
        vec![],
        vec![],
        asm,
    );
    // Environ static
    let environ = StaticFieldDesc::new(
        *asm.main_module(),
        asm.alloc_string("environ"),
        uint8_ptr_ptr,
    );
    let environ = asm.alloc_sfld(environ);
    let environ = asm.alloc_node(crate::v2::CILNode::LdStaticField(environ));
    let dictionary_local = get_environ.add_local(
        Type::ClassRef(ClassRef::i_dictionary(asm)),
        Some("dict"),
        asm,
    ) as u32;
    let envc = get_environ.add_local(Type::Int(Int::I32), Some("envc"), asm) as u32;
    let arr_ptr = get_environ.add_local(uint8_ptr_ptr, Some("arr_ptr"), asm) as u32;
    let idx = get_environ.add_local(Type::Int(Int::I32), Some("idx"), asm) as u32;
    let iter_local = get_environ.add_local(
        Type::ClassRef(ClassRef::dictionary_iterator(asm)),
        Some("iter"),
        asm,
    ) as u32;
    let keyval_tpe = ClassRef::dictionary_entry(asm);
    let keyval = get_environ.add_local(Type::ClassRef(keyval_tpe), Some("keyval"), asm) as u32;
    let encoded_keyval = get_environ.add_local(
        Type::ClassRef(ClassRef::string(asm)),
        Some("encoded_keyval"),
        asm,
    ) as u32;
    let first_check_bb = get_environ.new_bb();
    let init_bb = get_environ.new_bb();
    let loop_body_bb = get_environ.new_bb();
    let loop_end_bb = get_environ.new_bb();
    let ret_bb = get_environ.new_bb();
    let mut blocks = get_environ.blocks_mut();
    let first_check = &mut blocks[first_check_bb as usize];
    first_check.trees_mut().push(
        V1Root::BNe {
            target: ret_bb,
            sub_target: 0,
            a: Box::new(V1Node::V2(environ)),
            b: Box::new(conv_usize!(V1Node::V2(asm.alloc_node(0_i32))).cast_ptr(uint8_ptr_ptr)),
        }
        .into(),
    );
    first_check.trees_mut().push(
        V1Root::GoTo {
            target: init_bb,
            sub_target: 0,
        }
        .into(),
    );
    let init = &mut blocks[init_bb as usize];
    let i_dictionary = Type::ClassRef(ClassRef::i_dictionary(asm));
    let mref = MethodRef::new(
        ClassRef::enviroment(asm),
        asm.alloc_string("GetEnvironmentVariables"),
        asm.sig([], i_dictionary),
        MethodKind::Static,
        vec![].into(),
    );
    init.trees_mut().push(
        V1Root::STLoc {
            local: dictionary_local,
            tree: call!(asm.alloc_methodref(mref), []),
        }
        .into(),
    );
    let mref = MethodRef::new(
        ClassRef::i_collection(asm),
        asm.alloc_string("get_Count"),
        asm.sig([i_dictionary], Type::Int(Int::I32)),
        MethodKind::Virtual,
        vec![].into(),
    );
    init.trees_mut().push(
        V1Root::STLoc {
            local: envc,
            tree: call_virt!(
                asm.alloc_methodref(mref),
                [V1Node::LDLoc(dictionary_local)]
            ),
        }
        .into(),
    );
    let element_count = V1Node::LDLoc(envc) + V1Node::V2(asm.alloc_node(1_i32));
    let stride = asm.size_of(uint8_ptr_ptr);
    let stride = asm.int_cast(stride, Int::USize, ExtendKind::ZeroExtend);
    let arr_size = conv_usize!(element_count) * V1Node::V2(stride.into_idx(asm));
    let arr_align = conv_usize!(V1Node::V2(asm.size_of(uint8_ptr_ptr).into_idx(asm)));
    let aligned_alloc = MethodRef::aligned_alloc(asm);
    init.trees_mut().push(
        V1Root::STLoc {
            local: arr_ptr,
            tree: call!(asm.alloc_methodref(aligned_alloc), [arr_size, arr_align])
                .cast_ptr(uint8_ptr_ptr),
        }
        .into(),
    );
    init.trees_mut().push(
        V1Root::STLoc {
            local: idx,
            tree: V1Node::V2(asm.alloc_node(0_i32)),
        }
        .into(),
    );
    let dictionary_iterator = ClassRef::dictionary_iterator(asm);
    let mref = MethodRef::new(
        ClassRef::i_dictionary(asm),
        asm.alloc_string("GetEnumerator"),
        asm.sig([i_dictionary], Type::ClassRef(dictionary_iterator)),
        MethodKind::Virtual,
        vec![].into(),
    );
    init.trees_mut().push(
        V1Root::STLoc {
            local: iter_local,
            tree: call_virt!(
                asm.alloc_methodref(mref),
                [V1Node::LDLoc(dictionary_local)]
            ),
        }
        .into(),
    );

    init.trees_mut().push(
        V1Root::GoTo {
            target: loop_body_bb,
            sub_target: 0,
        }
        .into(),
    );
    let ret = &mut blocks[ret_bb as usize];

    ret.trees_mut()
        .push(V1Root::V2(asm.alloc_root(crate::v2::CILRoot::Ret(environ))).into());
    let loop_body = &mut blocks[loop_body_bb as usize];
    let move_next = MethodRef::new(
        ClassRef::i_enumerator(asm),
        asm.alloc_string("MoveNext"),
        asm.sig([Type::ClassRef(dictionary_iterator)], Type::Bool),
        MethodKind::Virtual,
        vec![].into(),
    );
    loop_body.trees_mut().push(
        V1Root::BFalse {
            target: loop_end_bb,
            sub_target: 0,
            cond: call_virt!(asm.alloc_methodref(move_next), [V1Node::LDLoc(iter_local)]),
        }
        .into(),
    );
    let get_current = MethodRef::new(
        ClassRef::i_enumerator(asm),
        asm.alloc_string("get_Current"),
        asm.sig([Type::ClassRef(dictionary_iterator)], Type::PlatformObject),
        MethodKind::Virtual,
        vec![].into(),
    );
    loop_body.trees_mut().push(
        V1Root::STLoc {
            local: keyval,
            tree: V1Node::UnboxAny(
                Box::new(call_virt!(
                    asm.alloc_methodref(get_current),
                    [V1Node::LDLoc(iter_local)]
                )),
                Box::new(Type::ClassRef(keyval_tpe)),
            ),
        }
        .into(),
    );
    let keyval_tpe_ref = asm.nref(Type::ClassRef(keyval_tpe));
    let sig = asm.sig([keyval_tpe_ref], Type::PlatformObject);
    let get_key = MethodRef::new(
        keyval_tpe,
        asm.alloc_string("get_Key"),
        sig,
        MethodKind::Instance,
        vec![].into(),
    );
    let kv = V1Node::V2(asm.alloc_node(crate::v2::CILNode::LdLocA(keyval)));
    let key = call!(asm.alloc_methodref(get_key), [kv.clone()]);
    let mref = MethodRef::new(
        keyval_tpe,
        asm.alloc_string("get_Value"),
        sig,
        MethodKind::Instance,
        vec![].into(),
    );
    let value = call!(asm.alloc_methodref(mref), [kv]);
    let concat = MethodRef::new(
        ClassRef::string(asm),
        asm.alloc_string("Concat"),
        asm.sig(
            [
                Type::PlatformObject,
                Type::PlatformObject,
                Type::PlatformObject,
            ],
            Type::PlatformString,
        ),
        MethodKind::Static,
        vec![].into(),
    );
    loop_body.trees_mut().push(
        V1Root::STLoc {
            local: encoded_keyval,
            tree: call!(
                asm.alloc_methodref(concat),
                [key, V1Node::LdStr("=".into()), value]
            ),
        }
        .into(),
    );
    let utf8_kval = mstring_to_utf8ptr(V1Node::LDLoc(encoded_keyval), asm);
    loop_body.trees_mut().push(
        V1Root::STIndPtr(
            V1Node::LDLoc(arr_ptr)
                + conv_usize!(
                    V1Node::LDLoc(idx) * V1Node::V2(asm.size_of(uint8_ptr_ptr).into_idx(asm))
                ),
            utf8_kval,
            Box::new(Type::Int(Int::U8)),
        )
        .into(),
    );
    loop_body.trees_mut().push(
        V1Root::STLoc {
            local: idx,
            tree: V1Node::LDLoc(idx) + V1Node::V2(asm.alloc_node(1_i32)),
        }
        .into(),
    );

    loop_body.trees_mut().push(
        V1Root::GoTo {
            target: loop_body_bb,
            sub_target: 0,
        }
        .into(),
    );
    let loop_end = &mut blocks[loop_end_bb as usize];
    let null_ptr = conv_usize!(V1Node::V2(asm.alloc_node(0_i32))).cast_ptr(uint8_ptr);
    loop_end.trees_mut().push(
        V1Root::STIndPtr(
            V1Node::LDLoc(arr_ptr)
                + conv_usize!(
                    V1Node::LDLoc(envc) * V1Node::V2(asm.size_of(uint8_ptr_ptr).into_idx(asm))
                ),
            null_ptr,
            Box::new(Type::Int(Int::U8)),
        )
        .into(),
    );
    loop_end.trees_mut().push(
        V1Root::SetStaticField {
            descr: Box::new(StaticFieldDesc::new(
                *asm.main_module(),
                asm.alloc_string("environ"),
                uint8_ptr_ptr,
            )),
            value: V1Node::LDLoc(arr_ptr),
        }
        .into(),
    );
    drop(blocks);

    let def = MethodDef::from_v1(&get_environ, asm, main_module);
    asm.new_method(def);
    asm.add_static(uint8_ptr_ptr, "environ", true, main_module, None, false);
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
    #[cfg(debug_assertions)]
    {
        let mut set = std::collections::HashSet::new();
        set.extend(val.iter());
        assert_eq!(set.len(), val.len(), "{msg:?}");
    }
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
/*
#[test]
fn argv() {
    let mut asm = Assembly::empty();
    argc_argv_init_method(&mut asm);
} */

#[test]
fn environ() {
    let mut asm = Assembly::default();
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
