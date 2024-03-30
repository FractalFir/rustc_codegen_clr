use crate::{
    call,
    cil::{CILOp, CallSite, FieldDescriptor, StaticFieldDescriptor},
    function_sig::FnSig,
    r#type::Type,
    IString,
};

use super::{append_vec, cil_root::CILRoot};
use rustc_middle::ty::TyCtxt;
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
    Rem(Box<Self>, Box<Self>),
    RemUn(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    XOr(Box<Self>, Box<Self>),
    Shr(Box<Self>, Box<Self>),
    Shl(Box<Self>, Box<Self>),
    ShrUn(Box<Self>, Box<Self>),
    // TODO: Remove this
    RawOpsParrentless {
        ops: Box<[CILOp]>,
    },
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
    ConvUSize(Box<Self>),
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
    LtUn(Box<Self>, Box<Self>),
    Gt(Box<Self>, Box<Self>),
    GtUn(Box<Self>, Box<Self>),
    TemporaryLocal(Box<(Type, Box<[CILRoot]>, Self)>),
    SubTrees(Box<[CILRoot]>, Box<Self>),
    LoadAddresOfTMPLocal,
    LoadTMPLocal,
    LDFtn(Box<CallSite>),
    LDTypeToken(Box<Type>),
    NewObj {
        site: Box<CallSite>,
        args: Box<[CILNode]>,
    },
    LdStr(IString),
    CallI {
        sig: FnSig,
        fn_ptr: Box<CILNode>,
        args: Box<[Self]>,
    },
}
impl CILNode {
    pub fn select(tpe: Type, a: CILNode, b: CILNode, predictate: CILNode) -> Self {
        match tpe {
            Type::U128 | Type::I128 => call!(
                CallSite::new(
                    None,
                    "select_u128".into(),
                    FnSig::new(&[Type::U128, Type::U128, Type::Bool], &Type::U128),
                    true
                ),
                [a, b, predictate]
            ),
            Type::USize | Type::ISize | Type::Ptr(_) => call!(
                CallSite::new(
                    None,
                    "select_usize".into(),
                    FnSig::new(&[Type::USize, Type::USize, Type::Bool], &Type::USize),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U64 | Type::I64 => call!(
                CallSite::new(
                    None,
                    "select_u64".into(),
                    FnSig::new(&[Type::U64, Type::U64, Type::Bool], &Type::U64),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U32 | Type::I32 => call!(
                CallSite::new(
                    None,
                    "select_u32".into(),
                    FnSig::new(&[Type::U32, Type::U32, Type::Bool], &Type::U32),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U16 | Type::I16 => call!(
                CallSite::new(
                    None,
                    "select_u16".into(),
                    FnSig::new(&[Type::U16, Type::U16, Type::Bool], &Type::U16),
                    true
                ),
                [a, b, predictate]
            ),
            Type::U8 | Type::I8 | Type::Bool => call!(
                CallSite::new(
                    None,
                    "select_u8".into(),
                    FnSig::new(&[Type::U8, Type::U8, Type::Bool], &Type::U8),
                    true
                ),
                [a, b, predictate]
            ),
            _ => todo!("Can't select type {tpe:?}"),
        }
    }
    fn opt_children(&mut self) {
        match self {
            CILNode::LDLoc(_) => (),
            CILNode::LDArg(_) => (),
            CILNode::LDLocA(_) => (),
            CILNode::LDArgA(_) => (),
            CILNode::BlackBox(inner)
            | CILNode::ConvF32(inner)
            | CILNode::ConvF64(inner)
            | CILNode::ConvF64Un(inner) => inner.opt(),
            CILNode::SizeOf(_) => (),
            CILNode::LDIndI8 { ptr }
            | CILNode::LDIndI16 { ptr }
            | CILNode::LDIndI32 { ptr }
            | CILNode::LDIndI64 { ptr }
            | CILNode::LDIndISize { ptr }
            | CILNode::LdObj { ptr, .. }
            | CILNode::LDIndF32 { ptr }
            | CILNode::LDIndF64 { ptr } => ptr.opt(),
            CILNode::LDFieldAdress { addr, field: _ } | CILNode::LDField { addr, field: _ } => addr.opt(),
            CILNode::Add(a, b)
            | CILNode::And(a, b)
            | CILNode::Sub(a, b)
            | CILNode::Mul(a, b)
            | CILNode::Div(a, b)
            | CILNode::Rem(a, b)
            | CILNode::RemUn(a, b)
            | CILNode::Or(a, b)
            | CILNode::XOr(a, b)
            | CILNode::Shr(a, b)
            | CILNode::Shl(a, b)
            | CILNode::ShrUn(a, b)
            | CILNode::Eq(a, b)
            | CILNode::Lt(a, b)
            | CILNode::LtUn(a, b)
            | CILNode::Gt(a, b)
            | CILNode::GtUn(a, b) => {
                a.opt();
                b.opt();
            }

            CILNode::RawOpsParrentless { ops: _ } => (),
            CILNode::Call { args, site: _ }
            | CILNode::NewObj { site: _, args }
            | CILNode::CallVirt { args, site: _ } => args.iter_mut().for_each(|arg| arg.opt()),
            CILNode::LdcI64(_)
            | CILNode::LdcU64(_)
            | CILNode::LdcI32(_)
            | CILNode::LdcU32(_)
            | CILNode::LdcF64(_)
            | CILNode::LdcF32(_)
            | CILNode::LoadGlobalAllocPtr { .. } => (),
            CILNode::ConvU8(inner)
            | CILNode::ConvU16(inner)
            | CILNode::ConvU32(inner)
            | CILNode::ConvU64(inner)
            | CILNode::ConvUSize(inner)
            | CILNode::ConvI8(inner)
            | CILNode::ConvI16(inner)
            | CILNode::ConvI32(inner)
            | CILNode::ConvI64(inner)
            | CILNode::ConvISize(inner)
            //| CILNode::Volatile(inner)
            | CILNode::Neg(inner)
            | CILNode::Not(inner) => inner.opt(),
            CILNode::TemporaryLocal(_inner) => (),
            CILNode::SubTrees(a, b) => {
                a.iter_mut().for_each(|tree| tree.opt());
                b.opt()
            }
            CILNode::LoadAddresOfTMPLocal => (),
            CILNode::LoadTMPLocal => (),
            CILNode::LDFtn(_) => (),
            CILNode::LDTypeToken(_) => (),
            CILNode::LdStr(_) => (),
            CILNode::CallI { sig: _, fn_ptr, args } => {
                args.iter_mut().for_each(|arg| arg.opt());
                fn_ptr.opt();
            }
            CILNode::LDStaticField(_static_field) => (),
        }
    }
    // This fucntion will get expanded, so a single match is a non-issue.
    #[allow(clippy::single_match)]
    /// Optimizes this CILNode.
    pub fn opt(&mut self) {
        self.opt_children();
        match self {
            Self::LDField { addr: fld_addr, .. } => match fld_addr.as_mut() {
                Self::ConvUSize(addr) => match addr.as_mut() {
                    Self::LDLocA(_) | Self::LDFieldAdress { .. } => *fld_addr = addr.clone(),
                    _ => (),
                },
                _ => (),
            },
            Self::LDFieldAdress {
                addr: fld_addr,
                field,
            } => match fld_addr.as_mut() {
                Self::ConvUSize(addr) => match addr.as_mut() {
                    Self::LDLocA(_) | Self::LDFieldAdress { .. } => {
                        *self = Self::ConvUSize(Box::new(Self::LDFieldAdress {
                            addr: addr.clone(),
                            field: field.clone(),
                        }))
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
    pub fn flatten(&self) -> Vec<CILOp> {
        let mut ops = match self {
            Self::CallI { sig, fn_ptr, args } => {
                let mut ops: Vec<_> = fn_ptr.flatten();
                ops.extend(args.iter().flat_map(|arg| arg.flatten()));
                ops.push(CILOp::CallI(sig.clone().into()));
                ops
            }
            Self::SubTrees(trees, root) => {
                let mut flattened: Vec<_> = trees.iter().flat_map(|tree| tree.into_ops()).collect();
                flattened.extend(root.flatten());
                flattened
            }
            Self::LoadTMPLocal => {
                panic!("Unresolved temporary local during the CIL flattening phase!")
            }
            Self::LoadAddresOfTMPLocal => {
                panic!("Unresolved temporary local during the CIL flattening phase!")
            }
            Self::LDFtn(site) => vec![CILOp::LDFtn(site.clone())],
            Self::LDTypeToken(tpe) => vec![CILOp::LDTypeToken(tpe.clone())],
            Self::TemporaryLocal(tuple) => {
                panic!("Unresolved temporary local `{tuple:?}` during the CIL flattening phase!")
            }
            Self::LDLoc(local) => vec![CILOp::LDLoc(*local)],
            Self::LDArg(local) => vec![CILOp::LDArg(*local)],
            Self::SizeOf(tpe) => match **tpe {
                Type::Void => vec![CILOp::LdcU32(0)],
                _ => vec![CILOp::SizeOf(tpe.clone())],
            },
            Self::LDArgA(local) => vec![CILOp::LDArgA(*local)],
            Self::LDLocA(local) => vec![CILOp::LDLocA(*local)],

            Self::BlackBox(inner) => inner.flatten(),
            /*
            Self::Volatile(inner) => {
                let mut res = vec![CILOp::Volatile];
                res.extend(inner.flatten());
                res
            } */
            Self::ConvUSize(inner) => append_vec(inner.flatten(), CILOp::ConvUSize(false)),
            Self::ConvU8(inner) => append_vec(inner.flatten(), CILOp::ConvU8(false)),
            Self::ConvU16(inner) => append_vec(inner.flatten(), CILOp::ConvU16(false)),
            Self::ConvU32(inner) => append_vec(inner.flatten(), CILOp::ConvU32(false)),
            Self::ConvU64(inner) => append_vec(inner.flatten(), CILOp::ConvU64(false)),

            Self::ConvISize(inner) => append_vec(inner.flatten(), CILOp::ConvISize(false)),
            Self::ConvI8(inner) => append_vec(inner.flatten(), CILOp::ConvI8(false)),
            Self::ConvI16(inner) => append_vec(inner.flatten(), CILOp::ConvI16(false)),
            Self::ConvI32(inner) => append_vec(inner.flatten(), CILOp::ConvI32(false)),
            Self::ConvI64(inner) => append_vec(inner.flatten(), CILOp::ConvI64(false)),

            Self::ConvF32(inner) => append_vec(inner.flatten(), CILOp::ConvF32),
            Self::ConvF64(inner) => append_vec(inner.flatten(), CILOp::ConvF64),
            Self::ConvF64Un(inner) => append_vec(inner.flatten(), CILOp::ConvF64Un),
            Self::LDIndI8 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI8),
            Self::LDIndI16 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI16),
            Self::LDIndI32 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI32),
            Self::LDIndI64 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI64),
            Self::LDIndISize { ptr } => append_vec(ptr.flatten(), CILOp::LDIndISize),
            Self::LdObj { ptr, obj } => append_vec(ptr.flatten(), CILOp::LdObj(obj.clone())),

            Self::Neg(inner) => append_vec(inner.flatten(), CILOp::Neg),
            Self::Not(inner) => append_vec(inner.flatten(), CILOp::Not),

            Self::LDFieldAdress { addr, field } => {
                append_vec(addr.flatten(), CILOp::LDFieldAdress(field.clone()))
            }
            Self::LDField { addr, field } => {
                append_vec(addr.flatten(), CILOp::LDField(field.clone()))
            }
            Self::LDIndF32 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndF32),
            Self::LDIndF64 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndF64),
            Self::Add(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Add);
                res
            }
            Self::And(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::And);
                res
            }
            Self::Shr(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Shr);
                res
            }
            Self::Shl(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Shl);
                res
            }
            Self::ShrUn(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::ShrUn);
                res
            }
            Self::Or(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Or);
                res
            }
            Self::XOr(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::XOr);
                res
            }
            Self::Div(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Div);
                res
            }
            Self::Rem(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Rem);
                res
            }
            Self::RemUn(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::RemUn);
                res
            }
            Self::Sub(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Sub);
                res
            }
            Self::Eq(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Eq);
                res
            }
            Self::Lt(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Lt);
                res
            }
            Self::LtUn(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::LtUn);
                res
            }
            Self::Gt(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Gt);
                res
            }
            Self::GtUn(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::GtUn);
                res
            }
            Self::Mul(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Mul);
                res
            }

            Self::RawOpsParrentless { ops } => ops.clone().into(),
            Self::Call { args, site } => {
                let mut res: Vec<CILOp> = args.iter().flat_map(|arg| arg.flatten()).collect();
                res.push(CILOp::Call(site.clone()));
                res
            }
            Self::NewObj { args, site } => {
                let mut res: Vec<CILOp> = args.iter().flat_map(|arg| arg.flatten()).collect();
                res.push(CILOp::NewObj(site.clone()));
                res
            }
            Self::CallVirt { args, site } => {
                let mut res: Vec<CILOp> = args.iter().flat_map(|arg| arg.flatten()).collect();
                res.push(CILOp::CallVirt(site.clone()));
                res
            }
            Self::LdcI64(val) => vec![CILOp::LdcI64(*val)],
            Self::LdcU64(val) => vec![CILOp::LdcU64(*val)],
            Self::LdcI32(val) => vec![CILOp::LdcI32(*val)],
            Self::LdcU32(val) => vec![CILOp::LdcU32(*val)],
            Self::LdcF64(val) => vec![CILOp::LdcF64(*val)],
            Self::LdcF32(val) => vec![CILOp::LdcF32(*val)],
            Self::LdStr(string) => vec![CILOp::LdStr(string.clone())],
            Self::LoadGlobalAllocPtr { alloc_id } => {
                panic!(
                    "Unresolved global alloc with id:{alloc_id}  during the CIL flattening phase!"
                )
            }
            Self::LDStaticField(sfield) => vec![CILOp::LDStaticField(sfield.clone())],
        };
        {
            ops.push(CILOp::Pop);
            crate::utilis::check_debugable(&ops, self, false);
            ops.pop();
        }
        ops
    }

    pub(crate) fn allocate_tmps(
        &mut self,
        curr_loc: Option<u32>,
        locals: &mut Vec<(Option<Box<str>>, Type)>,
    ) {
        match self {
            CILNode::LDLoc(_) |
            CILNode::LDArg(_) |
            CILNode::LDLocA(_)|
            CILNode::LDArgA(_) => (),
            CILNode::BlackBox(_) => todo!(),
            CILNode::SizeOf(_) => (),
            CILNode::LDIndI8 { ptr }|
            CILNode::LDIndI16 { ptr }|
            CILNode::LDIndI32 { ptr }|
            CILNode::LDIndI64 { ptr }|
            CILNode::LDIndISize { ptr }|
            CILNode::LdObj { ptr, .. }|
            CILNode::LDIndF32 { ptr } |
            CILNode::LDIndF64 { ptr } => ptr.allocate_tmps(curr_loc, locals),
            CILNode::LDFieldAdress { addr, field: _ } |
            CILNode::LDField { addr, field: _ }=> addr.allocate_tmps(curr_loc, locals),
            CILNode::Add(a, b)
            | CILNode::And(a, b)
            | CILNode::Sub(a, b)
            | CILNode::Mul(a, b)
            | CILNode::Div(a, b)
            | CILNode::Rem(a, b)
            | CILNode::RemUn(a, b)
            | CILNode::Or(a, b)
            | CILNode::XOr(a, b)
            | CILNode::Shr(a, b)
            | CILNode::Shl(a, b)
            | CILNode::ShrUn(a, b)
            | CILNode::Eq(a, b)
            | CILNode::Lt(a, b)
            | CILNode::LtUn(a, b)
            | CILNode::Gt(a, b)
            | CILNode::GtUn(a, b) => {
                a.allocate_tmps(curr_loc, locals);
                b.allocate_tmps(curr_loc, locals)
            }
            CILNode::RawOpsParrentless { ops: _ } => {
                eprintln!("WARNING: allocate_tmps does not work for `RawOpsParrentless`")
            }
            CILNode::Call { args, site: _ } |
            CILNode::CallVirt { args, site: _ } =>args.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals)),
            CILNode::LdcI64(_) |
            CILNode::LdcU64(_) |
            CILNode::LdcI32(_)  |
            CILNode::LdcU32(_) |
            CILNode::LdcF64(_) |
            CILNode::LdcF32(_) =>(),
            CILNode::LoadGlobalAllocPtr { alloc_id: _ } => (),
            CILNode::ConvF64Un(val) |
            CILNode::ConvF32(val)|
            CILNode::ConvF64(val) |
            CILNode::ConvU8(val)|
            CILNode::ConvU16(val)|
            CILNode::ConvU32(val)|
            CILNode::ConvU64(val)|
            CILNode::ConvUSize(val)|
            CILNode::ConvI8(val) |
            CILNode::ConvI16(val)|
            CILNode::ConvI32(val)|
            CILNode::ConvI64(val) |
            CILNode::ConvISize(val)|
            //CILNode::Volatile(_) => todo!(),
            CILNode::Neg(val) |
            CILNode::Not(val) =>val.allocate_tmps(curr_loc, locals),

            CILNode::TemporaryLocal(tmp_loc) => {
                let tpe = &mut tmp_loc.0;
                let end_loc = locals.len();
                locals.push((None,tpe.clone()));
                let roots = &mut tmp_loc.1;
                let main = &mut tmp_loc.2;
                roots.iter_mut().for_each(|tree|tree.allocate_tmps(Some(end_loc as u32), locals));
                main.allocate_tmps(Some(end_loc as u32), locals);
                *self= Self::SubTrees(roots.clone(), Box::new(main.clone()));
            },
            CILNode::SubTrees(trees, main) =>{
                trees.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc,locals));
                main.allocate_tmps(curr_loc, locals)
            }
            CILNode::LoadAddresOfTMPLocal => *self = CILNode::LDLocA(curr_loc.expect("Temporary local referenced when none present")),
            CILNode::LoadTMPLocal =>*self = CILNode::LDLoc(curr_loc.expect("Temporary local referenced when none present")),
            CILNode::LDFtn(_) => (),
            CILNode::LDTypeToken(_) =>(),
            CILNode::NewObj { site: _, args } => args.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals)),
            CILNode::LdStr(_) => (),
            CILNode::CallI { sig: _, fn_ptr, args } => {
               fn_ptr.allocate_tmps(curr_loc, locals);
                args.iter_mut().for_each(|arg|arg.allocate_tmps(curr_loc, locals))
            }
            CILNode::LDStaticField(_sfield)=>(),
        };
    }
    pub(crate) fn resolve_global_allocations(
        &mut self,
        asm: &mut crate::assembly::Assembly,
        tyctx: TyCtxt,
    ) {
        match self {
            CILNode::LDLoc(_) |
            CILNode::LDArg(_) |
            CILNode::LDLocA(_)|
            CILNode::LDArgA(_) => (),
            CILNode::BlackBox(_) => todo!(),
            CILNode::SizeOf(_) => (),
            CILNode::LDIndI8 { ptr }|
            CILNode::LDIndI16 { ptr }|
            CILNode::LDIndI32 { ptr }|
            CILNode::LDIndI64 { ptr }|
            CILNode::LDIndISize { ptr }|
            CILNode::LdObj { ptr, .. }|
            CILNode::LDIndF32 { ptr } |
            CILNode::LDIndF64 { ptr } => ptr.resolve_global_allocations(asm,tyctx),
            CILNode::LDFieldAdress { addr, field: _ }|
            CILNode::LDField { addr, field: _ } => addr.resolve_global_allocations(asm,tyctx),
            CILNode::Add(a, b)
            | CILNode::And(a, b)
            | CILNode::Sub(a, b)
            | CILNode::Mul(a, b)
            | CILNode::Div(a, b)
            | CILNode::Rem(a, b)
            | CILNode::RemUn(a, b)
            | CILNode::Or(a, b)
            | CILNode::XOr(a, b)
            | CILNode::Shr(a, b)
            | CILNode::Shl(a, b)
            | CILNode::ShrUn(a, b)
            | CILNode::Eq(a, b)
            | CILNode::Lt(a, b)
            | CILNode::LtUn(a, b)
            | CILNode::Gt(a, b)
            | CILNode::GtUn(a, b) => {
                a.resolve_global_allocations(asm,tyctx);
                b.resolve_global_allocations(asm,tyctx)
            }
            CILNode::RawOpsParrentless { ops: _ } => {
                eprintln!("WARNING: resolve_global_allocations does not work for `RawOpsParrentless`")
            }
            CILNode::Call { args, site: _ } |
            CILNode::CallVirt { args, site: _ } =>args.iter_mut().for_each(|arg|arg.resolve_global_allocations(asm,tyctx)),
            CILNode::LdcI64(_) |
            CILNode::LdcU64(_) |
            CILNode::LdcI32(_)  |
            CILNode::LdcU32(_) |
            CILNode::LdcF64(_) |
            CILNode::LdcF32(_) =>(),
            CILNode::LoadGlobalAllocPtr { alloc_id } => {
                *self = Self::LDStaticField(asm.add_allocation(*alloc_id,tyctx).into());
            }
            CILNode::ConvF64Un(val) |
            CILNode::ConvF32(val)|
            CILNode::ConvF64(val) |
            CILNode::ConvU8(val)|
            CILNode::ConvU16(val)|
            CILNode::ConvU32(val)|
            CILNode::ConvU64(val)|
            CILNode::ConvUSize(val)|
            CILNode::ConvI8(val) |
            CILNode::ConvI16(val)|
            CILNode::ConvI32(val)|
            CILNode::ConvI64(val) |
            CILNode::ConvISize(val)|
            //CILNode::Volatile(_) => todo!(),
            CILNode::Neg(val) |
            CILNode::Not(val) =>val.resolve_global_allocations(asm,tyctx),

            CILNode::TemporaryLocal(tmp_loc) => {
                tmp_loc.1.iter_mut().for_each(|tree|tree.resolve_global_allocations(asm,tyctx));
                tmp_loc.2.resolve_global_allocations(asm,tyctx);
            },
            CILNode::SubTrees(trees, main) =>{
                trees.iter_mut().for_each(|arg|arg.resolve_global_allocations(asm,tyctx));
                main.resolve_global_allocations(asm,tyctx)
            }
            CILNode::LoadAddresOfTMPLocal => (),
            CILNode::LoadTMPLocal => (),
            CILNode::LDFtn(_) => (),
            CILNode::LDTypeToken(_) => (),
            CILNode::NewObj { site: _, args } => args.iter_mut().for_each(|arg|arg.resolve_global_allocations(asm,tyctx)),
            CILNode::LdStr(_) => (),
            CILNode::CallI { sig: _, fn_ptr, args } => {
                fn_ptr.resolve_global_allocations(asm, tyctx);
                args.iter_mut().for_each(|arg|arg.resolve_global_allocations(asm,tyctx));
            }
            Self::LDStaticField(_sfield)=>(),
        }
    }

    pub(crate) fn sheed_trees(&mut self) -> Vec<CILRoot> {
        match self {
            CILNode::LDLoc(_) | CILNode::LDArg(_) | CILNode::LDLocA(_) | CILNode::LDArgA(_) => {
                vec![]
            }
            CILNode::BlackBox(inner) => inner.sheed_trees(),
            CILNode::LDStaticField(_) => vec![],
            CILNode::ConvF32(inner) | CILNode::ConvF64(inner) | CILNode::ConvF64Un(inner) => {
                inner.sheed_trees()
            }
            CILNode::SizeOf(_) => vec![],
            CILNode::LDIndI8 { ptr }
            | CILNode::LDIndI16 { ptr }
            | CILNode::LDIndI32 { ptr }
            | CILNode::LDIndI64 { ptr }
            | CILNode::LDIndISize { ptr }
            | CILNode::LdObj { ptr, .. }
            | CILNode::LDIndF32 { ptr }
            | CILNode::LDIndF64 { ptr } => ptr.sheed_trees(),
            CILNode::LDFieldAdress { addr, field: _ } | CILNode::LDField { addr, field: _ } => {
                addr.sheed_trees()
            }
            CILNode::Add(a, b)
            | CILNode::And(a, b)
            | CILNode::Sub(a, b)
            | CILNode::Mul(a, b)
            | CILNode::Div(a, b)
            | CILNode::Rem(a, b)
            | CILNode::RemUn(a, b)
            | CILNode::Or(a, b)
            | CILNode::XOr(a, b)
            | CILNode::Shr(a, b)
            | CILNode::Shl(a, b)
            | CILNode::ShrUn(a, b) => {
                let mut res = a.sheed_trees();
                res.extend(b.sheed_trees());
                res
            }
            CILNode::RawOpsParrentless { ops: _ } => vec![],
            CILNode::Call { args, site: _ } | CILNode::CallVirt { args, site: _ } => {
                args.iter_mut().flat_map(|arg| arg.sheed_trees()).collect()
            }
            CILNode::LdcI64(_)
            | CILNode::LdcU64(_)
            | CILNode::LdcI32(_)
            | CILNode::LdcU32(_)
            | CILNode::LdcF64(_)
            | CILNode::LdcF32(_) => vec![],
            CILNode::LoadGlobalAllocPtr { alloc_id: _ } => vec![],
            CILNode::ConvU8(val)
            | CILNode::ConvU16(val)
            | CILNode::ConvU32(val)
            | CILNode::ConvU64(val)
            | CILNode::ConvUSize(val)
            | CILNode::ConvI8(val)
            | CILNode::ConvI16(val)
            | CILNode::ConvI32(val)
            | CILNode::ConvI64(val)
            | CILNode::ConvISize(val) => val.sheed_trees(),
            CILNode::Neg(val) | CILNode::Not(val) => val.sheed_trees(),
            CILNode::Eq(a, b)
            | CILNode::Lt(a, b)
            | CILNode::LtUn(a, b)
            | CILNode::Gt(a, b)
            | CILNode::GtUn(a, b) => {
                let mut res = a.sheed_trees();
                res.extend(b.sheed_trees());
                res
            }
            CILNode::TemporaryLocal(_) => {
                panic!("Trees should be sheed after locals are allocated!")
            }
            CILNode::SubTrees(trees, main) => {
                let clone = *main.clone();
                let res = trees.to_vec();
                *self = clone;
                res
            }
            CILNode::LoadAddresOfTMPLocal => {
                panic!("Trees should be sheed after locals are allocated!")
            }
            CILNode::LoadTMPLocal => panic!("Trees should be sheed after locals are allocated!"),
            CILNode::LDFtn(_) | CILNode::LDTypeToken(_) => vec![],
            CILNode::NewObj { site: _, args } => {
                args.iter_mut().flat_map(|arg| arg.sheed_trees()).collect()
            }
            CILNode::LdStr(_) => vec![],
            CILNode::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                let mut res = fn_ptr.sheed_trees();
                res.extend(args.iter_mut().flat_map(|arg| arg.sheed_trees()));
                res
            }
        }
    }
}
#[macro_export]
macro_rules! add {
    ($a:expr,$b:expr) => {
        CILNode::Add($a.into(), $b.into())
    };
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
        CILNode::ConvUSize($a.into())
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
macro_rules! conv_f64_un {
    ($a:expr) => {
        CILNode::ConvF64Un($a.into())
    };
}
/// Loads a value of type `i32`.
#[macro_export]
macro_rules! ldc_i32 {
    ($val:expr) => {
        $crate::cil_tree::cil_node::CILNode::LdcI32($val)
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
