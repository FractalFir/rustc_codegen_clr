use crate::IString;

use super::{
    cilnode::{PtrCastRes, UnOp},
    method::LocalDef,
    Assembly, BinOp, CILNode, CILRoot, ClassRef, Int, SigIdx, Type,
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
}
impl BinOp {
    fn typecheck(&self, lhs: Type, rhs: Type) -> Result<Type, TypeCheckError> {
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
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::Eq => {
                if lhs == rhs {
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
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::LtUn | BinOp::GtUn => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs && !rhs.is_signed() => {
                    Ok(Type::Bool)
                }
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::Lt | BinOp::Gt => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs && rhs.is_signed() => Ok(Type::Bool),
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::Or | BinOp::XOr | BinOp::And => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs => Ok(Type::Int(lhs)),
                (Type::Bool, Type::Bool) => Ok(Type::Bool),
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::Rem => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs && rhs.is_signed() => {
                    Ok(Type::Int(lhs))
                }
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::RemUn => match (lhs, rhs) {
                (Type::Int(lhs), Type::Int(rhs)) if rhs == lhs && !rhs.is_signed() => {
                    Ok(Type::Int(lhs))
                }
                (Type::Float(lhs), Type::Float(rhs)) if rhs == lhs => Ok(Type::Bool),
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::Shl => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::I64
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
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::Shr => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::I64
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
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::ShrUn => match (lhs, rhs) {
                (
                    Type::Int(
                        lhs @ (Int::I64
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
                _ => Err(TypeCheckError::WrongBinopArgs {
                    lhs,
                    rhs,
                    op: *self,
                }),
            },
            BinOp::DivUn => todo!(),
            BinOp::Div => todo!(),
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
                op.typecheck(lhs, rhs)
            }
            CILNode::UnOp(arg, op) => {
                let arg = asm.get_node(*arg).clone();
                let arg_type = arg.typecheck(sig, locals, asm)?;
                match arg_type {
                    Type::Int(_) | Type::Float(_) => Ok(arg_type),
                    _ => Err(TypeCheckError::WrongUnOpArgs {
                        tpe: arg_type,
                        op: op.clone(),
                    }),
                }
            }
            CILNode::LdLoc(loc) => Ok(*asm.get_type(locals[*loc as usize].1)),
            CILNode::LdLocA(loc) => Ok(asm.nref(*asm.get_type(locals[*loc as usize].1))),
            CILNode::LdArg(arg) => Ok(asm.get_sig(sig).inputs()[*arg as usize]),
            CILNode::LdArgA(arg) => Ok(asm.nref(asm.get_sig(sig).inputs()[*arg as usize])),
            CILNode::Call(call_info) => {
                let (mref, args) = call_info.as_ref();
                let mref = asm.get_mref(*mref).clone();
                let inputs: Box<[_]> = mref.stack_inputs(asm).into();
                if args.len() != inputs.len() {
                    return Err(TypeCheckError::CallArgcWrong {
                        expected: inputs.len(),
                        got: args.len(),
                        mname: asm.get_string(mref.name()).clone(),
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
                            mname: asm.get_string(mref.name()).clone(),
                        });
                    }
                }
                Ok(mref.output(asm))
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = asm.get_node(*input).clone();
                let input = input.typecheck(sig, locals, asm)?;
                match input {
                    Type::Float(_) | Type::Int(_) | Type::Ptr(_) | Type::FnPtr(_) => {
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
                    Type::Ref(inner) | Type::Ptr(inner) => Ok(asm.nptr(*asm.get_type(inner))),
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
                        Type::Ptr(type_idx) | Type::Ref(type_idx) => Some(*asm.get_type(type_idx)),
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
                        Type::Ptr(type_idx) | Type::Ref(type_idx) => Some(*asm.get_type(type_idx)),
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
                let pointed_tpe = *asm.get_type(pointed_tpe);
                let tpe = *asm.get_type(*tpe);
                if !pointed_tpe.is_assignable_to(tpe, asm) {
                    Err(TypeCheckError::DerfWrongPtr {
                        expected: tpe,
                        got: pointed_tpe,
                    })
                } else {
                    Ok(pointed_tpe)
                }
            }
            CILNode::SizeOf(_) => Ok(Type::Int(Int::I32)),
            CILNode::GetException => todo!(),
            CILNode::IsInst(_, _) => todo!(),
            CILNode::CheckedCast(_, _) => todo!(),
            CILNode::CallI(_) => todo!(),
            CILNode::LocAlloc { size } => todo!(),
            CILNode::LdStaticField(sfld) => {
                let sfld = *asm.get_static_field(*sfld);
                Ok(sfld.tpe())
            }
            CILNode::LdFtn(_) => todo!(),
            CILNode::LdTypeToken(_) => todo!(),
            CILNode::LdLen(_) => todo!(),
            CILNode::LocAllocAlgined { tpe, align } => Ok(Type::Ptr(*tpe)),
            CILNode::LdElelemRef { array, index } => todo!(),
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
                Ok(*asm.get_type(*tpe))
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
