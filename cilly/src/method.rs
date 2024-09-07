use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call_site::CallSite,
    cil_iter::{CILIterElem, CILIterTrait},
    cil_iter_mut::CILIterElemMut,
    cil_node::CILNode,
    cil_root::CILRoot,
    cil_tree::CILTree,
    static_field_desc::StaticFieldDescriptor,
    v2::Assembly,
    FnSig, IString, Type,
};

/// Represenation of a CIL method.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Method {
    access: AccessModifer,
    method_type: MethodType,
    sig: FnSig,
    name: IString,
    locals: Vec<LocalDef>,
    blocks: Vec<BasicBlock>,
    attributes: Vec<Attribute>,
    arg_names: Vec<Option<IString>>,
}

/// Local varaible. Consists of an optional name and type.
pub type LocalDef = (Option<IString>, Type);
impl Eq for Method {}
impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sig.hash(state);
        self.name.hash(state);
    }
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
/// Method attribute.
pub enum Attribute {
    /// Set if the function is the assemblys entrypoint.
    EntryPoint,
    /// This method is nothing more than an alias for another method.
    AliasFor(Box<CallSite>),
}

impl Attribute {
    pub fn as_alias_for(&self) -> Option<&CallSite> {
        if let Self::AliasFor(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Method {
    pub fn alias_for(
        access: AccessModifer,
        method_type: MethodType,
        name: IString,
        alias_for: CallSite,
    ) -> Self {
        Self {
            access,
            method_type,
            sig: alias_for.signature().clone(),
            name,
            locals: vec![],
            blocks: vec![],
            attributes: vec![Attribute::AliasFor(Box::new(alias_for))],
            arg_names: vec![],
        }
    }
    pub fn maxstack(&self) -> usize {
        let trees = self.blocks().iter().flat_map(|block| block.trees());
        let max = trees.map(|tree| tree.root().into_iter().count() + 3).max();
        max.unwrap_or(6)
    }
    pub fn is_address_taken(&self, local: u32) -> bool {
        self.iter_cil()
            .nodes()
            .any(|node| matches!(node, CILNode::LDLocA(loc) if *loc == local))
    }
    /// For a `local`, returns the *values* of all its assigements. This *will not include the root, only the nodes*!
    /// WARNING: this ONLY works on "finalized" CIL, and DOES NOT SUPPORT SUBTREES
    pub fn local_sets(&self, local: u32) -> impl Iterator<Item = &CILNode> {
        {
            let this = &self;
            this.blocks()
                .iter()
                .flat_map(|block| block.iter_tree_roots())
        }
        .filter_map(move |root| match root {
            CILRoot::STLoc { local: loc, tree } if (*loc == local) => Some(tree),
            _ => None,
        })
    }
    pub fn direct_set_count(&self, local: u32) -> usize {
        self.local_sets(local).count()
    }
    pub fn const_opt_pass(&mut self) {
        use crate::cil_iter_mut::CILIterMutTrait;
        // If a local is set only once, and its address is never taken, it is likely to be const
        // TODO: this is inefficient Consider checking all locals at once?
        let luo = LocalUsageInfo::from_method(self);
        let locals_address_not_taken: Box<[_]> = (0..(self.locals().len()))
            .filter(|idx| !luo.is_address_taken(*idx))
            .collect();

        for local in locals_address_not_taken {
            let sets = self.local_sets(local as u32);
            if let Some(val) = all_evals_identical(sets) {
                let mut tmp: Vec<_> = self
                    .blocks
                    .iter_mut()
                    .flat_map(|block| block.all_trees_mut())
                    .map(|tree| tree.root_mut())
                    .collect();

                tmp.iter_mut()
                    .flat_map(|tree| tree.deref_mut().into_iter().nodes())
                    .for_each(|node| match node {
                        CILNode::LDLoc(loc) if *loc == local as u32 => *node = val.clone(),
                        CILNode::LDLocA(loc) if *loc == local as u32 => {
                            panic!("const propagation failed: the address of const taken")
                        }
                        _ => (),
                    });
            }
        }
    }

    /// Iterates over each `CILNode` and `CILRoot`.
    pub fn iter_cil(&self) -> impl Iterator<Item = CILIterElem> {
        self.blocks().iter().flat_map(|block| block.iter_cil())
    }
    /// Reallocates the local variables, removing any dead ones.
    pub fn realloc_locals(&mut self) {
        let blocks = &mut self.blocks;
        let mut locals = Vec::new();
        let mut local_map: FxHashMap<u32, u32> = FxHashMap::with_hasher(FxBuildHasher::default());
        let mut tmp: Vec<_> = blocks
            .iter_mut()
            .flat_map(|block| block.tree_iter())
            .map(|tree| tree.root_mut())
            .collect();
        tmp.iter_mut()
            .flat_map(|root| root.deref_mut().into_iter())
            .for_each(|node| match node {
                CILIterElemMut::Node(CILNode::LDLoc(loc) | CILNode::LDLocA(loc))
                | CILIterElemMut::Root(CILRoot::STLoc { local: loc, .. }) => {
                    if let Some(new_loc) = local_map.get(loc) {
                        *loc = *new_loc;
                    } else {
                        let new_loc = locals.len() as u32;
                        locals.push(self.locals[*loc as usize].clone());
                        local_map.insert(*loc, new_loc);
                        *loc = new_loc;
                    }
                }

                _ => (),
            });
        self.locals = locals;
    }

    /// Creates a new method with name `name`, signature `sig`, accessibility of `access`, and consists of `blocks` basic blocks.
    #[must_use]
    pub fn new(
        access: AccessModifer,
        method_type: MethodType,
        sig: FnSig,
        name: &str,
        mut locals: Vec<LocalDef>,
        blocks: Vec<BasicBlock>,
        mut arg_names: Vec<Option<IString>>,
    ) -> Self {
        let mut used_names = FxHashSet::with_hasher(FxBuildHasher::default());
        for name in arg_names
            .iter_mut()
            .chain(locals.iter_mut().map(|loc| &mut loc.0))
            .filter_map(|name| name.as_mut())
        {
            let mut postfix = 0;
            while used_names.contains(&if postfix == 0 {
                name.clone()
            } else {
                format!("{name}{postfix}").into()
            }) {
                postfix += 1;
            }
            if postfix != 0 {
                *name = format!("{name}{postfix}").into();
            }
            used_names.insert(name.clone());
        }
        let mut res = Self {
            access,
            method_type,
            sig,
            name: name.to_owned().into(),
            locals,
            blocks,
            attributes: Vec::new(),
            arg_names,
        };
        res.allocate_temporaries();
        res.sheed_trees();

        res
    }
    /// Sets the name of this method.
    pub fn set_name(&mut self, name: &str) {
        self.name = <IString>::from(name.to_owned());
    }
    /// Adds a local variable of type `local`
    pub fn add_local(&mut self, local: Type, name: Option<IString>) -> usize {
        let loc = self.locals.len();
        self.locals.push((name, local));
        loc
    }
    /// Extends local variables by `iter`.
    pub fn extend_locals<'a>(&mut self, iter: impl Iterator<Item = &'a Type>) {
        self.locals.extend(iter.map(|tpe| (None, *tpe)));
    }
    /// Checks if the method `self` is the entrypoint.
    #[must_use]
    pub fn is_entrypoint(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| *attr == Attribute::EntryPoint)
    }
    /// A list of function inputs, in a CIL compatible format. Does not include the implict `this` parameter for instance and virtual methods.
    pub fn explicit_inputs(&self) -> &[Type] {
        if self.is_static() {
            self.sig().inputs()
        } else {
            &self.sig().inputs()[1..]
        }
    }
    pub fn sheed_trees(&mut self) {
        self.blocks.iter_mut().for_each(|block| {
            block.sheed_trees();
        });
    }
    /// Returns the access modifier of this function.
    #[must_use]
    pub fn access(&self) -> AccessModifer {
        self.access
    }
    /// Returns true if this function is static, else it returns false.
    #[must_use]
    pub fn is_static(&self) -> bool {
        self.method_type == MethodType::Static
    }
    /// Returns the name of this function.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the signature of `self`.
    #[must_use]
    pub fn sig(&self) -> &FnSig {
        &self.sig
    }
    /// Returns the list of local types.
    #[must_use]
    pub fn locals(&self) -> &[(Option<IString>, Type)] {
        &self.locals
    }
    /// Returns the list of external calls this function preforms. Calls may repeat.
    // TODO: make this not call `into_ops`
    pub fn calls(&self) -> impl Iterator<Item = &CallSite> {
        self.blocks
            .iter()
            .flat_map(|block| block.iter_cil())
            .call_sites()
    }
    /// Returns the list of static fields this function references. Calls may repeat.
    // TODO: make this not call `into_ops`
    pub fn sflds(&self) -> impl Iterator<Item = &StaticFieldDescriptor> {
        self.blocks
            .iter()
            .flat_map(|block| block.iter_cil())
            .filter_map(|node| match node {
                CILIterElem::Node(CILNode::LDStaticField(field)) => Some(field.as_ref()),
                CILIterElem::Node(CILNode::AddressOfStaticField(field)) => Some(field.as_ref()),
                CILIterElem::Root(CILRoot::SetStaticField { descr, value: _ }) => Some(descr),
                _ => None,
            })
    }

    /// Returns a call site that describes this method.
    pub fn call_site(&self) -> CallSite {
        CallSite::new(
            None,
            self.name().to_owned().into(),
            self.sig().clone(),
            self.is_static(),
        )
    }
    /// Alocates all temporary variables within this method.
    pub fn allocate_temporaries(&mut self) {
        self.blocks
            .iter_mut()
            .flat_map(BasicBlock::trees_mut)
            .for_each(|tree| tree.allocate_tmps(&mut self.locals));
    }
    /// Adds method attribute `attr` to self.
    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }
    /// Sets the list of locals of self to `locals`.
    pub fn set_locals(&mut self, locals: impl Into<Vec<(Option<IString>, Type)>>) {
        self.locals = locals.into();
    }
    /// Returns the type of this method(static, instance or virtual)
    #[must_use]
    pub fn method_type(&self) -> MethodType {
        self.method_type
    }
    /// Returns a reference to a list of basic blocks that make up this method.
    #[must_use]
    pub fn blocks(&self) -> &[BasicBlock] {
        &self.blocks
    }
    /// Returns a mutable reference to a list of basic block that make up this method.
    pub fn blocks_mut(&mut self) -> BlockMutGuard<'_> {
        BlockMutGuard { method: self }
    }

    #[must_use]
    pub fn arg_names(&self) -> &[Option<IString>] {
        &self.arg_names
    }

    pub fn new_bb(&mut self) -> u32 {
        let new_bb = self.blocks.len() as u32;
        self.blocks.push(BasicBlock::new(vec![], new_bb, None));
        new_bb
    }
    pub fn append_preamble(&mut self, tree: CILTree) {
        let trees = self.blocks.iter_mut().next().unwrap().trees_mut();
        trees.insert(0, tree);
    }
    pub fn alloc_local(&mut self, tpe: Type, name: Option<IString>) -> usize {
        let new_loc = self.locals.len();
        self.locals.push((name, tpe));
        new_loc
    }
    pub fn adjust_aligement(&mut self, adjust: Vec<Option<u64>>, asm: &mut Assembly) {
        if !adjust.iter().any(|adjust| adjust.is_some()) {
            return;
        }
        for (unaligned_local, align) in adjust
            .iter()
            .enumerate()
            .filter_map(|(local_id, o)| o.as_ref().map(|align| (local_id, *align)))
        {
            let (name, tpe) = &self.locals[unaligned_local];
            let unaligned_local = unaligned_local as u32;
            let (name, tpe) = (name.clone(), *tpe);
            let new_loc = self.alloc_local(asm.nptr(tpe), name.clone()) as u32;
            self.append_preamble(
                CILRoot::STLoc {
                    local: new_loc,
                    tree: CILNode::LocAllocAligned {
                        tpe: Box::new(tpe),
                        align,
                    },
                }
                .into(),
            );
            let mut tmp: Vec<_> = self
                .blocks
                .iter_mut()
                .flat_map(|block| block.tree_iter())
                .map(|tree| tree.root_mut())
                .collect();
            tmp.iter_mut()
                .flat_map(|root| root.deref_mut().into_iter())
                .for_each(|node| match node {
                    CILIterElemMut::Root(root) => {
                        if let CILRoot::STLoc { local, tree } = root {
                            if *local == unaligned_local {
                                // We replace seting *a* with an indirect wirte to allocation pointed to by *b*.
                                *root = CILRoot::STObj {
                                    addr_calc: Box::new(CILNode::LDLoc(new_loc)),
                                    value_calc: Box::new(tree.clone()),
                                    tpe: Box::new(tpe),
                                };
                            }
                        }
                    }
                    CILIterElemMut::Node(node) => match node {
                        CILNode::LDLocA(local) => {
                            if *local == unaligned_local {
                                // We replace getting the adress of *a* with loading the pointer *b*, which points to our aligned local
                                *node = CILNode::LDLoc(new_loc);
                            }
                        }
                        CILNode::LDLoc(local) => {
                            if *local == unaligned_local {
                                // We replace getting the value of *a* with a read of the value *b* points to.
                                *node = CILNode::LdObj {
                                    ptr: Box::new(CILNode::LDLoc(new_loc)),
                                    obj: Box::new(tpe),
                                };
                            }
                        }
                        _ => (),
                    },
                });
        }
    }

    pub fn opt(&mut self) {
        for tree in self
            .blocks
            .iter_mut()
            .flat_map(|block| block.all_trees_mut())
        {
            let mut opt_counter: usize = 1;
            while opt_counter > 0 {
                // Reset `opt_counter`
                opt_counter = 0;
                tree.opt(&mut opt_counter);
            }
        }
        //self.const_opt_pass();
        self.opt_merge_bbs();
    }
    pub fn opt_merge_bbs(&mut self) {
        for block in 0..self.blocks().len() {
            // Get the last uncond jump, if present
            let Some(target_id) = self.blocks[block].final_uncond_jump() else {
                continue;
            };
            let target_index = self.block_with_id(target_id).unwrap();
            // Check if this is the only block jumping to target. If target_id is 0, then the entrypoint "jumps" to that target, so we are not the only ones jumping there.
            // We also can't optimize if we are jumping to ourselves
            if target_id == 0 || target_index == block {
                continue;
            }
            if self.count_jumps_to(target_id) > 1 {
                continue;
            }

            // Since only we jump to this block, we can append the target block at the end of this block, if our handlers match
            if self.blocks()[block].handler() == self.blocks()[target_index].handler() {
                // Remove the last unconditional jump
                self.blocks[block].trees_mut().pop();
                // Append the block
                let cloned = self.blocks[target_index].trees().to_vec();
                self.blocks[block].trees_mut().extend(cloned);
                // We empty out the now unnedded block
                *self.blocks[target_index].trees_mut() = vec![];
                // 6.5
            }
        }
        // Remove unneded blocks
        // let prev_c = self.blocks.len();
        self.blocks.retain(|block| !block.trees().is_empty());
    }
    fn count_jumps_to(&self, block_id: u32) -> usize {
        self.blocks()
            .iter()
            .flat_map(|block| block.targets())
            .filter(|(target, sub_target)| {
                if *sub_target != 0 {
                    *sub_target == block_id
                } else {
                    *target == block_id
                }
            })
            .count()
    }
    pub fn block_with_id(&self, id: u32) -> Option<usize> {
        self.blocks.iter().position(|block| block.id() == id)
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

/// A wrapper around mutably borrowed [`BasicBlock`]s of a method. Prevents certain bugs.
pub struct BlockMutGuard<'a> {
    method: &'a mut Method,
}
impl<'a> Drop for BlockMutGuard<'a> {
    fn drop(&mut self) {
        /*self.method.blocks.iter_mut().for_each(|block| {
            block
                .trees_mut()
                .retain(|tree| !matches!(tree.root(), CILRoot::Nop));
        });*/
        //self.method.allocate_temporaries();
        //self.method.sheed_trees();
    }
}
impl<'a> DerefMut for BlockMutGuard<'a> {
    fn deref_mut(&mut self) -> &mut Vec<BasicBlock> {
        &mut self.method.blocks
    }
}
impl<'a> Deref for BlockMutGuard<'a> {
    type Target = Vec<BasicBlock>;

