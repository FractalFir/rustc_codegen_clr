use crate::{
    access_modifier::AccessModifer, cil_op::CILOp, function_sig::FnSig, r#type::Type, IString,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
/// Represenation of a CIL method.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Method {
    access: AccessModifer,
    sig: FnSig,
    name: IString,
    locals: Vec<Type>,
    ops: Vec<CILOp>,
    attributes: Vec<Attribute>,
}
impl Eq for Method {}
impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sig.hash(state);
        self.name.hash(state);
    }
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Attribute {
    EntryPoint,
}
impl Method {
    pub fn new(access: AccessModifer, sig: FnSig, name: &str, locals: Vec<Type>) -> Self {
        Self {
            access,
            sig,
            name: name.into(),
            locals,
            ops: Vec::new(),
            attributes: Vec::new(),
        }
    }
    pub fn is_entrypoint(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| *attr == Attribute::EntryPoint)
    }
    pub fn ops_mut(&mut self) -> &mut Vec<CILOp> {
        &mut self.ops
    }
    pub fn access(&self) -> AccessModifer {
        self.access
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn sig(&self) -> &FnSig {
        &self.sig
    }
    pub fn locals(&self) -> &[Type] {
        &self.locals
    }
    pub(crate) fn set_ops(&mut self, ops: Vec<CILOp>) {
        self.ops = ops;
    }
    pub(crate) fn get_ops(&self) -> &[CILOp] {
        &self.ops
    }
    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }
    pub fn set_locals(&mut self, locals: impl Into<Vec<Type>>) {
        self.locals = locals.into();
    }
}
