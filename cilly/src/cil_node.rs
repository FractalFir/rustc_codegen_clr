use crate::{
    call, call_site::CallSite, cil_iter::CILIterTrait, cil_root::CILRoot,
    field_desc::FieldDescriptor, fn_sig::FnSig, ptr, static_field_desc::StaticFieldDescriptor,
    AsmStringContainer, DotnetTypeRef, IString, Type,
};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
/// A container for the arguments of a call, callvirt, or newobj instruction.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Debug)]
pub struct CallOpArgs {
    pub args: Box<[CILNode]>,
    pub site: Box<CallSite>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
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
    // 24 bytes, consider shrinking?
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
    // 24 bytes, consider shrinking?
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
    /// Loads the address of `field` of the object `addr` points to
    LDFieldAdress {
        /// Address of the object
        addr: Box<Self>,
        field: Box<FieldDescriptor>,
    },
    /// Loads the value of `field` of the object `addr` points to
    LDField {
        /// Address of the object
        addr: Box<Self>,
        field: Box<FieldDescriptor>,
    },
    /// Adds 2 values together
    Add(Box<Self>, Box<Self>),
    /// Binary-ands 2 values together
    And(Box<Self>, Box<Self>),
    /// Subtracts lhs from rhs
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

    Call(Box<CallOpArgs>),

    CallVirt(Box<CallOpArgs>),
    LdcI64(i64),
    LdcU64(u64),
    LdcU8(u8),
    LdcU16(u16),
    LdcI32(i32),
    LdcU32(u32),
    LdcI8(i8),
    LdcI16(i16),
    LdcF64(OrderedFloat<f64>),
    LdcF32(OrderedFloat<f32>),
    LoadGlobalAllocPtr {
        alloc_id: u64,
    },
    ConvU8(Box<Self>),
    ConvU16(Box<Self>),
    ConvU32(Box<Self>),
    ZeroExtendToU64(Box<Self>),
    ZeroExtendToUSize(Box<Self>),
    ZeroExtendToISize(Box<Self>),
    MRefToRawPtr(Box<Self>),
    ConvI8(Box<Self>),
    ConvI16(Box<Self>),
    ConvI32(Box<Self>),
    SignExtendToI64(Box<Self>),
    SignExtendToU64(Box<Self>),
    SignExtendToISize(Box<Self>),
    SignExtendToUSize(Box<Self>),
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

    SubTrees(Box<(Box<[CILRoot]>, Box<Self>)>),
    LoadAddresOfTMPLocal,
    LoadTMPLocal,
    LDFtn(Box<CallSite>),
    LDTypeToken(Box<Type>),
    NewObj(Box<CallOpArgs>),
    // 24 bytes - too big!
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
    // 24 bytes - too big!
    LDElelemRef {
        arr: Box<Self>,
        idx: Box<Self>,
    },
    PointerToConstValue(Box<u128>),

