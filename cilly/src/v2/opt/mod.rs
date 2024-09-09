use simplify_handlers::simplify_bbs;

use super::{
    cilroot::BranchCond, method::LocalDef, BasicBlock, BinOp, CILIter, CILIterElem, CILNode,
    CILRoot, Const, Int, MethodImpl, NodeIdx, RootIdx, Type,
};
use crate::v2::{Assembly, MethodDef};
use std::collections::HashMap;
mod opt_node;
mod simplify_handlers;
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct OptFuel(u32);
impl OptFuel {
    #[must_use]
    pub fn scale(self, scale: f32) -> Self {
        let inner = self.0;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let scale_div = (1.0 / scale) as u32;
        Self(inner / scale_div)
    }
    /// Creates *fuel* fuel
    #[must_use]
    pub fn new(fuel: u32) -> Self {
        Self(fuel)
    }
    /// Decreases the ammount of fuel avalible if fuel present, and returns false if not enough fuel present.
    pub fn consume(&mut self, cost: u32) -> bool {
        if self.0 > cost {
            self.0 -= 1;
            true
        } else {
            false
        }
    }
    /// Checks if no fuel remains
    #[must_use]
    pub fn exchausted(&self) -> bool {
        self.0 == 0
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
#[derive(Default)]
pub struct SideEffectInfoCache {
    side_effects: HashMap<NodeIdx, bool>,
}
impl SideEffectInfoCache {
    /// Checks if a node may have side effects(if dupilcating it and poping the result would change the way a program runs).
    #[allow(clippy::match_same_arms)]
    fn has_side_effects(&mut self, node: NodeIdx, asm: &Assembly) -> bool {
        if let Some(side_effect) = self.side_effects.get(&node) {
            return *side_effect;
        }
        let side_effect = match asm.get_node(node) {
            CILNode::LdTypeToken(_)
            | CILNode::LdFtn(_)
            | CILNode::Const(_)
            | CILNode::SizeOf(_) => false, // Constant, can't have side effects
            CILNode::BinOp(lhs, rhs, _) => {
                self.has_side_effects(*lhs, asm) || self.has_side_effects(*rhs, asm)
            }
            CILNode::UnOp(arg, _) => self.has_side_effects(*arg, asm), // UnOp, only has side effects if its arg has side effects
            CILNode::LdLoc(_) | CILNode::LdArg(_) => false, // Reading a variable has no side effects
            CILNode::LdLocA(_) | CILNode::LdArgA(_) => false, // Getting the address of something has no side effects.
            CILNode::Call(_) => true, // For now, we assume all calls have side effects.
            CILNode::RefToPtr(input)
            | CILNode::IntCast { input, .. }
            | CILNode::FloatCast { input, .. }
            | CILNode::PtrCast(input, _) => self.has_side_effects(*input, asm), // Casts don't have side effects, unless their input has one.
            CILNode::LdFieldAdress { addr, .. }
            | CILNode::LdField { addr, .. }
            | CILNode::LdInd { addr, .. } => self.has_side_effects(*addr, asm), // Reading a pointer or a field never has side effects.
            CILNode::GetException => true, // This is a low-level, unsafe operation, which manipulates the runtime stack, and can't be preformed twice. It for sure has side effects.
            CILNode::UnboxAny { object, .. }
            | CILNode::IsInst(object, _)
            | CILNode::CheckedCast(object, _) => {
                self.has_side_effects(*object, asm) // Class checks / casts / unboxes have no side effects.
            }
            CILNode::CallI(_) => true, // Indidrect calls may have side effects
            CILNode::LocAllocAlgined { .. } | CILNode::LocAlloc { .. } => true, // Allocation has side effects
            CILNode::LdStaticField(_) => false, // Loading static fields has no side effects.
            CILNode::LdLen(arr) => self.has_side_effects(*arr, asm), // Loading a length only has side effects if the index has array.
            CILNode::LdElelemRef { array, index } => {
                self.has_side_effects(*array, asm) | self.has_side_effects(*index, asm)
                // Indexing only has side effects if the index or array address has side effects.
            }
        };
        self.side_effects.insert(node, side_effect);
        side_effect
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
        root_map: &mut impl Fn(CILRoot, &mut Assembly) -> CILRoot,
        node_map: &mut impl Fn(CILNode, &mut Assembly) -> CILNode,
    ) {
        if let Some(roots) = self.iter_roots_mut() {
            roots.for_each(|root| {
                let val = asm.get_root(*root).clone().map(asm, root_map, node_map);
                *root = asm.alloc_root(val);
            });
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
        if fuel.consume(5) {
            self.implementation_mut().realloc_locals(asm);
        }
        if fuel.consume(5) {
            if let Some(roots) = self.iter_roots_mut() {
                let mut peekable = roots.peekable();
                while let Some(curr) = peekable.next() {
                    let Some(peek) = peekable.peek() else {
                        continue;
                    };
                    match (asm.get_root(*curr), asm.get_root(**peek)) {
                        (CILRoot::SourceFileInfo { .. }, CILRoot::SourceFileInfo { .. }) => {
                            *curr = nop;
                        }
                        // If a rethrow is followed by a rethrow, this is effectively just a single rethrow
                        (CILRoot::ReThrow, CILRoot::ReThrow) => *curr = nop,
                        /*// If SFI is followed by an uncodtional branch, then it has no effect, then it can be safely ommited.
                        (CILRoot::SourceFileInfo { .. }, CILRoot::Branch(info))
                            if is_branch_unconditional(info) =>
                        {
                            *curr = nop
                        }*/
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
        if fuel.consume(5) {
            self.map_roots(
                asm,
                &mut |root, asm| match root {
                    CILRoot::Pop(pop) => match asm.get_node(pop) {
                        CILNode::LdLoc(_) => CILRoot::Nop,
                        _ => root,
                    },
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
                                        Const::Bool(false) => {
                                            CILRoot::Branch(Box::new((*target, *sub_target, None)))
                                        }
                                        Const::Bool(true) => CILRoot::Nop,
                                        _ => root,
                                    },
                                    // a == b is false <=> a != b
                                    CILNode::BinOp(lhs, rhs, BinOp::Eq) => {
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Ne(*lhs, *rhs)),
                                        )))
                                    }
                                    // a > b is false <=> a <= b
                                    CILNode::BinOp(lhs, rhs, BinOp::Gt) => {
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Le(
                                                *lhs,
                                                *rhs,
                                                super::cilroot::CmpKind::Ordered,
                                            )),
                                        )))
                                    }
                                    CILNode::BinOp(lhs, rhs, BinOp::GtUn) => {
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Le(
                                                *lhs,
                                                *rhs,
                                                super::cilroot::CmpKind::Unordered,
                                            )),
                                        )))
                                    }
                                    // a < b is false <=> a >= b
                                    CILNode::BinOp(lhs, rhs, BinOp::Lt) => {
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Ge(
                                                *lhs,
                                                *rhs,
                                                super::cilroot::CmpKind::Ordered,
                                            )),
                                        )))
                                    }
                                    CILNode::BinOp(lhs, rhs, BinOp::LtUn) => {
                                        CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::Ge(
                                                *lhs,
                                                *rhs,
                                                super::cilroot::CmpKind::Unordered,
                                            )),
                                        )))
                                    }
                                    //CILNode::IntCast { input, target, extend }
                                    _ => root,
                                }
                            }
                            Some(BranchCond::True(cond)) => match asm.get_node(*cond) {
                                // a == b  is true <=> a == b
                                CILNode::BinOp(lhs, rhs, BinOp::Eq) => CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Eq(*lhs, *rhs)),
                                ))),
                                CILNode::BinOp(lhs, rhs, BinOp::GtUn) => {
                                    CILRoot::Branch(Box::new((
                                        *target,
                                        *sub_target,
                                        Some(BranchCond::Gt(
                                            *lhs,
                                            *rhs,
                                            super::cilroot::CmpKind::Unordered,
                                        )),
                                    )))
                                }
                                CILNode::BinOp(lhs, rhs, BinOp::Gt) => CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Gt(
                                        *lhs,
                                        *rhs,
                                        super::cilroot::CmpKind::Ordered,
                                    )),
                                ))),
                                CILNode::BinOp(lhs, rhs, BinOp::LtUn) => {
                                    CILRoot::Branch(Box::new((
                                        *target,
                                        *sub_target,
                                        Some(BranchCond::Lt(
                                            *lhs,
                                            *rhs,
                                            super::cilroot::CmpKind::Unordered,
                                        )),
                                    )))
                                }
                                CILNode::BinOp(lhs, rhs, BinOp::Lt) => CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Lt(
                                        *lhs,
                                        *rhs,
                                        super::cilroot::CmpKind::Ordered,
                                    )),
                                ))),
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
                                        | Const::U8(0) => CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::True(*lhs)),
                                        ))),
                                        // val != true <=> val is false
                                        Const::Bool(true) => CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::False(*lhs)),
                                        ))),
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
                                        | Const::U8(0) => CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::True(*rhs)),
                                        ))),
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
                                        | Const::U8(0) => CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::False(*lhs)),
                                        ))),
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
                                        | Const::U8(0) => CILRoot::Branch(Box::new((
                                            *target,
                                            *sub_target,
                                            Some(BranchCond::False(*rhs)),
                                        ))),
                                        _ => root,
                                    },
                                    _ => root,
                                }
                            }
                            Some(_) | None => root,
                        }
                    }
                    _ => root,
                },
                &mut opt_node::opt_node,
            );
        }
        if fuel.consume(5) {
            self.implementation_mut().remove_duplicate_sfi(asm);
        }
        if fuel.consume(6) {
            if let MethodImpl::MethodBody { blocks, .. } = self.implementation_mut() {
                for block in blocks.iter_mut() {
                    simplify_bbs(block.handler_mut(), asm, fuel);
                    /*
                    let Some(handler) = block.handler() else {
                        return;
                    };
                    // If tall the blocks only rethrow(and don't do anything else!), then this handler nothing, and we can ommit it
                    if handler.iter().all(|block| block.is_only_rethrow(asm)) {
                        //panic!("about to remove {handler}");
                        block.remove_handler();
                    }*/
                }
                // simplify_bbs(Some(blocks), asm, fuel)
            };
        }
    }
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

    /*
    .method public hidebysig static  float32 'mag'(float32 'x',float32 'y') cil managed {// Method ID MethodDefIdx(MethodRefIdx(18))
        .maxstack 8
         .locals (
          float32,
          float32,
          float32)
         bb0:
        .line 19:5 './add.rs'
        ldarg.0
        ldarg.0
        mul
        stloc.0
        .line 19:13 './add.rs'
        ldarg.1
        ldarg.1
        mul
        stloc.1
        .line 19:5 './add.rs'
        ldloc.0
        ldloc.1
        add
        stloc.2
        .line 20:2 './add.rs'
        ldloc.2
        ret
        }
    */
    asm.export("/tmp/opt_mag.exe", ILExporter::new(*ILASM_FLAVOUR, false));
}
/*
  Breaks:
   compile_test::fuzz46::stable::debug
    compile_test::fuzz46::stable::release
    compile_test::fuzz80::stable::debug
    compile_test::fuzz80::stable::release
    compile_test::fuzz95::stable::debug
    compile_test::fuzz95::stable::release




*/
#[must_use]
pub fn is_branch_unconditional(branch: &(u32, u32, Option<BranchCond>)) -> bool {
    branch.2.is_none()
}
