use inline::inline_trivial_call_root;

#[cfg(test)]
use super::Float;

use super::{
    cilroot::BranchCond, method::LocalDef, typecheck::display_typecheck_err, BasicBlock, BinOp,
    CILIter, CILIterElem, CILNode, CILRoot, Const, Int, MethodImpl, NodeIdx, RootIdx, Type,
};
use crate::v2::{Assembly, MethodDef};
pub use opt_fuel::OptFuel;
pub use side_effect::*;
mod inline;
mod opt_fuel;
mod opt_node;
mod side_effect;
mod simplify_handlers;
mod test;
pub fn opt_if_fuel<T>(new: T, original: T, fuel: &mut OptFuel) -> T {
    if fuel.consume(1) {
        new
    } else {
        original
    }
}
impl CILNode {
    // The complexity of this function is unavoidable.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn propagate_locals(
        &self,
        asm: &mut Assembly,
        idx: u32,
        tpe: Type,
        new_node: NodeIdx,
        fuel: &mut OptFuel,
    ) -> Self {
        match self {
            CILNode::BinOp(rhs, lhs, biop) => {
                let rhs = asm.get_node(*rhs).clone();
                let lhs = asm.get_node(*lhs).clone();
                let rhs = rhs.propagate_locals(asm, idx, tpe, new_node, fuel);
                let lhs = lhs.propagate_locals(asm, idx, tpe, new_node, fuel);
                CILNode::BinOp(asm.alloc_node(rhs), asm.alloc_node(lhs), *biop)
            }
            CILNode::UnOp(input, unop) => {
                let input = asm.get_node(*input).clone();
                let input = input.propagate_locals(asm, idx, tpe, new_node, fuel);
                let input = asm.alloc_node(input);
                CILNode::UnOp(input, unop.clone())
            }
            CILNode::LdLoc(loc) => {
                if *loc == idx {
                    if !fuel.consume(1) {
                        return self.clone();
                    }
                    match tpe {
                        Type::Float(_)
                        | Type::Bool
                        | Type::FnPtr(_)
                        | Type::Ptr(_)
                        | Type::ClassRef(_)
                        | Type::Int(
                            Int::I128
                            | Int::U128
                            | Int::USize
                            | Int::ISize
                            | Int::I64
                            | Int::U64
                            | Int::U32
                            | Int::I32,
                        )
                        | Type::Ref(_) => asm.get_node(new_node).clone(),
                        Type::Int(int @ (Int::I8 | Int::U8 | Int::I16 | Int::U16)) => {
                            CILNode::IntCast {
                                input: new_node,
                                target: int,
                                // Does not matter, since this does nothing for ints < 32 bits, which this arm handles.
                                extend: if int.is_signed() {
                                    super::cilnode::ExtendKind::SignExtend
                                } else {
                                    super::cilnode::ExtendKind::ZeroExtend
                                },
                            }
                        }
                        _ => CILNode::LdLoc(*loc),
                    }
                } else {
                    CILNode::LdLoc(*loc)
                }
            }
            CILNode::LdLocA(loc) => CILNode::LdLocA(*loc), // This takes an address, so we can't propagate it
            CILNode::LdArg(arg) => CILNode::LdArg(*arg),
            CILNode::LdArgA(arg) => CILNode::LdArgA(*arg),
            CILNode::Call(_) => todo!(),
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = asm.get_node(*input).clone();
                let input = input.propagate_locals(asm, idx, tpe, new_node, fuel);
                let input = asm.alloc_node(input);
                CILNode::IntCast {
                    input,
                    target: *target,
                    extend: *extend,
                }
            }
            CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => {
                let input = asm.get_node(*input).clone();
                let input = input.propagate_locals(asm, idx, tpe, new_node, fuel);
                let input = asm.alloc_node(input);
                CILNode::FloatCast {
                    input,
                    target: *target,
                    is_signed: *is_signed,
                }
            }
            CILNode::RefToPtr(ptr) => {
                let ptr = asm.get_node(*ptr).clone();
                let ptr = ptr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let ptr = asm.alloc_node(ptr);
                CILNode::RefToPtr(ptr)
            }
            CILNode::PtrCast(ptr, cast_res) => {
                let ptr = asm.get_node(*ptr).clone();
                let ptr = ptr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let ptr = asm.alloc_node(ptr);
                CILNode::PtrCast(ptr, cast_res.clone())
            }
            CILNode::LdFieldAdress { addr, field } => {
                let addr = asm.get_node(*addr).clone();
                let addr = addr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let addr = asm.alloc_node(addr);
                CILNode::LdFieldAdress {
                    addr,
                    field: *field,
                }
            }
            CILNode::LdField { addr, field } => {
                let addr = asm.get_node(*addr).clone();
                let addr = addr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let addr = asm.alloc_node(addr);
                CILNode::LdField {
                    addr,
                    field: *field,
                }
            }
            CILNode::LdInd {
                addr,
                tpe: tpe2,
                volitale,
            } => {
                let addr = asm.get_node(*addr).clone();
                let addr = addr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let addr = asm.alloc_node(addr);
                CILNode::LdInd {
                    addr,
                    tpe: *tpe2,
                    volitale: *volitale,
                }
            }
            CILNode::IsInst(_, _) => todo!(),
            CILNode::CheckedCast(_, _) => todo!(),
            CILNode::CallI(_) => todo!(),
            CILNode::GetException
            | CILNode::SizeOf(_)
            | CILNode::LocAlloc { .. }
            | CILNode::LdStaticField(_)
            | CILNode::LdStaticFieldAdress(_)
            | CILNode::LdFtn(_)
            | CILNode::LdTypeToken(_)
            | CILNode::LocAllocAlgined { .. }
            | CILNode::Const(_) => self.clone(),
            CILNode::LdLen(arr) => {
                let arr = asm.get_node(*arr).clone();
                let arr = arr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let arr = asm.alloc_node(arr);
                CILNode::LdLen(arr)
            }

            CILNode::LdElelemRef { array, index } => {
                let array = asm.get_node(*array).clone();
                let array = array.propagate_locals(asm, idx, tpe, new_node, fuel);
                let array = asm.alloc_node(array);
                CILNode::LdElelemRef {
                    array,
                    index: *index,
                }
            }
            CILNode::UnboxAny {
                object,
                tpe: unboxtpe,
            } => {
                let object = asm.get_node(*object).clone();
                let object = object.propagate_locals(asm, idx, tpe, new_node, fuel);
                let object = asm.alloc_node(object);
                CILNode::UnboxAny {
                    object,
                    tpe: *unboxtpe,
                }
            }
        }
    }
}
impl BasicBlock {
    /// Removes duplicate debug info, to reduce the size of the final assembly.
    #[allow(clippy::similar_names)]
    pub fn remove_duplicate_sfi(&mut self, asm: &mut Assembly) {
        let mut prev_ls = u32::MAX;
        let mut prev_ll: u16 = u16::MAX;
        // This variable could become used if I change the duplicate sfi removal rules.
        #[allow(unused_variables)]
        let mut prev_cs = u16::MAX;
        #[allow(unused_variables)]
        // This variable could become used if I change the duplicate sfi removal rules.
        let mut prev_cl = u16::MAX;
        let mut prev_file = asm.alloc_string("InvalidDebugInfoString");
        let nop = asm.alloc_root(CILRoot::Nop);
        for root in self.roots_mut().iter_mut() {
            if let CILRoot::SourceFileInfo {
                line_start,
                line_len,
                col_start,
                col_len,
                file,
            } = asm.get_root(*root)
            {
                // Check if this sfi is a duplciate. If so, replace it with a NOP; We ignore columns cos they are not all that important in most cases.
                if *file == prev_file
                    //&& *col_len == prev_cl
                    //&& *col_start == prev_cs
                    && *line_len == prev_ll
                    && *line_start == prev_ls
                {
                    *root = nop;
                }
                // Set the prev sfi to curr sfi
                prev_file = *file;
                #[allow(unused_assignments)]
                // This could become enabled if I ever want to change the SFI deduplcation rules.
                {
                    prev_cl = *col_len;
                    prev_cs = *col_start;
                }
                prev_ll = *line_len;
                prev_ls = *line_start;
            }
        }
        self.roots_mut().retain(|root| *root != nop);
    }
    /// Optimizes the [`BasicBlock`] by attempting to propagate the value of local assigments. This, in turn, allows for certain assigements to be removed in the future, and reduces the local varaible count.
    /// Having less locals allows the JIT to optimize this function more, and, in the case of valuetypes, shrinks down the stack usage.
    pub fn local_opt(
        &mut self,
        asm: &mut Assembly,
        locals: &[LocalDef],
        cache: &mut SideEffectInfoCache,
        fuel: &mut OptFuel,
    ) {
        let root_iter: Vec<_> = self
            .roots_mut()
            .iter_mut()
            .filter(|root| !matches!(asm.get_root(**root), CILRoot::SourceFileInfo { .. }))
            .collect();
        let mut root_iter = root_iter.into_iter();
        let Some(mut prev_root_id) = root_iter.next() else {
            return;
        };
        for root in root_iter {
            let prev_root = asm.get_root(*prev_root_id).clone();
            'm: {
                #[allow(clippy::single_match)]
                match prev_root {
                    CILRoot::StLoc(loc, tree) => {
                        // 1 st. check if the previous node is a candiate for propagation.

                        if cache.has_side_effects(tree, asm) {
                            break 'm;
                        }
                        // Check that it does not depend on itself
                        if CILIter::new(asm.get_node(tree).clone(), asm)
                            .any(|node| node == CILIterElem::Node(CILNode::LdLoc(loc)))
                        {
                            break 'm;
                        }
                        let mut tmp_root = asm.get_root(*root).clone();

                        for node in tmp_root.nodes_mut() {
                            // If this node has side effects, do not bother attempting propagation.
                            // Break out of the loop to prevent further propagation.
                            if cache.has_side_effects(*node, asm) {
                                break;
                            }
                            // Next, attempt propagating the node.
                            let new_node: CILNode = asm.get_node(*node).clone();
                            let new_node = new_node.propagate_locals(
                                asm,
                                loc,
                                *asm.get_type(locals[loc as usize].1),
                                tree,
                                fuel,
                            );
                            *node = asm.alloc_node(new_node);
                        }
                        *root = asm.alloc_root(tmp_root);
                    }
                    _ => (),
                }
            }
            prev_root_id = root;
        }
    }
}
impl MethodImpl {
    pub fn remove_duplicate_sfi(&mut self, asm: &mut Assembly) {
        // Optimization only suported for methods with locals
        let MethodImpl::MethodBody { blocks, .. } = self else {
            return;
        };
        blocks
            .iter_mut()
            .for_each(|block| block.remove_duplicate_sfi(asm));
    }
    /// Propagates writes to local variables.
    pub fn propagate_locals(
        &mut self,
        asm: &mut Assembly,
        cache: &mut SideEffectInfoCache,
        fuel: &mut OptFuel,
    ) {
        // Optimization only suported for methods with locals
        let MethodImpl::MethodBody { blocks, locals } = self else {
            return;
        };

        blocks
            .iter_mut()
            .for_each(|block| block.local_opt(asm, locals, cache, fuel));
    }
    /// Replaces writes to locals, which are never read, with pops.
    pub fn remove_dead_writes(
        &mut self,
        asm: &mut Assembly,
        cache: &mut SideEffectInfoCache,
        fuel: &mut OptFuel,
    ) {
        // Optimization only suported for methods with locals
        let MethodImpl::MethodBody { blocks, locals } = self else {
            return;
        };
        // Check if each local is ever read or its address is taken
        let mut local_reads = vec![false; locals.len()];
        let mut local_address_of = vec![false; locals.len()];
        /*
                let blocks_copy = blocks.clone();
                for block in blocks.iter_mut() {
                    let Some(root) = block.roots().last() else {
                        continue;
                    };
                    let CILRoot::Branch(info) = asm.get_root(*root) else {
                        continue;
                    };
                    let (target, sub_target, None) = info.as_ref() else {
                        continue;
                    };
                    let id = blockid_from_jump(*target, *sub_target);
                    if id == block.block_id() {
                        continue;
                    }
                    let Some(target_block) = block_with_id(&blocks_copy, id) else {
                        continue;
                    };
                    // If this block targets anyting, then it should not be appended to the old block. This is a trick to prevent cycles.
                    if target_block.targets(asm).count() > 0 {
                        continue;
                    }
                    let (None, None) = (block.handler(), target_block.handler()) else {
                        continue;
                    };
                    if fuel.consume(4) {
                        block.roots_mut().pop();
                        block.roots_mut().extend(target_block.roots());
                    }
                }
        */
        if !fuel.consume(8) {
            return;
        }
        for node in blocks
            .iter()
            .flat_map(super::basic_block::BasicBlock::iter_roots)
            .flat_map(|root| CILIter::new(asm.get_root(root).clone(), asm))
            .filter_map(super::iter::CILIterElem::as_node)
        {
            match node {
                CILNode::LdLoc(loc) => local_reads[loc as usize] = true,
                CILNode::LdLocA(loc) => local_address_of[loc as usize] = true,
                _ => (),
            }
        }
        // Remove writes to those dead locals
        for root in blocks
            .iter_mut()
            .flat_map(super::basic_block::BasicBlock::iter_roots_mut)
        {
            if let CILRoot::StLoc(local, tree) = asm.get_root(*root) {
                // If the local is never read nor adress of, replace it with a pop or a nop.
                if !local_reads[*local as usize] && !local_address_of[*local as usize] {
                    // Tree has side effects, so it has to be evalueted, so we replace it with a pop
                    if cache.has_side_effects(*tree, asm) {
                        *root = asm.alloc_root(CILRoot::Pop(*tree));
                    } else {
                        *root = asm.alloc_root(CILRoot::Nop);
                    }
                } else {
                    // Also, if we are assigining to the same value, this is equivalent to a nop, and can be safely removed.
                    match asm.get_node(*tree) {
                        CILNode::LdLoc(local_2) if local == local_2 => {
                            *root = asm.alloc_root(CILRoot::Nop);
                        }
                        _ => (),
                    }
                }
            }
        }
        self.remove_nops(asm);
    }
    pub fn remove_nops(&mut self, asm: &mut Assembly) {
        // Optimization only suported for methods with locals
        let MethodImpl::MethodBody { blocks, .. } = self else {
            return;
        };
        // Remove Nops
        for block in blocks.iter_mut() {
            block
                .roots_mut()
                .retain(|root| !matches!(asm.get_root(*root), CILRoot::Nop));
            // Remove Nops from the handler too.
            if let Some(blocks) = block.handler_mut() {
                for block in blocks.iter_mut() {
                    block
                        .roots_mut()
                        .retain(|root| !matches!(asm.get_root(*root), CILRoot::Nop));
                }
            }
        }
    }
}

