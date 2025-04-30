use crate::{
    asm::MissingMethodPatcher, tpe::simd::SIMDElem, Assembly, BasicBlock, BinOp, CILNode, CILRoot,
    Const, Int, Interned, MethodImpl, MethodRef, Type,
};
macro_rules! binop {
    ($op_name:ident,$op_dotnet:literal) => {
        pub fn $op_name(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
            let name = asm.alloc_string(stringify!($op_name));
            let generator = move |mref: $crate::v2::Interned<$crate::v2::MethodRef>,
                                  asm: &mut Assembly| {
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
                let res = asm.alloc_node(CILNode::call(equals, [lhs, rhs]));

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
binop!(simd_and, "BitwiseAnd");
binop!(simd_sub, "Subtract");
binop!(simd_mul, "Multiply");
binop!(simd_div, "Divides");
fn simd_binop(
    op: impl Fn(&mut Assembly, Interned<CILNode>, Interned<CILNode>, SIMDElem, Type) -> Interned<CILNode>
        + 'static,
    name: &str,
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
) {
    let name = asm.alloc_string(name);
    let generator = move |mref: Interned<MethodRef>, asm: &mut Assembly| {
        // Extrac types from signature
        let sig = &asm[asm[mref].sig()];
        let res = *sig.output();
        let res_elem = res.as_simdvector().unwrap().elem().into();
        let vec = sig.inputs()[0].as_simdvector().unwrap().clone();
        let elem = vec.elem();
        let res_ptr = asm.alloc_node(CILNode::LdLocA(0));
        let tpe: Type = elem.into();
        let tpe_idx = asm.alloc_type(tpe);
        // Get args
        let lhs = asm.alloc_node(CILNode::LdArgA(0));
        let rhs = asm.alloc_node(CILNode::LdArgA(1));
        let lhs = asm.cast_ptr(lhs, tpe);
        let rhs = asm.cast_ptr(rhs, tpe);
        // Iter trough all elements
        let mut roots = vec![];
        for idx in 0..vec.count() {
            let lhs = asm.offset(lhs, Const::USize(idx as u64), tpe);
            let rhs = asm.offset(rhs, Const::USize(idx as u64), tpe);
            let lhs = asm.alloc_node(CILNode::LdInd {
                addr: lhs,
                tpe: tpe_idx,
                volatile: false,
            });
            let rhs = asm.alloc_node(CILNode::LdInd {
                addr: rhs,
                tpe: tpe_idx,
                volatile: false,
            });
            let res_ptr = asm.cast_ptr(res_ptr, res_elem);
            let res_ptr = asm.offset(res_ptr, Const::USize(idx as u64), res_elem);
            let res = op(asm, lhs, rhs, elem, res_elem);
            roots.push(asm.alloc_root(CILRoot::StInd(Box::new((res_ptr, res, res_elem, false)))));
        }
        let ret = asm.alloc_node(CILNode::LdLoc(0));
        roots.push(asm.alloc_root(CILRoot::Ret(ret)));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(roots, 0, None)],
            locals: vec![(None, asm.alloc_type(res))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn fallback_simd(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    simd_binop(
        |asm, lhs, rhs, elem, res_tpe| {
            let res = asm.biop(lhs, rhs, BinOp::Lt);
            asm.int_cast(
                res,
                res_tpe.as_int().unwrap(),
                crate::cilnode::ExtendKind::ZeroExtend,
            )
        },
        "simd_lt",
        asm,
        patcher,
    );
    simd_binop(
        |asm, lhs, rhs, elem, res_tpe| {
            let res = asm.biop(lhs, rhs, BinOp::Eq);
            asm.int_cast(
                res,
                res_tpe.as_int().unwrap(),
                crate::cilnode::ExtendKind::ZeroExtend,
            )
        },
        "simd_eq",
        asm,
        patcher,
    );
    simd_binop(
        |asm, lhs, rhs, _, _| asm.biop(lhs, rhs, BinOp::Add),
        "simd_add",
        asm,
        patcher,
    );
    simd_binop(
        |asm, lhs, rhs, _, _| asm.biop(lhs, rhs, BinOp::Sub),
        "simd_sub",
        asm,
        patcher,
    );
}
