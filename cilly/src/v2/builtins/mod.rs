use super::{
    asm::MissingMethodPatcher, cilnode::MethodKind, Access, Assembly, BasicBlock, CILNode, CILRoot,
    ClassDef, ClassRef, Const, FieldDesc, Int, MethodDef, MethodImpl, MethodRef, Type,
};

pub mod atomics;
pub mod casts;
pub mod math;
pub mod select;
fn insert_rust_alloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_alloc");
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
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
            Box::new([ldarg_0, ldarg_1]),
        ))));
        let ret = asm.alloc_root(CILRoot::Ret(alloc));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn insert_rust_realloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name = asm.alloc_string("__rust_realloc");
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldarg_2 = asm.alloc_node(CILNode::LdArg(2));
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
            Box::new([ldarg_0, ldarg_1, ldarg_2]),
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
/*
BasicBlock::new(
    vec![
        CILRoot::CallI {
            sig: Box::new(FnSig::new(&[ptr!(Type::U8)], Type::Void)),
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
                        DotnetTypeRef::new::<&str, _>(None, "RustException")
                            .with_valuetype(false)
                    ))),
                }
                .into(),
                CILRoot::STLoc {
                    local: 0,
                    tree: ld_field!(
                        CILNode::CheckedCast(Box::new((
                            CILNode::LDLoc(1),
                            DotnetTypeRef::new::<&str, _>(None, "RustException")
                                .with_valuetype(false)
                        ))),
                        FieldDescriptor::new(
                            DotnetTypeRef::new::<&str, _>(None, "RustException")
                                .with_valuetype(false),
                            Type::USize,
                            "data_pointer".into()
                        )
                    ),
                }
                .into(),
                CILRoot::CallI {
                    sig: Box::new(FnSig::new(
                        &[ptr!(Type::U8), ptr!(Type::U8)],
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
                        DotnetTypeRef::console(),
                        "WriteLine".into(),
                        FnSig::new(
                            &[Type::DotnetType(Box::new(DotnetTypeRef::object_type()))],
                            Type::Void
                        ),
                        true
                    )),
                    args: Box::new([CILNode::LDLoc(1)])
                }
                .into(),
                CILRoot::CallI {
                    sig: Box::new(FnSig::new(
                        &[ptr!(Type::U8), ptr!(Type::U8)],
                        Type::Void
                    )),
                    fn_ptr: Box::new(CILNode::LDArg(2)),
                    args: Box::new([
                        CILNode::LDArg(1),
                        call!(
                            CallSite::builtin(
                                "exception_to_native".into(),
                                FnSig::new(
                                    vec![Type::DotnetType(Box::new(
                                        DotnetTypeRef::object_type()
                                    )),],
                                    ptr!(Type::U8)
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
), */
