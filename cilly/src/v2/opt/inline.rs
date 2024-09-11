use crate::v2::{
    cilnode::MethodKind, Assembly, CILIter, CILIterElem, CILNode, CILRoot, MethodDef, MethodImpl,
    MethodRefIdx, NodeIdx, RootIdx,
};

use super::OptFuel;
fn trivial_inline_block<'def, 'asm: 'def>(
    def: &'def MethodDef,
    asm: &'asm mut Assembly,
) -> Option<impl Iterator<Item = RootIdx> + 'def> {
    let method = def.resolved_implementation(asm);
    // Can only inline methods which have a concreate implementation.
    let MethodImpl::MethodBody { blocks, locals } = method else {
        return None;
    };
    // Can only trivialy inline methods with one and exactly one block
    let [ref block] = blocks[..] else {
        return None;
    };
    // Can only trivialy inline methods with no handlers
    let None = block.handler() else {
        return None;
    };
    // Can only trivialy inline methods with no locals
    if !locals.is_empty() {
        return None;
    }
    Some(block.meaningfull_roots(asm))
}
/// If the calle is nothing more than a handlerless block with one root, returning a value, then we can trivialy inline it. This is almost free, and should be always done when possible(excluding recursion).
fn trivial_inline_node(
    def: &MethodDef,
    call_args: &[NodeIdx],
    fuel: &mut OptFuel,
    asm: &mut Assembly,
) -> Option<CILNode> {
    let mut roots = trivial_inline_block(def, asm)?;
    // Can only trivaily inline methods with only one root
    let root = roots.next()?;
    let None = roots.next() else {
        return None;
    };
    drop(roots);

    // Can only trivialy inline methods with a single ret root
    let CILRoot::Ret(tree) = asm.get_root(root) else {
        return None;
    };
    let tree = asm.get_node(*tree).clone();

    // Can only trivialy inline methods if the address of the arguments is never taken - this is the only way to prove they are not overwritten.
    // TODO: consider allocating new locals, and taking their address instead. That ought to gurantee the original data is not clobbered.
    if CILIter::new(tree.clone(), asm)
        .any(|node| matches!(node, CILIterElem::Node(CILNode::LdArgA(_))))
    {
        return None;
    }
    // This is a valid trivial-inline candiate.
    if !fuel.consume(10) {
        return None;
    }
    let tree = tree.map(asm, &mut |node, asm| match node {
        CILNode::LdArg(arg) => asm.get_node(call_args[arg as usize]).clone(),
        CILNode::LdArgA(_)  => panic!("Attempted to access the address of an argument when inlining a method - this is not supported and should not happen."),
        CILNode::LdLocA(_) | CILNode::LdLoc(_) => panic!("Attempted to access a local when inlining a method with no locals."),
        _ => node,
    });
    Some(tree)
}
// _ZN4core3cmp5impls55_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$i64$GT$2lt17hd237a643fb9fb9d1E
pub fn trivial_inline_call(
    calle: MethodRefIdx,
    call_args: &[NodeIdx],
    fuel: &mut OptFuel,
    asm: &mut Assembly,
) -> CILNode {
    // Check if the calle is a method defined in this assembly.
    let Some(def) = asm.method_def_from_ref(calle).cloned() else {
        return CILNode::Call(Box::new((calle, call_args.into())));
    };
    // Check if the calle is a static method
    let MethodKind::Static = def.kind() else {
        return CILNode::Call(Box::new((calle, call_args.into())));
    };
    // TODO: only inline methods within the same class, to get around access.
    match trivial_inline_node(&def, call_args, fuel, asm) {
        Some(node) => node,
        None => CILNode::Call(Box::new((calle, call_args.into()))),
    }
}
fn trivial_inline_root(
    def: &MethodDef,
    call_args: &[NodeIdx],
    fuel: &mut OptFuel,
    asm: &mut Assembly,
) -> Option<CILRoot> {
    let mut roots = trivial_inline_block(def, asm)?;
    // Can only trivaily inline methods with only one root and a void ret
    let root = roots.next()?;
    let next_root = roots.next()?;
    drop(roots);
    let next_root = asm.get_root(next_root);
    let CILRoot::VoidRet = next_root else {
        return None;
    };
    let root = asm.get_root(root);
    // Can only trivialy inline methods if the address of the arguments is never taken - this is the only way to prove they are not overwritten.
    // TODO: consider allocating new locals, and taking their address instead. That ought to gurantee the original data is not clobbered.
    if CILIter::new(root.clone(), asm)
        .any(|node| matches!(node, CILIterElem::Node(CILNode::LdArgA(_))))
    {
        return None;
    }
    // This is a valid trivial-inline candiate.
    if !fuel.consume(10) {
        return None;
    }
    let root = root.clone().map(asm, &mut |root,_|{
        assert!(matches!(root,CILRoot::Nop | CILRoot::Break |CILRoot::Call(_) | CILRoot::CallI(_) | CILRoot::SetField(_) | CILRoot::SetStaticField { .. } | CILRoot::StInd(_) | CILRoot::Pop(_) | CILRoot::InitBlk(_) | CILRoot::CpBlk(_)), "Can't inline root {root:?}");
        root
    },&mut |node, asm| match node {
        CILNode::LdArg(arg) => asm.get_node(call_args[arg as usize]).clone(),
        CILNode::LdArgA(_)  => panic!("Attempted to access the address of an argument when inlining a method - this is not supported and should not happen."),
        CILNode::LdLocA(_) | CILNode::LdLoc(_) => panic!("Attempted to access a local when inlining a method with no locals."),
        _ => node,
    });
    Some(root)
}
pub fn inline_trivial_call_root(
    calle: MethodRefIdx,
    call_args: &[NodeIdx],
    fuel: &mut OptFuel,
    asm: &mut Assembly,
) -> CILRoot {
    // Check if the calle is a method defined in this assembly.
    let Some(def) = asm.method_def_from_ref(calle).cloned() else {
        return CILRoot::Call(Box::new((calle, call_args.into())));
    };
    // Check if the calle is a static method
    let MethodKind::Static = def.kind() else {
        return CILRoot::Call(Box::new((calle, call_args.into())));
    };
    match trivial_inline_root(&def, call_args, fuel, asm) {
        Some(node) => node,
        None => CILRoot::Call(Box::new((calle, call_args.into()))),
    }
}
