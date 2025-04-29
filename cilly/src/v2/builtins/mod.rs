use std::num::{NonZeroU32, NonZeroU8};

use crate::{utilis::mstring_to_utf8ptr, IntoAsmIndex, StaticFieldDesc};

use super::{
    asm::MissingMethodPatcher,
    bimap::Interned,
    cilnode::{MethodKind, PtrCastRes},
    cilroot::BranchCond,
    Access, Assembly, BasicBlock, CILNode, CILRoot, ClassDef, ClassRef, Const, FieldDesc, Int,
    MethodDef, MethodImpl, MethodRef, Type,
};

pub mod atomics;
pub mod casts;
pub mod math;
pub mod select;
pub mod thread;
pub use thread::*;
pub mod int128;
pub use int128::*;
pub mod f16;
pub use f16::*;
pub mod simd;

pub fn insert_swap_at_generic(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("swap_at_generic");
    let generator = move |_, asm: &mut Assembly| {
        let buf1 = asm.alloc_node(CILNode::LdArg(0));
        let buf2 = asm.alloc_node(CILNode::LdArg(1));
        let size = asm.alloc_node(CILNode::LdArg(2));
        let tmp_alloc = asm.alloc_node(CILNode::LocAlloc { size });
        let tmp = asm.alloc_node(CILNode::LdLoc(0));
        // Alloc the tmp buffer
        let alloc_buff = asm.alloc_root(CILRoot::StLoc(0, tmp_alloc));
        // Swap buffers
        let buf1_to_tmp = asm.alloc_root(CILRoot::CpBlk(Box::new((tmp, buf1, size))));
        let buf2_to_buff1 = asm.alloc_root(CILRoot::CpBlk(Box::new((buf1, buf2, size))));
        let tmp_to_buf2 = asm.alloc_root(CILRoot::CpBlk(Box::new((buf2, tmp, size))));
        // Ret
        let ret = asm.alloc_root(CILRoot::VoidRet);
        let uint8_ptr = asm.nptr(Type::Int(Int::U8));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![alloc_buff, buf1_to_tmp, buf2_to_buff1, tmp_to_buf2, ret],
                0,
                None,
            )],
            locals: vec![(Some(asm.alloc_string("tmp")), asm.alloc_type(uint8_ptr))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn insert_bounds_check(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("bounds_check");
    let generator = move |_, asm: &mut Assembly| {
        let idx = asm.alloc_node(CILNode::LdArg(0));
        let _size = asm.alloc_node(CILNode::LdArg(1));
        // Ret
        let ret = asm.alloc_root(CILRoot::Ret(idx));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn unaligned_read(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("unaligned_read");
    let generator = move |mref: Interned<MethodRef>, asm: &mut Assembly| {
        let tpe = asm[asm[mref].sig()].output();
        let tpe = asm.alloc_type(*tpe);
        // Copy to a local
        let ptr = asm.alloc_node(CILNode::LdArg(0));
        let local = asm.alloc_node(CILNode::LdLocA(0));
        let size = asm.size_of(tpe);
        let copy = asm.alloc_root(CILRoot::CpBlk(Box::new((local, ptr, size))));
        // Ret
        let local = asm.alloc_node(CILNode::LdLoc(0));
        let ret = asm.alloc_root(CILRoot::Ret(local));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![copy, ret], 0, None)],
            locals: vec![(None, tpe)],
        }
    };
    patcher.insert(name, Box::new(generator));
}

