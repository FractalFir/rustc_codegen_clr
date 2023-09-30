use crate::{function_sig::FnSig, r#type::DotnetTypeRef, IString};
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct FieldDescriptor {
    owner: DotnetTypeRef,
    tpe: crate::r#type::Type,
    name: IString,
}
impl FieldDescriptor {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn tpe(&self) -> &crate::r#type::Type {
        &self.tpe
    }
    pub fn owner(&self) -> &DotnetTypeRef {
        &self.owner
    }
    pub fn new(owner: DotnetTypeRef, tpe: crate::r#type::Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    pub fn boxed(owner: DotnetTypeRef, tpe: crate::r#type::Type, name: IString) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct StaticFieldDescriptor {
    owner: Option<DotnetTypeRef>,
    tpe: crate::r#type::Type,
    name: IString,
}
impl StaticFieldDescriptor {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn tpe(&self) -> &crate::r#type::Type {
        &self.tpe
    }
    pub fn owner(&self) -> Option<&DotnetTypeRef> {
        self.owner.as_ref()
    }
    pub fn new(owner: Option<DotnetTypeRef>, tpe: crate::r#type::Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    pub fn boxed(
        owner: Option<DotnetTypeRef>,
        tpe: crate::r#type::Type,
        name: IString,
    ) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
/// Represenation of a target of a call.
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct CallSite {
    class: Option<DotnetTypeRef>,
    name: IString,
    signature: FnSig,
    is_static: bool,
}
impl CallSite {
    pub fn new(
        class: Option<DotnetTypeRef>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Self {
        Self {
            class,
            name,
            signature,
            is_static,
        }
    }
    pub fn boxed(
        class: Option<DotnetTypeRef>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Box<Self> {
        Box::new(Self::new(class, name, signature, is_static))
    }
    pub fn signature(&self) -> &FnSig {
        &self.signature
    }
    pub fn class(&self) -> Option<&DotnetTypeRef> {
        self.class.as_ref()
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    // Returns true if a call is equivalent to a No-Op. Used to handle black_box.
    pub fn is_nop(&self) -> bool {
        if !self.is_static() {
            return false;
        }
        if self.class().is_some() {
            return false;
        };
        if self.name.as_ref() != "black_box" {
            return false;
        };
        if self.signature.inputs().len() != 1 {
            return false;
        };
        if self.signature.inputs()[0] != *self.signature.output() {
            return false;
        };
        true
    }
}
use serde::{Deserialize, Serialize};
/// Represenation of a CIL opcode.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum CILOp {
    // Control Flow
    Label(u32),
    GoTo(u32),
    BEq(u32),
    BLt(u32),
    BGe(u32),
    BZero(u32),
    Call(Box<CallSite>),
    Throw,
    Ret,
    // Load/Store/AdressOf local
    LDLoc(u32),
    LDArg(u32),
    STLoc(u32),
    STArg(u32),
    LDLocA(u32),
    LDArgA(u32),
    // Load constant sigined intieger
    LdcI32(i32),
    LdcI64(i64),
    // Load constant float
    LdcF32(f32),
    LdcF64(f64),
    // Load string literal
    LdStr(IString),
    // Signed intieger convertions
    ConvI8(bool),
    ConvI16(bool),
    ConvI32(bool),
    ConvI64(bool),
    ConvISize(bool),
    // Unsigned intieger convertions
    ConvU8(bool),
    ConvU16(bool),
    ConvU32(bool),
    ConvU64(bool),
    ConvUSize(bool),
    // Float convertions
    ConvF32(bool),
    ConvF64(bool),
    // Pointer
    LDIndI8,
    LDIndI16,
    LDIndI32,
    LDIndI64,
    LDIndISize,
    LDIndRef,
    STIndI8,
    STIndI16,
    STIndI32,
    STIndI64,
    STIndISize,
    //Debugging
    Comment(IString),
    // Arthmetic Operations
    Add,
    And,
    Div,
    Rem,
    Shr,
    Shl,
    Sub,
    Mul,
    Or,
    XOr,
    Not,
    Neg,
    // Comparisons
    Eq,
    Lt,
    Gt,
    //Special
    Pop,
    Dup,
    Nop,
    LocAlloc,
    //OOP
    NewObj(Box<CallSite>),
    LDField(Box<FieldDescriptor>),
    LDFieldAdress(Box<FieldDescriptor>),
    STField(Box<FieldDescriptor>),
    LdObj(Box<DotnetTypeRef>),
    STObj(Box<DotnetTypeRef>),
    SizeOf(Box<crate::r#type::Type>),
    LDStaticField(Box<StaticFieldDescriptor>),
}
