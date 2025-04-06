use crate::cilnode::IsPure;
use crate::cilnode::MethodKind;
use crate::v2::method::LocalDef;
use crate::TypeIdx;
use crate::{
    call,
    cil_root::CILRoot,
    hashable::{HashableF32, HashableF64},
    IString,
};
use crate::{
    Assembly, ClassRef, ClassRefIdx, FieldIdx, FnSig, Int, MethodRef, MethodRefIdx,
    StaticFieldDesc, Type,
};
use serde::{Deserialize, Serialize};
/// A container for the arguments of a call, callvirt, or newobj instruction.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Debug)]
pub struct CallOpArgs {
    pub args: Box<[CILNode]>,
    pub site: MethodRefIdx,
    pub is_pure: IsPure,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub enum CILNode {
    /// A translated V2 node.
    V2(crate::NodeIdx),
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
    pub fn stack_addr(val: Self, tpe_idx: TypeIdx, _asm: &mut Assembly) -> Self {
        CILNode::TemporaryLocal(Box::new((
            tpe_idx,
            [CILRoot::SetTMPLocal { value: val }].into(),
            CILNode::LoadAddresOfTMPLocal,
        )))
    }
    pub fn ovf_check_tuple(
        asm: &mut Assembly,
        tuple: ClassRefIdx,
        out_of_range: Self,
        val: Self,
        tpe: Type,
    ) -> CILNode {
        let main = asm.main_module();
        let sig = asm.sig([tpe, Type::Bool], Type::ClassRef(tuple));
        let site = asm.new_methodref(*main, "ovf_check_tuple", sig, MethodKind::Static, []);
        CILNode::Call(Box::new(CallOpArgs {
            args: vec![val, out_of_range].into(),
            site,
            is_pure: IsPure::PURE,
        }))
        /*
                let item2 = asm.alloc_string("Item2");
        let item1 = asm.alloc_string("Item1");
        CILNode::TemporaryLocal(Box::new((
            asm.alloc_type(tuple),
            [
                CILRoot::SetField {
                    addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                    value: Box::new(out_of_range),
                    desc: asm.alloc_field(crate::FieldDesc::new(tuple, item2, Type::Bool)),
                },
                CILRoot::SetField {
                    addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                    value: Box::new(val),
                    desc: asm.alloc_field(crate::FieldDesc::new(tuple, item1, tpe)),
                },
            ]
            .into(),
            CILNode::LoadTMPLocal,
        )))*/
    }
    pub fn create_slice(
        slice_tpe: ClassRefIdx,
        asm: &mut Assembly,
        metadata: Self,
        ptr: Self,
    ) -> Self {
        let void_ptr = asm.nptr(Type::Void);
        let main = asm.main_module();
        let sig = asm.sig([void_ptr, Type::Int(Int::USize)], Type::ClassRef(slice_tpe));
        let create_slice = asm.new_methodref(*main, "create_slice", sig, MethodKind::Static, []);
        CILNode::Call(Box::new(CallOpArgs {
            args: vec![ptr, metadata].into(),
            site: create_slice,
            is_pure: IsPure::PURE,
        }))
    }
    pub fn const_u128(value: u128, asm: &mut Assembly) -> CILNode {
        CILNode::V2(asm.alloc_node(value))
    }
    pub fn const_i128(value: u128, asm: &mut Assembly) -> CILNode {
        CILNode::V2(asm.alloc_node(value as i128))
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
                CILNode::Call(Box::new(crate::cil_node::CallOpArgs {
                    args: [a, b, predictate].into(),
                    site: (asm.alloc_methodref(select)),
                    is_pure: crate::cilnode::IsPure::PURE,
                }))
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
                CILNode::Call(Box::new(crate::cil_node::CallOpArgs {
                    args: [
                        a.cast_ptr(Type::Int(int)),
                        b.cast_ptr(Type::Int(int)),
                        predictate,
                    ]
                    .into(),
                    site: (asm.alloc_methodref(select)),
                    is_pure: crate::cilnode::IsPure::PURE,
                })).cast_ptr(tpe)
            }
            _ => todo!(),
        }
    }

    /// Creates an unintialized value of type *tpe*.
    pub fn uninit_val(tpe: Type, asm: &mut Assembly) -> Self {
        if tpe == Type::Void {
            let gv = asm.global_void();
            return CILNode::LDStaticField(Box::new(asm[gv]));
        }
        let main = asm.main_module();
        let sig = asm.sig([], tpe);
        let uninit_val = asm.new_methodref(*main, "uninit_val", sig, MethodKind::Static, []);
        CILNode::Call(Box::new(CallOpArgs {
            args: [].into(),
            site: uninit_val,
            is_pure: IsPure::PURE,
        }))
    }
    pub fn transmute_on_stack(self, src: Type, target: Type, asm: &mut Assembly) -> Self {
        if src == target {
            return self;
        }
        let main_module = *asm.main_module();

        let sig = asm.sig([src], target);
        let mref = asm.new_methodref(main_module, "transmute", sig, MethodKind::Static, vec![]);
        CILNode::Call(Box::new(CallOpArgs {
            args: Box::new([self]),
            site: mref,
            is_pure: crate::cilnode::IsPure::NOT,
        }))
    }
    pub fn cxchng_res_val(
        old_val: Self,
        expected: Self,
        destination_addr: Self,
        val_desc: FieldIdx,
        flag_desc: FieldIdx,
    ) -> CILRoot {
        // Set the value of the result.
        let set_val = CILRoot::SetField {
            addr: Box::new(destination_addr.clone()),
            value: Box::new(old_val),
            desc: val_desc,
        };
        // Get the result back
        let val = CILNode::SubTrees(Box::new((
            [set_val].into(),
            CILNode::LDField {
                addr: Box::new(destination_addr.clone()),
                field: val_desc,
            }
            .into(),
        )));

        let cmp = CILNode::Eq(val.into(), expected.into());

        CILRoot::SetField {
            addr: Box::new(destination_addr.clone()),
            value: Box::new(cmp),
            desc: flag_desc,
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
            Self::V2(_)=>(),
            Self::AddressOfStaticField(_)=>(),
            Self::LdNull(_tpe)=>(),
            Self::UnboxAny(val,_tpe )=>val.allocate_tmps(curr_loc, locals),
            Self::Volatile(inner)=>inner.allocate_tmps(curr_loc, locals),
            Self::CheckedCast(inner)=>inner.0.allocate_tmps(curr_loc, locals),
            Self::IsInst(inner)=>inner.0.allocate_tmps(curr_loc, locals),
            Self::GetException=>(),
            Self::LocAlloc{..}=>(),
            Self::LocAllocAligned {..}=>(),
            Self::CastPtr { val, new_ptr: _ }=>val.allocate_tmps(curr_loc, locals),
            Self:: PointerToConstValue(_arr)=>(),
            Self::LoadGlobalAllocPtr { alloc_id: _ } => (),
            Self::LDLoc(_) |
            Self::LDArg(_) |
            Self::LDLocA(_)|
            Self::LDArgA(_) => (),
            Self::BlackBox(inner) => inner.allocate_tmps(curr_loc, locals),
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
            is_pure: $crate::cilnode::IsPure::NOT,
        }))
    };
}

#[macro_export]
macro_rules! call_virt {
    ($call_site:expr,$args:expr) => {
        CILNode::CallVirt(Box::new($crate::cil_node::CallOpArgs {
            args: $args.into(),
            site: $call_site.into(),
            is_pure: $crate::cilnode::IsPure::NOT,
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
