use crate::v2::cilnode::MethodKind;
use crate::v2::method::LocalDef;
use crate::v2::{
    Assembly, ClassRef, ClassRefIdx, FieldIdx, FnSig, Int, MethodRef, MethodRefIdx,
    StaticFieldDesc, Type,
};
use crate::TypeIdx;
use crate::{
    call,
    cil_iter::CILIterTrait,
    cil_root::CILRoot,
    v2::hashable::{HashableF32, HashableF64},
    IString,
};
use serde::{Deserialize, Serialize};
/// A container for the arguments of a call, callvirt, or newobj instruction.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Debug)]
pub struct CallOpArgs {
    pub args: Box<[CILNode]>,
    pub site: MethodRefIdx,
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
    LDStaticField(Box<StaticFieldDesc>),
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
        field: FieldIdx,
    },
    /// Loads the value of `field` of the object `addr` points to
    LDField {
        /// Address of the object
        addr: Box<Self>,
        field: FieldIdx,
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
    LdcF64(HashableF64),
    LdcF32(HashableF32),
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
    TemporaryLocal(Box<(TypeIdx, Box<[CILRoot]>, Self)>),

    SubTrees(Box<(Box<[CILRoot]>, Box<Self>)>),
    LoadAddresOfTMPLocal,
    LoadTMPLocal,
    LDFtn(MethodRefIdx),
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
    CheckedCast(Box<(CILNode, ClassRefIdx)>),
    // Checks if `lhs` is of type `rhs`.  Returns a boolean.
    IsInst(Box<(CILNode, ClassRefIdx)>),
    /// Marks the inner pointer operation as volatile.
    Volatile(Box<Self>),
    UnboxAny(Box<Self>, Box<Type>),
    AddressOfStaticField(Box<StaticFieldDesc>),
    LdNull(ClassRefIdx),
}

impl CILNode {
    pub fn const_u128(value: u128, asm: &mut Assembly) -> CILNode {
        fn u128_low_u64(value: u128) -> u64 {
            u64::try_from(value & u128::from(u64::MAX)).expect("trucating cast error")
        }
        let low = u128_low_u64(value);
        let high = (value >> 64) as u64;
        let ref_u128 = asm.nref(Type::Int(Int::U128));
        let ctor_sig = asm.sig(
            [ref_u128, Type::Int(Int::U64), Type::Int(Int::U64)],
            Type::Void,
        );
        let uint_128 = ClassRef::uint_128(asm);
        let ctor = asm.alloc_string(".ctor");
        CILNode::NewObj(Box::new(CallOpArgs {
            site: asm.alloc_methodref(MethodRef::new(
                uint_128,
                ctor,
                ctor_sig,
                MethodKind::Constructor,
                vec![].into(),
            )),
            args: [
                crate::conv_u64!(crate::ldc_u64!(high)),
                crate::conv_u64!(crate::ldc_u64!(low)),
            ]
            .into(),
        }))
    }
    pub fn const_i128(value: u128, asm: &mut Assembly) -> CILNode {
        fn u128_low_u64(value: u128) -> u64 {
            u64::try_from(value & u128::from(u64::MAX)).expect("trucating cast error")
        }
        let low = u128_low_u64(value);
        let high = (value >> 64) as u64;
        let ref_u128 = asm.nref(Type::Int(Int::U128));
        let ctor_sig = asm.sig(
            [ref_u128, Type::Int(Int::U64), Type::Int(Int::U64)],
            Type::Void,
        );
        let ctor = asm.alloc_string(".ctor");
        let int_128 = ClassRef::int_128(asm);
        CILNode::NewObj(Box::new(CallOpArgs {
            site: asm.alloc_methodref(MethodRef::new(
                int_128,
                ctor,
                ctor_sig,
                MethodKind::Constructor,
                vec![].into(),
            )),
            args: [
                crate::conv_u64!(crate::ldc_u64!(high)),
                crate::conv_u64!(crate::ldc_u64!(low)),
            ]
            .into(),
        }))
    }
    /// Allocates a GC handle to the object, and converts that handle to a nint sized handleID.
    pub fn managed_ref_to_handle(self, asm: &mut Assembly) -> Self {
        let gc_handle_class = Type::ClassRef(ClassRef::gc_handle(asm));
        let mref = MethodRef::new(
            ClassRef::gc_handle(asm),
            asm.alloc_string("Alloc"),
            asm.sig([Type::PlatformObject], gc_handle_class),
            MethodKind::Static,
            vec![].into(),
        );
        let gc_handle = call!(asm.alloc_methodref(mref), [self]);
        let mref = MethodRef::new(
            ClassRef::gc_handle(asm),
            asm.alloc_string("op_Explicit"),
            asm.sig([gc_handle_class], Type::Int(Int::ISize)),
            MethodKind::Instance,
            vec![].into(),
        );
        call!(asm.alloc_methodref(mref), [gc_handle])
    }
    pub fn gc_handle_to_obj(self, obj: ClassRefIdx, asm: &mut Assembly) -> Self {
        let gc_handle_class = Type::ClassRef(ClassRef::gc_handle(asm));
        let mref = MethodRef::new(
            ClassRef::gc_handle(asm),
            asm.alloc_string("FromIntPtr"),
            asm.sig([Type::Int(Int::ISize)], gc_handle_class),
            MethodKind::Static,
            vec![].into(),
        );
        let gc_handle = call!(asm.alloc_methodref(mref), [self]);
        let gc_handle = CILNode::TemporaryLocal(Box::new((
            asm.alloc_type(gc_handle_class),
            [CILRoot::SetTMPLocal { value: gc_handle }].into(),
            CILNode::LoadAddresOfTMPLocal,
        )));

        let gc_handle_ref = asm.nref(gc_handle_class);
        let mref = MethodRef::new(
            ClassRef::gc_handle(asm),
            asm.alloc_string("get_Target"),
            asm.sig([gc_handle_ref], Type::PlatformObject),
            MethodKind::Instance,
            vec![].into(),
        );
        let object = call!(asm.alloc_methodref(mref), [gc_handle]);
        CILNode::CheckedCast(Box::new((object, obj)))
    }

