use crate::{function_sig::FnSig, r#type::DotnetTypeRef, IString};
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// This struct descibes a .NET field. It contains information about the type this field belongs to, the name of the field, and the fields type.
pub struct FieldDescriptor {
    owner: DotnetTypeRef,
    tpe: crate::r#type::Type,
    name: IString,
}
impl FieldDescriptor {
    /// Returns the name of the field
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the type of the field. For getting the type this field belongs to, see [self.owner]
    pub fn tpe(&self) -> &crate::r#type::Type {
        &self.tpe
    }
    /// Returns the the type this field belongs to. For getting the type of this field, see [self.tpe]
    pub fn owner(&self) -> &DotnetTypeRef {
        &self.owner
    }
    /// Constructs a new fieldref, reffering to field of type `tpe`, belonging to `owner`, and named `name`
    pub fn new(owner: DotnetTypeRef, tpe: crate::r#type::Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    /// The same as [`Self::new`], but also boxes the field descriptor.
    pub fn boxed(owner: DotnetTypeRef, tpe: crate::r#type::Type, name: IString) -> Box<Self> {
        Box::new(Self { owner, tpe, name })
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// This struct desribes a static .NET field.  It contains information about the type this static field belongs to, the name of the field, and the fields type.
pub struct StaticFieldDescriptor {
    owner: Option<DotnetTypeRef>,
    tpe: crate::r#type::Type,
    name: IString,
}
impl StaticFieldDescriptor {
    /// Returns the name of the static field
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the type of the static field. For getting the type this field belongs to, see [self.owner]
    pub fn tpe(&self) -> &crate::r#type::Type {
        &self.tpe
    }
    /// Returns the the type this static field belongs to. For getting the type of this field, see [self.tpe]
    pub fn owner(&self) -> Option<&DotnetTypeRef> {
        self.owner.as_ref()
    }
    /// Constructs a new static fieldref, reffering to field of type `tpe`, belonging to `owner`, and named `name`
    pub fn new(owner: Option<DotnetTypeRef>, tpe: crate::r#type::Type, name: IString) -> Self {
        Self { owner, tpe, name }
    }
    /// The same as [`Self::new`], but also boxes the field descriptor.
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
    /// Constructs a new call site targeting method `name`, with signature `signature` and bleonging to class `class`. If `class` is [`None`], then the `<Module>` class
    /// is assumed.
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
    /// The same as [`Self::new`], but boxes the result.
    pub fn boxed(
        class: Option<DotnetTypeRef>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Box<Self> {
        Box::new(Self::new(class, name, signature, is_static))
    }
    /// Returns the signature of the function this call site targets.
    pub fn signature(&self) -> &FnSig {
        &self.signature
    }
    /// Returns the class the targeted method belongs to.
    pub fn class(&self) -> Option<&DotnetTypeRef> {
        self.class.as_ref()
    }
    /// Returns `true` if the method in question is static.
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    /// Returns the name of the targteted method.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns true if a call is equivalent to a No-Op. Used to handle black_box.
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
        self.signature.inputs()
    }
    /// Inputs, with the implicit `this` skipped if needed.
    pub fn explicit_inputs(&self) -> &[crate::r#type::Type] {
        if self.is_static || self.inputs().is_empty() {
            self.signature.inputs()
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
    /// Lablel. Represents a position in code that can be jumped to. Does not translate to any real CIL ops, used only to calucalte jump targets.
    /// Should be placed automaticaly at the beiging of a basic block, and not constructed manualy.
    Label(u32),
    /// Unconditional jump to a label with the specified id.
    GoTo(u32),
    /// Jump to target if 2 top values on the stack are equal, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BEq(u32),
    /// Jump to target if 2 top values on the stack are not equal, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BNe(u32),
    /// Jump to target if the top value is less than the bottom one, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BLt(u32),
    /// Jump to target if the top value is greater than the bottom one, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BGe(u32),
    /// Jump to target if the top value on the stack is zero, continue otherwise. WARING: make sure the compared values have the same type, othewise IL is invalid.
    BZero(u32),
    /// Call the metod behind `call_site`.`
    Call(Box<CallSite>),
    /// Call the virtual method behind `call_site`.`
    CallVirt(Box<CallSite>),
    /// Throw the top value on the stack as an exception
    Throw,
    /// Rethrow the current exception
    Rethrow,
    /// Return the top value on the stack from this function
    Ret,

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

    // Syntetic("fake") ops used to simplify some more complex parts of the codegen. They later get turned into real CIL ops.
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// Sometimes, a temporary local variable is needed for codegen purposes(mainly when handling constants)
    /// This instrunction creates a new, short-lived temporary local variable, which can be accessed using [`LoadTMPLocal`],[`LoadAddresOfTMPLocal`]
    /// and [`SetTMPLocal`]. Each [`NewTMPLocal`] must be paired with a corresponding [`FreeTMPLocal`].
    /// No temporary variable is allowed to live across MIR statements.
    NewTMPLocal(Box<crate::r#type::Type>),
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// This instruction frees the last allocated temporary variable.
    FreeTMPLocal,
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// This instruction reads the value of the current temporary local. It is equivalent to `LDLoc(tmp)`.
    LoadTMPLocal,
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// Loads the TMP local n elements under the top of the TMP local stack.
    LoadUnderTMPLocal(u8),
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// This instruction reads the adress of the current temporary local. It is equivalent to `LDLocA(tmp)`.
    LoadAddresOfTMPLocal,
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// This instruction sets the value of the current temporary local. It is equivalent to `STLoc(tmp)`.
    SetTMPLocal,
    /// This is a Syntetic("fake") instruction, which is used **only** internaly. It is not present in the resulting assembly.
    /// This instruction loads a pointer to local allocation `alloc_id`.
    LoadLocalAllocPtr {
        alloc_id: u64,
    },

    // Load constant values.
    /// Load constant sigined 32 bit intieger and push it on top of the stack. Can be used to load u32s too.
    LdcI32(i32),
    /// Load constant sigined 64 bit intieger and push it on top of the stack. Can be used to load u64s too.
    LdcI64(i64),
    /// Load constant 32 bit floating-point number on top of the stack.
    LdcF32(f32),
    /// Load constant 64 bit floating-point number and push it on top of the stack.
    LdcF64(f64),
    /// Load string literal
    LdStr(IString),
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
    AddOvf,
    AddOvfUn,
    And,
    Div,
    Rem,
    Shr,
    Shl,
    Sub,
    SubOvf,
    SubOvfUn,
    Mul,
    MulOvf,
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
    pub fn call(&self) -> Option<&CallSite> {
        match self {
            Self::Call(site) => Some(site),
            Self::CallVirt(site) => Some(site),
            Self::NewObj(site) => Some(site),
            _ => None,
        }
    }
    pub fn throw_msg(msg: &str) -> [CILOp; 3] {
        let class = Some(DotnetTypeRef::new(
            Some("System.Runtime"),
            "System.Exception",
        ));
        let name = ".ctor".into();
        let signature = FnSig::new(
            &[crate::utilis::string_class().into()],
            &crate::r#type::Type::Void,
        );
        [
            CILOp::LdStr(msg.into()),
            CILOp::NewObj(CallSite::boxed(class, name, signature, false)),
            CILOp::Throw,
        ]
    }
    /// Descirbes the difference in stack size before and after the op.
    pub fn stack_diff(&self) -> isize {
        match self {
            CILOp::Nop => 0,
            CILOp::Comment(_) => 0,
            CILOp::Label(_) | CILOp::GoTo(_) => 0,
            CILOp::BZero(_) => -1,
            CILOp::BEq(_) | CILOp::BNe(_) | CILOp::BLt(_) | CILOp::BGe(_) => -2,
            CILOp::LDArg(_) | CILOp::LDArgA(_) | CILOp::LDLoc(_) | CILOp::LDLocA(_) => 1,
            CILOp::LdcI32(_)
            | CILOp::LdcI64(_)
            | CILOp::LdcF32(_)
            | CILOp::LdcF64(_)
            | CILOp::LdStr(_)
            | CILOp::LdNull
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
            CILOp::LocAlloc => 0,
            CILOp::NewObj(site) => 1 - (site.inputs().len() as isize),
            CILOp::LdObj(_) => 0,
            CILOp::LDStaticField(_) => 1,
            CILOp::STObj(_) => -2,
            CILOp::STField(_) => -2,
            CILOp::Add
            | CILOp::AddOvf
            | CILOp::AddOvfUn
            | CILOp::And
            | CILOp::Div
            | CILOp::Rem
            | CILOp::Shr
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
            CILOp::Rethrow => -1,
            CILOp::Ret => -1,
            // Syntetic instructions
            CILOp::NewTMPLocal(_) | CILOp::FreeTMPLocal => 0,
            CILOp::LoadAddresOfTMPLocal | CILOp::LoadUnderTMPLocal(_) | CILOp::LoadTMPLocal => 1,
            CILOp::SetTMPLocal => -1,
            CILOp::LoadLocalAllocPtr { alloc_id: _ } => 1,
        }
    }
    pub fn get_op_arg_pos(_ops: &[CILOp], _pos: usize, _arg: usize) -> Option<usize> {
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
    // There may be a bug there.
    pub fn flip_cond(&self) -> Self {
        match self{
            // a >= b
                CILOp::BGe(target) =>{
                    // b < a
                    CILOp::BLt(*target)
                }
                CILOp::BLt(target) =>{
                    CILOp::BGe(*target)
                }
                _=>todo!("Can't filp conditional operation {self:?}, either because it is not a conditional(bug) or it is not supported yet!"),
            }
    }
}
#[test]
fn test_tmp_locals() {
    use crate::method::Method;
    use crate::r#type::Type;
    let mut method = Method::new(
        crate::access_modifier::AccessModifer::Public,
        true,
        FnSig::new(&[], &Type::U32),
        "meth",
        vec![],
    );
    let ops = vec![
        CILOp::NewTMPLocal(Type::U32.into()),
        CILOp::LdcI32(8),
        CILOp::SetTMPLocal,
        CILOp::LdcI32(7),
        CILOp::LoadTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::Ret,
    ];
    method.set_ops(ops);
    let mut expected_method = Method::new(
        crate::access_modifier::AccessModifer::Public,
        true,
        FnSig::new(&[], &Type::U32),
        "meth",
        vec![Type::U32],
    );
    let expected_ops = vec![
        CILOp::LdcI32(8),
        CILOp::STLoc(0),
        CILOp::LdcI32(7),
        CILOp::LDLoc(0),
        CILOp::Ret,
    ];
    expected_method.set_ops(expected_ops);
    assert_ne!(
        method, expected_method,
        "The methods are different at first."
    );
    method.allocate_temporaries();
    assert_ne!(
        method, expected_method,
        "Methods match after temporary allocation."
    );
}
