use crate::{
    cil::{CILOp, CallSite, FieldDescriptor},
    r#type::Type,
};

use super::append_vec;

#[derive(Clone, Debug)]
pub enum CILNode {
    LDLoc(u32),
    LDArg(u32),
    LDLocA(u32),
    LDArgA(u32),
    BlackBox(Box<Self>),
    ConvUSize(Box<Self>),
    SizeOf(Box<Type>),
    LDIndI8 {
        ptr: Box<Self>,
    },
    LDIndI16 {
        ptr: Box<Self>,
    },
    LDIndI32 {
        ptr: Box<Self>,
    },
    LDIndI64 {
        ptr: Box<Self>,
    },
    LDIndISize {
        ptr: Box<Self>,
    },
    LdObj {
        ptr: Box<Self>,
        obj: Box<Type>,
    },
    LDIndF32 {
        ptr: Box<Self>,
    },
    LDIndF64 {
        ptr: Box<Self>,
    },
    LDFieldAdress {
        addr: Box<Self>,
        field: Box<FieldDescriptor>,
    },
    LDField {
        addr: Box<Self>,
        field: Box<FieldDescriptor>,
    },
    Add(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    // TODO: Remove this
    RawOps {
        parrent: Box<CILNode>,
        ops: Box<[CILOp]>,
    },
    Call {
        args: Box<[Self]>,
        site: Box<CallSite>,
    },
    LdcI64(i64),
    LdcU64(u64),
    LdcI32(i32),
    LdcU32(u32),
    LoadGlobalAllocPtr { alloc_id: u64 },
}
impl CILNode {
    pub fn flatten(&self) -> Vec<CILOp> {
        match self {
            Self::LDLoc(local) => vec![CILOp::LDLoc(*local)],
            Self::LDArg(local) => vec![CILOp::LDArg(*local)],
            Self::SizeOf(tpe) => vec![CILOp::SizeOf(tpe.clone())],
            Self::LDArgA(local) => vec![CILOp::LDArgA(*local)],
            Self::LDLocA(local) => vec![CILOp::LDLocA(*local)],
            Self::BlackBox(inner) => inner.flatten(),
            Self::ConvUSize(inner) => append_vec(inner.flatten(), CILOp::ConvUSize(false)),
            Self::LDIndI8 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI8),
            Self::LDIndI16 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI16),
            Self::LDIndI32 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI32),
            Self::LDIndI64 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndI64),
            Self::LDIndISize { ptr } => append_vec(ptr.flatten(), CILOp::LDIndISize),
            Self::LdObj { ptr, obj } => append_vec(ptr.flatten(), CILOp::LdObj(obj.clone())),
            Self::LDFieldAdress { addr, field } => {
                append_vec(addr.flatten(), CILOp::LDFieldAdress(field.clone().into()))
            }
            Self::LDField { addr, field } => {
                append_vec(addr.flatten(), CILOp::LDField(field.clone().into()))
            }
            Self::LDIndF32 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndF32),
            Self::LDIndF64 { ptr } => append_vec(ptr.flatten(), CILOp::LDIndF64),
            Self::Add(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Add);
                res
            }
            Self::Mul(a, b) => {
                let mut res = a.flatten();
                res.extend(b.flatten());
                res.push(CILOp::Add);
                res
            }
            Self::RawOps { parrent, ops } => {
                let mut parrent = parrent.flatten();
                parrent.extend(ops.iter().cloned());
                parrent
            }
            Self::Call { args, site } => {
                let mut res: Vec<CILOp> = args.iter().map(|arg| arg.flatten()).flatten().collect();
                res.push(CILOp::Call(site.clone()));
                res
            }
            Self::LdcI64(val) => vec![CILOp::LdcI64(*val)],
            Self::LdcU64(val) => vec![CILOp::LdcU64(*val)],
            Self::LdcI32(val) => vec![CILOp::LdcI32(*val)],
            Self::LdcU32(val) => vec![CILOp::LdcU32(*val)],
            Self::LoadGlobalAllocPtr{alloc_id} => vec![CILOp::LoadGlobalAllocPtr { alloc_id: *alloc_id }]
        }
    }
}