    #[must_use]
    pub fn select(tpe: Type, a: Self, b: Self, predictate: Self, asm: &mut Assembly) -> Self {
        match tpe {
            Type::Int(
                int @ (Int::I8
                | Int::U8
                | Int::I16
                | Int::U16
                | Int::I32
                | Int::U32
                | Int::I64
                | Int::U64
                | Int::I128
                | Int::U128
                | Int::ISize
                | Int::USize),
            ) => {
                let select = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string(format!("select_{}", int.name())),
                    asm.sig([Type::Int(int), Type::Int(int), Type::Bool], Type::Int(int)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(select), [a, b, predictate])
            }
            Type::Ptr(_) => {
                let int = Int::USize;
                let select = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string(format!("select_{}", int.name())),
                    asm.sig([Type::Int(int), Type::Int(int), Type::Bool], Type::Int(int)),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(
                    asm.alloc_methodref(select),
                    [
                        a.cast_ptr(Type::Int(int)),
                        b.cast_ptr(Type::Int(int)),
                        predictate
                    ]
                )
            }
            _ => todo!(),
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

    pub fn transmute_on_stack(self, src: Type, target: Type, asm: &mut Assembly) -> Self {
        let tmp_loc = Self::TemporaryLocal(Box::new((
            asm.alloc_type(src),
            Box::new([CILRoot::SetTMPLocal { value: self }]),
            CILNode::LoadAddresOfTMPLocal,
        )));
        Self::LdObj {
            ptr: Box::new(CILNode::MRefToRawPtr(Box::new(tmp_loc)).cast_ptr(asm.nptr(target))),
            obj: Box::new(target),
        }
    }
    #[track_caller]
    pub fn cast_ptr(self, new_ptr: Type) -> Self {
        assert!(
            matches!(
                new_ptr,
                Type::Ptr(_)
                    | Type::Ref(_)
                    | Type::FnPtr(_)
                    | Type::Int(Int::USize)
                    | Type::Int(Int::ISize)
            ),
            "Invalid new ptr {new_ptr:?}"
        );
        Self::CastPtr {
            val: Box::new(self),
            new_ptr: Box::new(new_ptr),
        }
    }
    pub(crate) fn allocate_tmps(&mut self, curr_loc: Option<u32>, locals: &mut Vec<LocalDef>) {
        match self {
            Self::AddressOfStaticField(_)=>(),
            Self::LdNull(_tpe)=>(),
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
                locals.push((None,*tpe));
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
            Self::LDLen { arr } =>{
               arr.allocate_tmps(curr_loc, locals);
            }
            Self::LDElelemRef { arr, idx }=>{
                arr.allocate_tmps(curr_loc, locals);
                idx.allocate_tmps(curr_loc, locals);
            }
        };
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
impl std::ops::BitAnd<Self> for CILNode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        and!(self, rhs)
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
}

impl<'a> ValidationContext<'a> {
    pub fn new(sig: &'a FnSig, locals: &'a [(Option<IString>, Type)]) -> Self {
        Self { sig, locals }
    }

    pub fn sig(&self) -> &FnSig {
        self.sig
    }

    pub fn locals(&self) -> &[(Option<IString>, Type)] {
        self.locals
    }
}
