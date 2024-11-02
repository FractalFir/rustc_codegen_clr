use super::{
    super::{
        asm::MissingMethodPatcher, cilnode::MethodKind, Access, Assembly, BasicBlock, CILNode,
        CILRoot, ClassDef, ClassRef, Const, FieldDesc, Int, MethodDef, MethodImpl, Type,
    },
    UNMANAGED_THREAD_START,
};
use crate::{
    v2::{
        cilnode::{ExtendKind, PtrCastRes},
        cilroot::BranchCond,
        tpe::GenericKind,
        BinOp, StaticFieldDesc,
    },
    ClassRefIdx,
};
fn handle_to_obj(asm: &mut Assembly, _: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("handle_to_obj");
    let main_module = asm.main_module();
    let mimpl = {
        // Turn the nint into a handle
        let gc_handle = ClassRef::gc_handle(asm);
        let handle = asm.alloc_node(CILNode::LdArg(0));
        let from_int_ptr = asm.alloc_string("FromIntPtr");
        let from_int_ptr = asm.class_ref(gc_handle).clone().static_mref(
            &[Type::Int(Int::ISize)],
            Type::ClassRef(gc_handle),
            from_int_ptr,
            asm,
        );
        let handle = asm.alloc_node(CILNode::Call(Box::new((from_int_ptr, [handle].into()))));
        let get_handle = asm.alloc_root(CILRoot::StLoc(0, handle));
        // Get the target of the handle
        let target = asm.alloc_string("get_Target");
        let target =
            asm.class_ref(gc_handle)
                .clone()
                .instance(&[], Type::PlatformObject, target, asm);
        let handle = asm.alloc_node(CILNode::LdLocA(0));
        let target = asm.alloc_node(CILNode::Call(Box::new((target, [handle].into()))));
        let ret = asm.alloc_root(CILRoot::Ret(target));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![get_handle, ret], 0, None)],
            locals: vec![(None, asm.alloc_type(Type::ClassRef(gc_handle)))],
        }
    };
    let sig = asm.sig([Type::Int(Int::ISize)], Type::PlatformObject);
    asm.new_method(MethodDef::new(
        Access::Public,
        main_module,
        name,
        sig,
        MethodKind::Static,
        mimpl,
        vec![None],
    ));
}
fn insert_pthread_attr_setstacksize(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_attr_setstacksize");
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));

        let three = asm.alloc_node(Const::I32(2));
        let usize_tpe = asm.alloc_type(Int::USize);
        let usize_size = asm.alloc_node(CILNode::SizeOf(usize_tpe));
        let offset = asm.alloc_node(CILNode::BinOp(usize_size, three, BinOp::Mul));

        let addr = asm.biop(
            CILNode::IntCast {
                input: offset,
                target: Int::USize,
                extend: ExtendKind::ZeroExtend,
            },
            CILNode::LdArg(0),
            BinOp::Add,
        );
        let addr = asm.alloc_node(addr);
        let init = asm.alloc_root(CILRoot::StInd(Box::new((
            addr,
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
        let size = asm.alloc_node(CILNode::BinOp(usize_size, three, BinOp::Mul));
        let size = asm.alloc_node(CILNode::IntCast {
            input: size,
            target: Int::USize,
            extend: ExtendKind::ZeroExtend,
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

fn insert_pthread_self(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_self");
    let generator = move |_, asm: &mut Assembly| {
        // This is incorrect. pthread_self ought to return the thread id.
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_pthread_setname_np(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_setname_np");
    let generator = move |_, asm: &mut Assembly| {
        // This is incorrect. pthread_setname_np should set the thread name.
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}

fn insert_pthread_attr_destroy(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_attr_destroy");
    let generator = move |_, asm: &mut Assembly| {
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
// TODO: impl detach
fn insert_pthread_detach(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let fn_name = asm.alloc_string("pthread_detach");
    let generator = move |_, asm: &mut Assembly| {
        // Return 0 to signal success.
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret_0 = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret_0], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(fn_name, Box::new(generator));
}
fn insert_pthread_join(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let fn_name = asm.alloc_string("pthread_join");
    let generator = move |_, asm: &mut Assembly| {
        // Get the thread from handle
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let handle_to_obj = asm.alloc_string("handle_to_obj");
        let main_module = asm.main_module();
        let handle_to_obj = asm.class_ref(*main_module).clone().static_mref(
            &[Type::Int(Int::ISize)],
            Type::PlatformObject,
            handle_to_obj,
            asm,
        );
        let thread = ClassRef::thread(asm);
        let ldarg_0 = asm.alloc_node(CILNode::IntCast {
            input: ldarg_0,
            target: Int::ISize,
            extend: ExtendKind::ZeroExtend,
        });
        let obj = asm.alloc_node(CILNode::Call(Box::new((handle_to_obj, [ldarg_0].into()))));
        let thread_idx = asm.alloc_type(Type::ClassRef(thread));
        let obj = asm.alloc_node(CILNode::CheckedCast(obj, thread_idx));
        let get_thread = asm.alloc_root(CILRoot::StLoc(0, obj));
        // Join the thread
        let joined_thread = asm.alloc_node(CILNode::LdLoc(0));
        let result_ptr = asm.alloc_node(CILNode::LdArg(1));
        let join = asm.alloc_string("Join");
        let join = asm
            .class_ref(thread)
            .clone()
            .virtual_mref(&[], Type::Void, join, asm);
        let join = asm.alloc_root(CILRoot::Call(Box::new((join, Box::new([joined_thread])))));
        // Check if result_ptr == null. If so, jump to ret.
        let check_res_nonnull = asm.alloc_root(CILRoot::Branch(Box::new((
            1,
            0,
            Some(BranchCond::False(result_ptr)),
        ))));
        // Get the thread result from the thread result dictionary, and set (*result_ptr) = thread_res.
        let thread_id = asm.alloc_string("get_ManagedThreadId");
        let thread_id =
            asm.class_ref(thread)
                .clone()
                .virtual_mref(&[], Type::Int(Int::I32), thread_id, asm);
        let thread_id =
            asm.alloc_node(CILNode::Call(Box::new((thread_id, [joined_thread].into()))));
        let thread_results = asm.alloc_string("thread_results");
        let dict = ClassRef::concurent_dictionary(Type::Int(Int::I32), Type::Int(Int::ISize), asm);
        let thread_results = asm.alloc_sfld(StaticFieldDesc::new(
            *main_module,
            thread_results,
            Type::ClassRef(dict),
        ));
        let thread_results = asm.alloc_node(CILNode::LdStaticField(thread_results));
        let get_item = asm.alloc_string("get_Item");
        let get_dict = asm.class_ref(dict).clone().virtual_mref(
            &[Type::PlatformGeneric(0, GenericKind::MethodGeneric)],
            Type::PlatformGeneric(1, GenericKind::MethodGeneric),
            get_item,
            asm,
        );
        let thread_result = asm.alloc_node(CILNode::Call(Box::new((
            get_dict,
            Box::new([thread_results, thread_id]),
        ))));
        let set_result = asm.alloc_root(CILRoot::StInd(Box::new((
            result_ptr,
            thread_result,
            Type::Int(Int::ISize),
            false,
        ))));
        // Return 0 to signal success.
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret_0 = asm.alloc_root(CILRoot::Ret(const_0));
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(
                    vec![get_thread, join, check_res_nonnull, set_result, ret_0],
                    0,
                    None,
                ),
                BasicBlock::new(vec![ret_0], 1, None),
            ],
            locals: vec![(None, asm.alloc_type(Type::ClassRef(thread)))],
        }
    };
    patcher.insert(fn_name, Box::new(generator));
}
fn insert_pthread_create(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let fn_name = asm.alloc_string("pthread_create");
    let generator = move |_, asm: &mut Assembly| {
        // Common
        let void_ptr = asm.nptr(Type::Void);
        let start_sig = asm.sig([void_ptr], void_ptr);
        let start_fn_ptr_type = asm.alloc_type(Type::FnPtr(start_sig));
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        // Thread info is ignored
        let _ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldarg_3 = asm.alloc_node(CILNode::LdArg(3));
        let void = asm.alloc_type(Type::Void);
        let ldarg_3 = asm.alloc_node(CILNode::PtrCast(ldarg_3, Box::new(PtrCastRes::Ptr(void))));

        let arg2_addr = asm.alloc_node(CILNode::LdArgA(2));
        let arg2_addr = asm.alloc_node(CILNode::RefToPtr(arg2_addr));

        let arg2_addr = asm.alloc_node(CILNode::PtrCast(
            arg2_addr,
            Box::new(PtrCastRes::Ptr(start_fn_ptr_type)),
        ));
        let transmute_arg_2 = asm.alloc_node(CILNode::LdInd {
            addr: arg2_addr,
            tpe: start_fn_ptr_type,
            volatile: false,
        });
        let transmute_arg_2 = asm.alloc_root(CILRoot::StLoc(2, transmute_arg_2));
        // Arg2 needs to be transmuted, and the local 2 holds the transmuted value of arg2.
        let ldarg_2 = asm.alloc_node(CILNode::LdLoc(2));

        let unmanaged_thread_start = asm.alloc_string(UNMANAGED_THREAD_START);
        let unmanaged_thread_start = ClassRef::new(unmanaged_thread_start, None, false, [].into());
        // UnmanagedThreadStart constructor
        let unmanaged_thread_start_ctor =
            unmanaged_thread_start.ctor(&[Type::FnPtr(start_sig), void_ptr], asm);
        let start = asm.alloc_string("Start");
        let unmanaged_thread_start_start =
            unmanaged_thread_start.virtual_mref(&[], Type::Void, start, asm);
        // Create UnmanagedThreadStart
        let unmanaged_thread_start_obj = asm.alloc_node(CILNode::Call(Box::new((
            unmanaged_thread_start_ctor,
            [ldarg_2, ldarg_3].into(),
        ))));
        // Get pointer to UnmanagedThreadStart.Start

        let unmanaged_thread_start_fn =
            asm.alloc_node(CILNode::LdFtn(unmanaged_thread_start_start));
        let unmanaged_thread_start_fn = asm.alloc_node(CILNode::PtrCast(
            unmanaged_thread_start_fn,
            Box::new(PtrCastRes::ISize),
        ));
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
        let thread_start_type = asm.alloc_type(Type::ClassRef(thread_start));
        let thread_start_obj =
            asm.alloc_node(CILNode::CheckedCast(thread_start_obj, thread_start_type));
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
                (None, start_fn_ptr_type),
            ],
        }
    };
    patcher.insert(fn_name, Box::new(generator));
}
pub fn instert_threading(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    insert_pthread_attr_init(asm, patcher);
    insert_pthread_attr_setstacksize(asm, patcher);
    insert_pthread_create(asm, patcher);
    insert_pthread_join(asm, patcher);
    insert_pthread_detach(asm, patcher);
    insert_pthread_self(asm, patcher);
    insert_pthread_attr_destroy(asm, patcher);
    insert_pthread_setname_np(asm, patcher);
    insert_pthread_key_delete(asm, patcher);
    let main_mod = asm.main_module();
    asm.add_static(Type::Int(PTHREAD_KEY_T), "last_val", false, main_mod);
    let thread_key_dict = thread_key_dict(asm);
    asm.add_static(
        Type::ClassRef(thread_key_dict),
        "pthread_keys",
        true,
        main_mod,
    );
    let pthread_keys = asm.alloc_string("pthread_keys");
    let pthread_keys_static = asm.alloc_sfld(StaticFieldDesc::new(
        *main_mod,
        pthread_keys,
        Type::ClassRef(thread_key_dict),
    ));
    let thread_key_dict_ctor = asm[thread_key_dict].clone().ctor(&[], asm);
    let ctor = asm.alloc_node(CILNode::Call(Box::new((thread_key_dict_ctor, [].into()))));
    let init_dict = asm.alloc_root(CILRoot::SetStaticField {
        field: pthread_keys_static,
        val: ctor,
    });
    let last_val = asm.alloc_string("last_val");
    let last_val_static = StaticFieldDesc::new(*main_mod, last_val, Type::Int(PTHREAD_KEY_T));
    let last_val_static = asm.alloc_sfld(last_val_static);
    let val = asm.alloc_node(Const::I32(1));
    let init_val = asm.alloc_root(CILRoot::SetStaticField {
        field: last_val_static,
        val,
    });
    asm.add_tcctor(&[init_val, init_dict]);
    insert_pthread_key_create(asm, patcher);
    insert_pthread_setspecific(asm, patcher);
    handle_to_obj(asm, patcher);

    let uts = asm.alloc_string(UNMANAGED_THREAD_START);
    let object = ClassRef::object(asm);
    let void_ptr = asm.nptr(Type::Void);
    let start_fn_sig = asm.sig([void_ptr], void_ptr);
    let start_fn_tpe = Type::FnPtr(start_fn_sig);
    let start_fn = asm.alloc_string("start_fn");
    let data = asm.alloc_string("data");
    let unmanaged_start = asm.class_def(ClassDef::new(
        uts,
        false,
        0,
        Some(object),
        vec![(start_fn_tpe, start_fn, None), (void_ptr, data, None)],
        vec![],
        // TODO: fix the bug which causes this to be cleaned up by dead code elimination when access is set to Public.
        Access::Extern,
        None,
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
    // Call tcctor
    let tctor = asm.tcctor();
    let call_tcctor = asm.alloc_root(CILRoot::Call(Box::new((*tctor, [].into()))));
    // Call the thread main function.
    let this = asm.alloc_node(CILNode::LdArg(0));
    let start_fn_node = asm.alloc_node(CILNode::LdField {
        addr: this,
        field: start_field,
    });
    let data_node = asm.alloc_node(CILNode::LdField {
        addr: this,
        field: data_field,
    });
    let call = asm.alloc_node(CILNode::CallI(Box::new((
        start_fn_node,
        start_fn_sig,
        [data_node].into(),
    ))));
    let call = asm.alloc_root(CILRoot::StLoc(0, call));
    // Get the ID of this thread
    let thread = ClassRef::thread(asm);
    let current_thread = asm.alloc_string("get_CurrentThread");
    let current_thread =
        asm.class_ref(thread)
            .clone()
            .static_mref(&[], Type::ClassRef(thread), current_thread, asm);
    let current_thread = asm.alloc_node(CILNode::Call(Box::new((current_thread, [].into()))));
    let thread_id = asm.alloc_string("get_ManagedThreadId");
    let thread_id =
        asm.class_ref(thread)
            .clone()
            .virtual_mref(&[], Type::Int(Int::I32), thread_id, asm);
    let thread_id = asm.alloc_node(CILNode::Call(Box::new((
        thread_id,
        [current_thread].into(),
    ))));
    // Thread result static
    let main_module = asm.main_module();
    let thread_results = asm.alloc_string("thread_results");
    let dict = ClassRef::concurent_dictionary(Type::Int(Int::I32), Type::Int(Int::ISize), asm);
    asm.class_mut(main_module).static_fields_mut().push((
        Type::ClassRef(dict),
        thread_results,
        false,
    ));
    let thread_results = asm.alloc_sfld(StaticFieldDesc::new(
        *main_module,
        thread_results,
        Type::ClassRef(dict),
    ));
    let dict_ctor = asm.class_ref(dict).clone().ctor(&[], asm);
    let init_thread_results = asm.alloc_node(CILNode::Call(Box::new((dict_ctor, [].into()))));
    let init_thread_results = asm.alloc_root(CILRoot::SetStaticField {
        field: thread_results,
        val: init_thread_results,
    });
    asm.add_cctor(&[init_thread_results]);
    // Get the thread result static
    let thread_results: crate::v2::NodeIdx = asm.alloc_node(CILNode::LdStaticField(thread_results));
    let set_item = asm.alloc_string("set_Item");
    let set_dict = asm.class_ref(dict).clone().virtual_mref(
        &[
            Type::PlatformGeneric(0, GenericKind::MethodGeneric),
            Type::PlatformGeneric(1, GenericKind::MethodGeneric),
        ],
        Type::Void,
        set_item,
        asm,
    );
    let ldloc_0 = asm.alloc_node(CILNode::LdLoc(0));
    let set_result = asm.alloc_root(CILRoot::Call(Box::new((
        set_dict,
        [thread_results, thread_id, ldloc_0].into(),
    ))));
    let void_ptr_idx = asm.alloc_type(void_ptr);
    asm.new_method(MethodDef::new(
        Access::Public,
        unmanaged_start,
        start,
        start_sig,
        MethodKind::Virtual,
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![call_tcctor, call, set_result, ret],
                0,
                None,
            )],
            locals: vec![(None, void_ptr_idx)],
        },
        vec![None],
    ));

    assert_eq!(
        *unmanaged_start,
        asm.alloc_class_ref(ClassRef::new(uts, None, false, [].into()))
    );
}
const PTHREAD_KEY_T: Int = Int::I32;
fn thread_key_dict(asm: &mut Assembly) -> ClassRefIdx {
    ClassRef::dictionary(Type::Int(PTHREAD_KEY_T), Type::Int(Int::ISize), asm)
}
fn insert_pthread_setspecific(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_setspecific");
    let generator = move |_, asm: &mut Assembly| {
        let main_mod = *asm.main_module();
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));

        // Insert a new key into the dict.
        let thread_key_dict = thread_key_dict(asm);
        let pthread_keys = asm.alloc_string("pthread_keys");
        let pthread_keys_static = asm.alloc_sfld(StaticFieldDesc::new(
            main_mod,
            pthread_keys,
            Type::ClassRef(thread_key_dict),
        ));
        let pthread_keys = asm.alloc_node(CILNode::LdStaticField(pthread_keys_static));

        let set_item = asm.alloc_string("set_Item");
        let dict_add = asm[thread_key_dict].clone().virtual_mref(
            &[
                Type::PlatformGeneric(0, GenericKind::TypeGeneric),
                Type::PlatformGeneric(1, GenericKind::TypeGeneric),
            ],
            Type::Void,
            set_item,
            asm,
        );
        let arg_0 = asm.alloc_node(CILNode::LdArg(0));
        let arg_1 = asm.alloc_node(CILNode::LdArg(1));
        let insert_key = asm.alloc_root(CILRoot::Call(Box::new((
            dict_add,
            [pthread_keys, arg_0, arg_1].into(),
        ))));
        // Set the key_t to this key.

        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![insert_key, ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_pthread_key_create(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_key_create");
    let generator = move |_, asm: &mut Assembly| {
        let main_mod = *asm.main_module();
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        let last_val = asm.alloc_string("last_val");
        let last_val_static = StaticFieldDesc::new(main_mod, last_val, Type::Int(PTHREAD_KEY_T));
        let last_val_static = asm.alloc_sfld(last_val_static);
        // Save the last counter value
        let last_value = asm.alloc_node(CILNode::LdStaticField(last_val_static));
        let save_last_val = asm.alloc_root(CILRoot::StLoc(0, last_value));
        // Increment the last counter value
        let one = asm.alloc_node(Const::I32(0));
        let incremented = asm.alloc_node(CILNode::BinOp(last_value, one, BinOp::Add));
        let increment_val = asm.alloc_root(CILRoot::StLoc(0, incremented));
        // Insert a new key into the dict.
        let thread_key_dict = thread_key_dict(asm);
        let pthread_keys = asm.alloc_string("pthread_keys");
        let pthread_keys_static = asm.alloc_sfld(StaticFieldDesc::new(
            main_mod,
            pthread_keys,
            Type::ClassRef(thread_key_dict),
        ));
        let pthread_keys = asm.alloc_node(CILNode::LdStaticField(pthread_keys_static));
        let zero_isize = asm.alloc_node(Const::ISize(0));
        let loc_0 = asm.alloc_node(CILNode::LdLoc(0));
        let add_name = asm.alloc_string("set_Item");
        let dict_add = asm[thread_key_dict].clone().virtual_mref(
            &[
                Type::PlatformGeneric(0, GenericKind::TypeGeneric),
                Type::PlatformGeneric(1, GenericKind::TypeGeneric),
            ],
            Type::Void,
            add_name,
            asm,
        );
        let insert_key = asm.alloc_root(CILRoot::Call(Box::new((
            dict_add,
            [pthread_keys, zero_isize, loc_0].into(),
        ))));
        // Set the key_t to this key.
        let arg_0 = asm.alloc_node(CILNode::LdArg(0));
        let key_t = asm.alloc_type(Type::Int(PTHREAD_KEY_T));
        let arg_0 = asm.alloc_node(CILNode::PtrCast(arg_0, Box::new(PtrCastRes::Ptr(key_t))));
        let set_key = asm.alloc_root(CILRoot::StInd(Box::new((
            arg_0,
            loc_0,
            Type::Int(PTHREAD_KEY_T),
            false,
        ))));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(
                vec![save_last_val, increment_val, insert_key, set_key, ret],
                0,
                None,
            )],
            locals: vec![(None, key_t)],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_pthread_key_delete(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("pthread_key_delete");
    let generator = move |_, asm: &mut Assembly| {
        let main_mod = *asm.main_module();
        let const_0 = asm.alloc_node(Const::I32(0));
        let ret = asm.alloc_root(CILRoot::Ret(const_0));
        let thread_key_dict = thread_key_dict(asm);
        let pthread_keys = asm.alloc_string("pthread_keys");
        let pthread_keys_static = asm.alloc_sfld(StaticFieldDesc::new(
            main_mod,
            pthread_keys,
            Type::ClassRef(thread_key_dict),
        ));
        let pthread_keys = asm.alloc_node(CILNode::LdStaticField(pthread_keys_static));
        let arg_0 = asm.alloc_node(CILNode::LdArg(0));
        let add_name = asm.alloc_string("Remove");
        let dict_rem = asm[thread_key_dict].clone().virtual_mref(
            &[Type::PlatformGeneric(0, GenericKind::TypeGeneric)],
            Type::Bool,
            add_name,
            asm,
        );
        let remove_key = asm.alloc_node(CILNode::Call(Box::new((
            dict_rem,
            [pthread_keys, arg_0].into(),
        ))));
        let remove_key = asm.alloc_root(CILRoot::Pop(remove_key));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![remove_key, ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
