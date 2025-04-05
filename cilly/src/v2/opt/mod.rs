use root::root_opt;

#[cfg(test)]
use super::Float;

use super::{
    cilroot::BranchCond, method::LocalDef, typecheck::display_typecheck_err, BasicBlock, CILIter,
    CILIterElem, CILNode, CILRoot, Int, MethodImpl, NodeIdx, RootIdx, SigIdx, Type,
};
use crate::{Assembly, MethodDef};
pub use opt_fuel::OptFuel;
pub use side_effect::*;
mod inline;
mod opt_fuel;
mod opt_node;
mod root;
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
#[derive(Clone, Copy, PartialEq, Eq)]
enum LocalPropagate {
    Local(u32),
    Arg(u32),
    Field(super::FieldIdx, NodeIdx),
}
impl CILNode {
    // The complexity of this function is unavoidable.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    fn propagate_locals(
        &self,
        asm: &mut Assembly,
        idx: LocalPropagate,
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
            CILNode::LdField { addr, field } if LocalPropagate::Field(*field, *addr) == idx => {
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
                    _ => self.clone(),
                }
            }
            CILNode::LdLoc(loc) if LocalPropagate::Local(*loc) == idx => {
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
            }
            CILNode::LdArg(loc) if LocalPropagate::Arg(*loc) == idx => {
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
                    _ => CILNode::LdArg(*loc),
                }
            }
            CILNode::LdLoc(loc) => CILNode::LdLoc(*loc),
            CILNode::LdLocA(loc) => CILNode::LdLocA(*loc), // This takes an address, so we can't propagate it
            CILNode::LdArg(arg) => CILNode::LdArg(*arg),
            CILNode::LdArgA(arg) => CILNode::LdArgA(*arg),
            CILNode::Call(info) => {
                let (mref, mut args, is_pure) = *info.clone();
                args.iter_mut().for_each(|arg_mut| {
                    let arg = asm[*arg_mut].clone();
                    let arg = arg.propagate_locals(asm, idx, tpe, new_node, fuel);
                    *arg_mut = asm.alloc_node(arg);
                });
                CILNode::Call(Box::new((mref, args, is_pure)))
            }
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
                volatile: volitale,
            } => {
                let addr = asm.get_node(*addr).clone();
                let addr = addr.propagate_locals(asm, idx, tpe, new_node, fuel);
                let addr = asm.alloc_node(addr);
                CILNode::LdInd {
                    addr,
                    tpe: *tpe2,
                    volatile: *volitale,
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
        sig: SigIdx,
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
            propagate_roots(asm, root, prev_root, cache, locals, sig, fuel);
            prev_root_id = root;
        }
    }
}
fn propagate_roots(
    asm: &mut Assembly,
    root: &mut RootIdx,
    prev_root: CILRoot,
    cache: &mut SideEffectInfoCache,
    locals: &[LocalDef],
    sig: SigIdx,
    fuel: &mut OptFuel,
) -> bool {
    match prev_root {
        CILRoot::StLoc(loc, tree) => {
            // 1 st. check if the previous node is a candiate for propagation.
            if cache.has_side_effects(tree, asm) {
                return true;
            }
            // Check that the tree is not too big
            if CILIter::new(asm.get_node(tree).clone(), asm).count() > 16 {
                return true;
            }
            // Check that it does not depend on itself
            if CILIter::new(asm.get_node(tree).clone(), asm)
                .any(|node| node == CILIterElem::Node(CILNode::LdLoc(loc)))
            {
                return true;
            }
            propagate_root(
                asm,
                root,
                cache,
                LocalPropagate::Local(loc),
                asm[locals[loc as usize].1],
                tree,
                fuel,
            )
        }
        CILRoot::SetField(info) => {
            let (field, addr, tree) = info.as_ref();
            // 1 st. check if the previous node is a candiate for propagation.
            if cache.has_side_effects(*tree, asm) {
                return true;
            }
            // Check that the tree is not too big
            if CILIter::new(asm.get_node(*tree).clone(), asm).count() > 16 {
                return true;
            }
            // Check that it does not depend on itself
            if CILIter::new(asm.get_node(*tree).clone(), asm)
                .any(|node| node == CILIterElem::Node(asm[*addr].clone()))
            {
                return true;
            }

            let res = propagate_root(
                asm,
                root,
                cache,
                LocalPropagate::Field(*field, *addr),
                asm[*field].tpe(),
                *tree,
                fuel,
            );
            if let CILNode::LdLocA(loc) = &asm[*addr] {
                let loc_node = asm.alloc_node(CILNode::LdLoc(*loc));
                // Check that it does not depend on itself
                if CILIter::new(asm.get_node(*tree).clone(), asm)
                    .any(|node| node == CILIterElem::Node(asm[loc_node].clone()))
                {
                    return res;
                }
                propagate_root(
                    asm,
                    root,
                    cache,
                    LocalPropagate::Field(*field, loc_node),
                    asm[*field].tpe(),
                    *tree,
                    fuel,
                )
            } else {
                res
            }
        }
        CILRoot::StArg(arg, tree) => {
            // 1 st. check if the previous node is a candiate for propagation.
            if cache.has_side_effects(tree, asm) {
                return true;
            }
            // Check that the tree is not too big
            if CILIter::new(asm.get_node(tree).clone(), asm).count() > 16 {
                return true;
            }
            // Check that it does not depend on itself
            if CILIter::new(asm.get_node(tree).clone(), asm)
                .any(|node| node == CILIterElem::Node(CILNode::LdArg(arg)))
            {
                return true;
            }
            propagate_root(
                asm,
                root,
                cache,
                LocalPropagate::Arg(arg),
                asm[sig].inputs()[arg as usize],
                tree,
                fuel,
            )
        }
        _ => true,
    }
}
fn propagate_root(
    asm: &mut Assembly,
    root: &mut RootIdx,
    cache: &mut SideEffectInfoCache,
    idx: LocalPropagate,
    tpe: Type,
    tree: NodeIdx,
    fuel: &mut OptFuel,
) -> bool {
    let mut tmp_root = asm.get_root(*root).clone();
    let mut cant_prop = false;
    for node in tmp_root.nodes_mut() {
        let new_node: CILNode = asm.get_node(*node).clone();

        let new_node = new_node.map(asm, &mut |node: CILNode, asm| {
            if cache.has_side_effects(asm.alloc_node(node.clone()), asm) {
                cant_prop = true;
            }

            if cant_prop {
                node
            } else {
                node.propagate_locals(asm, idx, tpe, tree, fuel)
            }
        });
        *node = asm.alloc_node(new_node);
    }
    *root = asm.alloc_root(tmp_root);
    cant_prop
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
        sig: SigIdx,
    ) {
        // Optimization only suported for methods with locals
        let MethodImpl::MethodBody { blocks, locals } = self else {
            return;
        };

        blocks
            .iter_mut()
            .for_each(|block| block.local_opt(asm, locals, cache, fuel, sig));
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
        let mut local_address_of = vec![0_i32; locals.len()];

        if !fuel.consume(8) {
            return;
        }
        for node in blocks
            .iter()
            .flat_map(super::basic_block::BasicBlock::iter_roots)
            .flat_map(|root| CILIter::new(asm.get_root(root).clone(), asm))
        {
            match node {
                CILIterElem::Node(CILNode::LdLoc(loc)) => local_reads[loc as usize] = true,
                CILIterElem::Node(CILNode::LdLocA(loc)) => local_address_of[loc as usize] += 1,
                CILIterElem::Root(CILRoot::SetField(info)) => {
                    if let CILNode::LdLocA(loc) = asm[info.1] {
                        local_address_of[loc as usize] -= 1
                    }
                }
                _ => (),
            }
        }
        // Remove writes to those dead locals
        for root in blocks
            .iter_mut()
            .flat_map(super::basic_block::BasicBlock::iter_roots_mut)
        {
            match asm.get_root(*root) {
                CILRoot::StLoc(local, tree) => {
                    // If the local is never read nor adress of, replace it with a pop or a nop.
                    if !local_reads[*local as usize] && (local_address_of[*local as usize] <= 0) {
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
                CILRoot::SetField(info) => {
                    if let CILNode::LdLocA(loc) = asm[info.1] {
                        if !local_reads[loc as usize] && (local_address_of[loc as usize] <= 0) {
                            // Tree has side effects, so it has to be evalueted, so we replace it with a pop
                            if cache.has_side_effects(info.2, asm) {
                                *root = asm.alloc_root(CILRoot::Pop(info.2));
                            } else {
                                *root = asm.alloc_root(CILRoot::Nop);
                            }
                        }
                    }
                }
                _ => (),
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
                    display_typecheck_err(*root, asm, sig, &locals);
                };
                check.unwrap_or_else(|_| eprintln!("Could not verify method {}", &asm[name]))
            })
        }
    }
    pub fn optimize(
        &mut self,
        asm: &mut Assembly,
        cache: &mut SideEffectInfoCache,
        fuel: &mut OptFuel,
    ) {
        let sig = self.sig();
        self.implementation_mut()
            .propagate_locals(asm, cache, fuel, sig);
        self.implementation_mut()
            .remove_dead_writes(asm, cache, fuel);
        if fuel.consume(1) {
            self.implementation_mut().realloc_locals(asm);
        }

        if fuel.consume(15) {
            self.dedup_roots(asm);
        }
        if fuel.consume(1) {
            self.opt_roots(fuel, cache, asm);
        }
        if fuel.consume(1) {
            self.implementation_mut().remove_duplicate_sfi(asm);
        }
        if let MethodImpl::MethodBody { blocks, .. } = self.implementation_mut() {
            if let Some(block) = linearize_blocks(blocks, asm) {
                *blocks = vec![block];
            }
            // Better, not yet done OPT.
            // linearilze_best_span(blocks, asm);
            // Linear, so supports some additional opts.
            if blocks.len() == 1 {}
        }

        self.remove_useless_handlers(asm, fuel, cache);
    }
    fn remove_useless_handlers(
        &mut self,
        asm: &mut Assembly,
        fuel: &mut OptFuel,
        cache: &mut SideEffectInfoCache,
    ) {
        if let MethodImpl::MethodBody { blocks, .. } = self.implementation_mut() {
            /*let has_targets: FxHashMap<_, bool> = blocks
                .iter()
                .map(|block| (block.block_id(), block.targets(asm).next().is_some()))
                .collect();
            let blocks_copy: FxHashMap<_, _> = blocks
                .iter()
                .map(|block| (block.block_id(), block.clone()))
                .collect();*/
            for block in blocks.iter_mut() {
                /* if let CILRoot::Branch(info) =
                    &asm[*block.roots().last().expect("Blocks can't be empty")]
                {
                    if block.roots().iter().all(|root| match &asm[*root] {
                        CILRoot::StLoc(_, _)
                        | CILRoot::SourceFileInfo { .. }
                        | CILRoot::Nop
                        | CILRoot::SetField(_) => true,
                        CILRoot::Branch(info) => is_branch_unconditional(&info),
                        _ => false,
                    }) {
                        let (target, _, None) = info.as_ref() else {
                            continue;
                        };
                        // Ret or throw
                        if !has_targets[target] && blocks_copy[target].roots().len() < 60 {
                            let roots = block.roots_mut();
                            roots.pop();
                            roots.extend(blocks_copy[target].roots());
                        }
                    }
                }*/
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
                    block.remove_handler(asm);
                }
            }
        };
    }

    fn dedup_roots(&mut self, asm: &mut Assembly) {
        let nop = asm.alloc_root(CILRoot::Nop);
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
                    (CILRoot::SourceFileInfo { .. }, CILRoot::SourceFileInfo { .. }) => *curr = nop,
                    // If a rethrow is followed by a rethrow, this is effectively just a single rethrow
                    (CILRoot::ReThrow, CILRoot::ReThrow) => *curr = nop,
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

    fn opt_roots(
        &mut self,
        fuel: &mut OptFuel,
        cache: &mut SideEffectInfoCache,
        asm: &mut Assembly,
    ) {
        if self.implementation().blocks().is_none() {
            return;
        }
        // TODO: this is a hack, which makes root inlining optimizations not consume fuel.
        let fuel = std::sync::Mutex::new(&mut *fuel);
        let locals = self.locals().map(|locs| locs.to_vec()).unwrap();
        let mut cache2 = SideEffectInfoCache::default();
        self.map_roots(
            asm,
            &mut |root, asm| {
                let mut root_fuel = fuel.lock().unwrap();
                root_opt(root, asm, &mut root_fuel, cache, &locals)
            },
            &mut |node, asm| {
                let mut fuel = fuel.lock().unwrap();
                opt_node::opt_node(node, asm, *fuel, &mut cache2)
            },
        );
    }
}

