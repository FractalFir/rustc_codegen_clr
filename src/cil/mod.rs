use crate::{function_sig::FnSig, r#type::DotnetTypeRef, IString};
mod call_site;
pub use call_site::*;
mod field_desc;
pub use field_desc::*;
mod static_field_desc;
use serde::{Deserialize, Serialize};
pub use static_field_desc::*;
/// Represenation of a CIL opcode.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum CILOp {
    // Control Flow
    /// Lablel. Represents a position in code that can be jumped to. Does not translate to any real CIL ops, used only to calucalte jump targets.
    /// Should be placed automaticaly at the beiging of a basic block, and not constructed manualy.
    Label(u32, u32),
    /// Unconditional jump to a label with the specified id.
    GoTo(u32, u32),
    /// Jump to target if 2 top values on the stack are equal, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BEq(u32, u32),
    /// Jump to target if 2 top values on the stack are not equal, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BNe(u32, u32),
    /// Jump to target if the top value is less than the bottom one, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BLt(u32, u32),
    /// Jump to target if the top value is greater than or equal to the bottom one, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BGe(u32, u32),
    /// Jump to target if the top value is less than or equal to the bottom one, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BLe(u32, u32),
    /// Jump to target if the top value on the stack is zero, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BZero(u32, u32),
    /// Jump to target if the top value on the stack is zero, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BTrue(u32, u32),
    /// Call the metod behind `call_site`.`
    Call(Box<CallSite>),
    /// Call the virtual method behind `call_site`.`
    CallVirt(Box<CallSite>),
    /// Throw the top value on the stack as an exception
    Throw,
    /// Rethrow the current exception
    ReThrow,
    /// Return the top value on the stack from this function
    Ret,
    /// Debugger breakpoint
    Break,
    // Load/Store/AdressOf locals
    /// Load the local number `n` on top of the stack
    LDLoc(u32),
    /// Load the argument number `n` on top of the stack
    LDArg(u32),
    /// Set the local number `n` to the value poped from the stack
    STLoc(u32),
    /// Set the argument number `n`to the value poped from the stack
    STArg(u32),
    /// Load the adddres of local varible `n` on top of the stack
    LDLocA(u32),
    /// Load the adddres of argument `n` on top of the stack
    LDArgA(u32),

    // Load constant values.
    /// Load constant sigined 32 bit intieger and push it on top of the stack.
    LdcI32(i32),
    /// Load constant unsigined 32 bit intieger and push it on top of the stack.
    LdcU32(u32),
    /// Load constant sigined 64 bit intieger and push it on top of the stack.
    LdcI64(i64),
    /// Load constant sigined 64 bit intieger and push it on top of the stack.
    LdcU64(u64),
    /// Load constant 32 bit floating-point number on top of the stack.
    LdcF32(f32),
    /// Load constant 64 bit floating-point number and push it on top of the stack.
    LdcF64(f64),
    /// Load string literal
    LdStr(IString),
    /// Load the address of a function
    LDFtn(Box<CallSite>),
    /// Load null reference
    LdNull,
    /// Signed intieger convertions
    /// Convert the value on top of the stack to an i8. Preform checked convertion if true.
    ConvI8(bool),
    /// Convert the value on top of the stack to an i16. Preform checked convertion if true.
    ConvI16(bool),
    /// Convert the value on top of the stack to an i32. Preform checked convertion if true.
    ConvI32(bool),
    /// Convert the value on top of the stack to an i64. Preform checked convertion if true.
    ConvI64(bool),
    /// Convert the value on top of the stack to an isize. Preform checked convertion if true.
    ConvISize(bool),

    // Unsigned intieger convertions
    /// Convert the value on top of the stack to an u8. Preform checked convertion if true.
    ConvU8(bool),
    /// Convert the value on top of the stack to an u16. Preform checked convertion if true.
    ConvU16(bool),
    /// Convert the value on top of the stack to an u32. Preform checked convertion if true.
    ConvU32(bool),
    /// Convert the value on top of the stack to an u64. Preform checked convertion if true.
    ConvU64(bool),
    /// Convert the value on top of the stack to an usize. Preform checked convertion if true.
    ConvUSize(bool),
    // Float convertions
    /// Convert the value on top of the stack to an f32.
    ConvF32,
    /// Convert the value on top of the stack to an f64.
    ConvF64,
    /// Convert the unsigned value on top of the stack to an f64.
    ConvF64Un,
    // Pointer
    /// Load a value of type i8 at adress represented by the pointer at the top of the stack.
    LDIndI8,
    /// Load a value of type i16 at adress represented by the pointer at the top of the stack.
    LDIndI16,
    /// Load a value of type i32 at adress represented by the pointer at the top of the stack.
    LDIndI32,
    /// Load a value of type i64 at adress represented by the pointer at the top of the stack.
    LDIndI64,
    /// Load a value of type isize at adress represented by the pointer at the top of the stack.
    LDIndISize,
    /// Load a value of type f32 at adress represented by the pointer at the top of the stack.
    LDIndF32,
    /// Load a value of type f64 at adress represented by the pointer at the top of the stack.
    LDIndF64,
    /// Load a value of managed type `ref T` at adress represented by the pointer at the top of the stack.
    LDIndRef,
    /// Set a value of type i8 at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndI8,
    /// Set a value of type i16 at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndI16,
    /// Set a value of type i32 at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndI32,
    /// Set a value of type i64 at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndI64,
    /// Set a value of type isize at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndISize,
    /// Set a value of type f32 at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndF32,
    /// Set a value of type f64 at adress represented by the pointer at the top of the stack, to the value contained at the stack.
    STIndF64,
    //Debugging
    /// Debug comment. Apears in generated ILASM, prevents optimzations.
    Comment(IString),
    // Arthmetic Operations
    /// Adds the 2 top values on the stack togeter, pushing their sum on top of the stack.
    Add,
    /// Variant of `Add` which throws an exception on overflow and underflow, signed.
    AddOvf,
    /// Variant of `Add` which throws an exception on overflow, unsigned.
    AddOvfUn,
    /// Binray ands's the 2 top values on the stack togeter, pushing their sum on top of the stack.
    And,
    /// Divides the value on top of the stack, by the value under it.
    Div,
    /// Divides the value on top of the stack, by the value under it, and pushes the reminder on the top of the stack.
    Rem,
    /// Divides the value on top of the stack, by the value under it, and pushes the reminder on the top of the stack.
    RemUn,
    /// Shifts the value on top of the stack to right by the value under it.
    Shr,
    /// Shifts the value on top of the stack to right by the value under it. Unsigned variant.
    ShrUn,
    /// Shifts the value on top of the stack to left by the value under it.
    Shl,
    /// Subtracts from the value on top of the stack, the value under it.
    Sub,
    /// Subtracts from the value on top of the stack, the value under it. Throws exception on over/underflow, signed.
    SubOvf,
    /// Subtracts from the value on top of the stack, the value under it. Throws exception on overflow, unsigned.
    SubOvfUn,
    /// Multiplies the 2 top values on the stack togeter, pushing their sum on top of the stack.
    Mul,
    /// Multiplies the 2 top values on the stack togeter, pushing their sum on top of the stack. Throws an exception on oveflow.
    MulOvf,
    /// Binray or's the 2 top values on the stack togeter, pushing their sum on top of the stack.
    Or,
    /// Binray xor's the 2 top values on the stack togeter, pushing their sum on top of the stack.
    XOr,
    /// Binray nots the top value on the stack togeter, pushing their sum on top of the stack.
    Not,
    /// Flips the sign of the top value of the stack.
    Neg,
    // Comparisons
    /// Checks if the 2 top values on the stack are equal, pushes 0 if not, and 1 if they are.
    Eq,
    /// Checks if the upper value on the stack is less than the lower one, pushes 0 if not, and 1 if it is.
    Lt,
    /// Checks if the upper value on the stack is greater than the lower one, pushes 0 if not, and 1 if it is.
    Gt,
    /// Checks if the upper value on the stack is less than the lower one, pushes 0 if not, and 1 if it is. Unsigned version.
    LtUn,
    /// Checks if the upper value on the stack is greater than the lower one, pushes 0 if not, and 1 if it is. Unsigned version.
    GtUn,
    //Special
    /// Discards the top value on the stack.
    Pop,
    /// Duplicates the top value on the stack.
    Dup,
    /// Does nothing.
    Nop,
    /// Allocates a temporary buffer of size equal to the value on top of the stack. It lives trough the entire function call, and is deallocated after return.
    LocAlloc,
    //OOP
    /// Allocates a new object using the constructor in `call_site`
    NewObj(Box<CallSite>),
    /// Loads the value field described by `field_describtor`
    LDField(Box<FieldDescriptor>),
    /// Loads the adress of the field described by `field_describtor`
    LDFieldAdress(Box<FieldDescriptor>),
    /// Sets the value field described by `field_describtor`
    STField(Box<FieldDescriptor>),
    /// Loads the value of `type` behind the pointer on top of the stack.
    LdObj(Box<crate::r#type::Type>),
    /// Sets the value of `type` behind the pointer on top of the stack, to the value under it.
    STObj(Box<crate::r#type::Type>),
    /// Returns the size of object of `type`.
    SizeOf(Box<crate::r#type::Type>),
    /// Loads the value of the static field represented by `StaticFieldDescriptor`.
    LDStaticField(Box<StaticFieldDescriptor>),
    /// Sets the value of the static field represented by `StaticFieldDescriptor`.
    STStaticField(Box<StaticFieldDescriptor>),
    /// Copies to *dst* from *src* *count* bytes.  
    CpBlk,
    /// Calls the variable on top of the stack as a function with signature `sig`.
    CallI(Box<FnSig>),
    /// Initializes object of type *ty* at pointer
    InitObj(Box<crate::r#type::Type>),
    /// Initializes bytes at addr to value val of length len.
    InitBlk,
    Volatile,
    /// Loads the token corresponding to type *ty*
    LDTypeToken(Box<crate::r#type::Type>),
    BlockEnd(u32),
    BlockStart(u32),
    EHClause {
        start: u32,
        end: u32,
        hstart: u32,
    },
    CustomLabel(IString),
    BeginTry,
    BeginCatch,
    EndTry,
    Leave(u32),
}
impl CILOp {
    /// If the cil op is a call, virtual call, new object cosntructor OR it loads a pointer to a function, returns the [`CallSite`] representing this function.
    pub fn call(&self) -> Option<&CallSite> {
        match self {
            Self::Call(site) => Some(site),
            Self::CallVirt(site) => Some(site),
            Self::NewObj(site) => Some(site),
            Self::LDFtn(site) => Some(site),
            _ => None,
        }
    }
    /// Returns the ops necesary to construct and throw a new `System.Exception` with message `msg`.
    pub fn throw_msg(msg: &str) -> [CILOp; 3] {
        let mut class = DotnetTypeRef::new(Some("System.Runtime"), "System.Exception");
        class.set_valuetype(false);
        let name = ".ctor".into();
        let signature = FnSig::new(
            &[class.clone().into(), DotnetTypeRef::string_type().into()],
            &crate::r#type::Type::Void,
        );
        [
            CILOp::LdStr(msg.into()),
            CILOp::NewObj(CallSite::boxed(Some(class), name, signature, false)),
            CILOp::Throw,
        ]
    }
    /// Returns the ops necesary to  write message `msg` to STDOUT. Ends with new line.
    #[must_use]
    pub fn debug_msg(msg: &str) -> [CILOp; 2] {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "WriteLine".into();
        let signature = FnSig::new(
            &[DotnetTypeRef::string_type().into()],
            &crate::r#type::Type::Void,
        );
        [
            CILOp::LdStr(msg.into()),
            CILOp::Call(CallSite::new(Some(class), name, signature, true).into()),
        ]
    }
    /// Returns the ops necesary to  write message `msg` to STDOUT. Ends with new line.
    #[must_use]
    pub fn new_line() -> Self {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "WriteLine".into();
        let signature = FnSig::new(&[], &crate::r#type::Type::Void);

        CILOp::Call(CallSite::new(Some(class), name, signature, true).into())
    }
    /// Returns the ops necesary to  write message `msg` to STDOUT. Does not end with new line.
    #[must_use]
    pub fn debug_msg_no_nl(msg: &str) -> [CILOp; 2] {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "Write".into();
        let signature = FnSig::new(
            &[DotnetTypeRef::string_type().into()],
            &crate::r#type::Type::Void,
        );
        [
            CILOp::LdStr(msg.into()),
            CILOp::Call(CallSite::new(Some(class), name, signature, true).into()),
        ]
    }
    /// Returns the ops necesary to  write message bool from stack to stdout. Ends without a new line.
    #[must_use]
    pub fn debug_bool() -> CILOp {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "Write".into();
        let signature = FnSig::new(&[crate::r#type::Type::Bool], &crate::r#type::Type::Void);
        CILOp::Call(CallSite::new(Some(class), name, signature, true).into())
    }
    /// Returns the ops necesary to  write message i32 from stack to stdout. Ends without a new line.
    #[must_use]
    pub fn debug_i32() -> CILOp {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "Write".into();
        let signature = FnSig::new(&[crate::r#type::Type::I32], &crate::r#type::Type::Void);
        CILOp::Call(CallSite::new(Some(class), name, signature, true).into())
    }
    /// Returns the ops necesary to  write message f32 from stack to stdout. Ends without a new line.
    #[must_use]
    pub fn debug_f32() -> CILOp {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "Write".into();
        let signature = FnSig::new(&[crate::r#type::Type::F32], &crate::r#type::Type::Void);
        CILOp::Call(CallSite::new(Some(class), name, signature, true).into())
    }
    /// Returns the ops necesary to u64 write message u64 from stack to stdout. Ends without a new line.
    #[must_use]
    pub fn debug_u64() -> CILOp {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "Write".into();
        let signature = FnSig::new(&[crate::r#type::Type::U64], &crate::r#type::Type::Void);
        CILOp::Call(CallSite::new(Some(class), name, signature, true).into())
    }
    /// Descirbes the difference in stack size before and after the op.
    #[allow(clippy::match_same_arms)]
    pub fn stack_diff(&self) -> isize {
        match self {
            CILOp::Leave(_) => 0,
            CILOp::BeginTry | CILOp::EndTry => 0,
            CILOp::BeginCatch => 1,
            CILOp::EHClause { .. } => 0,
            CILOp::CustomLabel(_) => 0,
            CILOp::Nop => 0,
            CILOp::Comment(_) => 0,
            CILOp::Label(_, _) | CILOp::GoTo(_, _) | CILOp::BlockStart(_) | CILOp::BlockEnd(_) => 0,
            CILOp::BZero(_, _) | CILOp::BTrue(_, _) => -1,
            CILOp::BEq(_, _)
            | CILOp::BNe(_, _)
            | CILOp::BLt(_, _)
            | CILOp::BGe(_, _)
            | CILOp::BLe(_, _) => -2,
            CILOp::LDArg(_) | CILOp::LDArgA(_) | CILOp::LDLoc(_) | CILOp::LDLocA(_) => 1,
            CILOp::LdcI32(_)
            | CILOp::LdcU32(_)
            | CILOp::LdcU64(_)
            | CILOp::LdcI64(_)
            | CILOp::LdcF32(_)
            | CILOp::LdcF64(_)
            | CILOp::LdStr(_)
            | CILOp::LdNull
            | CILOp::SizeOf(_) => 1,
            CILOp::LDTypeToken(_) => 1,
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
            | CILOp::ConvF32
            | CILOp::ConvF64
            | CILOp::ConvF64Un => 0,
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
            CILOp::LocAlloc => 0,
            CILOp::Break => 0,
            CILOp::NewObj(site) => 1 - (site.explicit_inputs().len() as isize),
            CILOp::LdObj(_) => 0,
            CILOp::LDStaticField(_) => 1,
            CILOp::STStaticField(_) => -1,
            CILOp::STObj(_) => -2,
            CILOp::STField(_) => -2,
            CILOp::Add
            | CILOp::AddOvf
            | CILOp::AddOvfUn
            | CILOp::And
            | CILOp::Div
            | CILOp::Rem
            | CILOp::RemUn
            | CILOp::Shr
            | CILOp::ShrUn
            | CILOp::Shl
            | CILOp::Sub
            | CILOp::SubOvf
            | CILOp::SubOvfUn
            | CILOp::Mul
            | CILOp::MulOvf
            | CILOp::Or
            | CILOp::XOr
            | CILOp::Eq
            | CILOp::Lt
            | CILOp::Gt
            | CILOp::LtUn
            | CILOp::GtUn => -1,
            CILOp::Not | CILOp::Neg => 0,
            CILOp::STLoc(_) | CILOp::STArg(_) => -1,
            CILOp::Call(site) | CILOp::CallVirt(site) => {
                if *site.signature().output() == crate::r#type::Type::Void {
                    -(site.signature().inputs().len() as isize)
                } else {
                    1 - (site.signature().inputs().len() as isize)
                }
            }
            CILOp::InitObj(_) => -1,
            CILOp::InitBlk => -3,
            CILOp::Throw => -1,
            CILOp::ReThrow => 0,
            CILOp::Ret => -1,
            CILOp::CpBlk => -3,
            // Syntetic instructions
            CILOp::LDFtn(_) => 1,

            CILOp::CallI(fn_sig) => {
                if fn_sig.output() == &crate::r#type::Type::Void {
                    -(1 + fn_sig.inputs().len() as isize)
                } else {
                    1 - (1 + fn_sig.inputs().len() as isize)
                }
            }
            CILOp::Volatile => 0,
        }
    }
}
