use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    cil::CallSite,
    function_sig::FnSig,
    r#type::{DotnetTypeRef, Type},
    IString,
};
use rustc_middle::ty::TyCtxt;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
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
    /// Creates a new method with name `name`, signature `sig`, accessibility of `access`, and consists of `blocks` basic blocks.
    #[must_use]
    pub fn new(
        access: AccessModifer,
        method_type: MethodType,
        sig: FnSig,
        name: &str,
        locals: Vec<LocalDef>,
        mut blocks: Vec<BasicBlock>,
    ) -> Self {
        blocks
            .iter_mut()
            .flat_map(|blck| blck.trees_mut().iter_mut())
            .for_each(|tree| tree.opt());
        let mut res = Self {
            access,
            method_type,
            sig,
            name: name.into(),
            locals,
            blocks,
            attributes: Vec::new(),
        };
        res.allocate_temporaries();
        res.blocks_mut().iter_mut().for_each(|bb| bb.sheed_trees());
        res
    }
    /// Calcualtes the maximum number of vairables on the evaulation stack.
    pub fn maxstack(&self) -> usize {
        crate::utilis::max_stack(
            self.blocks
                .iter()
                .flat_map(|bb| bb.into_ops())
                .collect::<Vec<_>>()
                .as_ref(),
            *self.sig().output() == Type::Void,
        ) + 10
    }
    /// Sets the name of this method.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    /// Adds a local variable of type `local`
    pub fn add_local(&mut self, local: Type) {
        self.locals.push((None, local.clone()));
    }
    /// Extends local variables by `iter`.
    pub fn extend_locals<'a>(&mut self, iter: impl Iterator<Item = &'a Type>) {
        self.locals.extend(iter.map(|tpe| (None, tpe.clone())));
    }
    /// Checks if the method `self` is the entrypoint.
    pub fn is_entrypoint(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| *attr == Attribute::EntryPoint)
    }
    /// A list of function inputs, in a CIL compatible format. Does not include the implict `this` parameter for instance and virtual methods.
    pub(crate) fn explicit_inputs(&self) -> &[Type] {
        if self.is_static() {
            self.sig().inputs()
        } else {
            &self.sig().inputs()[1..]
        }
    }
    /// Returns the access modifier of this function.
    pub fn access(&self) -> AccessModifer {
        self.access
    }
    /// Returns true if this function is static, else it returns false.
    pub fn is_static(&self) -> bool {
        self.method_type == MethodType::Static
    }
    /// Returns the name of this function.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the signature of `self`.
    pub fn sig(&self) -> &FnSig {
        &self.sig
    }
    /// Returns the list of local types.
    pub fn locals(&self) -> &[(Option<IString>, Type)] {
        &self.locals
    }
    /// Returns the list of external calls this function preforms. Calls may repeat.
    pub(crate) fn calls(&self) -> Vec<CallSite> {
        self.blocks
            .iter()
            .flat_map(|bb| bb.into_ops())
            .filter_map(|op| op.call().cloned())
            .collect()
    }
    /// Returns a list of type references that are used within this type.
    pub(crate) fn dotnet_types(&self) -> Vec<DotnetTypeRef> {
        self.sig()
            .inputs()
            .iter()
            .filter_map(|tpe| tpe.dotnet_refs())
            .chain(self.locals().iter().filter_map(|tpe| tpe.1.dotnet_refs()))
            .chain(
                self.sig()
                    .inputs()
                    .iter()
                    .filter_map(|tpe| tpe.dotnet_refs()),
            )
            .chain(
                [self.sig().output()]
                    .iter()
                    .filter_map(|tpe| tpe.dotnet_refs()),
            )
            .collect()
    }
    /// Returns a call site that describes this method.
    pub(crate) fn call_site(&self) -> CallSite {
        CallSite::new(None, self.name().into(), self.sig().clone(), true)
    }
    /// Alocates all temporary variables within this method.
    pub(crate) fn allocate_temporaries(&mut self) {
        self.blocks
            .iter_mut()
            .flat_map(|block| block.trees_mut())
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
    pub fn method_type(&self) -> MethodType {
        self.method_type
    }

    pub(crate) fn resolve_global_allocations(
        &mut self,
        arg: &mut crate::assembly::Assembly,
        tyctx: TyCtxt,
    ) {
        self.blocks
            .iter_mut()
            .flat_map(|block| block.trees_mut())
            .for_each(|tree| tree.resolve_global_allocations(arg, tyctx));
    }
    /// Returns a reference to a list of basic blocks that make up this method.
    pub fn blocks(&self) -> &[BasicBlock] {
        &self.blocks
    }
    /// Returns a mutable reference to a list of basic block that make up this method.
    pub fn blocks_mut(&mut self) -> &mut Vec<BasicBlock> {
        &mut self.blocks
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