impl MethodDef {
    pub fn iter_roots_mut(&mut self) -> Option<impl Iterator<Item = &mut RootIdx>> {
        self.implementation_mut().blocks_mut().map(|blocks| {
            blocks
                .iter_mut()
                .flat_map(super::basic_block::BasicBlock::iter_roots_mut)
        })
    }
    pub fn map_roots(
        &mut self,
        asm: &mut Assembly,
        root_map: &mut impl FnMut(CILRoot, &mut Assembly) -> CILRoot,
        node_map: &mut impl FnMut(CILNode, &mut Assembly) -> CILNode,
    ) {
        if let Some(roots) = self.iter_roots_mut() {
            roots.for_each(|root| {
                let val = asm.get_root(*root).clone().map(asm, root_map, node_map);
                *root = asm.alloc_root(val);
            });
        }
    }
    pub fn typecheck(&mut self, asm: &mut Assembly) {
        let sig = self.sig();
        let locals = self.iter_locals(asm).cloned().collect::<Vec<_>>();
        let name = self.name();
        if let Some(roots) = self.iter_roots_mut() {
            roots.for_each(|root| {
                let check = asm.get_root(*root).clone().typecheck(sig, &locals, asm);
                if check.is_err() {
                    display_typecheck_err(asm.get_root(*root).clone(), asm, sig, &locals);
                };
                check.unwrap_or_else(|_| {
                    eprintln!("Could not verify method {}", asm.get_string(name))
                })
            })
        }
    }
    pub fn optimize(
        &mut self,
        asm: &mut Assembly,
        cache: &mut SideEffectInfoCache,
        fuel: &mut OptFuel,
    ) {
        let nop = asm.alloc_root(CILRoot::Nop);
        self.implementation_mut().propagate_locals(asm, cache, fuel);
        self.implementation_mut()
            .remove_dead_writes(asm, cache, fuel);
        if fuel.consume(1) {
            self.implementation_mut().realloc_locals(asm);
        }

        if fuel.consume(1) {
            // Remove unneded SFI
            if let Some(roots) = self.iter_roots_mut() {
                let mut peekable = (roots).into_iter().peekable();
                while let Some(curr) = peekable.next() {
                    let Some(peek) = peekable.peek() else {
                        continue;
                    };
                    if let (CILRoot::SourceFileInfo { .. }, CILRoot::SourceFileInfo { .. }) =
                        (asm.get_root(*curr), asm.get_root(**peek))
                    {
                        *curr = nop;
                    }
                }
            }
        }
        if fuel.consume(15) {
            if let Some(roots) = self.iter_roots_mut() {
                let roots: Vec<_> = roots
                    .filter(|root| {
                        !matches!(
                            asm.get_root(**root),
                            CILRoot::Nop | CILRoot::SourceFileInfo { .. }
                        )
                    })
                    .collect();
                let mut peekable = roots.into_iter().peekable();
                while let Some(curr) = peekable.next() {
                    let Some(peek) = peekable.peek() else {
                        continue;
                    };
                    match (asm.get_root(*curr), asm.get_root(**peek)) {
                        // If a rethrow is followed by a rethrow, this is effectively just a single rethrow
                        (CILRoot::ReThrow, CILRoot::ReThrow) => *curr = nop,
                        /*// If SFI is followed by an uncodtional branch, then it has no effect, then it can be safely ommited.
                        (CILRoot::SourceFileInfo { .. }, CILRoot::Branch(info))
                            if is_branch_unconditional(info) =>
                        {
                            *curr = nop
                        }*/
                        // If we return var a immeditaly after assigining it, we can just return it.
                        (CILRoot::StLoc(set_loc, tree), CILRoot::Ret(ret_loc)) => {
                            let CILNode::LdLoc(ret_loc) = asm.get_node(*ret_loc) else {
                                continue;
                            };
                            if set_loc != ret_loc {
                                continue;
                            }
                            let tree = *tree;
                            *curr = nop;
                            let curr = peekable.next().unwrap();
                            *curr = asm.alloc_root(CILRoot::Ret(tree));
                        }
                        (CILRoot::Branch(info), CILRoot::Branch(info2))
                            if is_branch_unconditional(info2) =>
                        {
                            let (target, subtarget, cond) = info.as_ref();
                            let (target2, subtarget2, _) = info2.as_ref();
                            // If a conditional jump to a target is followed by an unconditonal jump to the same target, we just need to perform the unconditonla jump.
                            if target == target2 && subtarget == subtarget2 {
                                match cond {
                                    None => *curr = nop,
                                    Some(BranchCond::True(tr) | BranchCond::False(tr)) => {
                                        *curr = asm.alloc_root(CILRoot::Pop(*tr));
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        if fuel.consume(1) {
            // TODO: this is a hack, which makes root inlining optimizations not consume fuel.
            let fuel = std::sync::Mutex::new(&mut *fuel);
            self.map_roots(
                asm,
                &mut |root, asm| {
                    let mut root_fuel = fuel.lock().unwrap();
                    match root {
                        CILRoot::Pop(pop) => match asm.get_node(pop) {
                            CILNode::LdLoc(_) => CILRoot::Nop,
                            _ => root,
                        },
                        CILRoot::Call(info) => {
                            inline_trivial_call_root(info.0, &info.1, *root_fuel, asm)
                        }
                        /*
                        CILRoot::StInd(ref info) => match asm.get_node(info.1) {
                            CILNode::LdInd {
                                addr: src_addr,
                                tpe,
                                volitale:_,
                            } => {
                                assert_eq!(*asm.get_type(*tpe), info.2);
                                CILRoot::CpObj {
                                    src: *src_addr,
                                    dst: info.0,
                                    tpe: asm.alloc_type(info.2),
                                }
                            }
                            _ => root,
                        },*/
                        CILRoot::Branch(ref info) => {
                            let (target, sub_target, cond) = info.as_ref();
                            match cond {
                                Some(BranchCond::False(cond)) => {
                                    match asm.get_node(*cond) {
                                        CILNode::Const(cst) => match cst.as_ref() {
                                            Const::Bool(false) => opt_if_fuel(
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    None,
                                                ))),
                                                root,
                                                *root_fuel,
                                            ),
                                            Const::Bool(true) => {
                                                opt_if_fuel(CILRoot::Nop, root, *root_fuel)
                                            }
                                            _ => root,
                                        },
                                        // a == b is false <=> a != b
                                        CILNode::BinOp(lhs, rhs, BinOp::Eq) => opt_if_fuel(
                                            {
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::Ne(*lhs, *rhs)),
                                                )))
                                            },
                                            root,
                                            *root_fuel,
                                        ),
                                        // a > b is false <=> a <= b
                                        CILNode::BinOp(lhs, rhs, BinOp::Gt) => opt_if_fuel(
                                            {
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::Le(
                                                        *lhs,
                                                        *rhs,
                                                        super::cilroot::CmpKind::Ordered,
                                                    )),
                                                )))
                                            },
                                            root,
                                            *root_fuel,
                                        ),
                                        CILNode::BinOp(lhs, rhs, BinOp::GtUn) => opt_if_fuel(
                                            {
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::Le(
                                                        *lhs,
                                                        *rhs,
                                                        super::cilroot::CmpKind::Unordered,
                                                    )),
                                                )))
                                            },
                                            root,
                                            *root_fuel,
                                        ),
                                        // a < b is false <=> a >= b
                                        CILNode::BinOp(lhs, rhs, BinOp::Lt) => opt_if_fuel(
                                            {
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::Ge(
                                                        *lhs,
                                                        *rhs,
                                                        super::cilroot::CmpKind::Ordered,
                                                    )),
                                                )))
                                            },
                                            root,
                                            *root_fuel,
                                        ),
                                        CILNode::BinOp(lhs, rhs, BinOp::LtUn) => opt_if_fuel(
                                            {
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::Ge(
                                                        *lhs,
                                                        *rhs,
                                                        super::cilroot::CmpKind::Unordered,
                                                    )),
                                                )))
                                            },
                                            root,
                                            *root_fuel,
                                        ),
                                        //CILNode::IntCast { input, target, extend }
                                        _ => root,
                                    }
                                }
                                Some(BranchCond::True(cond)) => match asm.get_node(*cond) {
                                    // a == b  is true <=> a == b
                                    CILNode::BinOp(lhs, rhs, BinOp::Eq) => opt_if_fuel(
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Eq(*lhs, *rhs)),
                                        ))),
                                        root,
                                        *root_fuel,
                                    ),
                                    CILNode::BinOp(lhs, rhs, BinOp::GtUn) => opt_if_fuel(
                                        {
                                            CILRoot::Branch(Box::new((
                                                *target,
                                                *sub_target,
                                                Some(BranchCond::Gt(
                                                    *lhs,
                                                    *rhs,
                                                    super::cilroot::CmpKind::Unordered,
                                                )),
                                            )))
                                        },
                                        root,
                                        *root_fuel,
                                    ),
                                    CILNode::BinOp(lhs, rhs, BinOp::Gt) => opt_if_fuel(
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Gt(
                                                *lhs,
                                                *rhs,
                                                super::cilroot::CmpKind::Ordered,
                                            )),
                                        ))),
                                        root,
                                        *root_fuel,
                                    ),
                                    CILNode::BinOp(lhs, rhs, BinOp::LtUn) => opt_if_fuel(
                                        {
                                            CILRoot::Branch(Box::new((
                                                *target,
                                                *sub_target,
                                                Some(BranchCond::Lt(
                                                    *lhs,
                                                    *rhs,
                                                    super::cilroot::CmpKind::Unordered,
                                                )),
                                            )))
                                        },
                                        root,
                                        *root_fuel,
                                    ),
                                    CILNode::BinOp(lhs, rhs, BinOp::Lt) => opt_if_fuel(
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Lt(
                                                *lhs,
                                                *rhs,
                                                super::cilroot::CmpKind::Ordered,
                                            )),
                                        ))),
                                        root,
                                        *root_fuel,
                                    ),
                                    _ => root,
                                },
                                Some(BranchCond::Ne(lhs, rhs)) => {
                                    match (asm.get_node(*lhs), asm.get_node(*rhs)) {
                                        (_, CILNode::Const(cst)) => match cst.as_ref() {
                                            // val != false <=> val is true
                                            Const::Bool(false)
                                            | Const::ISize(0)
                                            | Const::USize(0)
                                            | Const::I64(0)
                                            | Const::U64(0)
                                            | Const::I32(0)
                                            | Const::U32(0)
                                            | Const::I16(0)
                                            | Const::U16(0)
                                            | Const::I8(0)
                                            | Const::U8(0) => opt_if_fuel(
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::True(*lhs)),
                                                ))),
                                                root,
                                                *root_fuel,
                                            ),
                                            // val != true <=> val is false
                                            Const::Bool(true) => opt_if_fuel(
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::False(*lhs)),
                                                ))),
                                                root,
                                                *root_fuel,
                                            ),
                                            _ => root,
                                        },
                                        (CILNode::Const(cst), _) => match cst.as_ref() {
                                            // val != false <=> val is true
                                            Const::Bool(false)
                                            | Const::ISize(0)
                                            | Const::USize(0)
                                            | Const::I64(0)
                                            | Const::U64(0)
                                            | Const::I32(0)
                                            | Const::U32(0)
                                            | Const::I16(0)
                                            | Const::U16(0)
                                            | Const::I8(0)
                                            | Const::U8(0) => opt_if_fuel(
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::True(*rhs)),
                                                ))),
                                                root,
                                                *root_fuel,
                                            ),
                                            _ => root,
                                        },
                                        _ => root,
                                    }
                                }
                                Some(BranchCond::Eq(lhs, rhs)) => {
                                    match (asm.get_node(*lhs), asm.get_node(*rhs)) {
                                        (_, CILNode::Const(cst)) => match cst.as_ref() {
                                            Const::Bool(false)
                                            | Const::ISize(0)
                                            | Const::USize(0)
                                            | Const::I64(0)
                                            | Const::U64(0)
                                            | Const::I32(0)
                                            | Const::U32(0)
                                            | Const::I16(0)
                                            | Const::U16(0)
                                            | Const::I8(0)
                                            | Const::U8(0) => opt_if_fuel(
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::False(*lhs)),
                                                ))),
                                                root,
                                                *root_fuel,
                                            ),
                                            _ => root,
                                        },
                                        (CILNode::Const(cst), _) => match cst.as_ref() {
                                            Const::Bool(false)
                                            | Const::ISize(0)
                                            | Const::USize(0)
                                            | Const::I64(0)
                                            | Const::U64(0)
                                            | Const::I32(0)
                                            | Const::U32(0)
                                            | Const::I16(0)
                                            | Const::U16(0)
                                            | Const::I8(0)
                                            | Const::U8(0) => opt_if_fuel(
                                                CILRoot::Branch(Box::new((
                                                    *target,
                                                    *sub_target,
                                                    Some(BranchCond::False(*rhs)),
                                                ))),
                                                root,
                                                *root_fuel,
                                            ),
                                            _ => root,
                                        },
                                        _ => root,
                                    }
                                }
                                Some(_) | None => root,
                            }
                        }
                        _ => root,
                    }
                },
                &mut |node, asm| {
                    let mut fuel = fuel.lock().unwrap();
                    opt_node::opt_node(node, asm, *fuel)
                },
            );
        }
        if fuel.consume(1) {
            self.implementation_mut().remove_duplicate_sfi(asm);
        }

        if let MethodImpl::MethodBody { blocks, .. } = self.implementation_mut() {
            for block in blocks.iter_mut() {
                let Some(handler) = block.handler() else {
                    continue;
                };
                // If this handler does nothing besides jumping around, seting locals, and then rethrows, then this handler should optimize away into nothing.
                if handler.iter().all(|block| {
                    block
                        .meaningfull_roots(asm)
                        .all(|root| match asm.get_root(root) {
                            CILRoot::Branch(info) => {
                                if let Some(cond) = &info.2 {
                                    cond.nodes()
                                        .iter()
                                        .all(|node| !cache.has_side_effects(*node, asm))
                                } else {
                                    true
                                }
                            }
                            CILRoot::StLoc(_, tree) => !cache.has_side_effects(*tree, asm),
                            CILRoot::ReThrow | CILRoot::Nop | CILRoot::SetStaticField { .. } => {
                                true
                            }
                            _ => false,
                        })
                }) && fuel.consume(6)
                {
                    block.remove_handler();
                }
            }
            /*// If no jumps away, then this is the only alive block, then we can remove all other blocks
            if blocks.len() > 1 && blocks[0].targets(asm).count() == 0 && fuel.consume(1) {
                *blocks = vec![blocks[0].clone()];
            }*/
        };
    }
}
#[must_use]
pub fn is_branch_unconditional(branch: &(u32, u32, Option<BranchCond>)) -> bool {
    branch.2.is_none()
}
fn blockid_from_jump(target: u32, sub_target: u32) -> u32 {
    if sub_target == 0 {
        target
    } else {
        sub_target
    }
}
fn block_with_id(blocks: &[BasicBlock], id: u32) -> Option<&BasicBlock> {
    blocks.iter().find(|block| block.block_id() == id)
}
#[test]
fn find_block() {
    let blocks = vec![];
    assert!(block_with_id(&blocks, 0).is_none());
    let blocks = vec![
        BasicBlock::new(vec![], 0, None),
        BasicBlock::new(vec![], 1, None),
    ];
    assert!(block_with_id(&blocks, 0).is_some());
    assert!(block_with_id(&blocks, 1).is_some());
    assert!(block_with_id(&blocks, 2).is_none());
}
#[test]
fn blockid() {
    assert_eq!(blockid_from_jump(0, 0), 0);
    assert_eq!(blockid_from_jump(2, 1), 1);
    assert_eq!(blockid_from_jump(1, 2), 2);
    assert_eq!(blockid_from_jump(2, 0), 2);
}
#[test]
fn opt_mag() {
    use super::{BinOp, Float};
    use crate::v2::{asm::ILASM_FLAVOUR, cilnode::MethodKind, il_exporter::ILExporter};
    let mut asm = Assembly::default();

    // Arg gets
    let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
    let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
    // Local gets
    let ldloc_0 = asm.alloc_node(CILNode::LdLoc(0));
    let ldloc_1 = asm.alloc_node(CILNode::LdLoc(1));
    let ldloc_2 = asm.alloc_node(CILNode::LdLoc(2));

    let mul_arg0 = asm.alloc_node(CILNode::BinOp(ldarg_0, ldarg_0, BinOp::Mul));
    let mul_arg1 = asm.alloc_node(CILNode::BinOp(ldarg_1, ldarg_1, BinOp::Mul));
    let stloc_0 = asm.alloc_root(CILRoot::StLoc(0, mul_arg0));
    let stloc_1 = asm.alloc_root(CILRoot::StLoc(1, mul_arg1));
    let mag = asm.alloc_node(CILNode::BinOp(ldloc_0, ldloc_1, BinOp::Add));
    let stloc_2 = asm.alloc_root(CILRoot::StLoc(2, mag));
    let ret = asm.alloc_root(CILRoot::Ret(ldloc_2));

    let bb = BasicBlock::new(vec![stloc_0, stloc_1, stloc_2, ret], 0, None);
    // Locals
    let locals = vec![
        (None, asm.alloc_type(Float::F32)),
        (None, asm.alloc_type(Float::F32)),
        (None, asm.alloc_type(Float::F32)),
    ];

    let mimpl = MethodImpl::MethodBody {
        blocks: vec![bb],
        locals,
    };

    let main_module = asm.main_module();
    let sig = asm.sig(
        [Type::Float(Float::F32), Type::Float(Float::F32)],
        Type::Float(Float::F32),
    );
    let name = asm.alloc_string("mag");
    asm.new_method(MethodDef::new(
        crate::v2::Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        mimpl,
        vec![None, None],
    ));
    let name = asm.alloc_string("entrypoint");
    asm.new_method(MethodDef::new(
        crate::v2::Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        MethodImpl::Missing,
        vec![None, None],
    ));
    let mut fuel = OptFuel::new(77);

    asm.opt(&mut fuel);

    asm.export("/tmp/opt_mag.exe", ILExporter::new(*ILASM_FLAVOUR, false));
}