fn insert_rust_alloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_alloc");
    let generator = move |_, asm: &mut Assembly| {
        let size = asm.alloc_node(CILNode::LdArg(0));
        let align = asm.alloc_node(CILNode::LdArg(1));
        let void_ptr = asm.nptr(Type::Void);
        let sig = asm.sig([Type::Int(Int::USize), Type::Int(Int::USize)], void_ptr);
        let aligned_alloc = asm.alloc_string("AlignedAlloc");
        let native_mem = ClassRef::native_mem(asm);
        let call_method = asm.alloc_methodref(MethodRef::new(
            native_mem,
            aligned_alloc,
            sig,
            MethodKind::Static,
            [].into(),
        ));
        let alloc = asm.alloc_node(CILNode::call(call_method, [size, align]));
        let ret = asm.alloc_root(CILRoot::Ret(alloc));
        let cap = asm.alloc_node(Const::USize(ALLOC_CAP));
        let check = asm.alloc_root(CILRoot::Branch(Box::new((
            1,
            0,
            Some(super::cilroot::BranchCond::Gt(
                size,
                cap,
                super::cilroot::CmpKind::Unsigned,
            )),
        ))));
        let zero = asm.alloc_node(Const::USize(0));
        let ret_zero = CILRoot::Ret(zero);

        let throw = asm.alloc_root(ret_zero);
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(vec![check, ret], 0, None),
                BasicBlock::new(vec![throw], 1, None),
            ],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_rust_alloc_zeroed(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_alloc_zeroed");
    let generator = move |_, asm: &mut Assembly| {
        let size = asm.alloc_node(CILNode::LdArg(0));
        let align = asm.alloc_node(CILNode::LdArg(1));
        let void_ptr = asm.nptr(Type::Void);
        let void_idx = asm.alloc_type(Type::Void);
        let sig = asm.sig([Type::Int(Int::USize), Type::Int(Int::USize)], void_ptr);
        let aligned_alloc = asm.alloc_string("AlignedAlloc");
        let native_mem = ClassRef::native_mem(asm);
        let call_method = asm.alloc_methodref(MethodRef::new(
            native_mem,
            aligned_alloc,
            sig,
            MethodKind::Static,
            [].into(),
        ));
        let alloc = asm.alloc_node(CILNode::call(call_method, [size, align]));
        let alloc = asm.alloc_node(CILNode::PtrCast(
            alloc,
            Box::new(super::cilnode::PtrCastRes::Ptr(void_idx)),
        ));
        let alloc = asm.alloc_root(CILRoot::StLoc(0, alloc));
        let cap = asm.alloc_node(Const::USize(ALLOC_CAP));
        let check = asm.alloc_root(CILRoot::Branch(Box::new((
            1,
            0,
            Some(super::cilroot::BranchCond::Gt(
                size,
                cap,
                super::cilroot::CmpKind::Unsigned,
            )),
        ))));
        let throw =
            crate::cil_root::CILRoot::throw(&format!("Alloc limit of {ALLOC_CAP} exceeded.",), asm);
        let throw = CILRoot::from_v1(&throw, asm);
        let throw = asm.alloc_root(throw);
        let zero = asm.alloc_node(Const::U8(0));
        let alloc_val = asm.alloc_node(CILNode::LdLoc(0));
        let zero = asm.alloc_root(CILRoot::InitBlk(Box::new((alloc_val, zero, size))));
        let ret = asm.alloc_root(CILRoot::Ret(alloc_val));
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(vec![check, alloc, zero, ret], 0, None),
                BasicBlock::new(vec![throw], 1, None),
            ],
            locals: vec![(None, asm.alloc_type(void_ptr))],
        }
    };
    patcher.insert(name, Box::new(generator));
}

