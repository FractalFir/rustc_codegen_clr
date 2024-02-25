use crate::{
    access_modifier::AccessModifer,
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    r#type::{DotnetTypeRef, Type},
    IString,
};
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
    ops: Vec<CILOp>,
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
    /// Creates new method with `access` access modifier, signature `sig`, name `name`, locals `locals`, and `is_static` if method is static.
    #[must_use]
    pub fn new(
        access: AccessModifer,
        method_type: MethodType,
        sig: FnSig,
        name: &str,
        locals: Vec<LocalDef>,
    ) -> Self {
        Self {
            access,
            method_type,
            sig,
            name: name.into(),
            locals,
            ops: Vec::new(),
            attributes: Vec::new(),
        }
    }
    pub(crate) fn ensure_valid(&mut self) {
        let last = self.ops.iter().last();
        let last = match last {
            Some(last) => last,
            None => return,
        };
        match last{
            CILOp::Ret=>(),
            CILOp::Throw=>(),
            CILOp::ReThrow=>(),
            CILOp::GoTo(_)=>(),
            _=>self.ops.extend(CILOp::throw_msg("Critical error: reached the end of a function not termianted with a return statement")),
        }
    }
    pub fn maxstack(&self) -> usize {
        crate::utilis::max_stack(&self.ops, *self.sig().output() == Type::Void) + 10
    }
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

    pub(crate) fn explicit_inputs(&self) -> &[Type] {
        if self.is_static() {
            self.sig().inputs()
        } else {
            &self.sig().inputs()[1..]
        }
    }
    /// Returns a mutable reference to this functions ops.
    pub fn ops_mut(&mut self) -> &mut Vec<CILOp> {
        &mut self.ops
    }
    pub fn ops_n_locals_mut(&mut self) -> (&mut Vec<CILOp>, &mut [(Option<IString>, Type)]) {
        (&mut self.ops, &mut self.locals)
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
    /// Sets this methods CIL ops to `ops`.
    pub fn set_ops(&mut self, ops: Vec<CILOp>) {
        self.ops = ops;
    }
    /// Returns the ops of this method.
    pub fn get_ops(&self) -> &[CILOp] {
        &self.ops
    }
    /// Returns the list of external calls this function preforms. Calls may repeat.
    pub(crate) fn calls(&self) -> impl Iterator<Item = &CallSite> {
        self.ops.iter().filter_map(|op| op.call())
    }
    pub(crate) fn dotnet_types<'a>(&'a self) -> Vec<DotnetTypeRef>{
        self.sig().inputs().iter().filter_map(|tpe|tpe.as_dotnet()).chain(
            self.locals().iter().filter_map(|tpe|tpe.1.as_dotnet())
        ).chain([self.sig().output()].iter().filter_map(|tpe|tpe.as_dotnet())).collect()
    }
    pub(crate) fn call_site(&self) -> CallSite {
        CallSite::new(None, self.name().into(), self.sig().clone(), true)
    }
    /*
    pub(crate) fn failed_to_compile(name:&str,reason:&str)->Self{
        Self:: new(AccessModifer::Public,true,)
    }*/
    pub(crate) fn allocate_temporaries(&mut self) {
        let mut tmp_stack = vec![];
        let ops = &mut self.ops;
        for op in ops {
            match op {
                CILOp::NewTMPLocal(tpe) => {
                    let index = self.locals.len();
                    self.locals.push((None, tpe.as_ref().clone()));
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
                    *op = CILOp::LDLoc(tmp_stack[(tmp_stack.len() - 1) - (*under as usize)] as u32);
                }
                CILOp::LoadAdressUnderTMPLocal(under) => {
                    *op =
                        CILOp::LDLocA(tmp_stack[(tmp_stack.len() - 1) - (*under as usize)] as u32);
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
}
/// Type of this method(static, instance or virtual).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MethodType {
    Instance,
    Virtual,
    Static,
}
