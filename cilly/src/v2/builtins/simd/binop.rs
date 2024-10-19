use crate::{
    v2::asm::MissingMethodPatcher, Assembly, BasicBlock, CILNode, CILRoot, MethodImpl,
    MethodRefIdx, Type,
};
macro_rules! binop {
    ($op_name:ident,$op_dotnet:literal) => {
        pub fn $op_name(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
            let name = asm.alloc_string(stringify!($op_name));
            let generator = move |mref: MethodRefIdx, asm: &mut Assembly| {
                let sig = asm[asm[mref].sig()].clone();

                let Some(comparands) = sig.inputs()[0].as_simdvector() else {
                    let name = stringify!($op_name);
                    todo!("Can't {name} {comparands:?} ", comparands = sig.inputs()[0])
                };
                let elem: Type = comparands.elem().into();

                let extension_class = comparands.extension_class(asm);
                let extension_class = asm[extension_class].clone();
                let equals = asm.alloc_string($op_dotnet);
                // Generic vec
                let generic_class = comparands.class(asm);
                let mut generic_class = asm[generic_class].clone();
                generic_class.set_generics(vec![Type::PlatformGeneric(
                    0,
                    crate::tpe::GenericKind::CallGeneric,
                )]);
                let generic_class = asm.alloc_class_ref(generic_class);
                let equals = extension_class.static_mref_generic(
                    &[Type::ClassRef(generic_class), Type::ClassRef(generic_class)],
                    Type::ClassRef(generic_class),
                    equals,
                    asm,
                    [elem].into(),
                );
                let lhs = asm.alloc_node(CILNode::LdArg(0));
                let rhs = asm.alloc_node(CILNode::LdArg(1));
                let res = asm.alloc_node(CILNode::Call(Box::new((equals, [lhs, rhs].into()))));

                let ret = asm.alloc_root(CILRoot::Ret(res));
                MethodImpl::MethodBody {
                    blocks: vec![BasicBlock::new(vec![ret], 0, None)],
                    locals: vec![],
                }
            };
            patcher.insert(name, Box::new(generator));
        }
    };
}
binop!(simd_or, "BitwiseOr");
binop!(simd_add, "Add");
binop!(simd_sub, "Subtract");
binop!(simd_mul, "Multiply");
binop!(simd_div, "Divides");