pub fn uninit_val(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("uninit_val");
    let generator = move |mref: Interned<MethodRef>, asm: &mut Assembly| {
        let ret = asm.alloc_node(CILNode::LdLoc(0));
        let res = *asm[asm[mref].sig()].output();
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![asm.alloc_root(CILRoot::Ret(ret))],
                0,
                None,
            )],
            locals: vec![(None, asm.alloc_type(res))],
        }
    };

    patcher.insert(name, Box::new(generator));
}
pub fn ovf_check_tuple(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("ovf_check_tuple");
    let generator = move |mref: Interned<MethodRef>, asm: &mut Assembly| {
        let res = *asm[asm[mref].sig()].output();
        let addr = asm.alloc_node(CILNode::LdLocA(0));
        let arg0 = asm.alloc_node(CILNode::LdArg(0));
        let arg1 = asm.alloc_node(CILNode::LdArg(1));
        let ret = asm.alloc_node(CILNode::LdLoc(0));
        let item1 = asm.alloc_string("Item1");
        let item2 = asm.alloc_string("Item2");
        let tpe = asm[asm[mref].sig()].inputs()[0];
        let item1 = asm.alloc_field(FieldDesc::new(res.as_class_ref().unwrap(), item1, tpe));
        let item2 = asm.alloc_field(FieldDesc::new(
            res.as_class_ref().unwrap(),
            item2,
            Type::Bool,
        ));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![
                    asm.alloc_root(CILRoot::SetField(Box::new((item1, addr, arg0)))),
                    asm.alloc_root(CILRoot::SetField(Box::new((item2, addr, arg1)))),
                    asm.alloc_root(CILRoot::Ret(ret)),
                ],
                0,
                None,
            )],
            locals: vec![(None, asm.alloc_type(res))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn create_slice(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("create_slice");
    let generator = move |mref: Interned<MethodRef>, asm: &mut Assembly| {
        let res = *asm[asm[mref].sig()].output();
        let addr = asm.alloc_node(CILNode::LdLocA(0));
        let arg0 = asm.alloc_node(CILNode::LdArg(0));
        let arg1 = asm.alloc_node(CILNode::LdArg(1));
        let ret = asm.alloc_node(CILNode::LdLoc(0));
        let data_ptr = Interned::data_ptr(asm, res.as_class_ref().unwrap());
        let metadata = Interned::metadata(asm, res.as_class_ref().unwrap());
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![
                    asm.alloc_root(CILRoot::SetField(Box::new((data_ptr, addr, arg0)))),
                    asm.alloc_root(CILRoot::SetField(Box::new((metadata, addr, arg1)))),
                    asm.alloc_root(CILRoot::Ret(ret)),
                ],
                0,
                None,
            )],
            locals: vec![(None, asm.alloc_type(res))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_rust_realloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, use_libc: bool) {
    let name = asm.alloc_string("__rust_realloc");
    if use_libc {
        let generator = move |_, asm: &mut Assembly| {
            let ptr = asm.alloc_node(CILNode::LdArg(0));
            let void_idx = asm.alloc_type(Type::Void);
            let ptr = asm.alloc_node(CILNode::PtrCast(
                ptr,
                Box::new(super::cilnode::PtrCastRes::Ptr(void_idx)),
            ));
            let align = asm.alloc_node(CILNode::LdArg(2));

            let align = asm.alloc_node(CILNode::IntCast {
                input: align,
                target: Int::USize,
                extend: super::cilnode::ExtendKind::ZeroExtend,
            });
            let new_size = asm.alloc_node(CILNode::LdArg(3));
            let new_size = asm.alloc_node(CILNode::IntCast {
                input: new_size,
                target: Int::USize,
                extend: super::cilnode::ExtendKind::ZeroExtend,
            });
            let old_size = asm.alloc_node(CILNode::LdArg(1));
            let old_size = asm.alloc_node(CILNode::IntCast {
                input: old_size,
                target: Int::USize,
                extend: super::cilnode::ExtendKind::ZeroExtend,
            });
            let void_ptr = asm.nptr(Type::Void);
            let mm_malloc_sig = asm.sig([Type::Int(Int::USize), Type::Int(Int::USize)], void_ptr);
            // 1. call _mm_malloc
            let aligned_realloc = asm.alloc_string("_mm_malloc");
            let main_module = asm.main_module();
            let _mm_malloc = asm.alloc_methodref(MethodRef::new(
                *main_module,
                aligned_realloc,
                mm_malloc_sig,
                MethodKind::Static,
                [].into(),
            ));
            let _mm_malloc = asm.alloc_node(CILNode::call(_mm_malloc, [new_size, align]));
            let call_mm_malloc = asm.alloc_root(CILRoot::StLoc(0, _mm_malloc));
            // 2. memcpy the buffer.
            let buff = asm.alloc_node(CILNode::LdLoc(0));
            let copy = asm.alloc_root(CILRoot::CpBlk(Box::new((buff, ptr, old_size))));
            // 3. free the old buffer
            let aligned_free = asm.alloc_string("_mm_free");
            let mm_free_sig = asm.sig([void_ptr], Type::Void);
            let aligned_free = asm.alloc_methodref(MethodRef::new(
                *main_module,
                aligned_free,
                mm_free_sig,
                MethodKind::Static,
                [].into(),
            ));
            let call_aligned_free = asm.alloc_root(CILRoot::call(aligned_free, [ptr]));
            let ret = asm.alloc_root(CILRoot::Ret(buff));
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(
                    vec![call_mm_malloc, copy, call_aligned_free, ret],
                    0,
                    None,
                )],
                locals: vec![(None, asm.alloc_type(void_ptr))],
            }
        };
        patcher.insert(name, Box::new(generator));
    } else {
        let generator = move |_, asm: &mut Assembly| {
            let ptr = asm.alloc_node(CILNode::LdArg(0));
            let align = asm.alloc_node(CILNode::LdArg(2));
            let new_size = asm.alloc_node(CILNode::LdArg(3));
            let align = asm.alloc_node(CILNode::IntCast {
                input: align,
                target: Int::USize,
                extend: super::cilnode::ExtendKind::ZeroExtend,
            });
            let new_size = asm.alloc_node(CILNode::IntCast {
                input: new_size,
                target: Int::USize,
                extend: super::cilnode::ExtendKind::ZeroExtend,
            });
            let void_ptr = asm.nptr(Type::Void);
            let sig = asm.sig(
                [void_ptr, Type::Int(Int::USize), Type::Int(Int::USize)],
                void_ptr,
            );
            let aligned_realloc = asm.alloc_string("AlignedRealloc");
            let native_mem = ClassRef::native_mem(asm);
            let call_method = asm.alloc_methodref(MethodRef::new(
                native_mem,
                aligned_realloc,
                sig,
                MethodKind::Static,
                [].into(),
            ));
            let alloc = asm.alloc_node(CILNode::call(call_method, [ptr, new_size, align]));
            let ret = asm.alloc_root(CILRoot::Ret(alloc));
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![ret], 0, None)],
                locals: vec![],
            }
        };
        patcher.insert(name, Box::new(generator));
    }
}
fn insert_rust_dealloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, use_libc: bool) {
    let name = asm.alloc_string("__rust_dealloc");
    if use_libc {
        let generator = move |_, asm: &mut Assembly| {
            let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
            let void_idx = asm.alloc_type(Type::Void);
            let ldarg_0 = asm.alloc_node(CILNode::PtrCast(
                ldarg_0,
                Box::new(PtrCastRes::Ptr(void_idx)),
            ));
            let void_ptr = asm.nptr(Type::Void);
            let sig = asm.sig([void_ptr], Type::Void);
            let mm_free = asm.alloc_string("_mm_free");
            let main_module = asm.main_module();
            let call_method = asm.alloc_methodref(MethodRef::new(
                *main_module,
                mm_free,
                sig,
                MethodKind::Static,
                [].into(),
            ));
            let alloc = asm.alloc_node(CILNode::call(call_method, [ldarg_0]));
            let ret = asm.alloc_root(CILRoot::Ret(alloc));
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![ret], 0, None)],
                locals: vec![],
            }
        };
        patcher.insert(name, Box::new(generator));
    } else {
        let generator = move |_, asm: &mut Assembly| {
            let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
            let void_ptr = asm.nptr(Type::Void);
            let sig = asm.sig([void_ptr], Type::Void);
            let aligned_realloc = asm.alloc_string("AlignedFree");
            let native_mem = ClassRef::native_mem(asm);
            let call_method = asm.alloc_methodref(MethodRef::new(
                native_mem,
                aligned_realloc,
                sig,
                MethodKind::Static,
                [].into(),
            ));
            let alloc = asm.alloc_node(CILNode::call(call_method, [ldarg_0]));
            let ret = asm.alloc_root(CILRoot::Ret(alloc));
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![ret], 0, None)],
                locals: vec![],
            }
        };
        patcher.insert(name, Box::new(generator));
    }
}
pub fn insert_exeception_stub(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let rust_exception = asm.alloc_string("RustException");
    let data_pointer = asm.alloc_string("data_pointer");
    let extends = Some(ClassRef::exception(asm));
    asm.class_def(ClassDef::new(
        rust_exception,
        false,
        0,
        extends,
        vec![(Type::Int(Int::USize), data_pointer, Some(0))],
        vec![],
        Access::Public,
        Some(NonZeroU32::new(8).unwrap()),
        None,
        true,
    ));
    insert_catch_unwind_stub(asm, patcher);
}
pub fn insert_exception(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let rust_exception = asm.alloc_string("RustException");
    let data_pointer = asm.alloc_string("data_pointer");
    let this = asm.alloc_string("this");
    let extends = Some(ClassRef::exception(asm));
    let rust_exception = asm
        .class_def(ClassDef::new(
            rust_exception,
            false,
            0,
            extends,
            vec![(Type::Int(Int::USize), data_pointer, None)],
            vec![],
            Access::Public,
            None,
            None,
            true,
        ))
        .unwrap();
    let ctor = asm.alloc_string(".ctor");
    let sig = asm.sig(
        [Type::ClassRef(*rust_exception), Type::Int(Int::USize)],
        Type::Void,
    );
    let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
    let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
    let field = asm.alloc_field(FieldDesc::new(
        *rust_exception,
        data_pointer,
        Type::Int(Int::USize),
    ));
    let set_field = asm.alloc_root(CILRoot::SetField(Box::new((field, ldarg_0, ldarg_1))));
    let void_ret = asm.alloc_root(CILRoot::VoidRet);

    asm.new_method(MethodDef::new(
        Access::Public,
        rust_exception,
        ctor,
        sig,
        MethodKind::Constructor,
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![set_field, void_ret], 0, None)],
            locals: vec![],
        },
        vec![Some(this), Some(data_pointer)],
    ));
    insert_catch_unwind(asm, patcher);
}
pub fn insert_heap(asm: &mut Assembly, patcher: &mut MissingMethodPatcher, use_libc: bool) {
    insert_rust_alloc(asm, patcher);
    insert_rust_alloc_zeroed(asm, patcher);
    insert_rust_realloc(asm, patcher, use_libc);
    insert_rust_dealloc(asm, patcher, use_libc);
    insert_pause(asm, patcher);
}

