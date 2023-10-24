use crate::{
    access_modifier::AccessModifer, cil_op::CILOp, function_sig::FnSig, r#type::Type, IString,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
/// Represenation of a CIL method.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Method {
    access: AccessModifer,
    is_static: bool,
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
    pub fn new(
        access: AccessModifer,
        is_static: bool,
        sig: FnSig,
        name: &str,
        locals: Vec<Type>,
    ) -> Self {
        Self {
            access,
            is_static,
            sig,
            name: name.into(),
            locals,
            ops: Vec::new(),
            attributes: Vec::new(),
        }
    }
    pub fn ensure_valid(&mut self) {
        if let Some(CILOp::Ret) = self.ops.iter().last() {
            //Do nothing
        } else {
            self.ops.push(CILOp::Ret);
        }
    }
    pub fn is_entrypoint(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| *attr == Attribute::EntryPoint)
    }
    pub fn explicit_inputs(&self) -> &[Type] {
        if self.is_static() {
            self.sig().inputs()
        } else {
            &self.sig().inputs()[1..]
        }
    }
    pub fn ops_mut(&mut self) -> &mut Vec<CILOp> {
        &mut self.ops
    }
    pub fn access(&self) -> AccessModifer {
        self.access
    }
    pub fn is_static(&self) -> bool {
        self.is_static
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
    pub(crate) fn allocate_temporaries(&mut self) {
        let mut tmp_stack = vec![];
        let ops = &mut self.ops;
        for op in ops {
            match op {
                CILOp::NewTMPLocal(tpe) => {
                    let index = self.locals.len();
                    self.locals.push(tpe.as_ref().clone());
                    tmp_stack.push(index);
                    *op = CILOp::Nop;
                }
                CILOp::FreeTMPLocal => {
                    tmp_stack
                        .pop()
                        .expect("Freeing TMP local when none existed");
                    *op = CILOp::Nop;
                }
                CILOp::LoadTMPLocal => {
                    *op = CILOp::LDLoc(*tmp_stack.iter().last().expect(
                        "Using a TMP local with `LoadTMPLocal` when no TMP local allocated!",
                    ) as u32);
                }
                CILOp::LoadUnderTMPLocal(under) => {
                    println!("tmp_stack:{tmp_stack:?} under:{under}");
                    *op = CILOp::LDLoc(tmp_stack[(tmp_stack.len() - 1) - (*under as usize)] as u32);
                }
                CILOp::LoadAddresOfTMPLocal => {
                    *op = CILOp::LDLocA(*tmp_stack.iter().last().expect(
                        "Using a TMP local with `LoadTMPLocal` when no TMP local allocated!",
                    ) as u32);
                }
                CILOp::SetTMPLocal => {
                    *op = CILOp::STLoc(*tmp_stack.iter().last().expect(
                        "Using a TMP local with `LoadTMPLocal` when no TMP local allocated!",
                    ) as u32);
                }
                _ => (),
            }
        }
        //todo!("Can't allocate temporaries quite yet!");
    }
    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }
    pub fn set_locals(&mut self, locals: impl Into<Vec<Type>>) {
        self.locals = locals.into();
    }
}
