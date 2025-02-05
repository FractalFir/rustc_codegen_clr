use crate::{
    tpe::simd::SIMDVector, v2::asm::MissingMethodPatcher, Assembly, BasicBlock, CILNode, CILRoot,
    MethodImpl, MethodRefIdx, NodeIdx, Type,
};
mod eq;
use eq::*;
mod binop;
use binop::*;
fn dotnet_vec_cast(
    src: NodeIdx,
    src_type: SIMDVector,
    target_type: SIMDVector,
    asm: &mut Assembly,
) -> NodeIdx {
    if src_type == target_type {
        return src;
    }
    eprintln!("Can't cast {src_type:?} -> {target_type:?}");
    let _ = asm;
    src
}

fn simd_ones_compliment(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name: crate::StringIdx = asm.alloc_string("simd_ones_compliment");
    let generator = move |mref: MethodRefIdx, asm: &mut Assembly| {
        let sig = asm[asm[mref].sig()].clone();

        let Some(vec_type) = sig.inputs()[0].as_simdvector() else {
            todo!(
                "Can't calc the ones compliment of {vec_type:?}",
                vec_type = sig.inputs()[0]
            )
        };
        let elem: Type = vec_type.elem().into();
        let extension_class = vec_type.extension_class(asm);
        let extension_class = asm[extension_class].clone();
        let ones_compliment = asm.alloc_string("OnesComplement");
        // Generic vec
        let generic_class = vec_type.class(asm);
        let mut generic_class = asm[generic_class].clone();
        generic_class.set_generics(vec![Type::PlatformGeneric(
            0,
            crate::tpe::GenericKind::CallGeneric,
        )]);
        let generic_class = asm.alloc_class_ref(generic_class);
        let ones_compliment = extension_class.static_mref_generic(
            &[Type::ClassRef(generic_class)],
            Type::ClassRef(generic_class),
            ones_compliment,
            asm,
            [elem].into(),
        );
        let val = asm.alloc_node(CILNode::LdArg(0));
        let res = asm.alloc_node(CILNode::call(ones_compliment, [val]));
        let ret = asm.alloc_root(CILRoot::Ret(res));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}

fn simd_neg(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name: crate::StringIdx = asm.alloc_string("simd_neg");
    let generator = move |mref: MethodRefIdx, asm: &mut Assembly| {
        let sig = asm[asm[mref].sig()].clone();

        let Some(vec_type) = sig.inputs()[0].as_simdvector() else {
            todo!(
                "Can't calc the ones compliment of {vec_type:?}",
                vec_type = sig.inputs()[0]
            )
        };
        let elem: Type = vec_type.elem().into();
        let extension_class = vec_type.extension_class(asm);
        let extension_class = asm[extension_class].clone();
        let ones_compliment = asm.alloc_string("Negate");
        // Generic vec
        let generic_class = vec_type.class(asm);
        let mut generic_class = asm[generic_class].clone();
        generic_class.set_generics(vec![Type::PlatformGeneric(
            0,
            crate::tpe::GenericKind::CallGeneric,
        )]);
        let generic_class = asm.alloc_class_ref(generic_class);
        let ones_compliment = extension_class.static_mref_generic(
            &[Type::ClassRef(generic_class)],
            Type::ClassRef(generic_class),
            ones_compliment,
            asm,
            [elem].into(),
        );
        let val = asm.alloc_node(CILNode::LdArg(0));
        let res = asm.alloc_node(CILNode::call(ones_compliment, [val]));
        let ret = asm.alloc_root(CILRoot::Ret(res));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn simd_abs(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name: crate::StringIdx = asm.alloc_string("simd_abs");
    let generator = move |mref: MethodRefIdx, asm: &mut Assembly| {
        let sig = asm[asm[mref].sig()].clone();

        let Some(vec_type) = sig.inputs()[0].as_simdvector() else {
            todo!(
                "Can't calc simd_abs of {vec_type:?}",
                vec_type = sig.inputs()[0]
            )
        };
        let elem: Type = vec_type.elem().into();
        let extension_class = vec_type.extension_class(asm);
        let extension_class = asm[extension_class].clone();
        let ones_compliment = asm.alloc_string("Abs");
        // Generic vec
        let generic_class = vec_type.class(asm);
        let mut generic_class = asm[generic_class].clone();
        generic_class.set_generics(vec![Type::PlatformGeneric(
            0,
            crate::tpe::GenericKind::CallGeneric,
        )]);
        let generic_class = asm.alloc_class_ref(generic_class);
        let ones_compliment = extension_class.static_mref_generic(
            &[Type::ClassRef(generic_class)],
            Type::ClassRef(generic_class),
            ones_compliment,
            asm,
            [elem].into(),
        );
        let val = asm.alloc_node(CILNode::LdArg(0));
        let res = asm.alloc_node(CILNode::call(ones_compliment, [val]));
        let ret = asm.alloc_root(CILRoot::Ret(res));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
#[allow(dead_code)]
fn simd_shuffle(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name: crate::StringIdx = asm.alloc_string("simd_shuffle");
    let generator = move |_mref: MethodRefIdx, _asm: &mut Assembly| {
        todo!("simd_shuffle not supported yet!");
        /*MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![], 0, None)],
            locals: vec![],
        }*/
    };
    patcher.insert(name, Box::new(generator));
}
fn simd_vec_from_val(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name: crate::StringIdx = asm.alloc_string("simd_vec_from_val");
    let generator = move |mref: MethodRefIdx, asm: &mut Assembly| {
        let sig = asm[asm[mref].sig()].clone();
        let Some(vec_type) = sig.output().as_simdvector() else {
            todo!(
                "Can't simd_vec_from_val  {vec_type:?}",
                vec_type = sig.output()
            )
        };
        let extension_class = vec_type.extension_class(asm);
        let extension_class = asm[extension_class].clone();
        let create = asm.alloc_string("Create");
        let create = extension_class.static_mref(&[sig.inputs()[0]], *sig.output(), create, asm);
        let val = asm.alloc_node(CILNode::LdArg(0));
        let res = asm.alloc_node(CILNode::call(create, [val]));
        let ret = asm.alloc_root(CILRoot::Ret(res));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}
fn simd_allset(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    let name: crate::StringIdx = asm.alloc_string("simd_allset");
    let generator = move |mref: MethodRefIdx, asm: &mut Assembly| {
        let sig = asm[asm[mref].sig()].clone();
        let Some(vec_type) = sig.output().as_simdvector() else {
            todo!("Can't simd_allset {vec_type:?}", vec_type = sig.output())
        };
        let class = vec_type.class(asm);
        let class = asm[class].clone();
        let generic_class = vec_type.class(asm);
        let mut generic_class = asm[generic_class].clone();
        generic_class.set_generics(vec![Type::PlatformGeneric(
            0,
            crate::tpe::GenericKind::TypeGeneric,
        )]);
        let generic_class = asm.alloc_class_ref(generic_class);
        let create = asm.alloc_string("get_AllBitsSet");
        let create = class.static_mref(&[], Type::ClassRef(generic_class), create, asm);
        let res = asm.alloc_node(CILNode::call(create, []));
        let ret = asm.alloc_root(CILRoot::Ret(res));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![ret], 0, None)],
            locals: vec![],
        }
    };
    patcher.insert(name, Box::new(generator));
}

pub fn simd(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    simd_eq(asm, patcher);
    simd_ones_compliment(asm, patcher);
    simd_neg(asm, patcher);
    simd_abs(asm, patcher);
    simd_vec_from_val(asm, patcher);
    simd_or(asm, patcher);
    simd_add(asm, patcher);
    simd_and(asm, patcher);
    simd_sub(asm, patcher);
    simd_allset(asm, patcher);
    simd_eq_all(asm, patcher);
    simd_eq_any(asm, patcher);
    simd_mul(asm, patcher);
    simd_div(asm, patcher);
}