#[must_use]
pub fn is_branch_unconditional(branch: &(u32, u32, Option<BranchCond>)) -> bool {
    branch.2.is_none()
}
pub fn blockid_from_jump(target: u32, sub_target: u32) -> u32 {
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
    use crate::{asm::ILASM_FLAVOUR, cilnode::MethodKind, il_exporter::ILExporter};
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
        crate::Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        mimpl,
        vec![None, None],
    ));
    let name = asm.alloc_string("entrypoint");
    asm.new_method(MethodDef::new(
        crate::Access::Extern,
        main_module,
        name,
        sig,
        MethodKind::Static,
        MethodImpl::Missing,
        vec![None, None],
    ));
    let mut fuel = OptFuel::new(77);

    asm.opt(&mut fuel);
    #[cfg(not(miri))]
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
    use crate::BinOp;
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
    let sig = asm.sig([], Type::Void);
    block.local_opt(&mut asm, &[(None, isize_tpe)], &mut cache, &mut fuel, sig);
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
    use crate::BinOp;
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
fn is_linearizable(blocks: &[BasicBlock], asm: &Assembly) -> bool {
    // 1. This optimization *only* works if no handlers present.
    if blocks.iter().any(|block| block.handler().is_some()) {
        return false;
    }
    // 2. This optimization only works if all blocks jump unconditionaly.
    if blocks
        .iter()
        .flat_map(|block| block.roots().iter())
        .any(|root| match &asm[*root] {
            CILRoot::Branch(info) => !is_branch_unconditional(info),
            CILRoot::ExitSpecialRegion { .. } => true,
            _ => false,
        })
    {
        return false;
    }
    true
}
/// Replaces a sequence of blocks with unconditional jumps with a single block.
fn linearize_blocks(blocks: &[BasicBlock], asm: &Assembly) -> Option<BasicBlock> {
    if !is_linearizable(blocks, asm) {
        return None;
    }
    let mut res = Vec::new();
    let mut curr_block = &blocks[0];
    let mut should_countinue = true;
    // Cap the optimization to prevent infinite loops from causing issues
    let mut ic = 0;
    while should_countinue {
        should_countinue = false;
        for root in curr_block.roots() {
            ic += 1;
            if ic > 2000 {
                return None;
            }
            match &asm[*root] {
                CILRoot::Branch(info) => {
                    let block = block_with_id(blocks, blockid_from_jump(info.0, info.1))?;
                    curr_block = block;
                    should_countinue = true;
                    break;
                }
                _ => res.push(*root),
            }
        }
    }
    Some(BasicBlock::new(res, blocks[0].block_id(), None))
}
#[allow(dead_code)]
fn linearilze_best_span(blocks: &mut [BasicBlock], asm: &Assembly) {
    let mut best_score = 1;
    let mut best_span_start = 0;
    let mut best_block = None;
    for s in 0..(blocks.len()) {
        for e in (s + best_score)..(blocks.len()) {
            let score = e - s;
            let Some(block) = linearize_blocks(&blocks[s..=e], asm) else {
                continue;
            };
            best_block = Some(block);
            best_score = score;
            best_span_start = s;
        }
    }
    let Some(best_block) = best_block else { return };
    blocks[best_span_start] = best_block;
}
