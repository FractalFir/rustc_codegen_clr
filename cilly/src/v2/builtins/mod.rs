use super::{
    asm::MissingMethodPatcher, cilnode::MethodKind, Assembly, BasicBlock, CILNode, CILRoot,
    ClassRef, Int, MethodImpl, MethodRef, Type,
};

pub mod atomics;
pub mod casts;
pub mod math;
pub mod select;
pub fn insert_rust_alloc(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
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
pub fn insert_heap(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    insert_rust_alloc(asm, patcher);
}
