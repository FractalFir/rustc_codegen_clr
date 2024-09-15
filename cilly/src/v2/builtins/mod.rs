use super::{
    asm::MissingMethodPatcher, cilnode::MethodKind, cilroot::BranchCond, Access, Assembly,
    BasicBlock, CILNode, CILRoot, ClassDef, ClassRef, Const, FieldDesc, Int, MethodDef, MethodImpl,
    MethodRef, Type,
};

pub mod atomics;
pub mod casts;
pub mod math;
pub mod select;
pub mod thread;
pub use thread::*;
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
        let void_ptr = asm.nptr(Type::Void);
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![alloc_buff, buf1_to_tmp, buf2_to_buff1, tmp_to_buf2, ret],
                0,
                None,
            )],
            locals: vec![(Some(asm.alloc_string("tmp")), asm.alloc_type(void_ptr))],
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
        let alloc = asm.alloc_node(CILNode::Call(Box::new((
            call_method,
            Box::new([size, align]),
        ))));
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
        let throw =
            crate::cil_root::CILRoot::throw(&format!("Alloc limit of {ALLOC_CAP} exceeded.",), asm);
        let throw = CILRoot::from_v1(&throw, asm);
        let throw = asm.alloc_root(throw);
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
        let alloc = asm.alloc_node(CILNode::Call(Box::new((
            call_method,
            Box::new([size, align]),
        ))));
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
            locals: vec![(None, asm.alloc_type(Type::Int(Int::USize)))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_rust_realloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_realloc");
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
        let alloc = asm.alloc_node(CILNode::Call(Box::new((
            call_method,
            Box::new([ptr, new_size, align]),
        ))));
        let ret = asm.alloc_root(CILRoot::Ret(alloc));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_rust_dealloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_dealloc");
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
        let alloc = asm.alloc_node(CILNode::Call(Box::new((call_method, Box::new([ldarg_0])))));
        let ret = asm.alloc_root(CILRoot::Ret(alloc));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}

pub fn insert_heap(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    insert_rust_alloc(asm, patcher);
    insert_rust_alloc_zeroed(asm, patcher);
    insert_rust_realloc(asm, patcher);
    insert_rust_dealloc(asm, patcher);
    insert_catch_unwind(asm, patcher);
    insert_pause(asm, patcher);
    let rust_exception = asm.alloc_string("RustException");
    let data_pointer = asm.alloc_string("data_pointer");
    let this = asm.alloc_string("this");
    let extends = Some(ClassRef::exception(asm));
    let rust_exception = asm.class_def(ClassDef::new(
        rust_exception,
        false,
        0,
        extends,
        vec![(Type::Int(Int::USize), data_pointer, None)],
        vec![],
        Access::Public,
        None,
    ));
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
