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
    /// All inputs. Includes impilcit `this` argument for instance functions.
    pub fn inputs(&self) -> &[crate::r#type::Type] {
        &self.signature.inputs()
    }
    /// Inputs, with the implicit `this` skipped if needed.
    pub fn explicit_inputs(&self) -> &[crate::r#type::Type] {
        if self.is_static || self.inputs().is_empty() {
            &self.signature.inputs()
        } else {
            &self.signature.inputs()[1..]
        }
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
    BNe(u32),
    BLt(u32),
    BGe(u32),
    BZero(u32),
    Call(Box<CallSite>),
    CallVirt(Box<CallSite>),
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
    LDIndF32,
    LDIndF64,
    LDIndRef,
    STIndI8,
    STIndI16,
    STIndI32,
    STIndI64,
    STIndISize,
    STIndF32,
    STIndF64,
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
    LdObj(Box<crate::r#type::Type>),
    STObj(Box<crate::r#type::Type>),
    SizeOf(Box<crate::r#type::Type>),
    LDStaticField(Box<StaticFieldDescriptor>),
}
impl CILOp {
    /// Descirbes the difference in stack size before and after the op.
    pub fn stack_diff(&self) -> isize {
        match self {
            CILOp::Nop => 0,
            CILOp::Comment(_) => 0,
            CILOp::Label(_) | CILOp::GoTo(_) => 0,
            CILOp::BZero(_) => -1,
            CILOp::BEq(_) | CILOp::BNe(_) | CILOp::BLt(_) | CILOp::BGe(_) => -1,
            CILOp::LDArg(_) | CILOp::LDArgA(_) | CILOp::LDLoc(_) | CILOp::LDLocA(_) => 1,
            CILOp::LdcI32(_)
            | CILOp::LdcI64(_)
            | CILOp::LdcF32(_)
            | CILOp::LdcF64(_)
            | CILOp::LdStr(_)
            | CILOp::SizeOf(_) => 1,
            CILOp::ConvI8(_)
            | CILOp::ConvI16(_)
            | CILOp::ConvI32(_)
            | CILOp::ConvI64(_)
            | CILOp::ConvISize(_)
            | CILOp::ConvU8(_)
            | CILOp::ConvU16(_)
            | CILOp::ConvU32(_)
            | CILOp::ConvU64(_)
            | CILOp::ConvUSize(_)
            | CILOp::ConvF32(_)
            | CILOp::ConvF64(_) => 0,
            CILOp::LDIndI8
            | CILOp::LDIndI16
            | CILOp::LDIndI32
            | CILOp::LDIndI64
            | CILOp::LDIndISize
            | CILOp::LDIndF32
            | CILOp::LDIndF64
            | CILOp::LDIndRef => 0,
            CILOp::STIndI8
            | CILOp::STIndI16
            | CILOp::STIndI32
            | CILOp::STIndI64
            | CILOp::STIndISize
            | CILOp::STIndF32
            | CILOp::STIndF64 => -2,
            CILOp::Pop => -1,
            CILOp::Dup => 1,
            CILOp::LDField(_) | CILOp::LDFieldAdress(_) => 0,
            CILOp::LocAlloc | CILOp::NewObj(_) => 1,
            CILOp::LdObj(_) => 1,
            CILOp::LDStaticField(_) => 1,
            CILOp::STObj(_) => -2,
            CILOp::STField(_) => -2,
            CILOp::Add
            | CILOp::And
            | CILOp::Div
            | CILOp::Rem
            | CILOp::Shr
            | CILOp::Shl
            | CILOp::Sub
            | CILOp::Mul
            | CILOp::Or
            | CILOp::XOr
            | CILOp::Eq
            | CILOp::Lt
            | CILOp::Gt => -1,
            CILOp::Not | CILOp::Neg => 0,
            CILOp::STLoc(_) | CILOp::STArg(_) => -1,
            CILOp::Call(site) | CILOp::CallVirt(site) => {
                if *site.signature().output() == crate::r#type::Type::Void {
                    -(site.signature().inputs().len() as isize)
                } else {
                    1 - (site.signature().inputs().len() as isize)
                }
            }
            CILOp::Throw => -1,
            CILOp::Ret => -1,
        }
    }
    pub fn get_op_arg_pos(ops: &[CILOp], pos: usize, arg: usize) -> Option<usize> {
        todo!();
        /*
        let mut depth = (arg + 1) as isize;
        while depth > 0{
            if pos == 0 || pos > ops.len(){
                return None;
            }
        }*/
    }
    /// Flips a conditional, changing the order of its arguments. Eg. BLt(a,b) [a < b] becomes BGe(b,a) [b >= a].
    pub fn flip_cond(&self) -> Self {
        match self{
                CILOp::BGe(target) =>{
                    CILOp::BLt(*target)
                }
                CILOp::BLt(target) =>{
                    CILOp::BGe(*target)
                }
                _=>todo!("Can't filp conditional operation {self:?}, either because it is not a conditional(bug) or it is not supported yet!"),
            }
    }
}
