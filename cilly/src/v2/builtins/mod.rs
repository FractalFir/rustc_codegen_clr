use super::{
    asm::MissingMethodPatcher, cilnode::MethodKind, Access, Assembly, BasicBlock, CILNode, CILRoot,
    ClassDef, ClassRef, Const, FieldDesc, Int, MethodDef, MethodImpl, MethodRef, Type,
};

pub mod atomics;
pub mod casts;
pub mod math;
pub mod select;
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
        let size = asm.alloc_node(CILNode::LdArg(1));
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
fn insert_rust_realloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_realloc");
    let generator = move |_, asm: &mut Assembly| {
        let ptr = asm.alloc_node(CILNode::LdArg(0));
        let align = asm.alloc_node(CILNode::LdArg(2));
        let new_size = asm.alloc_node(CILNode::LdArg(3));
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
fn insert_pthread_attr_setstacksize(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_attr_setstacksize");
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let const_0_u8 = asm.alloc_node(Const::U8(0));
        let three = asm.alloc_node(Const::I32(2));
        let usize_tpe = asm.alloc_type(Int::USize);
        let usize_size = asm.alloc_node(CILNode::SizeOf(usize_tpe));
        let offset = asm.alloc_node(CILNode::BinOp(usize_size, three, super::BinOp::Mul));
        let offset = asm.alloc_node(CILNode::IntCast {
            input: offset,
            target: Int::USize,
            extend: super::cilnode::ExtendKind::ZeroExtend,
        });
        let init = asm.alloc_root(CILRoot::StInd(Box::new((
            ldarg_0,
            ldarg_1,
            Type::Int(Int::USize),
            false,
        ))));
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![init, ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_pthread_attr_init(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_attr_init");
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let const_0_u8 = asm.alloc_node(Const::U8(0));
        let three = asm.alloc_node(Const::I32(3));
        let usize_tpe = asm.alloc_type(Int::USize);
        let usize_size = asm.alloc_node(CILNode::SizeOf(usize_tpe));
        let size = asm.alloc_node(CILNode::BinOp(usize_size, three, super::BinOp::Mul));
        let size = asm.alloc_node(CILNode::IntCast {
            input: size,
            target: Int::USize,
            extend: super::cilnode::ExtendKind::ZeroExtend,
        });
        let init = asm.alloc_root(CILRoot::InitBlk(Box::new((ldarg_0, const_0_u8, size))));
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![init, ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_pthread_create(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let fn_name = asm.alloc_string("pthread_create");
    let generator = move |_, asm: &mut Assembly| {
        // Common
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldarg_3 = asm.alloc_node(CILNode::LdArg(3));
        let isize_tpe = asm.alloc_type(Type::Int(Int::ISize));
        let arg2_addr = asm.alloc_node(CILNode::LdArgA(2));
        let arg2_addr = asm.alloc_node(CILNode::RefToPtr(arg2_addr));
        let transmuted_arg_2 = asm.alloc_node(CILNode::LdInd {
            addr: arg2_addr,
            tpe: isize_tpe,
            volitale: false,
        });
        let transmute_arg_2 = asm.alloc_root(CILRoot::StLoc(2, transmuted_arg_2));
        // Arg2 needs to be transmuted, and the local 2 holds the transmuted value of arg2.
        let ldarg_2 = asm.alloc_node(CILNode::LdLoc(2));
        // Common types
        let void_ptr = asm.nptr(Type::Void);
        let start_sig = asm.sig([void_ptr], void_ptr);
        let unmanaged_thread_start = asm.alloc_string(UNMANAGED_THREAD_START);
        let unmanaged_thread_start = ClassRef::new(unmanaged_thread_start, None, false, [].into());
        // UnmanagedThreadStart constructor
        let unmanaged_thread_start_ctor =
            unmanaged_thread_start.ctor(&[Type::FnPtr(start_sig), void_ptr], asm);
        let start = asm.alloc_string("Start");
        let unmanaged_thread_start_start =
            unmanaged_thread_start.instance(&[], Type::Void, start, asm);
        // Create UnmanagedThreadStart
        let unmanaged_thread_start_obj = asm.alloc_node(CILNode::Call(Box::new((
            unmanaged_thread_start_ctor,
            [ldarg_2, ldarg_3].into(),
        ))));
        // Get pointer to UnmanagedThreadStart.Start

        let unmanaged_thread_start_fn =
            asm.alloc_node(CILNode::LdFtn(unmanaged_thread_start_start));
        // Create a ThreadStart object
        let thread_start = ClassRef::thread_start(asm);
        let thread_start_ctor = asm
            .class_ref(thread_start)
            .clone()
            .ctor(&[Type::PlatformObject, Type::Int(Int::ISize)], asm);
        let thread_start_obj = asm.alloc_node(CILNode::Call(Box::new((
            thread_start_ctor,
            [unmanaged_thread_start_obj, unmanaged_thread_start_fn].into(),
        ))));
        let create_thread_start = asm.alloc_root(CILRoot::StLoc(1, thread_start_obj));
        let thread_start_obj = asm.alloc_node(CILNode::LdLoc(1));
        let thread_type = ClassRef::thread(asm);
        let thread_ctor = asm
            .class_ref(thread_type)
            .clone()
            .ctor(&[Type::ClassRef(thread_start)], asm);
        let thread_obj = asm.alloc_node(CILNode::Call(Box::new((
            thread_ctor,
            [thread_start_obj].into(),
        ))));
        let create_thread = asm.alloc_root(CILRoot::StLoc(0, thread_obj));
        let thread_start =
            asm.class_ref(thread_type)
                .clone()
                .virtual_mref(&[], Type::Void, start, asm);
        let thread = asm.alloc_node(CILNode::LdLoc(0));
        let start_thread = asm.alloc_root(CILRoot::Call(Box::new((thread_start, [thread].into()))));
        let thread_handle = CILNode::LdLoc(0).ref_to_handle(asm);
        let thread_handle = asm.alloc_node(thread_handle);
        let set_thread_handle = asm.alloc_root(CILRoot::StInd(Box::new((
            ldarg_0,
            thread_handle,
            Type::Int(Int::ISize),
            false,
        ))));
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        let thread_type = asm.alloc_type(Type::ClassRef(thread_type));
        let unmanaged_thread_start = asm.alloc_class_ref(unmanaged_thread_start);
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![
                    transmute_arg_2,
                    create_thread_start,
                    create_thread,
                    start_thread,
                    set_thread_handle,
                    ret,
                ],
                0,
                None,
            )],
            locals: vec![
                (None, thread_type),
                (None, asm.alloc_type(Type::ClassRef(unmanaged_thread_start))),
                (None, asm.alloc_type(Type::Int(Int::ISize))),
            ],
        }
    };
    patcher.insert(fn_name, Box::new(generator));
}
pub fn instert_threading(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    insert_pthread_attr_init(asm, patcher);
    insert_pthread_attr_setstacksize(asm, patcher);
    insert_pthread_create(asm, patcher);
    let uts = asm.alloc_string(UNMANAGED_THREAD_START);
    let object = ClassRef::object(asm);
    let void_ptr = asm.nptr(Type::Void);
    let start_fn_sig = asm.sig([void_ptr], void_ptr);
    let start_fn_tpe = (Type::FnPtr(start_fn_sig));
    let start_fn = asm.alloc_string("start_fn");
    let data = asm.alloc_string("data");
    let unmanaged_start = asm.class_def(ClassDef::new(
        uts,
        false,
        0,
        Some(object),
        vec![(start_fn_tpe, start_fn, None), (void_ptr, data, None)],
        vec![],
        // TODO: fix the bug which causes this to be leaned up by dead code elimination when access is set to Public.
        Access::Extern,
        None,
    ));

    let ctor = asm.alloc_string(".ctor");
    let this = asm.alloc_node(CILNode::LdArg(0));
    let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
    let ldarg_2 = asm.alloc_node(CILNode::LdArg(2));

    let start_field = asm.alloc_field(FieldDesc::new(*unmanaged_start, start_fn, start_fn_tpe));
    let data_field = asm.alloc_field(FieldDesc::new(*unmanaged_start, data, void_ptr));
    let set_start = asm.alloc_root(CILRoot::SetField(Box::new((start_field, this, ldarg_1))));
    let set_data = asm.alloc_root(CILRoot::SetField(Box::new((data_field, this, ldarg_2))));
    let ret = asm.alloc_root(CILRoot::VoidRet);
    let ctor_sig = asm.sig(
        [
            Type::ClassRef(*unmanaged_start),
            Type::FnPtr(start_fn_sig),
            void_ptr,
        ],
        Type::Void,
    );
    asm.new_method(MethodDef::new(
        Access::Public,
        unmanaged_start,
        ctor,
        ctor_sig,
        MethodKind::Constructor,
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![set_start, set_data, ret], 0, None)],
            locals: vec![],
        },
        vec![None, Some(start_fn), Some(data)],
    ));
    let start_sig = asm.sig([Type::ClassRef(*unmanaged_start)], Type::Void);
    let start = asm.alloc_string("Start");
    asm.new_method(MethodDef::new(
        Access::Public,
        unmanaged_start,
        start,
        start_sig,
        MethodKind::Virtual,
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        },
        vec![None],
    ));

    assert_eq!(
        *unmanaged_start,
        asm.alloc_class_ref(ClassRef::new(uts, None, false, [].into()))
    )
}
pub fn insert_heap(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    insert_rust_alloc(asm, patcher);
    insert_rust_realloc(asm, patcher);
    insert_rust_dealloc(asm, patcher);
    insert_catch_unwind(asm, patcher);
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
fn insert_catch_unwind(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("catch_unwind");
    let generator = move |_, asm: &mut Assembly| {
        let uint8_ptr = asm.nptr(Type::Int(Int::U8));
        let try_sig = asm.sig([uint8_ptr], Type::Void);
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldarg_2 = asm.alloc_node(CILNode::LdArg(2));
        let ldloc_0 = asm.alloc_node(CILNode::LdLoc(0));
        // Call indirect try
        let calli_try = asm.alloc_root(CILRoot::CallI(Box::new((
            ldarg_0,
            try_sig,
            [ldarg_1].into(),
        ))));
        let exit_try_success = asm.alloc_root(CILRoot::ExitSpecialRegion {
            target: 3,
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
        let cast_exception = asm.alloc_node(CILNode::CheckedCast(ldloc_0, rust_exception_tpe));
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
            try_sig,
            [ldarg_1, exception_ptr].into(),
        ))));
        let const_0 = asm.alloc_node(Const::I32(0));
        let const_1 = asm.alloc_node(Const::I32(1));
        let ret_0 = asm.alloc_root(CILRoot::Ret(const_0));
        let ret_1 = asm.alloc_root(CILRoot::Ret(const_1));
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(
                    vec![calli_try, exit_try_success],
                    0,
                    Some(vec![BasicBlock::new(
                        vec![set_exception, calli_catch, exit_try_faliure],
                        1,
                        None,
                    )]),
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
const UNMANAGED_THREAD_START: &str = "UnmanagedThreadStart";