#[test]
fn is_branch_unconditional_test() {
    assert!(is_branch_unconditional(&(0, 0, None)));
    let mut asm = Assembly::default();
    let arg0 = asm.alloc_node(CILNode::LdArg(0));
    assert!(!is_branch_unconditional(&(
        0,
        0,
        Some(BranchCond::True(arg0))
    )));
}
#[test]
fn local_prop() {
    let mut asm = Assembly::default();
    let arg0 = asm.alloc_node(CILNode::LdArg(0));
    let stloc_0 = asm.alloc_root(CILRoot::StLoc(0, arg0));
    let loc0 = CILNode::LdLoc(0);
    let sum = asm.biop(loc0.clone(), loc0, BinOp::Add);
    let sum = asm.alloc_node(sum);
    let ret = asm.alloc_root(CILRoot::Ret(sum));
    let mut block = BasicBlock::new(vec![stloc_0, ret], 0, None);
    let mut cache = SideEffectInfoCache::default();
    let mut fuel = OptFuel::new(1000);
    let isize_tpe = asm.alloc_type(Type::Int(Int::ISize));
    block.local_opt(&mut asm, &[(None, isize_tpe)], &mut cache, &mut fuel);
    let mut iter = block.roots().iter();
    assert!(iter.next().is_some());
    let opt_ret = iter.next().unwrap();
    let sum_arg0 = asm.biop(CILNode::LdArg(0), CILNode::LdArg(0), BinOp::Add);
    let sum_arg0 = asm.alloc_node(sum_arg0);
    let ret = asm.alloc_root(CILRoot::Ret(sum_arg0));
    assert_eq!(ret, *opt_ret);
}
#[test]
fn remove_nops() {
    let mut asm = Assembly::default();
    let arg0 = asm.alloc_node(CILNode::LdArg(0));
    let stloc_0 = asm.alloc_root(CILRoot::StLoc(0, arg0));
    let loc0 = CILNode::LdLoc(0);
    let sum = asm.biop(loc0.clone(), loc0, BinOp::Add);
    let sum = asm.alloc_node(sum);
    let ret = asm.alloc_root(CILRoot::Ret(sum));
    let nop = asm.alloc_root(CILRoot::Nop);
    let block = BasicBlock::new(vec![nop, stloc_0, nop, ret, nop], 0, None);
    let loc = asm.alloc_type(Type::Float(Float::F32));
    let mut mimpl = MethodImpl::MethodBody {
        blocks: vec![block],
        locals: vec![(None, loc)],
    };
    assert_eq!(mimpl.blocks_mut().unwrap()[0].roots().len(), 5);
    mimpl.remove_nops(&mut asm);
    assert_eq!(mimpl.blocks_mut().unwrap()[0].roots().len(), 2);
}