    fn deref(&self) -> &Self::Target {
        &self.method.blocks
    }
}

/// Type of this method(static, instance or virtual).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MethodType {
    /// This method belongs to a type. Its first argument MUST be a referenece to that type!
    Instance,
    /// This is an instance method, and it depends on the exact type of the object it is called on.
    Virtual,
    /// A "normal" method.
    Static,
}

pub(crate) fn all_evals_identical<'a>(
    mut nodes: impl Iterator<Item = &'a CILNode>,
) -> Option<CILNode> {
    let first = nodes.next()?;
    let first_val = first.try_const_eval()?;
    if nodes.all(|node| {
        let Some(val) = node.try_const_eval() else {
            return false;
        };
        val == first_val
    }) {
        Some(first_val)
    } else {
        None
    }
}
struct LocalUsageInfo {
    is_address_taken: Box<[bool]>,
}

impl LocalUsageInfo {
    fn from_method(method: &Method) -> Self {
        let mut is_address_taken: Box<[_]> = vec![false; method.locals().len()].into();
        for node in method.iter_cil().nodes() {
            if let CILNode::LDLocA(loc) = node {
                is_address_taken[*loc as usize] = true
            }
        }
        Self { is_address_taken }
    }

    fn is_address_taken(&self, idx: usize) -> bool {
        self.is_address_taken[idx]
    }
}
