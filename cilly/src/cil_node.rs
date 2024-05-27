use crate::{
    call, call_site::CallSite, cil_root::CILRoot, field_desc::FieldDescriptor, fn_sig::FnSig,
    static_field_desc::StaticFieldDescriptor, DotnetTypeRef, IString, Type,
};

use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum CILNode {
    /// Loads the value of local variable number `n`.
    LDLoc(u32),
    /// Loads the value of argument number `n`.
    LDArg(u32),
    /// Loads the address of local variable number `n`.
    LDLocA(u32),
    /// Loads the address of argument number `n`.
    LDArgA(u32),
    /// A black box that prevents the bulit-in optimization engine from doing any optimizations.
    BlackBox(Box<Self>),
    /// Loads the value of a static variable described by the descripstor.
    LDStaticField(Box<StaticFieldDescriptor>),
    /// Converts the signed inner value to a 32 bit floating-point number.
    ConvF32(Box<Self>),
    /// Converts the signed inner value to a 64 bit floating-point number.
    ConvF64(Box<Self>),
    /// Converts the unsigned inner value to a 64 bit floating-point number.
    ConvF64Un(Box<Self>),
    /// Returns the size of a type.
    SizeOf(Box<Type>),
    /// Loads a i8 from a pointer
    LDIndI8 {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a bool from a pointer
    LDIndBool {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a i16 from a pointer
    LDIndI16 {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a i32 from a pointer
    LDIndI32 {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a i64 from a pointer
    LDIndI64 {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a isize from a pointer
    LDIndISize {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a isize from a pointer
    LDIndPtr {
        /// Address of the value
        ptr: Box<Self>,
        loaded_ptr: Box<Type>,
    },
    /// Loads a isize from a pointer
    LDIndUSize {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads an object from a pointer
    LdObj {
        /// Address of the value
        ptr: Box<Self>,
        /// Type of the loaded value
        obj: Box<Type>,
    },
    /// Loads a f32 from a pointer
    LDIndF32 {
        /// Address of the value
        ptr: Box<Self>,
    },
    /// Loads a f64 from a pointer
    LDIndF64 {
        /// Address of the value
        ptr: Box<Self>,
    },
    LDFieldAdress {
        /// Address of the object
        addr: Box<Self>,
        field: Box<FieldDescriptor>,
    },
    LDField {
        /// Address of the object
        addr: Box<Self>,
        field: Box<FieldDescriptor>,
    },
    Add(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    DivUn(Box<Self>, Box<Self>),
    Rem(Box<Self>, Box<Self>),
    RemUn(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    XOr(Box<Self>, Box<Self>),
    Shr(Box<Self>, Box<Self>),
    Shl(Box<Self>, Box<Self>),
    ShrUn(Box<Self>, Box<Self>),

    Call {
        args: Box<[Self]>,
        site: Box<CallSite>,
    },
    CallVirt {
        args: Box<[Self]>,
        site: Box<CallSite>,
    },
    LdcI64(i64),
    LdcU64(u64),
    LdcI32(i32),
    LdcU32(u32),
    LdcF64(f64),
    LdcF32(f32),
    LoadGlobalAllocPtr {
        alloc_id: u64,
    },
    ConvU8(Box<Self>),
    ConvU16(Box<Self>),
    ConvU32(Box<Self>),
    ConvU64(Box<Self>),
    ZeroExtendToUSize(Box<Self>),
    ZeroExtendToISize(Box<Self>),
    MRefToRawPtr(Box<Self>),
    ConvI8(Box<Self>),
    ConvI16(Box<Self>),
    ConvI32(Box<Self>),
    ConvI64(Box<Self>),
    ConvISize(Box<Self>),
    //Volatile(Box<Self>),
    Neg(Box<Self>),
    Not(Box<Self>),
    Eq(Box<Self>, Box<Self>),
    Lt(Box<Self>, Box<Self>),
    /// Compares two operands, returning true if lhs < rhs. Unsigned for intigers, unordered(in respect to NaNs) for floats.
    LtUn(Box<Self>, Box<Self>),
    Gt(Box<Self>, Box<Self>),
    /// Compares two operands, returning true if lhs < rhs. Unsigned for intigers, unordered(in respect to NaNs) for floats.
    GtUn(Box<Self>, Box<Self>),
    TemporaryLocal(Box<(Type, Box<[CILRoot]>, Self)>),
    SubTrees(Box<[CILRoot]>, Box<Self>),
    LoadAddresOfTMPLocal,
    LoadTMPLocal,
    LDFtn(Box<CallSite>),
    LDTypeToken(Box<Type>),
    NewObj {
        site: Box<CallSite>,
        args: Box<[Self]>,
    },
    LdStr(IString),
    CallI(Box<(FnSig, Self, Box<[Self]>)>),
    LDIndU8 {
        ptr: Box<Self>,
    },
    LDIndU16 {
        ptr: Box<Self>,
    },
    LDIndU32 {
        ptr: Box<Self>,
    },
    LDIndU64 {
        ptr: Box<Self>,
    },
    /// Loads the length of an array - as a nint.
    LDLen {
        arr: Box<Self>,
    },
    /// Loads an object reference from a managed array
    LDElelemRef {
        arr: Box<Self>,
        idx: Box<Self>,
    },
    PointerToConstValue(u128),
    /// Unsafe, low-level internal op for checking the state of the CIL eval stack. WILL cause serious issues if not used **very** carefully, and only inside the `inspect` arm of `InspectValue`.
    GetStackTop,
    /// Unsafe, low-level internal op for inspecting the state of the CIL eval stack. Must be paired with exactly one `GetStackTop`, in the `inspect` arm.
    InspectValue {
        val: Box<Self>,
        inspect: Box<[CILRoot]>,
    },
    /// Tells the codegen a pointer value type is changed. Used during verification, to implement things like`transmute`.
    TransmutePtr {
        val: Box<Self>,
        new_ptr: Box<Type>,
    },
    /// Equivalent to `CILNode::LdcI32(0`), but with addtional typechecking info.
    LdFalse,
    /// Equivalent to `CILNode::LdcI32(1`), but with addtional typechecking info.
    LdTrue,
    /// Allocates a buffer of size at least `sizeof(tpe)` with aligement of `align`
    LocAllocAligned {
        tpe: Box<Type>,
        align: u64,
    },
}

impl CILNode {
    #[must_use]
    pub fn print_debug_val(format_start: &str, value: Self, format_end: &str, tpe: Type) -> Self {
        match tpe {
            Type::U64 | Type::I64 | Type::U32 | Type::I32 => Self::InspectValue {
                val: Box::new(value),
                inspect: Box::new([
                    CILRoot::debug(format_start),
                    CILRoot::Call {
                        site: CallSite::new_extern(
                            DotnetTypeRef::console(),
                            "Write".into(),
                            FnSig::new(&[tpe], Type::Void),
                            true,
                        ),
                        args: Box::new([Self::GetStackTop]),
                    },
                    CILRoot::debug(format_end),
                ]),
            },
            Type::U8 | Type::U16 => Self::InspectValue {
                val: Box::new(value),
                inspect: Box::new([
                    CILRoot::debug(format_start),
                    CILRoot::Call {
                        site: CallSite::new_extern(
                            DotnetTypeRef::console(),
                            "Write".into(),
                            FnSig::new(&[Type::U32], Type::Void),
                            true,
                        ),
                        args: Box::new([Self::ConvU32(Box::new(Self::GetStackTop))]),
                    },
                    CILRoot::debug(format_end),
                ]),
            },
            Type::I8 | Type::I16 => Self::InspectValue {
                val: Box::new(value),
                inspect: Box::new([
                    CILRoot::debug(format_start),
                    CILRoot::Call {
                        site: CallSite::new_extern(
                            DotnetTypeRef::console(),
                            "Write".into(),
                            FnSig::new(&[Type::I32], Type::Void),
                            true,
                        ),
                        args: Box::new([Self::ConvI32(Box::new(Self::GetStackTop))]),
                    },
                    CILRoot::debug(format_end),
                ]),
            },
            Type::USize => Self::InspectValue {
                val: Box::new(value),
                inspect: Box::new([
                    CILRoot::debug(format_start),
                    CILRoot::Call {
                        site: CallSite::new_extern(
                            DotnetTypeRef::console(),
                            "Write".into(),
                            FnSig::new(&[Type::U64], Type::Void),
                            true,
                        ),
                        args: Box::new([Self::ZeroExtendToUSize(Box::new(Self::GetStackTop))]),
                    },
                    CILRoot::debug(format_end),
                ]),
            },
            Type::ISize => Self::InspectValue {
                val: Box::new(value),
                inspect: Box::new([
                    CILRoot::debug(format_start),
                    CILRoot::Call {
                        site: CallSite::new_extern(
                            DotnetTypeRef::console(),
                            "Write".into(),
                            FnSig::new(&[Type::I64], Type::Void),
                            true,
                        ),
                        args: Box::new([Self::ConvISize(Box::new(Self::GetStackTop))]),
                    },
                    CILRoot::debug(format_end),
                ]),
            },
            Type::Void => value,
            _ => Self::InspectValue {
                val: Box::new(value),
                inspect: Box::new([
                    CILRoot::debug(format_start),
                    CILRoot::Pop {
                        tree: Self::GetStackTop,
                    },
                    CILRoot::debug(format_end),
                ]),
            },
        }
    }
    #[must_use]
    pub fn select(tpe: Type, a: Self, b: Self, predictate: Self) -> Self {
        match tpe {
            Type::U128 | Type::I128 => call!(
                CallSite::builtin(
                    "select_u128".into(),
                    FnSig::new(&[Type::U128, Type::U128, Type::Bool], Type::U128),
                    true
                ),
                [a, b, predictate]
            ),
            Type::USize | Type::ISize | Type::Ptr(_) => call!(
                CallSite::builtin(
                    "select_usize".into(),
                    FnSig::new(&[Type::USize, Type::USize, Type::Bool], Type::USize),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U64 | Type::I64 => call!(
                CallSite::builtin(
                    "select_u64".into(),
                    FnSig::new(&[Type::U64, Type::U64, Type::Bool], Type::U64),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U32 | Type::I32 => call!(
                CallSite::builtin(
                    "select_u32".into(),
                    FnSig::new(&[Type::U32, Type::U32, Type::Bool], Type::U32),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U16 | Type::I16 => call!(
                CallSite::builtin(
                    "select_u16".into(),
                    FnSig::new(&[Type::U16, Type::U16, Type::Bool], Type::U16),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U8 | Type::I8 | Type::Bool => call!(
                CallSite::builtin(
                    "select_u8".into(),
                    FnSig::new(&[Type::U8, Type::U8, Type::Bool], Type::U8),
                    true
                ),
                [a, b, predictate]
            ),
            _ => todo!("Can't select type {tpe:?}"),
        }
    }
    fn opt_children(&mut self) {
        match self {
            Self::LocAllocAligned { .. }=>(),
            Self::LdFalse | Self::LdTrue=>(),
            Self::TransmutePtr { val, new_ptr: _ }=>val.opt(),
            Self::InspectValue { val, inspect }=>{val.opt_children();inspect.iter_mut().for_each(super::cil_root::CILRoot::opt)},
            Self::LDLoc(_) |  Self::GetStackTop | Self::LDArg(_) | Self::LDLocA(_) | Self::LDArgA(_)=> (),
            Self::BlackBox(inner)
            | Self::ConvF32(inner)
            | Self::ConvF64(inner)
            | Self::ConvF64Un(inner) => inner.opt(),
            Self::SizeOf(_) => (),
            Self::LDIndI8 { ptr }
            | Self::LDIndBool { ptr }
            | Self::LDIndI16 { ptr }
            | Self::LDIndI32 { ptr }
            | Self::LDIndI64 { ptr }
            | Self::LDIndU8 { ptr }
            | Self::LDIndU16 { ptr }
            | Self::LDIndU32 { ptr }
            | Self::LDIndU64 { ptr }
            | Self::LDIndPtr{ ptr, .. }
            | Self::LDIndISize { ptr }
            | Self::LDIndUSize { ptr }
            | Self::LdObj { ptr, .. }
            | Self::LDIndF32 { ptr }
            | Self::LDIndF64 { ptr } => ptr.opt(),
            Self::LDFieldAdress { addr, field: _ } | Self::LDField { addr, field: _ } => addr.opt(),
            Self::Add(a, b)
            | Self::And(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::DivUn(a, b)
            | Self::Rem(a, b)
            | Self::RemUn(a, b)
            | Self::Or(a, b)
            | Self::XOr(a, b)
            | Self::Shr(a, b)
            | Self::Shl(a, b)
            | Self::ShrUn(a, b)
            | Self::Eq(a, b)
            | Self::Lt(a, b)
            | Self::LtUn(a, b)
            | Self::Gt(a, b)
            | Self::GtUn(a, b) => {
                a.opt();
                b.opt();
            }
            Self::Call { args, site: _ }
            | Self::NewObj { site: _, args }
            | Self::CallVirt { args, site: _ } => args.iter_mut().for_each(Self::opt),
            Self::LdcI64(_)
            | Self::LdcU64(_)
            | Self::LdcI32(_)
            | Self::LdcU32(_)
            | Self::LdcF64(_)
            | Self::LdcF32(_)
            | Self::LoadGlobalAllocPtr { .. }
            | Self::PointerToConstValue(_)=> (),
            Self::ConvU8(inner)
            | Self::ConvU16(inner)
            | Self::ConvU32(inner)
            | Self::ConvU64(inner)
            | Self::ZeroExtendToUSize(inner)
            | Self::ZeroExtendToISize(inner)
            | Self::MRefToRawPtr(inner)
            | Self::ConvI8(inner)
            | Self::ConvI16(inner)
            | Self::ConvI32(inner)
            | Self::ConvI64(inner)
            | Self::ConvISize(inner)
            //| Self::Volatile(inner)
            | Self::Neg(inner)
            | Self::Not(inner) => inner.opt(),
            Self::TemporaryLocal(_inner) => (),
            Self::SubTrees(a, b) => {
                a.iter_mut().for_each(super::cil_root::CILRoot::opt);
                b.opt();
            }
            Self::LoadAddresOfTMPLocal => (),
            Self::LoadTMPLocal => (),
            Self::LDFtn(_) => (),
            Self::LDTypeToken(_) => (),
            Self::LdStr(_) => (),
            Self::CallI (ptr_sig_arg ) => {
                ptr_sig_arg.2.iter_mut().for_each(Self::opt);
                ptr_sig_arg.1.opt();
            }
            Self::LDStaticField(_static_field) => (),
            Self::LDLen { arr } => arr.opt(),
            Self::LDElelemRef { arr, idx } =>{
                idx.opt();
                arr.opt();
            },
        }
    }
    // This fucntion will get expanded, so a single match is a non-issue.
    #[allow(clippy::single_match)]
    // This clippy lint is wrong
    #[allow(clippy::assigning_clones)]
    /// Optimizes this `CILNode`.
    pub fn opt(&mut self) {
        self.opt_children();
        match self {
            Self::LDField { addr: fld_addr, .. } => match fld_addr.as_mut() {
                Self::MRefToRawPtr(addr) => match addr.as_mut() {
                    Self::LDLocA(_) | Self::LDFieldAdress { .. } => *fld_addr = addr.clone(),
                    _ => (),
                },
                _ => (),
            },
            Self::LDFieldAdress {
                addr: fld_addr,
                field,
            } => match fld_addr.as_mut() {
                Self::MRefToRawPtr(addr) => match addr.as_mut() {
                    Self::LDLocA(_) | Self::LDFieldAdress { .. } => {
                        *self = Self::MRefToRawPtr(Box::new(Self::LDFieldAdress {
                            addr: addr.clone(),
                            field: field.clone(),
                        }));
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    pub(crate) fn allocate_tmps(
        &mut self,
        curr_loc: Option<u32>,
        locals: &mut Vec<(Option<Box<str>>, Type)>,
    ) {
        match self {
            Self::LocAllocAligned {..}=>(),
            Self::LdFalse=>(),
            Self::LdTrue=>(),
            Self::TransmutePtr { val, new_ptr: _ }=>val.allocate_tmps(curr_loc, locals),
            Self::GetStackTop =>(),
            Self::InspectValue { val, inspect }=>{
                val.allocate_tmps(curr_loc, locals);
                inspect.iter_mut().for_each(|root|root.allocate_tmps(curr_loc, locals));
            },
            Self:: PointerToConstValue(_arr)=>(),
            Self::LoadGlobalAllocPtr { alloc_id: _ } => (),
            Self::LDLoc(_) |
            Self::LDArg(_) |
            Self::LDLocA(_)|
            Self::LDArgA(_) => (),
            Self::BlackBox(_) => todo!(),
            Self::SizeOf(_) => (),
            Self::LDIndI8 { ptr }|
            Self::LDIndBool { ptr }|
            Self::LDIndI16 { ptr }|
            Self::LDIndI32 { ptr }|
            Self::LDIndI64 { ptr }|
            Self::LDIndU8 { ptr }|
            Self::LDIndU16 { ptr }|
            Self::LDIndU32 { ptr }|
            Self::LDIndU64 { ptr }|
            Self::LDIndISize { ptr }|
            Self::LDIndPtr { ptr, .. }|
            Self::LDIndUSize { ptr }|
            Self::LdObj { ptr, .. }|
            Self::LDIndF32 { ptr } |
            Self::LDIndF64 { ptr } => ptr.allocate_tmps(curr_loc, locals),
            Self::LDFieldAdress { addr, field: _ } |
            Self::LDField { addr, field: _ }=> addr.allocate_tmps(curr_loc, locals),
            Self::Add(a, b)
            | Self::And(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::DivUn(a, b)
            | Self::Rem(a, b)
            | Self::RemUn(a, b)
            | Self::Or(a, b)
            | Self::XOr(a, b)
            | Self::Shr(a, b)
            | Self::Shl(a, b)
            | Self::ShrUn(a, b)
            | Self::Eq(a, b)
            | Self::Lt(a, b)
            | Self::LtUn(a, b)
            | Self::Gt(a, b)
            | Self::GtUn(a, b) => {
                a.allocate_tmps(curr_loc, locals);
                b.allocate_tmps(curr_loc, locals);
            }
            Self::Call { args, site: _ } |
            Self::CallVirt { args, site: _ } =>args.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals)),
            Self::LdcI64(_) |
            Self::LdcU64(_) |
            Self::LdcI32(_)  |
            Self::LdcU32(_) |
            Self::LdcF64(_) |
            Self::LdcF32(_) =>(),
            Self::ConvF64Un(val) |
            Self::ConvF32(val)|
            Self::ConvF64(val) |
            Self::ConvU8(val)|
            Self::ConvU16(val)|
            Self::ConvU32(val)|
            Self::ConvU64(val)|
            Self::MRefToRawPtr(val) |
            Self::ZeroExtendToUSize(val)|
            Self::ZeroExtendToISize(val)|
            Self::ConvI8(val) |
            Self::ConvI16(val)|
            Self::ConvI32(val)|
            Self::ConvI64(val) |
            Self::ConvISize(val)|
            //Self::Volatile(_) => todo!(),
            Self::Neg(val) |
            Self::Not(val) =>val.allocate_tmps(curr_loc, locals),

            Self::TemporaryLocal(tmp_loc) => {
                let tpe = &mut tmp_loc.0;
                let end_loc = locals.len();
                locals.push((None,tpe.clone()));
                let roots = &mut tmp_loc.1;
                let main = &mut tmp_loc.2;
                roots.iter_mut().for_each(|tree|tree.allocate_tmps(Some(end_loc as u32), locals));
                main.allocate_tmps(Some(end_loc as u32), locals);
                *self= Self::SubTrees(roots.clone(), Box::new(main.clone()));
            },
            Self::SubTrees(trees, main) =>{
                trees.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc,locals));
                main.allocate_tmps(curr_loc, locals);
            }
            Self::LoadAddresOfTMPLocal => *self = Self::LDLocA(curr_loc.expect("Temporary local referenced when none present")),
            Self::LoadTMPLocal =>*self = Self::LDLoc(curr_loc.expect("Temporary local referenced when none present")),
            Self::LDFtn(_) => (),
            Self::LDTypeToken(_) =>(),
            Self::NewObj { site: _, args } => args.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals)),
            Self::LdStr(_) => (),
            Self::CallI (sig_ptr_args) => {
                sig_ptr_args.1.allocate_tmps(curr_loc, locals);
                sig_ptr_args.2.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals));
            }
            Self::LDStaticField(_sfield)=>(),
            Self::LDLen { arr } =>{
               arr.allocate_tmps(curr_loc, locals);
            }
            Self::LDElelemRef { arr, idx }=>{
                arr.allocate_tmps(curr_loc, locals);
                idx.allocate_tmps(curr_loc, locals);
            }
        };
    }

    pub(crate) fn sheed_trees(&mut self) -> Vec<CILRoot> {
        match self {
            Self::LocAllocAligned { .. } => vec![],
            Self::LdFalse => vec![],
            Self::LdTrue => vec![],
            Self::TransmutePtr { val, new_ptr: _ } => val.sheed_trees(),
            Self::GetStackTop => vec![],
            Self::InspectValue { val, inspect: _ } => val.sheed_trees(),
            Self::LDLoc(_) | Self::LDArg(_) | Self::LDLocA(_) | Self::LDArgA(_) => {
                vec![]
            }
            Self::BlackBox(inner) => inner.sheed_trees(),
            Self::LDStaticField(_) => vec![],
            Self::ConvF32(inner) | Self::ConvF64(inner) | Self::ConvF64Un(inner) => {
                inner.sheed_trees()
            }
            Self::SizeOf(_) => vec![],
            Self::LDIndI8 { ptr }
            | Self::LDIndBool { ptr }
            | Self::LDIndI16 { ptr }
            | Self::LDIndI32 { ptr }
            | Self::LDIndI64 { ptr }
            | Self::LDIndU8 { ptr }
            | Self::LDIndU16 { ptr }
            | Self::LDIndU32 { ptr }
            | Self::LDIndU64 { ptr }
            | Self::LDIndISize { ptr }
            | Self::LDIndPtr { ptr, .. }
            | Self::LDIndUSize { ptr }
            | Self::LdObj { ptr, .. }
            | Self::LDIndF32 { ptr }
            | Self::LDIndF64 { ptr } => ptr.sheed_trees(),
            Self::LDFieldAdress { addr, field: _ } | Self::LDField { addr, field: _ } => {
                addr.sheed_trees()
            }
            Self::Add(a, b)
            | Self::And(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::DivUn(a, b)
            | Self::Rem(a, b)
            | Self::RemUn(a, b)
            | Self::Or(a, b)
            | Self::XOr(a, b)
            | Self::Shr(a, b)
            | Self::Shl(a, b)
            | Self::ShrUn(a, b) => {
                let mut res = a.sheed_trees();
                res.extend(b.sheed_trees());
                res
            }

            Self::Call { args, site: _ } | Self::CallVirt { args, site: _ } => {
                args.iter_mut().flat_map(Self::sheed_trees).collect()
            }
            Self::LdcI64(_)
            | Self::LdcU64(_)
            | Self::LdcI32(_)
            | Self::LdcU32(_)
            | Self::LdcF64(_)
            | Self::LdcF32(_) => vec![],
            Self::LoadGlobalAllocPtr { alloc_id: _ } => vec![],
            Self::PointerToConstValue(_value) => vec![],
            Self::ConvU8(val)
            | Self::ConvU16(val)
            | Self::ConvU32(val)
            | Self::ConvU64(val)
            | Self::ZeroExtendToUSize(val)
            | Self::ZeroExtendToISize(val)
            | Self::MRefToRawPtr(val)
            | Self::ConvI8(val)
            | Self::ConvI16(val)
            | Self::ConvI32(val)
            | Self::ConvI64(val)
            | Self::ConvISize(val) => val.sheed_trees(),
            Self::Neg(val) | Self::Not(val) => val.sheed_trees(),
            Self::Eq(a, b)
            | Self::Lt(a, b)
            | Self::LtUn(a, b)
            | Self::Gt(a, b)
            | Self::GtUn(a, b) => {
                let mut res = a.sheed_trees();
                res.extend(b.sheed_trees());
                res
            }
            Self::TemporaryLocal(_) => {
                panic!("Trees should be sheed after locals are allocated!")
            }
            Self::SubTrees(trees, main) => {
                let clone = *main.clone();
                let res = trees.to_vec();
                *self = clone;
                res
            }
            Self::LoadAddresOfTMPLocal => {
                panic!("Trees should be sheed after locals are allocated!")
            }
            Self::LoadTMPLocal => panic!("Trees should be sheed after locals are allocated!"),
            Self::LDFtn(_) | Self::LDTypeToken(_) => vec![],
            Self::NewObj { site: _, args } => args.iter_mut().flat_map(Self::sheed_trees).collect(),
            Self::LdStr(_) => vec![],
            Self::CallI(sig_ptr_args) => {
                let mut res = sig_ptr_args.1.sheed_trees();
                res.extend(sig_ptr_args.2.iter_mut().flat_map(Self::sheed_trees));
                res
            }
            Self::LDLen { arr } => arr.sheed_trees(),
            Self::LDElelemRef { arr, idx } => {
                let mut res = arr.sheed_trees();
                res.extend(idx.sheed_trees());
                res
            }
        }
    }
    /*
    pub(crate) fn validate(&self, method: &Method) -> Result<Type, String> {
        match self {
            Self::SubTrees(trees, main) => {
                for tree in trees.iter() {
                    tree.validate(method)?;
                }
                main.validate(method)
            }
            Self::LdTrue => Ok(Type::Bool),
            Self::InspectValue { val, inspect: _ } => val.validate(method),
            Self::LDField { addr, field } => {
                let addr = addr.validate(method)?;
                match addr {
                    Type::ManagedReference(tpe) | Type::Ptr(tpe) => {
                        if tpe.as_dotnet() != Some(field.owner().clone()) {
                            return Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}"
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Tired to load a field of a non-pointer type! addr:{addr:?}"
                        ))
                    }
                }
                Ok(field.tpe().clone())
            }
            Self::LDFieldAdress { addr, field } => {
                let addr = addr.validate(method)?;
                match addr {
                    Type::ManagedReference(tpe) => {
                        if tpe.as_dotnet() != Some(field.owner().clone()) {
                            Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}"
                            ))
                        } else {
                            Ok(Type::ManagedReference(Box::new(field.tpe().clone())))
                        }
                    }
                    Type::Ptr(tpe) => {
                        if tpe.as_dotnet() != Some(field.owner().clone()) {
                            Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}"
                            ))
                        } else {
                            Ok(Type::Ptr(Box::new(field.tpe().clone())))
                        }
                    }
                    _ => Err(format!(
                        "Tired to load a field of a non-pointer type! addr:{addr:?}"
                    )),
                }
            }
            Self::LDStaticField(sfd) => Ok(sfd.tpe().clone()),
            Self::LDLocA(loc) => match method.locals().get(*loc as usize) {
                Some(local) => Ok(Type::ManagedReference(Box::new(local.1.clone()))),
                None => Err(format!("Local {loc }out of range.")),
            },
            Self::LDLoc(loc) => match method.locals().get(*loc as usize) {
                Some(local) => Ok(local.1.clone()),
                None => Err(format!("Local {loc }out of range.")),
            },
            Self::LDArg(arg) => match method.sig().inputs().get(*arg as usize) {
                Some(arg) => Ok(arg.clone()),
                None => Err(format!("Argument {arg} out of range.")),
            },
            Self::LDArgA(arg) => match method.sig().inputs().get(*arg as usize) {
                Some(arg) => Ok(Type::ManagedReference(Box::new(arg.clone()))),
                None => Err(format!("Argument {arg} out of range.")),
            },
            Self::LdObj { ptr, obj } => {
                let ptr = ptr.validate(method)?;
                match ptr {
                    Type::Ptr(pointed) | Type::ManagedReference(pointed) => {
                        if pointed != *obj {
                            Err(format!("Tried to load a object of type {obj:?} from a pointer to type {pointed:?}"))
                        } else {
                            Ok(*obj.clone())
                        }
                    }
                    _ => Err(format!(
                        "{ptr:?} is not a pointer type, so LdObj can't operate on it."
                    )),
                }
            }
            Self::LDIndISize { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::ISize))
                    && ptr != Type::ManagedReference(Box::new(Type::ISize))
                {
                    return Err(format!("Tried to load isize from pointer of type {ptr:?}"));
                }
                Ok(Type::ISize)
            }
            Self::LDIndPtr { ptr, loaded_ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(loaded_ptr.clone())
                    && ptr != Type::ManagedReference(loaded_ptr.clone())
                {
                    return Err(format!(
                        "Tried to load {loaded_ptr:?} from pointer of type {ptr:?}"
                    ));
                }
                Ok(*(loaded_ptr).clone())
            }
            Self::LDIndUSize { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::USize))
                    && ptr != Type::ManagedReference(Box::new(Type::USize))
                {
                    return Err(format!("Tried to load usize from pointer of type {ptr:?}"));
                }
                Ok(Type::USize)
            }
            Self::LDIndU32 { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::U32))
                    && ptr != Type::ManagedReference(Box::new(Type::U32))
                {
                    return Err(format!("Tried to load isize from pointer of type {ptr:?}"));
                }
                Ok(Type::U32)
            }
            Self::LDIndI32 { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::I32))
                    && ptr != Type::ManagedReference(Box::new(Type::I32))
                {
                    return Err(format!("Tried to load i32 from pointer of type {ptr:?}"));
                }
                Ok(Type::I32)
            }
            Self::LDIndU16 { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::U16))
                    && ptr != Type::ManagedReference(Box::new(Type::U16))
                {
                    return Err(format!("Tried to load u16 from pointer of type {ptr:?}"));
                }
                Ok(Type::U16)
            }
            Self::LDIndI16 { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::I16))
                    && ptr != Type::ManagedReference(Box::new(Type::I16))
                {
                    return Err(format!("Tried to load i16 from pointer of type {ptr:?}"));
                }
                Ok(Type::I16)
            }
            Self::LDIndU8 { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::U8))
                    && ptr != Type::ManagedReference(Box::new(Type::U8))
                {
                    return Err(format!("Tried to load u8 from pointer of type {ptr:?}"));
                }
                Ok(Type::U8)
            }
            Self::LDIndI8 { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::I8))
                    && ptr != Type::ManagedReference(Box::new(Type::I8))
                {
                    return Err(format!("Tried to load i8 from pointer of type {ptr:?}"));
                }
                Ok(Type::I8)
            }
            Self::LDIndBool { ptr } => {
                let ptr = ptr.validate(method)?;
                if ptr != Type::Ptr(Box::new(Type::Bool))
                    && ptr != Type::ManagedReference(Box::new(Type::Bool))
                {
                    return Err(format!("Tried to load bool from pointer of type {ptr:?}"));
                }
                Ok(Type::Bool)
            }
            Self::LdcU64(_) => Ok(Type::U64),
            Self::LdcI64(_) => Ok(Type::I64),
            Self::LdcU32(_) => Ok(Type::U32),
            Self::LdcI32(_) => Ok(Type::I32),
            Self::LdcF32(_) => Ok(Type::F32),
            Self::LdcF64(_) => Ok(Type::F64),
            Self::ConvISize(src) => {
                src.validate(method)?;
                Ok(Type::ISize)
            }
            Self::ConvI64(src) => {
                src.validate(method)?;
                Ok(Type::I64)
            }
            Self::ZeroExtendToUSize(src) => {
                let _tpe = src.validate(method)?;
                Ok(Type::USize)
            }
            Self::ZeroExtendToISize(src) => {
                let _tpe = src.validate(method)?;
                Ok(Type::ISize)
            }
            Self::MRefToRawPtr(src) => {
                let tpe = src.validate(method)?;
                if let Type::ManagedReference(pointed) = tpe {
                    Ok(Type::Ptr(pointed))
                } else {
                    Err(format!(
                        "MRefToRawPtr expected a managed ref, but got {tpe:?}"
                    ))
                }
            }
            Self::ConvU64(src) => {
                src.validate(method)?;
                Ok(Type::U64)
            }
            Self::ConvU32(src) => {
                src.validate(method)?;
                Ok(Type::U32)
            }
            Self::ConvI32(src) => {
                src.validate(method)?;
                Ok(Type::I32)
            }
            Self::ConvF32(src) => {
                src.validate(method)?;
                Ok(Type::F32)
            }
            Self::ConvF64(src) | Self::ConvF64Un(src) => {
                src.validate(method)?;
                Ok(Type::F64)
            }
            Self::ConvU16(src) => {
                src.validate(method)?;
                Ok(Type::U16)
            }
            Self::ConvI16(src) => {
                src.validate(method)?;
                Ok(Type::I16)
            }
            Self::ConvU8(src) => {
                src.validate(method)?;
                Ok(Type::U8)
            }
            Self::ConvI8(src) => {
                src.validate(method)?;
                Ok(Type::I8)
            }
            Self::LtUn(a, b)
            | Self::Lt(a, b)
            | Self::GtUn(a, b)
            | Self::Gt(a, b)
            | Self::Eq(a, b) => {
                let a = a.validate(method)?;
                let b = b.validate(method)?;
                if a != b {
                    return Err(format!(
                        "Invalid arguments of the {self:?} instruction. {a:?} != {b:?}"
                    ));
                }

                if !a.is_primitive_numeric() {
                    return Err(format!(
                        "The instruction {self:?} can't operate on a non-primitve CIL type {a:?}."
                    ));
                }
                Ok(Type::Bool)
            }
            Self::Add(a, b)
            | Self::Mul(a, b)
            | Self::And(a, b)
            | Self::Sub(a, b)
            | Self::Or(a, b)
            | Self::Rem(a, b)
            | Self::RemUn(a, b)
            | Self::Div(a, b)
            | Self::DivUn(a, b)
            | Self::XOr(a, b) => {
                let a = a.validate(method)?;
                let b = b.validate(method)?;
                if a != b {
                    match (&a, &b) {
                        (Type::Ptr(_), Type::ISize | Type::USize) => return Ok(a),
                        _ => {
                            return Err(format!(
                                "Invalid arguments of the Add instruction. {a:?} != {b:?}"
                            ))
                        }
                    }
                }

                if !a.is_primitive_numeric() {
                    return Err(format!(
                        "The instruction Add can't operate on a non-primitve CIL type {a:?}."
                    ));
                }
                Ok(a)
            }
            Self::Not(a) | Self::Neg(a) => {
                let a = a.validate(method)?;
                if !a.is_primitive_numeric() {
                    return Err(format!(
                        "The instruction Add can't operate on a non-primitve CIL type {a:?}."
                    ));
                }
                Ok(a)
            }
            Self::Shr(a, b) | Self::ShrUn(a, b) | Self::Shl(a, b) => {
                let a = a.validate(method)?;
                let b = b.validate(method)?;
                if !a.is_primitive_numeric() {
                    return Err(format!(
                        "The instruction Add can't operate on a non-primitve CIL type {a:?}."
                    ));
                }
                if !b.is_primitive_numeric() {
                    return Err(format!(
                        "The instruction Add can't operate on a non-primitve CIL type {a:?}."
                    ));
                }
                Ok(a)
            }
            Self::LdStr(_) => Ok(Type::DotnetType(DotnetTypeRef::string_type().into())),
            Self::NewObj { site, args } => {
                if site.explicit_inputs().len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        site.explicit_inputs().len(),
                        args.len()
                    ));
                }
                for (arg, tpe) in args.iter().zip(site.explicit_inputs().iter()) {
                    let arg = arg.validate(method)?;
                    if arg != *tpe {
                        return Err(format!(
                            "Expected an argument of type {tpe:?}, but got {arg:?}"
                        ));
                    }
                }
                match site.class() {
                    Some(class) => {
                        if class.asm() == Some("System.Runtime") {
                            if *class == DotnetTypeRef::int_128() {
                                return Ok(Type::I128);
                            }
                            if *class == DotnetTypeRef::uint_128() {
                                return Ok(Type::U128);
                            }
                        }
                        Ok(Type::DotnetType(class.clone().into()))
                    }
                    None => Err("Newobj instruction witn no class specified".into()),
                }
            }
            Self::Call { site, args } => {
                if site.inputs().len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        site.inputs().len(),
                        args.len()
                    ));
                }
                for (arg, tpe) in args.iter().zip(site.inputs().iter()) {
                    let arg = arg.validate(method)?;
                    if arg != *tpe {
                        return Err(format!(
                            "Expected an argument of type {tpe:?}, but got {arg:?}"
                        ));
                    }
                }
                Ok(site.signature().output().clone())
            }
            Self::CallI(packed) => {
                let (sig, ptr, args) = packed.as_ref();
                let _ptr = ptr.validate(method)?;
                if sig.inputs().len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        sig.inputs().len(),
                        args.len()
                    ));
                }
                for (arg, tpe) in args.iter().zip(sig.inputs().iter()) {
                    let arg = arg.validate(method)?;
                    if arg != *tpe {
                        return Err(format!(
                            "Expected an argument of type {tpe:?}, but got {arg:?}"
                        ));
                    }
                }
                Ok(sig.output().clone())
            }
            Self::TransmutePtr { val, new_ptr } => {
                let val = val.validate(method)?;
                match val {
                    Type::USize => (),
                    Type::ISize => (),
                    Type::Ptr(_) => (),
                    // ManagedReference is accepted, because sometimes we use this to transmute local
                    Type::ManagedReference(_) => (),
                    _ => {
                        return Err(format!(
                            "Invalid TransmutePtr input: {val:?} is not a pointer or usize/isize"
                        ))
                    }
                }
                Ok(*new_ptr.clone())
            }
            Self::LdFalse => Ok(Type::Bool),
            Self::SizeOf(_) => Ok(Type::I32),
            Self::LDFtn(ftn) => Ok(Type::DelegatePtr(Box::new(ftn.signature().clone()))),
            _ => todo!("Can't check the type safety of {self:?}"),
        }
    }*/
}

#[macro_export]
macro_rules! and {
    ($a:expr,$b:expr) => {
        CILNode::And($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! shr {
    ($a:expr,$b:expr) => {
        CILNode::Shr($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! shl {
    ($a:expr,$b:expr) => {
        CILNode::Shl($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! shr_un {
    ($a:expr,$b:expr) => {
        CILNode::ShrUn($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! or {
    ($a:expr,$b:expr) => {
        CILNode::Or($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! xor {
    ($a:expr,$b:expr) => {
        CILNode::XOr($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! div {
    ($a:expr,$b:expr) => {
        CILNode::Div($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! rem {
    ($a:expr,$b:expr) => {
        CILNode::Rem($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! rem_un {
    ($a:expr,$b:expr) => {
        CILNode::RemUn($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! sub {
    ($a:expr,$b:expr) => {
        CILNode::Sub($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! mul {
    ($a:expr,$b:expr) => {
        CILNode::Mul($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! eq {
    ($a:expr,$b:expr) => {
        CILNode::Eq($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! lt {
    ($a:expr,$b:expr) => {
        CILNode::Lt($a.into(), $b.into())
    };
}

#[macro_export]
macro_rules! lt_un {
    ($a:expr,$b:expr) => {
        CILNode::LtUn($a.into(), $b.into())
    };
}
#[macro_export]
macro_rules! gt {
    ($a:expr,$b:expr) => {
        CILNode::Gt($a.into(), $b.into())
    };
}

#[macro_export]
macro_rules! gt_un {
    ($a:expr,$b:expr) => {
        CILNode::GtUn($a.into(), $b.into())
    };
}

#[macro_export]
macro_rules! size_of {
    ($a:expr) => {
        CILNode::SizeOf($a.into())
    };
}
#[macro_export]
macro_rules! ld_field {
    ($addr_calc:expr,$field:expr) => {
        CILNode::LDField {
            addr: $addr_calc.into(),
            field: $field.into(),
        }
    };
}
#[macro_export]
macro_rules! ld_field_address {
    ($addr_calc:expr,$field:expr) => {
        CILNode::LDFieldAdress {
            addr: $addr_calc.into(),
            field: $field.into(),
        }
    };
}
#[macro_export]
macro_rules! call {
    ($call_site:expr,$args:expr) => {
        CILNode::Call {
            args: $args.into(),
            site: $call_site.into(),
        }
    };
}
#[macro_export]
macro_rules! call_virt {
    ($call_site:expr,$args:expr) => {
        CILNode::CallVirt {
            args: $args.into(),
            site: $call_site.into(),
        }
    };
}
#[macro_export]
macro_rules! conv_usize {
    ($a:expr) => {
        CILNode::ZeroExtendToUSize($a.into())
    };
}
#[macro_export]
macro_rules! conv_isize {
    ($a:expr) => {
        CILNode::ConvISize($a.into())
    };
}
#[macro_export]
macro_rules! conv_u64 {
    ($a:expr) => {
        CILNode::ConvU64($a.into())
    };
}
#[macro_export]
macro_rules! conv_i64 {
    ($a:expr) => {
        CILNode::ConvI64($a.into())
    };
}
#[macro_export]
macro_rules! conv_u32 {
    ($a:expr) => {
        CILNode::ConvU32($a.into())
    };
}
#[macro_export]
macro_rules! conv_i32 {
    ($a:expr) => {
        CILNode::ConvI32($a.into())
    };
}
#[macro_export]
macro_rules! conv_u16 {
    ($a:expr) => {
        CILNode::ConvU16($a.into())
    };
}
#[macro_export]
macro_rules! conv_i16 {
    ($a:expr) => {
        CILNode::ConvI16($a.into())
    };
}
#[macro_export]
macro_rules! conv_i8 {
    ($a:expr) => {
        CILNode::ConvI8($a.into())
    };
}
#[macro_export]
macro_rules! conv_u8 {
    ($a:expr) => {
        CILNode::ConvU8($a.into())
    };
}

#[macro_export]
macro_rules! conv_f32 {
    ($a:expr) => {
        CILNode::ConvF32($a.into())
    };
}

#[macro_export]
macro_rules! conv_f64 {
    ($a:expr) => {
        CILNode::ConvF64($a.into())
    };
}
#[macro_export]
macro_rules! conv_f_un {
    ($a:expr) => {
        CILNode::ConvF64Un($a.into())
    };
}
/// Loads a value of type `i32`.
#[macro_export]
macro_rules! ldc_i32 {
    ($val:expr) => {
        CILNode::LdcI32($val)
    };
}
/// Loads a value of type `i64`.
#[macro_export]
macro_rules! ldc_i64 {
    ($val:expr) => {
        CILNode::LdcI64($val)
    };
}
/// Loads a value of type `u32`.
#[macro_export]
macro_rules! ldc_u32 {
    ($val:expr) => {
        CILNode::LdcU32($val)
    };
}
/// Loads a value of type `u64`.
#[macro_export]
macro_rules! ldc_u64 {
    ($val:expr) => {
        CILNode::LdcU64($val)
    };
}
impl std::ops::Add<Self> for CILNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(self.into(), rhs.into())
    }
}
impl std::ops::Sub<Self> for CILNode {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(self.into(), rhs.into())
    }
}
impl std::ops::Mul<Self> for CILNode {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(self.into(), rhs.into())
    }
}
impl std::ops::BitOr<Self> for CILNode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        or!(self, rhs)
    }
}

impl std::ops::Neg for CILNode {
    fn neg(self) -> Self::Output {
        Self::Neg(self.into())
    }

    type Output = Self;
}
