use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    hash::Hasher,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call_site::CallSite,
    cil_iter::{CILIterElem, CILIterTrait},
    cil_iter_mut::CILIterElemMut,
    cil_node::{CILNode, ValidationContext},
    cil_root::CILRoot,
    cil_tree::CILTree,
    ilasm_op::{non_void_type_cil, type_cil, DepthSetting},
    static_field_desc::StaticFieldDescriptor,
    DotnetTypeRef, FnSig, IString, IlasmFlavour, Type,
};

/// Represenation of a CIL method.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Method {
    access: AccessModifer,
    method_type: MethodType,
    sig: FnSig,
    name: IString,
    locals: Vec<LocalDef>,
    pub(in crate::method) blocks: Vec<BasicBlock>,
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
}

impl Method {
    pub fn maxstack(&self) -> usize {
        let trees = self.blocks().iter().flat_map(|block| block.trees());
        let max = trees.map(|tree| tree.root().into_iter().count() + 3).max();
        max.unwrap_or(6)
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
        self.opt_merge_bbs();
    }
    /// Iterates over each `CILNode` and `CILRoot`.
    pub fn iter_cil(&self) -> impl Iterator<Item = CILIterElem> {
        self.blocks().iter().flat_map(|block| block.iter_cil())
    }
    /// Reallocates the local variables, removing any dead ones.
    pub fn realloc_locals(&mut self) {
        let blocks = &mut self.blocks;
        let mut locals = Vec::new();
        let mut local_map: HashMap<u32, u32> = HashMap::new();
        blocks
            .iter_mut()
            .flat_map(|block| block.iter_cil_mut())
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
    pub fn validate(&self) -> Result<(), String> {
        let errs: Vec<String> = self
            .blocks()
            .iter()
            .map(|tree| tree.validate(self.into()))
            .filter_map(|err| match err {
                Ok(()) => None,
                Err(err) => Some(err),
            })
            .collect::<Vec<_>>();
        if !errs.is_empty() {
            return Err(errs[0].clone());
        }
        Ok(())
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
        let mut used_names = HashSet::new();
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
            name: name.into(),
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
        self.name = name.into();
    }
    /// Adds a local variable of type `local`
    pub fn add_local(&mut self, local: Type, name: Option<IString>) -> usize {
        let loc = self.locals.len();
        self.locals.push((name, local.clone()));
        loc
    }
    /// Extends local variables by `iter`.
    pub fn extend_locals<'a>(&mut self, iter: impl Iterator<Item = &'a Type>) {
        self.locals.extend(iter.map(|tpe| (None, tpe.clone())));
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
    pub fn sflds(&self) -> Vec<&StaticFieldDescriptor> {
        self.blocks
            .iter()
            .flat_map(|block| block.iter_cil())
            .filter_map(|node| match node {
                CILIterElem::Node(CILNode::LDStaticField(field)) => Some(field.as_ref()),
                CILIterElem::Root(CILRoot::SetStaticField { descr, value: _ }) => Some(descr),
                _ => None,
            })
            .collect()
    }
    /// Returns a list of type references that are used within this method.
    pub fn dotnet_types(&self) -> Vec<DotnetTypeRef> {
        self.sig()
            .inputs()
            .iter()
            .filter_map(Type::dotnet_refs)
            .chain(self.locals().iter().filter_map(|tpe| tpe.1.dotnet_refs()))
            .chain(self.sig().inputs().iter().filter_map(Type::dotnet_refs))
            .chain(
                [self.sig().output()]
                    .iter()
                    .filter_map(|tpe| tpe.dotnet_refs()),
            )
            .chain(self.iter_cil().filter_map(|node| match node {
                CILIterElem::Node(CILNode::SizeOf(tpe)) => tpe.dotnet_refs(),
                CILIterElem::Node(CILNode::NewObj(call_op_args)) => {
                    call_op_args.site.class().cloned()
                }
                CILIterElem::Node(CILNode::LDField { addr, field }) => Some(field.owner().clone()),
                _ => None,
            }))
            .collect()
    }
    /// Returns a call site that describes this method.
    pub fn call_site(&self) -> CallSite {
        CallSite::new(None, self.name().into(), self.sig().clone(), true)
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
    pub fn adjust_aligement(&mut self, adjust: Vec<Option<u64>>) {
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
            let (name, tpe) = (name.clone(), tpe.clone());
            let new_loc = self.alloc_local(Type::Ptr(Box::new(tpe.clone())), name.clone()) as u32;
            self.append_preamble(
                CILRoot::STLoc {
                    local: new_loc,
                    tree: CILNode::LocAllocAligned {
                        tpe: Box::new(tpe.clone()),
                        align,
                    },
                }
                .into(),
            );
            self.blocks
                .iter_mut()
                .flat_map(|blck| blck.iter_cil_mut())
                .for_each(|node| match node {
                    CILIterElemMut::Root(root) => {
                        if let CILRoot::STLoc { local, tree } = root {
                            if *local == unaligned_local {
                                // We replace seting *a* with an indirect wirte to allocation pointed to by *b*.
                                *root = CILRoot::STObj {
                                    addr_calc: Box::new(CILNode::LDLoc(new_loc)),
                                    value_calc: Box::new(tree.clone()),
                                    tpe: Box::new(tpe.clone()),
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
                                    obj: Box::new(tpe.clone()),
                                };
                            }
                        }
                        _ => (),
                    },
                });
        }
    }
    pub fn export(
        &self,
        w: &mut impl std::fmt::Write,
        flavour: IlasmFlavour,
        init_locals: bool,
        print_stack_traces: bool,
    ) -> std::fmt::Result {
        let access = if let AccessModifer::Private = self.access() {
            "private"
        } else {
            "public"
        };
        let static_inst = match self.method_type() {
            MethodType::Static => "static",
            MethodType::Virtual => "virtual instance",
            MethodType::Instance => "instance",
        };
        let output = type_cil(self.sig().output());
        let name = self.name();
        write!(
            w,
            ".method {access} hidebysig {static_inst} {output} '{name}'("
        )?;

        let mut input_iter = self.explicit_inputs().iter();
        if self.arg_names().is_empty() || self.arg_names().len() != self.explicit_inputs().len() {
            if self.arg_names().len() != self.explicit_inputs().len() {
                println!("WARNING: debug arg count invalid!");
            }
            if let Some(input) = input_iter.next() {
                write!(w, "{}", non_void_type_cil(input))?;
            }
            for input in input_iter {
                write!(w, ",{}", non_void_type_cil(input))?;
            }
        } else {
            assert_eq!(self.arg_names().len(), self.explicit_inputs().len());
            let mut input_iter = self.explicit_inputs().iter().zip(self.arg_names().iter());
            if let Some((input, name)) = input_iter.next() {
                match name {
                    Some(name) => write!(w, "{} '{name}'", non_void_type_cil(input))?,
                    None => write!(w, "{}", non_void_type_cil(input))?,
                }
            }
            for (input, name) in input_iter {
                match name {
                    Some(name) => write!(w, ",{} '{name}'", non_void_type_cil(input))?,
                    None => write!(w, ",{}", non_void_type_cil(input))?,
                }
            }
        }
        writeln!(w, "){{")?;
        if self.is_entrypoint() {
            writeln!(w, ".entrypoint")?;
        }
        if init_locals {
            writeln!(w, "\t.locals init(")?;
        } else {
            writeln!(w, "\t.locals (")?;
        }
        let mut locals_iter = self.locals().iter().enumerate();
        if let Some((local_id, local)) = locals_iter.next() {
            match &local.0 {
                None => write!(
                    w,
                    "\t\t[{local_id}] {escaped_type}",
                    escaped_type = non_void_type_cil(&local.1)
                )?,
                Some(name) => write!(
                    w,
                    "\t\t[{local_id}] {escaped_type} '{name}'",
                    escaped_type = non_void_type_cil(&local.1)
                )?,
            }
        }
        for (local_id, local) in locals_iter {
            match &local.0 {
                None => write!(
                    w,
                    ",\n\t\t[{local_id}] {escaped_type}",
                    escaped_type = non_void_type_cil(&local.1)
                )?,
                Some(name) => write!(
                    w,
                    ",\n\t\t[{local_id}] {escaped_type} '{name}'",
                    escaped_type = non_void_type_cil(&local.1)
                )?,
            }
        }
        writeln!(
            w,
            "\n\t)\n.maxstack {maxstack}\n",
            maxstack = self.maxstack()
        )?;
        if print_stack_traces {
            write!(w,"call string [System.Runtime]System.Environment::get_StackTrace()\ncall void [System.Console]System.Console::WriteLine(string)")?;
        }
        for block in self.blocks().iter() {
            crate::basic_block::export(w, block, DepthSetting::with_pading(), flavour).unwrap();
        }

        writeln!(w, "}}")
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
        /*let new_c = self.blocks.len();
        eprintln!(
            "block opt result: removed {rem} out of {prev_c} blocks. Removed {prec}% of blocks.",
            rem = prev_c - new_c,
            prec = ((prev_c - new_c) as f64 / prev_c as f64) * 100.0
        );*/
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
impl<'a> Into<ValidationContext<'a>> for &'a Method {
    fn into(self) -> ValidationContext<'a> {
        ValidationContext::new(self.sig(), self.locals())
    }
}
