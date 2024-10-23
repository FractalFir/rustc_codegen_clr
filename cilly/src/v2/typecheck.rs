use fxhash::FxHashSet;

use crate::{v2::bimap::IntoBiMapIndex, IString};

use super::{
    cilnode::{PtrCastRes, UnOp},
    method::LocalDef,
    Assembly, BinOp, CILNode, CILRoot, ClassRef, Int, NodeIdx, SigIdx, Type,
};
#[derive(Debug)]
pub enum TypeCheckError {
    WrongBinopArgs {
        lhs: Type,
        rhs: Type,
        op: BinOp,
    },
    RefToPtrArgNotRef {
        arg: Type,
    },
    InvalidPtrCast {
        expected: PtrCastRes,
        got: Type,
    },
    TypeNotPtr {
        tpe: Type,
    },
    DerfWrongPtr {
        expected: Type,
        got: Type,
    },
    CallArgcWrong {
        expected: usize,
        got: usize,
        mname: IString,
    },
    CallArgTypeWrong {
        got: Type,
        expected: Type,
        idx: usize,
        mname: IString,
    },
    IntCastInvalidInput {
        got: Type,
        target: Int,
    },
    FieldAccessInvalidType {
        tpe: Type,
        field: crate::v2::FieldDesc,
    },
    FieldOwnerMismatch {
        owner: crate::v2::ClassRefIdx,
        expected_owner: crate::v2::ClassRefIdx,
        field: crate::v2::FieldDesc,
    },
    ExpectedClassGotValuetype {
        cref: ClassRef,
    },
    TypeNotClass {
        object: Type,
    },
    FloatCastInvalidInput {
        got: Type,
        target: super::Float,
    },
    WrongUnOpArgs {
        tpe: Type,
        op: UnOp,
    },
    IndirectCallArgcWrong {
        expected: usize,
        got: usize,
    },
    IndirectCallArgTypeWrong {
        got: Type,
        expected: Type,
        idx: usize,
    },
    LdLenArgNotArray {
        got: Type,
    },
    LdLenArrNot1D {
        got: Type,
    },
    ArrIndexInvalidType {
        index_tpe: Type,
    },
    IndirectCallInvalidFnPtrType {
        fn_ptr: Type,
    },
    IndirectCallInvalidFnPtrSig {
        expected: super::FnSig,
        got: super::FnSig,
    },
    SizeOfVoid,
}
pub fn display_typecheck_err(root: CILRoot, asm: &mut Assembly, sig: SigIdx, locals: &[LocalDef]) {
    let mut set = FxHashSet::default();
    let nodes = root
        .nodes()
        .iter()
        .map(|node| display_node(**node, asm, sig, locals, &mut set))
        .collect::<String>();
    eprintln!("digraph G{{edge [dir=\"back\"];\n{nodes}}}");
}
fn display_node(
    nodeidx: NodeIdx,
    asm: &mut Assembly,
    sig: SigIdx,
    locals: &[LocalDef],
    set: &mut FxHashSet<NodeIdx>,
) -> String {
    let node = asm.get_node(nodeidx).clone();
    set.insert(nodeidx);
    let tpe = node.typecheck(sig, locals, asm);
    let node_def = match tpe {
        Ok(tpe) => format!(
            "n{nodeidx} [label = {node:?} color = \"green\"]",
            nodeidx = nodeidx.as_bimap_index(),
            node = format!("{node:?}\n{}", tpe.mangle(asm))
        ),
        Err(err) => format!(
            "n{nodeidx} [label = {node:?} color = \"red\"]",
            nodeidx = nodeidx.as_bimap_index(),
            node = format!("{node:?}\n{err:?}")
        ),
    };
    let node_children = node.child_nodes();
    let node_children_str: String = node_children
        .iter()
        .map(|node| format!(" n{nodeidx} ", nodeidx = node.as_bimap_index(),))
        .collect();
    if node_children.is_empty() {
        format!("{node_def}\n")
    } else {
        let mut res = format!(
            "{node_def}\n n{nodeidx}  -> {{{node_children_str}}}\n",
            nodeidx = nodeidx.as_bimap_index(),
        );
        for nodeidx in node.child_nodes() {
            res.push_str(&display_node(nodeidx, asm, sig, locals, set));
        }
        res
    }
}
impl BinOp {
    fn typecheck(&self, lhs: Type, rhs: Type, asm: &Assembly) -> Result<Type, TypeCheckError> {
        match self {
            BinOp::Add | BinOp::Sub => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs => Ok(Type::Int(lhs)),
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Float(lhs)),
                (Type::Ptr(lhs), Type::Ptr(rhs)) if rhs == lhs => Ok(Type::Ptr(lhs)),
                (Type::FnPtr(lhs), Type::FnPtr(rhs)) if rhs == lhs => Ok(Type::FnPtr(lhs)),
                (Type::Ptr(_) | Type::FnPtr(_), Type::Int(Int::ISize | Int::USize)) => Ok(lhs),
                (Type::Int(Int::ISize | Int::USize), Type::Ptr(_) | Type::FnPtr(_)) => Ok(rhs),
                // TODO: investigate the cause of this issue. Changing a reference is not valid.
                (Type::Ref(_), Type::Int(Int::ISize | Int::USize)) => Ok(lhs),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Eq => {
                if lhs == rhs || lhs.is_assignable_to(rhs, asm) {
                    Ok(Type::Bool)
                } else {
                    Err(TypeCheckError::WrongBinopArgs {
                        lhs,
                        rhs,
                        op: *self,
                    })
                }
            }

            BinOp::Mul => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs => Ok(Type::Int(lhs)),
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Float(lhs)),
                (Type::Int(Int::ISize | Int::USize), Type::Ptr(_) | Type::FnPtr(_)) => Ok(rhs),
                // Relaxes the rules to prevent some wierd issue with sizeof
                (Type::Int(Int::ISize), Type::Int(Int::I32)) => Ok(Type::Int(Int::ISize)),
                (Type::Int(Int::USize), Type::Int(Int::I32)) => Ok(Type::Int(Int::USize)),
                _ => {
                    if lhs.is_assignable_to(rhs, asm) {
                        Ok(rhs)
                    } else if rhs.is_assignable_to(lhs, asm) {
                        Ok(lhs)
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::LtUn | BinOp::GtUn => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs => Ok(Type::Bool),
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                (Type::Ptr(lhs), Type::Ptr(rhs)) if rhs == lhs => Ok(Type::Bool),
                (Type::FnPtr(lhs), Type::FnPtr(rhs)) if rhs == lhs => Ok(Type::Bool),
                (Type::Bool, Type::Bool) => Ok(Type::Bool),
                _ => {
                    if lhs == rhs || lhs.is_assignable_to(rhs, asm) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Lt | BinOp::Gt => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs => Ok(Type::Bool),
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                (Type::Bool, Type::Bool) => Ok(Type::Bool),
                _ => {
                    if lhs == rhs || lhs.is_assignable_to(rhs, asm) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Or | BinOp::XOr | BinOp::And => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs => Ok(Type::Int(lhs)),
                (Type::Bool, Type::Bool) => Ok(Type::Bool),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Rem => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs && rhs.is_signed() => {
                    Ok(Type::Int(lhs))
                }
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::RemUn => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs && !rhs.is_signed() => {
                    Ok(Type::Int(lhs))
                }
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Shl => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::I128
                        | Int::U128
                        | Int::I64
                        | Int::U64
                        | Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8),
                    ),
                    Type::Int(
                        Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8,
                    ),
                ) => Ok(Type::Int(lhs)),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Shr => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::I128
                        | Int::U128
                        | Int::I64
                        | Int::U64
                        | Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8),
                    ),
                    Type::Int(
                        Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8,
                    ),
                ) if lhs.is_signed() => Ok(Type::Int(lhs)),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::ShrUn => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::I128
                        | Int::U128
                        | Int::I64
                        | Int::U64
                        | Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8),
                    ),
                    Type::Int(
                        Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8,
                    ),
                ) if !lhs.is_signed() => Ok(Type::Int(lhs)),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::DivUn => match (lhs, rhs) {
                (
                    Type::Int(lhs @ (Int::U64 | Int::USize | Int::U32 | Int::U16 | Int::U8)),
                    Type::Int(rhs @ (Int::U64 | Int::USize | Int::U32 | Int::U16 | Int::U8)),
                ) if lhs == rhs => Ok(Type::Int(lhs)),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
            BinOp::Div => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::U64
                        | Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8),
                    ),
                    Type::Int(
                        rhs @ (Int::U64
                        | Int::USize
                        | Int::ISize
                        | Int::I32
                        | Int::U32
                        | Int::I16
                        | Int::U16
                        | Int::U8
                        | Int::I8),
                    ),
                ) if lhs.is_signed() && lhs == rhs => Ok(Type::Int(lhs)),
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Float(lhs)),
                _ => {
                    if lhs.is_assignable_to(rhs, asm)
                        && (lhs.as_int().is_some() || rhs.as_int().is_some())
                    {
                        Ok(Type::Int(lhs.as_int().or(rhs.as_int()).unwrap()))
                    } else {
                        Err(TypeCheckError::WrongBinopArgs {
                            lhs,
                            rhs,
                            op: *self,
                        })
                    }
                }
            },
        }
    }
}
impl CILNode {
    #[allow(unused_variables)]
    /// Typechecks this node, and returns its type if its valid.
    /// # Errors
    /// Returns an error if this node can't pass type checks.
    pub fn typecheck(
        &self,
        sig: SigIdx,
        locals: &[LocalDef],
        asm: &mut Assembly,
    ) -> Result<Type, TypeCheckError> {
        match self {
            CILNode::Const(cst) => Ok(cst.as_ref().get_type()),
            CILNode::BinOp(lhs, rhs, op) => {
                let lhs = asm.get_node(*lhs).clone();
                let rhs = asm.get_node(*rhs).clone();
                let lhs = lhs.typecheck(sig, locals, asm)?;
                let rhs = rhs.typecheck(sig, locals, asm)?;
                op.typecheck(lhs, rhs, asm)
            }
            CILNode::UnOp(arg, op) => {
                let arg = asm.get_node(*arg).clone();
                let arg_type = arg.typecheck(sig, locals, asm)?;
                match arg_type {
                    Type::Int(_) | Type::Float(_) | Type::Ptr(_) => Ok(arg_type),
                    _ => Err(TypeCheckError::WrongUnOpArgs {
                        tpe: arg_type,
                        op: op.clone(),
                    }),
                }
            }
            CILNode::LdLoc(loc) => Ok(asm[locals[*loc as usize].1]),
            CILNode::LdLocA(loc) => Ok(asm.nref(asm[locals[*loc as usize].1])),
            CILNode::LdArg(arg) => Ok(asm[sig].inputs()[*arg as usize]),
            CILNode::LdArgA(arg) => Ok(asm.nref(asm[sig].inputs()[*arg as usize])),
            CILNode::Call(call_info) => {
                let (mref, args) = call_info.as_ref();
                let mref = asm[*mref].clone();
                let inputs: Box<[_]> = mref.stack_inputs(asm).into();
                if args.len() != inputs.len() {
                    return Err(TypeCheckError::CallArgcWrong {
                        expected: inputs.len(),
                        got: args.len(),
                        mname: asm[mref.name()].into(),
                    });
                }
                for (idx, (arg, input_type)) in args.iter().zip(inputs.iter()).enumerate() {
                    let arg = asm.get_node(*arg).clone();
                    let arg_type = arg.typecheck(sig, locals, asm)?;
                    if !arg_type.is_assignable_to(*input_type, asm) {
                        return Err(TypeCheckError::CallArgTypeWrong {
                            got: arg_type,
                            expected: *input_type,
                            idx,
                            mname: asm[mref.name()].into(),
                        });
                    }
                }
                Ok(mref.output(asm))
            }
            CILNode::CallI(info) => {
                let (fn_ptr, called_sig, args) = info.as_ref();
                let fn_ptr = asm.get_node(*fn_ptr).clone();
                let fn_ptr = fn_ptr.typecheck(sig, locals, asm)?;
                let called_sig = asm[*called_sig].clone();
                if args.len() != called_sig.inputs().len() {
                    return Err(TypeCheckError::IndirectCallArgcWrong {
                        expected: called_sig.inputs().len(),
                        got: args.len(),
                    });
                }

                for (idx, (arg, input_type)) in
                    args.iter().zip(called_sig.inputs().iter()).enumerate()
                {
                    let arg = asm.get_node(*arg).clone();
                    let arg_type = arg.typecheck(sig, locals, asm)?;
                    if !arg_type.is_assignable_to(*input_type, asm) {
                        return Err(TypeCheckError::IndirectCallArgTypeWrong {
                            got: arg_type,
                            expected: *input_type,
                            idx,
                        });
                    }
                }
                let Type::FnPtr(ptr_sig) = fn_ptr else {
                    return Err(TypeCheckError::IndirectCallInvalidFnPtrType { fn_ptr });
                };
                let ptr_sig = &asm[ptr_sig];
                if *ptr_sig != called_sig {
                    return Err(TypeCheckError::IndirectCallInvalidFnPtrSig {
                        expected: called_sig,
                        got: ptr_sig.clone(),
                    });
                }
                Ok(*called_sig.output())
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = asm.get_node(*input).clone();
                let input = input.typecheck(sig, locals, asm)?;
                match input {
                    Type::Float(_) | Type::Int(_) | Type::Ptr(_) | Type::FnPtr(_) | Type::Bool => {
                        Ok(Type::Int(*target))
                    }
                    _ => Err(TypeCheckError::IntCastInvalidInput {
                        got: input,
                        target: *target,
                    }),
                }
            }
            CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => {
                let input = asm.get_node(*input).clone();
                let input = input.typecheck(sig, locals, asm)?;
                match input {
                    Type::Float(_) | Type::Int(_) => Ok(Type::Float(*target)),
                    _ => Err(TypeCheckError::FloatCastInvalidInput {
                        got: input,
                        target: *target,
                    }),
                }
            }
            CILNode::RefToPtr(refn) => {
                let refn = asm.get_node(*refn).clone();
                let tpe = refn.typecheck(sig, locals, asm)?;
                match tpe {
                    Type::Ref(inner) | Type::Ptr(inner) => Ok(asm.nptr(asm[inner])),
                    _ => Err(TypeCheckError::RefToPtrArgNotRef { arg: tpe }),
                }
            }
            CILNode::PtrCast(arg, res) => {
                let arg = asm.get_node(*arg).clone();
                let arg_tpe = arg.typecheck(sig, locals, asm)?;
                match arg_tpe {
                    Type::Ptr(_)
                    | Type::Ref(_)
                    | Type::Int(Int::USize | Int::ISize)
                    | Type::FnPtr(_) => (),
                    _ => Err(TypeCheckError::InvalidPtrCast {
                        expected: res.as_ref().clone(),
                        got: arg_tpe,
                    })?,
                };
                Ok(res.as_ref().as_type())
            }
            CILNode::LdFieldAdress { addr, field } => {
                let field = *asm.get_field(*field);
                let addr = asm.get_node(*addr).clone();
                let addr_tpe = addr.typecheck(sig, locals, asm)?;
                let pointed_tpe = {
                    match addr_tpe {
                        Type::Ptr(type_idx) | Type::Ref(type_idx) => Some(asm[type_idx]),
                        Type::ClassRef(_) => Some(addr_tpe),
                        _ => None,
                    }
                }
                .ok_or(TypeCheckError::TypeNotPtr { tpe: addr_tpe })?;

                let Type::ClassRef(pointed_owner) = pointed_tpe else {
                    return Err(TypeCheckError::FieldAccessInvalidType {
                        tpe: pointed_tpe,
                        field,
                    });
                };
                if pointed_owner != field.owner() {
                    return Err(TypeCheckError::FieldOwnerMismatch {
                        owner: pointed_owner,
                        expected_owner: field.owner(),
                        field,
                    });
                }
                match addr_tpe {
                    Type::Ref(_) => Ok(asm.nref(field.tpe())),
                    Type::Ptr(_) => Ok(asm.nptr(field.tpe())),
                    _ => panic!("impossible. Type not a pointer or ref, but got dereferned during typechecks. {addr_tpe:?}"),
                }
            }

            CILNode::LdField { addr, field } => {
                let field = *asm.get_field(*field);
                let addr = asm.get_node(*addr).clone();
                let addr_tpe = addr.typecheck(sig, locals, asm)?;
                let pointed_tpe = {
                    match addr_tpe {
                        Type::Ptr(type_idx) | Type::Ref(type_idx) => Some(asm[type_idx]),
                        Type::ClassRef(_) => Some(addr_tpe),
                        _ => None,
                    }
                }
                .ok_or(TypeCheckError::TypeNotPtr { tpe: addr_tpe })?;
                let Type::ClassRef(pointed_owner) = pointed_tpe else {
                    return Err(TypeCheckError::FieldAccessInvalidType {
                        tpe: pointed_tpe,
                        field,
                    });
                };
                if pointed_owner != field.owner() {
                    return Err(TypeCheckError::FieldOwnerMismatch {
                        owner: pointed_owner,
                        expected_owner: field.owner(),
                        field,
                    });
                }
                Ok(field.tpe())
            }
            CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => {
                let addr = asm.get_node(*addr).clone();
                let addr_tpe = addr.typecheck(sig, locals, asm)?;
                let pointed_tpe = addr_tpe
                    .pointed_to()
                    .ok_or(TypeCheckError::TypeNotPtr { tpe: addr_tpe })?;
                let pointed_tpe = asm[pointed_tpe];
                let tpe = asm[*tpe];
                if !pointed_tpe.is_assignable_to(tpe, asm) {
                    Err(TypeCheckError::DerfWrongPtr {
                        expected: tpe,
                        got: pointed_tpe,
                    })
                } else {
                    Ok(pointed_tpe)
                }
            }
            CILNode::SizeOf(tpe) => match asm[*tpe] {
                Type::Void => Err(TypeCheckError::SizeOfVoid),
                _ => Ok(Type::Int(Int::I32)),
            },
            CILNode::GetException => Ok(Type::ClassRef(ClassRef::exception(asm))),
            CILNode::IsInst(obj, _) => {
                let obj = asm.get_node(*obj).clone();
                let _obj = obj.typecheck(sig, locals, asm)?;
                // TODO: check obj
                Ok(Type::Bool)
            }
            CILNode::CheckedCast(obj, cast_res) => {
                let obj = asm.get_node(*obj).clone();
                let _obj = obj.typecheck(sig, locals, asm)?;
                // TODO: check obj
                Ok(asm[*cast_res])
            }

            CILNode::LocAlloc { size } => todo!(),
            CILNode::LdStaticField(sfld) => {
                let sfld = *asm.get_static_field(*sfld);
                Ok(sfld.tpe())
            }
            CILNode::LdStaticFieldAdress(sfld) => {
                let sfld = *asm.get_static_field(*sfld);
                Ok(asm.nptr(sfld.tpe()))
            }
            CILNode::LdFtn(mref) => {
                let mref = &asm[*mref];
                Ok(Type::FnPtr(mref.sig()))
            }
            CILNode::LdTypeToken(_) => Ok(Type::ClassRef(ClassRef::runtime_type_hadle(asm))),
            CILNode::LdLen(arr) => {
                let arr = asm.get_node(*arr).clone();
                let arr_tpe = arr.typecheck(sig, locals, asm)?;
                let Type::PlatformArray { elem: _, dims } = arr_tpe else {
                    return Err(TypeCheckError::LdLenArgNotArray { got: arr_tpe });
                };
                if dims.get() != 1 {
                    return Err(TypeCheckError::LdLenArrNot1D { got: arr_tpe });
                }
                Ok(Type::Int(Int::I32))
            }
            CILNode::LocAllocAlgined { tpe, align } => Ok(Type::Ptr(*tpe)),
            CILNode::LdElelemRef { array, index } => {
                let arr = asm.get_node(*array).clone();
                let arr_tpe = arr.typecheck(sig, locals, asm)?;
                let index = asm.get_node(*index).clone();
                let index_tpe = index.typecheck(sig, locals, asm)?;
                let Type::PlatformArray { elem, dims } = arr_tpe else {
                    return Err(TypeCheckError::LdLenArgNotArray { got: arr_tpe });
                };
                if dims.get() != 1 {
                    return Err(TypeCheckError::LdLenArrNot1D { got: arr_tpe });
                }
                match index_tpe {
                    Type::Int(Int::I32 | Int::U32 | Int::I64 | Int::USize | Int::ISize) => (),
                    _ => return Err(TypeCheckError::ArrIndexInvalidType { index_tpe }),
                }
                Ok(asm[elem])
            }
            CILNode::UnboxAny { object, tpe } => {
                let object = asm.get_node(*object).clone();
                let object = object.typecheck(sig, locals, asm)?;
                match object {
                    Type::ClassRef(cref) => {
                        let cref = asm.class_ref(cref);
                        if cref.is_valuetype() {
                            return Err(TypeCheckError::ExpectedClassGotValuetype {
                                cref: cref.clone(),
                            });
                        }
                    }
                    Type::PlatformObject | Type::PlatformGeneric(_, _) | Type::PlatformString => (),
                    _ => return Err(TypeCheckError::TypeNotClass { object }),
                };
                Ok(asm[*tpe])
            }
        }
    }
}
impl CILRoot {
    pub fn typecheck(
        &self,
        sig: SigIdx,
        locals: &[LocalDef],
        asm: &mut Assembly,
    ) -> Result<(), TypeCheckError> {
        for node in self.nodes() {
            asm.get_node(*node).clone().typecheck(sig, locals, asm)?;
        }
        Ok(())
    }
}
#[test]
fn test() {
    let mut asm = Assembly::default();
    let lhs = super::Const::I64(0);
    let rhs = super::Const::F64(super::hashable::HashableF64(0.0));
    let sum = asm.biop(lhs, rhs, BinOp::Add);
    let _sum = asm.alloc_node(sum);
    let _sig = asm.sig([], Type::Void);
}