fn insert_pause(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("llvm.x86.sse2.pause");
    let generator = move |_, asm: &mut Assembly| {
        let ret = asm.alloc_root(CILRoot::VoidRet);
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn rust_assert(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("rust_assert");
    let generator = move |_, asm: &mut Assembly| {
        let ret = asm.alloc_root(CILRoot::VoidRet);
        let assert = asm.alloc_node(CILNode::LdArg(0));
        let assert = asm.alloc_root(CILRoot::Branch(Box::new((
            1,
            0,
            Some(BranchCond::False(assert)),
        ))));
        let mref = Interned::abort(asm);
        let assert_failed = asm.alloc_root(CILRoot::call(mref, vec![]));
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(vec![assert, ret], 0, None),
                BasicBlock::new(vec![assert_failed, ret], 1, None),
            ],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}

fn insert_catch_unwind_stub(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("catch_unwind");
    let generator = move |_, asm: &mut Assembly| {
        let uint8_ptr = asm.nptr(Type::Int(Int::U8));
        let try_sig = asm.sig([uint8_ptr], Type::Void);

        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));

        // Call indirect try
        let calli_try = asm.alloc_root(CILRoot::CallI(Box::new((
            ldarg_0,
            try_sig,
            [ldarg_1].into(),
        ))));

        let const_0 = asm.alloc_node(Const::I32(0));
        let ret_0 = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![calli_try, ret_0], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_catch_unwind(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("catch_unwind");
    let generator = move |_, asm: &mut Assembly| {
        let uint8_ptr = asm.nptr(Type::Int(Int::U8));
        let try_sig = asm.sig([uint8_ptr], Type::Void);
        let catch_sig = asm.sig([uint8_ptr, uint8_ptr], Type::Void);
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldarg_2 = asm.alloc_node(CILNode::LdArg(2));
        let ldloc_1 = asm.alloc_node(CILNode::LdLoc(1));
        // Call indirect try
        let calli_try = asm.alloc_root(CILRoot::CallI(Box::new((
            ldarg_0,
            try_sig,
            [ldarg_1].into(),
        ))));
        let exit_try_success = asm.alloc_root(CILRoot::ExitSpecialRegion {
            target: 2,
            source: 0,
        });
        let exit_try_faliure = asm.alloc_root(CILRoot::ExitSpecialRegion {
            target: 3,
            source: 0,
        });
        let get_exception = asm.alloc_node(CILNode::GetException);
        let set_exception = asm.alloc_root(CILRoot::StLoc(1, get_exception));
        let exception = Type::ClassRef(ClassRef::exception(asm));
        let exception = asm.alloc_type(exception);
        let rust_exception = asm.alloc_string("RustException");
        let rust_exception =
            asm.alloc_class_ref(ClassRef::new(rust_exception, None, false, [].into()));
        let rust_exception_tpe = Type::ClassRef(rust_exception);
        let rust_exception_tpe = asm.alloc_type(rust_exception_tpe);
        // Check if exception is the right type, otherwise jump away
        let check_exception_tpe = asm.alloc_node(CILNode::IsInst(ldloc_1, rust_exception_tpe));
        let rethrow_if_wrong_exception = asm.alloc_root(CILRoot::Branch(Box::new((
            0,
            4,
            Some(BranchCond::False(check_exception_tpe)),
        ))));
        // Cast the excpetion
        let cast_exception = asm.alloc_node(CILNode::CheckedCast(ldloc_1, rust_exception_tpe));
        let data_pointer = asm.alloc_string("data_pointer");
        let ptr_field = asm.alloc_field(FieldDesc::new(
            rust_exception,
            data_pointer,
            Type::Int(Int::USize),
        ));
        let exception_ptr = asm.alloc_node(CILNode::LdField {
            addr: cast_exception,
            field: ptr_field,
        });
        let calli_catch = asm.alloc_root(CILRoot::CallI(Box::new((
            ldarg_2,
            catch_sig,
            [ldarg_1, exception_ptr].into(),
        ))));
        let const_0 = asm.alloc_node(Const::I32(0));
        let const_1 = asm.alloc_node(Const::I32(1));
        let ret_0 = asm.alloc_root(CILRoot::Ret(const_0));
        let ret_1 = asm.alloc_root(CILRoot::Ret(const_1));
        let rethrow = asm.alloc_root(CILRoot::ReThrow);
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(
                    vec![calli_try, exit_try_success],
                    0,
                    Some(vec![
                        BasicBlock::new(
                            vec![
                                set_exception,
                                rethrow_if_wrong_exception,
                                calli_catch,
                                exit_try_faliure,
                            ],
                            1,
                            None,
                        ),
                        BasicBlock::new(vec![rethrow], 4, None),
                    ]),
                ),
                BasicBlock::new(vec![ret_0], 2, None),
                BasicBlock::new(vec![ret_1], 3, None),
            ],
            locals: vec![
                (
                    Some(asm.alloc_string("data_ptr")),
                    asm.alloc_type(Type::Int(Int::USize)),
                ),
                (Some(asm.alloc_string("exception")), exception),
            ],
        }
    };
    patcher.insert(name, Box::new(generator));
}
const ALLOC_CAP: u64 = u32::MAX as u64;
pub(crate) const UNMANAGED_THREAD_START: &str = "UnmanagedThreadStart";
/// THIS BUILTIN MUST ALWAYS BE INLINED!
pub fn stack_addr(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("stack_addr");
    let generator = move |_, asm: &mut Assembly| {
        let addr = asm.alloc_node(CILNode::LdArgA(0));
        let ptr = asm.alloc_node(CILNode::RefToPtr(addr));
        let ret = asm.alloc_root(CILRoot::Ret(ptr));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn transmute(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("transmute");
    let generator = move |mref: Interned<MethodRef>, asm: &mut Assembly| {
        let target = *asm[asm[mref].sig()].output();
        let source = asm[asm[mref].sig()].inputs()[0];
        let source = asm.alloc_type(source);
        let target_idx = asm.alloc_type(target);
        let addr = asm.alloc_node(CILNode::LdArgA(0));
        if asm.alignof_type(source) >= asm.alignof_type(target_idx) {
            let ptr = asm.alloc_node(CILNode::RefToPtr(addr));
            let ptr = asm.alloc_node(CILNode::PtrCast(ptr, Box::new(PtrCastRes::Ptr(target_idx))));
            let valuetype = asm.alloc_node(CILNode::LdInd {
                addr: ptr,
                tpe: target_idx,
                volatile: false,
            });
            let ret = asm.alloc_root(CILRoot::Ret(valuetype));
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![ret], 0, None)],
                locals: vec![],
            }
        } else {
            let dst = asm.alloc_node(CILNode::LdLocA(0));
            let size = asm.alloc_node(CILNode::SizeOf(source));
            let load = asm.alloc_root(CILRoot::CpBlk(Box::new((dst, addr, size))));
            let ret = asm.alloc_node(CILNode::LdLoc(0));
            let ret = asm.alloc_root(CILRoot::Ret(ret));
            MethodImpl::MethodBody {
                blocks: vec![BasicBlock::new(vec![load, ret], 0, None)],
                locals: vec![(None, target_idx)],
            }
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn argc_argv_init(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("argc_argv_init");
    let generator = move |_, asm: &mut Assembly| {
        let main_module = asm.main_module();
        use crate::cil_node::CILNode;
        use crate::cil_root::CILRoot;
        use crate::method::Method;
        use crate::FnSig;
        let mut init_method = Method::new(
            crate::Access::Extern,
            crate::method::MethodType::Static,
            FnSig::new([], Type::Void),
            "argc_argv_init",
            vec![],
            vec![],
            vec![],
            asm,
        );

        // Allocate the variables necesarry for initializng args.
        let argc =
            u32::try_from(init_method.add_local(Type::Int(Int::I32), Some("argument_count"), asm))
                .unwrap();
        let uint8_ptr = asm.nptr(Type::Int(Int::I8));
        let argv =
            u32::try_from(init_method.add_local(asm.nptr(uint8_ptr), Some("argument_array"), asm))
                .unwrap();
        let managed_args = u32::try_from(init_method.add_local(
            Type::PlatformArray {
                elem: asm.alloc_type(Type::PlatformString),
                dims: NonZeroU8::new(1).unwrap(),
            },
            Some("managed_args"),
            asm,
        ))
        .unwrap();
        let arg_idx =
            u32::try_from(init_method.add_local(Type::Int(Int::I32), Some("arg_idx"), asm))
                .unwrap();
        // Get managed args
        let string = asm.alloc_type(Type::PlatformString);
        let mref = MethodRef::new(
            ClassRef::enviroment(asm),
            asm.alloc_string("GetCommandLineArgs"),
            asm.sig(
                [],
                Type::PlatformArray {
                    elem: string,
                    dims: NonZeroU8::new(1).unwrap(),
                },
            ),
            MethodKind::Static,
            vec![].into(),
        );
        let margs_init = CILRoot::STLoc {
            local: managed_args,
            tree: crate::call!(asm.alloc_methodref(mref), []),
        };
        // Calculate argc
        let argc_init = CILRoot::STLoc {
            local: argc,
            tree: crate::conv_i32!(CILNode::LDLen {
                arr: CILNode::LDLoc(managed_args).into()
            }),
        };
        let aligned_alloc = MethodRef::aligned_alloc(asm);
        // Alloc argv
        let tree = crate::call!(
            asm.alloc_methodref(aligned_alloc),
            [
                CILNode::Mul(
                    crate::conv_usize!(CILNode::LDLoc(argc)).into(),
                    crate::conv_usize!(CILNode::V2(asm.size_of(Int::USize).into_idx(asm))).into()
                ),
                crate::conv_usize!(CILNode::V2(asm.alloc_node(8_i32)))
            ]
        )
        .cast_ptr(asm.nptr(uint8_ptr));
        let argv_alloc = CILRoot::STLoc { local: argv, tree };
        // Create the block which allocates argv and calculates argc.
        let start_bb = init_method.new_bb();
        let mut blocks = init_method.blocks_mut();

        // Fill up the start block
        let start_block = &mut blocks[start_bb as usize];
        let status = StaticFieldDesc::new(
            *asm.main_module(),
            asm.alloc_string("argv_argc_init_status"),
            Type::Bool,
        );
        start_block.trees_mut().push(
            CILRoot::BTrue {
                target: start_bb + 3,
                sub_target: 0,
                cond: CILNode::LDStaticField(Box::new(status)),
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
                tree: CILNode::V2(asm.alloc_node(0_i32)),
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
                    + crate::conv_usize!(
                        CILNode::V2(asm.size_of(Int::USize).into_idx(asm))
                            * CILNode::LDLoc(arg_idx)
                    ),
                uarg,
                Box::new(Type::Int(Int::I8)),
            )
            .into(),
        );
        // Incr the arg_idx
        loop_block.trees_mut().push(
            CILRoot::STLoc {
                local: arg_idx,
                tree: CILNode::LDLoc(arg_idx) + CILNode::V2(asm.alloc_node(1_i32)),
            }
            .into(),
        );
        //If no args left, jump to exit
        loop_block.trees_mut().push(
            CILRoot::BTrue {
                target: loop_bb + 1,
                sub_target: 0,
                cond: crate::eq!(
                    crate::lt!(
                        CILNode::LDLoc(arg_idx),
                        crate::conv_i32!(CILNode::LDLen {
                            arr: CILNode::LDLoc(managed_args).into()
                        })
                    ),
                    CILNode::V2(asm.alloc_node(false))
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
        let argv_static = StaticFieldDesc::new(
            *asm.main_module(),
            asm.alloc_string("argv"),
            asm.nptr(uint8_ptr),
        );
        loop_end_block.trees_mut().push(
            CILRoot::SetStaticField {
                descr: Box::new(argv_static),
                value: CILNode::LDLoc(argv),
            }
            .into(),
        );
        let argc_static = StaticFieldDesc::new(
            *asm.main_module(),
            asm.alloc_string("argc"),
            Type::Int(Int::I32),
        );
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
                value: CILNode::V2(asm.alloc_node(true)),
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
        let def = MethodDef::from_v1(&init_method, asm, main_module);
        asm.new_method(def);
        asm.add_static(
            Type::Bool,
            "argv_argc_init_status",
            false,
            main_module,
            None,
            false,
        );
        let uint8_ptr_ptr = asm.nptr(uint8_ptr);
        asm.add_static(uint8_ptr_ptr, "argv", false, main_module, None, false);
        asm.add_static(Type::Int(Int::I32), "argc", false, main_module, None, false);
        MethodDef::from_v1(&init_method, asm, main_module)
            .implementation()
            .clone()
    };
    patcher.insert(name, Box::new(generator));
}