    /// Tells the codegen a pointer value type is changed. Used during verification, to implement things like`transmute`.
    CastPtr {
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
    /// Allocates a local buffer of size
    LocAlloc {
        size: Box<Self>,
    },
    /// Gets the exception. Can only be used in handlers, only once per handler.
    GetException,
    /// Checks if `lhs` is of type `rhs`. If not, throws.
    CheckedCast(Box<(CILNode, DotnetTypeRef)>),
    // Checks if `lhs` is of type `rhs`.  Returns a boolean.
    IsInst(Box<(CILNode, DotnetTypeRef)>),
    /// Marks the inner pointer operation as volatile.
    Volatile(Box<Self>),
    UnboxAny(Box<Self>, Box<Type>),
    AddressOfStaticField(Box<StaticFieldDescriptor>),
}

impl CILNode {
    /// Allocates a GC handle to the object, and converts that handle to a nint sized handleID.
    pub fn managed_ref_to_handle(self) -> Self {
        let gc_handle = call!(
            CallSite::new(
                Some(DotnetTypeRef::gc_handle()),
                "Alloc".to_owned().into(),
                FnSig::new(
                    &[Type::DotnetType(Box::new(DotnetTypeRef::object_type()))],
                    Type::DotnetType(Box::new(DotnetTypeRef::gc_handle()))
                ),
                true
            ),
            [self]
        );
        call!(
            CallSite::new(
                Some(DotnetTypeRef::gc_handle()),
                "op_Explicit".to_owned().into(),
                FnSig::new(
                    &[Type::DotnetType(Box::new(DotnetTypeRef::gc_handle()))],
                    Type::ISize
                ),
                true,
            ),
            [gc_handle]
        )
    }
    pub fn gc_handle_to_obj(self, obj: DotnetTypeRef) -> Self {
        let gc_handle = call!(
            CallSite::new(
                Some(DotnetTypeRef::gc_handle()),
                "FromIntPtr".to_owned().into(),
                FnSig::new(
                    &[Type::ISize],
                    Type::DotnetType(Box::new(DotnetTypeRef::gc_handle()))
                ),
                true
            ),
            [self]
        );
        let gc_handle = CILNode::TemporaryLocal(Box::new((
            Type::DotnetType(Box::new(DotnetTypeRef::gc_handle())),
            [CILRoot::SetTMPLocal { value: gc_handle }].into(),
            CILNode::LoadAddresOfTMPLocal,
        )));
        let object = call!(
            CallSite::new(
                Some(DotnetTypeRef::gc_handle()),
                "get_Target".to_owned().into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::DotnetType(
                        Box::new(DotnetTypeRef::gc_handle())
                    )))],
                    Type::DotnetType(Box::new(DotnetTypeRef::object_type()))
                ),
                false,
            ),
            [gc_handle]
        );
        CILNode::CheckedCast(Box::new((object, obj)))
    }
    #[must_use]
    pub fn select(tpe: Type, a: Self, b: Self, predictate: Self) -> Self {
        match tpe {
            Type::U128 => call!(
                CallSite::builtin(
                    "select_u128".to_owned().into(),
                    FnSig::new(&[Type::U128, Type::U128, Type::Bool], Type::U128),
                    true
                ),
                [a, b, predictate]
            ),
            Type::I128 => call!(
                CallSite::builtin(
                    "select_i128".to_owned().into(),
                    FnSig::new(&[Type::I128, Type::I128, Type::Bool], Type::I128),
                    true
                ),
                [a, b, predictate]
            ),
            Type::USize => call!(
                CallSite::builtin(
                    "select_usize".to_owned().into(),
                    FnSig::new(&[Type::USize, Type::USize, Type::Bool], Type::USize),
                    true
                ),
                [a, b, predictate]
            ),
            Type::Ptr(_) => call!(
                CallSite::builtin(
                    "select_usize".to_owned().into(),
                    FnSig::new(&[Type::USize, Type::USize, Type::Bool], Type::USize),
                    true
                ),
                [a.cast_ptr(Type::USize), b.cast_ptr(Type::USize), predictate]
            )
            .cast_ptr(tpe),
            Type::ISize => call!(
                CallSite::builtin(
                    "select_isize".to_owned().into(),
                    FnSig::new(&[Type::ISize, Type::ISize, Type::Bool], Type::ISize),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U64 => call!(
                CallSite::builtin(
                    "select_u64".to_owned().into(),
                    FnSig::new(&[Type::U64, Type::U64, Type::Bool], Type::U64),
                    true
                ),
                [a, b, predictate]
            ),
            Type::I64 => call!(
                CallSite::builtin(
                    "select_i64".to_owned().into(),
                    FnSig::new(&[Type::I64, Type::I64, Type::Bool], Type::I64),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U32 => call!(
                CallSite::builtin(
                    "select_u32".to_owned().into(),
                    FnSig::new(&[Type::U32, Type::U32, Type::Bool], Type::U32),
                    true
                ),
                [a, b, predictate]
            ),
            Type::I32 => call!(
                CallSite::builtin(
                    "select_i32".to_owned().into(),
                    FnSig::new(&[Type::I32, Type::I32, Type::Bool], Type::I32),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U16 => call!(
                CallSite::builtin(
                    "select_u16".to_owned().into(),
                    FnSig::new(&[Type::U16, Type::U16, Type::Bool], Type::U16),
                    true
                ),
                [a, b, predictate]
            ),
            Type::I16 => call!(
                CallSite::builtin(
                    "select_i16".to_owned().into(),
                    FnSig::new(&[Type::I16, Type::I16, Type::Bool], Type::I16),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U8 | Type::Bool => call!(
                CallSite::builtin(
                    "select_u8".to_owned().into(),
                    FnSig::new(&[Type::U8, Type::U8, Type::Bool], Type::U8),
                    true
                ),
                [a, b, predictate]
            ),
            Type::I8 => call!(
                CallSite::builtin(
                    "select_i8".to_owned().into(),
                    FnSig::new(&[Type::I8, Type::I8, Type::Bool], Type::I8),
                    true
                ),
                [a, b, predictate]
            ),
            _ => todo!("Can't select type {tpe:?}"),
        }
    }
    fn opt_children(&mut self, opt_count: &mut usize) {
        match self {
            Self::UnboxAny(val,_tpe )=>val.opt(opt_count),
            Self::CheckedCast(inner)=>inner.0.opt(opt_count),
            Self::Volatile(inner)=>inner.opt(opt_count),
            Self::IsInst(inner)=>inner.0.opt(opt_count),
            Self::GetException=>(),
            Self::LocAlloc{size}=>size.opt(opt_count),
            Self::LocAllocAligned { .. }=>(),
            Self::LdFalse | Self::LdTrue=>(),
            Self::CastPtr { val, new_ptr: _ }=>val.opt(opt_count),
            Self::LDLoc(_)| Self::LDArg(_) | Self::LDLocA(_) | Self::LDArgA(_)=> (),
            Self::BlackBox(inner)
            | Self::ConvF32(inner)
            | Self::ConvF64(inner)
            | Self::ConvF64Un(inner) => inner.opt(opt_count),
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
            | Self::LDIndF64 { ptr } => ptr.opt(opt_count),
            Self::LDFieldAdress { addr, field: _ } | Self::LDField { addr, field: _ } => addr.opt(opt_count),
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
                a.opt(opt_count);
                b.opt(opt_count);
            }
            Self::Call (call_op_args) | Self::NewObj(call_op_args) | Self::CallVirt(call_op_args)=> call_op_args.args.iter_mut().for_each(|arg|arg.opt(opt_count)),
            Self::LdcI64(_)
            | Self::LdcU64(_)
            | Self::LdcI32(_)
            | Self::LdcI8(_)
            | Self::LdcU8(_)
            | Self::LdcI16(_)
            | Self::LdcU32(_)
            | Self::LdcU16(_)
            | Self::LdcF64(_)
            | Self::LdcF32(_)
            | Self::LoadGlobalAllocPtr { .. }
            | Self::PointerToConstValue(_)=> (),
            Self::ConvU8(inner)
            | Self::ConvU16(inner)
            | Self::ConvU32(inner)
            | Self::ZeroExtendToU64(inner)
            | Self::ZeroExtendToUSize(inner)
            | Self::ZeroExtendToISize(inner)
            | Self::MRefToRawPtr(inner)
            | Self::ConvI8(inner)
            | Self::ConvI16(inner)
            | Self::ConvI32(inner)
            | Self::SignExtendToI64(inner)
            | Self::SignExtendToU64(inner)
            | Self::SignExtendToISize(inner)
            | Self::SignExtendToUSize(inner)
            //| Self::Volatile(inner)
            | Self::Neg(inner)
            | Self::Not(inner) => inner.opt(opt_count),
            Self::TemporaryLocal(_inner) => (),
            Self::SubTrees(sub_trees) => {
                let (a,b) = sub_trees.as_mut();
                a.iter_mut().for_each(|root|root.opt(opt_count));
                b.opt(opt_count);
            }
            Self::LoadAddresOfTMPLocal => (),
            Self::LoadTMPLocal => (),
            Self::LDFtn(_) => (),
            Self::LDTypeToken(_) => (),
            Self::LdStr(_) => (),
            Self::CallI (ptr_sig_arg ) => {
                ptr_sig_arg.2.iter_mut().for_each(|arg|arg.opt(opt_count));
                ptr_sig_arg.1.opt(opt_count);
            }
            Self::LDStaticField(_static_field) => (),
            Self::AddressOfStaticField(_static_field) => (),
            Self::LDLen { arr } => arr.opt(opt_count),
            Self::LDElelemRef { arr, idx } =>{
                idx.opt(opt_count);
                arr.opt(opt_count);
            },
        }
    }
    /// Checks if this node may have side effects. `false` means that the node can't have side effects, `true` means that the node *may* have side effects, but it does not have to.
    pub fn has_side_effects(&self) -> bool {
        let contains_calls = self.into_iter().call_sites().next().is_some();
        if contains_calls {
            return true;
        }
        self.into_iter().any(|node| {
            matches!(
                node,
                crate::cil_iter::CILIterElem::Node(
                    CILNode::SubTrees(_) | CILNode::TemporaryLocal(_) | CILNode::GetException
                )
            )
        })
    }
    // This fucntion will get expanded, so a single match is a non-issue.
    #[allow(clippy::single_match)]
    // This clippy lint is wrong
    #[allow(clippy::assigning_clones)]
    /// Optimizes this `CILNode`.
    pub fn opt(&mut self, opt_count: &mut usize) {
        self.opt_children(opt_count);
        match self {
            Self::LDField { addr: fld_addr, .. } => match fld_addr.as_mut() {
                Self::MRefToRawPtr(addr) => match addr.as_mut() {
                    Self::LDLocA(_) | Self::LDFieldAdress { .. } => {
                        *fld_addr = addr.clone();
                        *opt_count += 1;
                    }
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
                        *opt_count += 1;
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    pub fn cast_ptr(self, new_ptr: Type) -> Self {
        assert!(
            matches!(
                new_ptr,
                Type::Ptr(_)
                    | Type::ManagedReference(_)
                    | Type::DelegatePtr(_)
                    | Type::USize
                    | Type::ISize
            ),
            "Invalid new ptr {new_ptr:?}"
        );
        Self::CastPtr {
            val: Box::new(self),
            new_ptr: Box::new(new_ptr),
        }
    }
    pub(crate) fn allocate_tmps(
        &mut self,
        curr_loc: Option<u32>,
        locals: &mut Vec<(Option<IString>, Type)>,
    ) {
        match self {
            Self::UnboxAny(val,_tpe )=>val.allocate_tmps(curr_loc, locals),
            Self::Volatile(inner)=>inner.allocate_tmps(curr_loc, locals),
            Self::CheckedCast(inner)=>inner.0.allocate_tmps(curr_loc, locals),
            Self::IsInst(inner)=>inner.0.allocate_tmps(curr_loc, locals),
            Self::GetException=>(),
            Self::LocAlloc{..}=>(),
            Self::LocAllocAligned {..}=>(),
            Self::LdFalse=>(),
            Self::LdTrue=>(),
            Self::CastPtr { val, new_ptr: _ }=>val.allocate_tmps(curr_loc, locals),
            Self:: PointerToConstValue(_arr)=>(),
            Self::LoadGlobalAllocPtr { alloc_id: _ } => (),
            Self::LDLoc(_) |
            Self::LDArg(_) |
            Self::LDLocA(_)|
            Self::LDArgA(_) => (),
            Self::BlackBox(inner) => inner.allocate_tmps(curr_loc, locals),
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
            Self::Call (call_op_args)  |  Self::CallVirt (call_op_args)  |  Self::NewObj (call_op_args) =>call_op_args.args.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals)),
            Self::LdcI64(_) |
            Self::LdcU64(_) |
            Self::LdcI32(_)  |
            Self::LdcU8(_) |
            Self::LdcU16(_) |
            Self::LdcU32(_) |
            Self::LdcI16(_) |
            Self::LdcI8(_) |
            Self::LdcF64(_) |
            Self::LdcF32(_) =>(),
            Self::ConvF64Un(val) |
            Self::ConvF32(val)|
            Self::ConvF64(val) |
            Self::ConvU8(val)|
            Self::ConvU16(val)|
            Self::ConvU32(val)|
            Self::ZeroExtendToU64(val)|
            Self::MRefToRawPtr(val) |
            Self::ZeroExtendToUSize(val)|
            Self::ZeroExtendToISize(val)|
            Self::ConvI8(val) |
            Self::ConvI16(val)|
            Self::ConvI32(val)|
            Self::SignExtendToI64(val) |
            Self::SignExtendToU64(val) |
            Self::SignExtendToISize(val)|
            Self::SignExtendToUSize(val)|
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
                debug_assert!(
                    !(&*main).into_iter().any(|node| {
                        matches!(node, crate::cil_iter::CILIterElem::Node(
                                crate::cil_node::CILNode::TemporaryLocal(_),
                            ))
                    }),
                    "self:{self:?}"
                );
                debug_assert!(
                    !roots.iter().flat_map(|root|root.into_iter()).any(|node| {
                        matches!(node, crate::cil_iter::CILIterElem::Node(
                                crate::cil_node::CILNode::TemporaryLocal(_),
                            ))
                    }),
                    "self:{self:?}"
                );
                *self=  Self::SubTrees(Box::new((roots.clone(), Box::new(main.clone()))));
            },
            Self::SubTrees(sub_trees) =>{
                let (trees, main) = sub_trees.as_mut();
                trees.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc,locals));
                main.allocate_tmps(curr_loc, locals);
            }
            Self::LoadAddresOfTMPLocal => *self = Self::LDLocA(curr_loc.expect("Temporary local referenced when none present")),
            Self::LoadTMPLocal =>*self = Self::LDLoc(curr_loc.expect("Temporary local referenced when none present")),
            Self::LDFtn(_) => (),
            Self::LDTypeToken(_) =>(),

            Self::LdStr(_) => (),
            Self::CallI (sig_ptr_args) => {
                sig_ptr_args.1.allocate_tmps(curr_loc, locals);
                sig_ptr_args.2.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals));
            }
            Self::LDStaticField(_sfield)=>(),
            Self::AddressOfStaticField(_sfield)=>(),
            Self::LDLen { arr } =>{
               arr.allocate_tmps(curr_loc, locals);
            }
            Self::LDElelemRef { arr, idx }=>{
                arr.allocate_tmps(curr_loc, locals);
                idx.allocate_tmps(curr_loc, locals);
            }
        };
    }
    pub fn is_valid_dbg(
        self,
        vctx: ValidationContext,
        tmp_loc: Option<&Type>,
    ) -> Result<Self, String> {
        // TODO: make this check debug only
        self.validate(vctx, tmp_loc)?;
        Ok(self)
    }
    pub fn validate(
        &self,
        vctx: ValidationContext,
        tmp_loc: Option<&Type>,
    ) -> Result<Type, String> {
        match self {
            Self::LDTypeToken(_) => Ok(Type::DotnetType(Box::new(
                DotnetTypeRef::type_handle_type(),
            ))),
            Self::SubTrees(tm) => {
                let (trees, main) = tm.as_ref();
                for tree in trees.iter() {
                    tree.validate(vctx, tmp_loc)?;
                }
                main.validate(vctx, tmp_loc)
            }
            Self::LdTrue => Ok(Type::Bool),
            Self::LDField { addr, field } => {
                let addr = addr.validate(vctx, tmp_loc)?;
                match addr {
                    Type::ManagedReference(tpe) | Type::Ptr(tpe) => {
                        if tpe.as_dotnet() != Some(field.owner().clone()) {
                            return Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}",
                                field = field.owner().clone()
                            ));
                        }
                    }
                    Type::DotnetType(tpe) => {
                        if tpe.as_ref() != field.owner() {
                            return Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}",
                                field = field.owner().clone()
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
                let addr = addr.validate(vctx, tmp_loc)?;
                match addr {
                    Type::ManagedReference(tpe) => {
                        if tpe.as_dotnet() != Some(field.owner().clone()) {
                            Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}",
                                field = field.owner().clone()
                            ))
                        } else {
                            Ok(Type::ManagedReference(Box::new(field.tpe().clone())))
                        }
                    }
                    Type::Ptr(tpe) => {
                        if tpe.as_dotnet() != Some(field.owner().clone()) {
                            Err(format!(
                                "Mismatched pointer type. Expected {field:?} got {tpe:?}",
                                field = field.owner().clone()
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
            Self::AddressOfStaticField(sfd) => {
                Ok(Type::ManagedReference(Box::new(sfd.tpe().clone())))
            }
            Self::LDLocA(loc) => match vctx.locals().get(*loc as usize) {
                Some(local) => Ok(Type::ManagedReference(Box::new(local.1.clone()))),
                None => Err(format!("Local {loc} out of range.")),
            },
            Self::LDLoc(loc) => match vctx.locals().get(*loc as usize) {
                Some(local) => Ok(local.1.clone()),
                None => Err(format!("Local {loc} out of range.")),
            },
            Self::LDArg(arg) => match vctx.sig().inputs().get(*arg as usize) {
                Some(arg) => Ok(arg.clone()),
                None => Err(format!("Argument {arg} out of range.")),
            },
            Self::LDArgA(arg) => match vctx.sig().inputs().get(*arg as usize) {
                Some(arg) => Ok(Type::ManagedReference(Box::new(arg.clone()))),
                None => Err(format!("Argument {arg} out of range.")),
            },
            Self::LdObj { ptr, obj } => {
                if **obj == Type::Void {
                    return Err("Using ldobj to load void is not allowed".to_owned());
                }
                let ptr = ptr.validate(vctx, tmp_loc)?;
                match ptr {
                    Type::Ptr(pointed) | Type::ManagedReference(pointed) => {
                        if pointed != *obj {
                            if matches!(*pointed, Type::I128) {
                                return Ok(Type::I128);
                            }
                            if matches!(*pointed, Type::U128) {
                                return Ok(Type::U128);
                            }
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
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::ISize))
                    && ptr != Type::ManagedReference(Box::new(Type::ISize))
                {
                    return Err(format!("Tried to load isize from pointer of type {ptr:?}"));
                }
                Ok(Type::ISize)
            }
            Self::LDIndPtr { ptr, loaded_ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
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
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::USize))
                    && ptr != Type::ManagedReference(Box::new(Type::USize))
                {
                    return Err(format!("Tried to load usize from pointer of type {ptr:?}"));
                }
                Ok(Type::USize)
            }
            Self::LDIndU32 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::U32))
                    && ptr != Type::ManagedReference(Box::new(Type::U32))
                {
                    return Err(format!("Tried to load isize from pointer of type {ptr:?}"));
                }
                Ok(Type::U32)
            }
            Self::LDIndI32 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::I32))
                    && ptr != Type::ManagedReference(Box::new(Type::I32))
                {
                    return Err(format!("Tried to load i32 from pointer of type {ptr:?}"));
                }
                Ok(Type::I32)
            }
            Self::LDIndF32 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::F32))
                    && ptr != Type::ManagedReference(Box::new(Type::F32))
                {
                    return Err(format!("Tried to load f32 from pointer of type {ptr:?}"));
                }
                Ok(Type::F32)
            }
            Self::LDIndF64 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::F64))
                    && ptr != Type::ManagedReference(Box::new(Type::F64))
                {
                    return Err(format!("Tried to load f64 from pointer of type {ptr:?}"));
                }
                Ok(Type::F64)
            }
            Self::LDIndU16 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::U16))
                    && ptr != Type::ManagedReference(Box::new(Type::U16))
                {
                    return Err(format!("Tried to load u16 from pointer of type {ptr:?}"));
                }
                Ok(Type::U16)
            }
            Self::LDIndI16 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::I16))
                    && ptr != Type::ManagedReference(Box::new(Type::I16))
                {
                    return Err(format!("Tried to load i16 from pointer of type {ptr:?}"));
                }
                Ok(Type::I16)
            }
            Self::LDIndU8 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::U8))
                    && ptr != Type::ManagedReference(Box::new(Type::U8))
                {
                    return Err(format!("Tried to load u8 from pointer of type {ptr:?}"));
                }
                Ok(Type::U8)
            }
            Self::LDIndI8 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::I8))
                    && ptr != Type::ManagedReference(Box::new(Type::I8))
                {
                    return Err(format!("Tried to load i8 from pointer of type {ptr:?}"));
                }
                Ok(Type::I8)
            }
            Self::LDIndI64 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::I64))
                    && ptr != Type::ManagedReference(Box::new(Type::I64))
                {
                    return Err(format!("Tried to load i8 from pointer of type {ptr:?}"));
                }
                Ok(Type::I64)
            }
            Self::LDIndU64 { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::U64))
                    && ptr != Type::ManagedReference(Box::new(Type::U64))
                {
                    return Err(format!("Tried to load i8 from pointer of type {ptr:?}"));
                }
                Ok(Type::U64)
            }
            Self::LDIndBool { ptr } => {
                let ptr = ptr.validate(vctx, tmp_loc)?;
                if ptr != Type::Ptr(Box::new(Type::Bool))
                    && ptr != Type::ManagedReference(Box::new(Type::Bool))
                {
                    return Err(format!("Tried to load bool from pointer of type {ptr:?}"));
                }
                Ok(Type::Bool)
            }
            Self::LdcU64(_) => Ok(Type::U64),
            Self::LdcI64(_) => Ok(Type::I64),
            Self::LdcU16(_) => Ok(Type::U16),
            Self::LdcU32(_) => Ok(Type::U32),
            Self::LdcU8(_) => Ok(Type::U8),
            Self::LdcI32(_) => Ok(Type::I32),
            Self::LdcI8(_) => Ok(Type::I8),
            Self::LdcI16(_) => Ok(Type::I16),
            Self::LdcF32(_) => Ok(Type::F32),
            Self::LdcF64(_) => Ok(Type::F64),
            Self::SignExtendToISize(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::ISize)
            }
            Self::SignExtendToI64(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::I64)
            }
            Self::SignExtendToU64(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::U64)
            }
            Self::ZeroExtendToUSize(src) => {
                let _tpe = src.validate(vctx, tmp_loc)?;
                Ok(Type::USize)
            }
            Self::ZeroExtendToISize(src) => {
                let _tpe = src.validate(vctx, tmp_loc)?;
                Ok(Type::ISize)
            }
            Self::SignExtendToUSize(src) => {
                let _tpe = src.validate(vctx, tmp_loc)?;
                Ok(Type::USize)
            }

            Self::MRefToRawPtr(src) => {
                let tpe = src.validate(vctx, tmp_loc)?;
                if let Type::ManagedReference(pointed) = tpe {
                    Ok(Type::Ptr(pointed))
                } else if let Type::Ptr(pointed) = tpe {
                    Ok(Type::Ptr(pointed))
                } else {
                    Err(format!(
                        "MRefToRawPtr expected a managed ref, but got {tpe:?}"
                    ))
                }
            }
            Self::ZeroExtendToU64(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::U64)
            }
            Self::ConvU32(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::U32)
            }
            Self::ConvI32(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::I32)
            }
            Self::ConvF32(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::F32)
            }
            Self::ConvF64(src) | Self::ConvF64Un(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::F64)
            }
            Self::ConvU16(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::U16)
            }
            Self::ConvI16(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::I16)
            }
            Self::ConvU8(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::U8)
            }
            Self::ConvI8(src) => {
                src.validate(vctx, tmp_loc)?;
                Ok(Type::I8)
            }
            Self::LtUn(a, b)
            | Self::Lt(a, b)
            | Self::GtUn(a, b)
            | Self::Gt(a, b)
            | Self::Eq(a, b) => {
                let a = a.validate(vctx, tmp_loc)?;
                let b = b.validate(vctx, tmp_loc)?;
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
                let a = a.validate(vctx, tmp_loc)?;
                let b = b.validate(vctx, tmp_loc)?;
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
                let a = a.validate(vctx, tmp_loc)?;
                if !a.is_primitive_numeric() {
                    return Err(format!(
                        "The instruction Add can't operate on a non-primitve CIL type {a:?}."
                    ));
                }
                Ok(a)
            }
            Self::Shr(a, b) | Self::ShrUn(a, b) | Self::Shl(a, b) => {
                let a = a.validate(vctx, tmp_loc)?;
                let b = b.validate(vctx, tmp_loc)?;
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
            Self::PointerToConstValue(_) => Ok(Type::Ptr(Box::new(Type::U8))),
            Self::LdStr(_) => Ok(Type::DotnetType(DotnetTypeRef::string_type().into())),
            Self::LoadGlobalAllocPtr { alloc_id: _ } => Ok(Type::Ptr(Box::new(Type::U8))),
            Self::NewObj(call_op_args) => {
                if call_op_args.site.explicit_inputs().len() != call_op_args.args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        call_op_args.site.explicit_inputs().len(),
                        call_op_args.args.len()
                    ));
                }
                for (arg, tpe) in call_op_args
                    .args
                    .iter()
                    .zip(call_op_args.site.explicit_inputs().iter())
                {
                    let arg = arg.validate(vctx, tmp_loc)?;
                    if arg != *tpe {
                        match (&arg, &tpe) {
                            (Type::ManagedReference(arg), Type::Ptr(tpe)) => {
                                if arg != tpe {
                                    return Err(format!(
                                        "Expected an argument of type {tpe:?}, but got {arg:?} when calling the ctor of {class:?}", class = call_op_args.site.class() 
                                    ));
                                }
                            }
                            (Type::DelegatePtr(_),Type::ISize)=>(),
                            (Type::DotnetType(ty_a), Type::DotnetType(ty_b)) if !ty_a.is_valuetype() && !ty_b.is_valuetype() =>(),
                            _ => {
                                return Err(format!(
                                    "Expected an argument of type {tpe:?}, but got {arg:?} when calling the ctor of {class:?}", class = call_op_args.site.class()
                                ))
                            }
                        };
                    }
                }
                match call_op_args.site.class() {
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
                    None => Err("Newobj instruction witn no class specified"
                        .to_owned()
                        .into()),
                }
            }
            Self::Call(call_op_args) | Self::CallVirt(call_op_args) => {
                if call_op_args.site.inputs().len() != call_op_args.args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        call_op_args.site.inputs().len(),
                        call_op_args.args.len()
                    ));
                }
                for (arg_node, arg_tpe) in call_op_args
                    .args
                    .iter()
                    .zip(call_op_args.site.inputs().iter())
                {
                    let got = arg_node.validate(vctx, tmp_loc)?;

                    if got != *arg_tpe {
                        if matches!(arg_tpe, Type::USize) && matches!(got, Type::Ptr(_)) {
                            continue;
                        }
                        if matches!(arg_tpe, Type::GenericArg(_)) {
                            continue;
                        }
                        if (matches!(arg_tpe, Type::ManagedReference(_))
                            && matches!(got, Type::Ptr(_)))
                            || (matches!(arg_tpe, Type::Ptr(_))
                                && matches!(got, Type::ManagedReference(_)))
                        {
                            // TODO: check the mref and ptr point to the same type.
                            continue;
                        }
                        if let Type::DotnetType(dt) = arg_tpe {
                            if dt.name_path().contains("System.Object") {
                                continue;
                            }
                        }
                        return Err(format!(
                            "Expected an argument of type {arg_tpe:?}, but got {got:?} when calling {name}",name = call_op_args.site.name()
                        ));
                    }
                }
                match call_op_args.site.signature().output() {
                    Type::GenericArg(arg) => {
                        Ok(call_op_args.site.class().unwrap().generics()[*arg as usize].clone())
                    }
                    _ => Ok(call_op_args.site.signature().output().clone()),
                }
            }
            Self::CallI(packed) => {
                let (sig, ptr, args) = packed.as_ref();
                let _ptr = ptr.validate(vctx, tmp_loc)?;
                if sig.inputs().len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        sig.inputs().len(),
                        args.len()
                    ));
                }
                for (arg, tpe) in args.iter().zip(sig.inputs().iter()) {
                    let arg = arg.validate(vctx, tmp_loc)?;
                    if arg != *tpe {
                        return Err(format!(
                            "Expected an argument of type {tpe:?}, but got {arg:?} in indirect call."
                        ));
                    }
                }
                Ok(sig.output().clone())
            }
            Self::CastPtr { val, new_ptr } => {
                let val = val.validate(vctx, tmp_loc)?;
                match val {
                    Type::USize => (),
                    Type::ISize => (),
                    Type::Ptr(_) => (),
                    // ManagedReference is accepted, because sometimes we use this to transmute local
                    Type::ManagedReference(_) => (),
                    Type::DelegatePtr(_) => (),
                    _ => {
                        return Err(format!(
                            "Invalid CastPtr input: {val:?} is not a pointer or usize/isize"
                        ))
                    }
                }
                Ok(*new_ptr.clone())
            }
            Self::LdFalse => Ok(Type::Bool),
            Self::SizeOf(_) => Ok(Type::I32),
            Self::LDFtn(ftn) => Ok(Type::DelegatePtr(Box::new(ftn.signature().clone()))),
            Self::TemporaryLocal(tmp) => {
                let (tpe, roots, main) = tmp.as_ref();
                for root in roots {
                    root.validate(vctx, Some(tpe))?;
                }
                main.validate(vctx, Some(tpe))
            }
            Self::LoadAddresOfTMPLocal => Ok(Type::ManagedReference(Box::new(
                tmp_loc
                    .cloned()
                    .ok_or_else(|| ("TMP local requred when no tmp locals!".to_string()))?,
            ))),
            Self::LoadTMPLocal => tmp_loc
                .cloned()
                .ok_or_else(|| ("TMP local requred when no tmp locals!".to_string())),
            Self::LDElelemRef { arr, idx } => {
                let arr = arr.validate(vctx, tmp_loc)?;
                let _idx = idx.validate(vctx, tmp_loc)?;
                match arr {
                    Type::ManagedArray { element, dims: _ } => Ok(*element),
                    _ => panic!("{arr:?} is not an array!"),
                }
            }
            Self::LDLen { arr } => {
                let arr = arr.validate(vctx, tmp_loc)?;
                match arr {
                    Type::ManagedArray {
                        element: _,
                        dims: _,
                    } => Ok(Type::I32),
                    _ => panic!("{arr:?} is not an array!"),
                }
            }
            Self::LocAlloc { size } => {
                size.validate(vctx, tmp_loc)?;
                Ok(ptr!(Type::Void))
            }
            Self::LocAllocAligned { tpe, align: _ } => Ok(ptr!(tpe.as_ref().clone())),
            Self::Volatile(inner) => inner.validate(vctx, tmp_loc),
            Self::UnboxAny(val, tpe) => {
                let _val = val.validate(vctx, tmp_loc)?;
                Ok(tpe.as_ref().clone())
            }
            Self::CheckedCast(inner) => {
                let (val, tpe) = inner.as_ref();
                let _val = val.validate(vctx, tmp_loc)?;
                Ok(Type::DotnetType(Box::new(tpe.clone())))
            }
            _ => todo!("Can't check the type safety of {self:?}"),
        }
    }

    pub(crate) fn try_const_eval(&self) -> Option<Self> {
        match self {
            CILNode::LdFalse | CILNode::LdTrue => Some(self.clone()),
            _ => None,
        }
    }
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
    ($a:expr) => {{
        let tmp: Box<Type> = $a.into();
        if *tmp.as_ref() == Type::Void {
            CILNode::LdcI32(0)
        } else {
            CILNode::SizeOf(tmp)
        }
    }};
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
        CILNode::Call(Box::new($crate::cil_node::CallOpArgs {
            args: $args.into(),
            site: $call_site.into(),
        }))
    };
}
#[macro_export]
macro_rules! call_virt {
    ($call_site:expr,$args:expr) => {
        CILNode::CallVirt(Box::new($crate::cil_node::CallOpArgs {
            args: $args.into(),
            site: $call_site.into(),
        }))
    };
}
#[macro_export]
macro_rules! conv_usize {
    ($a:expr) => {
        $crate::cil_node::CILNode::ZeroExtendToUSize($a.into())
    };
}
#[macro_export]
macro_rules! conv_isize {
    ($a:expr) => {
        CILNode::SignExtendToISize($a.into())
    };
}
#[macro_export]
macro_rules! conv_u64 {
    ($a:expr) => {
        CILNode::ZeroExtendToU64($a.into())
    };
}
#[macro_export]
macro_rules! conv_i64 {
    ($a:expr) => {
        CILNode::SignExtendToI64($a.into())
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
/// Loads a false bool.
#[macro_export]
macro_rules! ld_false {
    () => {
        CILNode::LdFalse
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
#[derive(Clone, Copy)]
pub struct ValidationContext<'a> {
    sig: &'a FnSig,
    locals: &'a [(Option<IString>, Type)],
    strings: &'a AsmStringContainer,
}

impl<'a> ValidationContext<'a> {
    pub fn new(
        sig: &'a FnSig,
        locals: &'a [(Option<IString>, Type)],
        strings: &'a AsmStringContainer,
    ) -> Self {
        Self {
            sig,
            locals,
            strings,
        }
    }

    pub fn sig(&self) -> &FnSig {
        self.sig
    }

    pub fn locals(&self) -> &[(Option<IString>, Type)] {
        self.locals
    }

    pub fn strings(&self) -> &AsmStringContainer {
        self.strings
    }
}
