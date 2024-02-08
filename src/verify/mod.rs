use crate::{
    call_info,
    cil::CILOp,
    method::Method,
    r#type::{DotnetTypeRef, Type},
};
#[derive(Debug, Clone)]
enum VerificationFailure {
    LocalOutOfRange(usize),
    ArgOutOfRange(usize),
    InvalidLocalAssigement {
        index: usize,
        expected: Type,
        got: Type,
    },
    InvalidArgAssigement {
        index: usize,
        expected: Type,
        got: Type,
    },
    StackUnderflow,
    InvalidExceptionType(Type),
    StackUnexpected(CILOp, Type),
    OperandsMismatch(CILOp, Type, Type),
}
pub fn verify(method: &Method) {
    let mut stack = Vec::with_capacity(64);
    let mut errors = Vec::new();
    for (index, op) in method.get_ops().iter().enumerate() {
        if let Err(error) = verify_op(op, &mut stack, method) {
            errors.push((error, index));
        }
    }
    if !errors.is_empty() {
        eprintln!(
            "Could not verify the method {name} due to {ecount} errors:",
            name = method.name(),
            ecount = errors.len()
        );
        for (err, index) in errors {
            eprintln!(
                "op {index}:{op:?} resulted in error {err:?}",
                op = method.get_ops()[index]
            );
        }
    }
}
fn get_nth_local(method: &Method, index: usize) -> Result<Type, VerificationFailure> {
    method
        .locals()
        .get(index)
        .map(|(_type, tpe)| tpe)
        .cloned()
        .ok_or(VerificationFailure::LocalOutOfRange(index))
        .map(to_stacktype)
}
fn get_nth_arg(method: &Method, index: usize) -> Result<Type, VerificationFailure> {
    method
        .sig()
        .inputs()
        .get(index)
        .cloned()
        .ok_or(VerificationFailure::ArgOutOfRange(index))
        .map(to_stacktype)
}
fn to_stacktype(tpe: Type) -> Type {
    match tpe {
        Type::U8 | Type::U16 | Type::DotnetChar => Type::U32,
        Type::I8 | Type::I16 => Type::I32,
        Type::Bool => Type::I32,
        Type::Ptr(_) => Type::USize,
        Type::DelegatePtr(_) => Type::USize,
        _ => tpe,
    }
}
macro_rules! conv {
    ($stack:ident,$op:ident,$res:expr) => {{
        let tpe = $stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
        match tpe {
            Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32
            | Type::U64
            | Type::I64
            | Type::USize
            | Type::ISize
            | Type::F32
            | Type::F64 => Ok($stack.push($res)),
            _ => Err(VerificationFailure::StackUnexpected($op.clone(), tpe)),
        }
    }};
}
fn equivalent(a: &Type, b: &Type) -> bool {
    if a == b {
        return true;
    }
    match (a, b) {
        (Type::Unresolved, _) => true,
        (_, Type::Unresolved) => true,
        (Type::I32, Type::U32) => true,
        (Type::U32, Type::I32) => true,
        (Type::ISize, Type::USize) => true,
        (Type::USize, Type::ISize) => true,
        _ => false,
    }
}
fn verify_op(
    op: &CILOp,
    stack: &mut Vec<Type>,
    method: &Method,
) -> Result<(), VerificationFailure> {
    match op {
        CILOp::Label(_) | CILOp::Comment(_) | CILOp::Break | CILOp::Nop => Ok(()),
        CILOp::GoTo(_) => Ok(()),
        CILOp::LdcI32(_) => Ok(stack.push(Type::I32)),
        CILOp::LdcF32(_) => Ok(stack.push(Type::F32)),
        CILOp::LdcI64(_) => Ok(stack.push(Type::I64)),
        CILOp::LdcF64(_) => Ok(stack.push(Type::F64)),
        CILOp::LdStr(_) => Ok(stack.push(DotnetTypeRef::string_type().into())),
        CILOp::Ret => {
            let ret = to_stacktype(method.sig().output().clone());
            match ret {
                Type::Void => Ok(()),
                _ => {
                    let got = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
                    if !equivalent(&got, &ret) {
                        eprintln!("ret:{ret:?} got:{got:?}");
                        Err(VerificationFailure::StackUnexpected(CILOp::Ret, got))
                    } else {
                        Ok(())
                    }
                }
            }
        }
        CILOp::LDLoc(index) => {
            let tpe = get_nth_local(method, *index as usize)?;
            stack.push(tpe);
            Ok(())
        }
        CILOp::LDArg(index) => {
            let tpe = get_nth_arg(method, *index as usize)?;
            stack.push(tpe);
            Ok(())
        }
        CILOp::STLoc(index) => {
            let expected = get_nth_local(method, *index as usize)?;
            let got = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&expected, &got) {
                Err(VerificationFailure::InvalidLocalAssigement {
                    index: *index as usize,
                    expected,
                    got,
                })
            } else {
                Ok(())
            }
        }
        CILOp::STArg(index) => {
            let expected = get_nth_arg(method, *index as usize)?;
            let got = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&expected, &got) {
                Err(VerificationFailure::InvalidArgAssigement {
                    index: *index as usize,
                    expected,
                    got,
                })
            } else {
                Ok(())
            }
        }
        CILOp::LDLocA(index) => {
            let tpe = get_nth_local(method, *index as usize)?;
            stack.push(Type::ManagedReference(tpe.into()));
            Ok(())
        }
        CILOp::LDArgA(index) => {
            let tpe = get_nth_arg(method, *index as usize)?;
            stack.push(Type::ManagedReference(tpe.into()));
            Ok(())
        }
        CILOp::ConvU64(_) => conv!(stack, op, Type::U64),
        CILOp::ConvI64(_) => conv!(stack, op, Type::I64),
        CILOp::ConvUSize(_) => {
            let tpe = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match tpe {
                Type::U8
                | Type::I8
                | Type::U16
                | Type::I16
                | Type::U32
                | Type::I32
                | Type::U64
                | Type::I64
                | Type::USize
                | Type::ISize
                | Type::F32
                | Type::F64
                | Type::Ptr(_)
                | Type::ManagedReference(_)
                | Type::DelegatePtr(_) => Ok(stack.push(Type::USize)),
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), tpe)),
            }
        }
        CILOp::ConvISize(_) => conv!(stack, op, Type::ISize),
        CILOp::ConvF32(_) => conv!(stack, op, Type::F32),
        CILOp::ConvF64(_) => conv!(stack, op, Type::F64),
        CILOp::ConvI8(_) | CILOp::ConvI16(_) | CILOp::ConvI32(_) => conv!(stack, op, Type::I32),
        CILOp::ConvU8(_) | CILOp::ConvU16(_) | CILOp::ConvU32(_) => conv!(stack, op, Type::U32),
        CILOp::Add | CILOp::Sub | CILOp::Mul | CILOp::Div | CILOp::Rem => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let b = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&a, &b) {
                return Err(VerificationFailure::OperandsMismatch(op.clone(), a, b));
            }
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize | Type::F32 | Type::F64 => {
                    Ok(stack.push(a))
                }
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), a)),
            }
        }
        CILOp::Or | CILOp::XOr | CILOp::And => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let b = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&a, &b) {
                return Err(VerificationFailure::OperandsMismatch(op.clone(), a, b));
            }
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize => Ok(stack.push(a)),
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), a)),
            }
        }
        CILOp::Shr => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let b = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize => Ok(stack.push(a)),
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), a)),
            }?;
            match b {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize => Ok(()),
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), b)),
            }
        }
        CILOp::Not | CILOp::Neg => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize => Ok(stack.push(a)),
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), a)),
            }
        }
        CILOp::Eq | CILOp::Gt | CILOp::Lt => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let b = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&a, &b) {
                Err(VerificationFailure::OperandsMismatch(op.clone(), a, b))
            } else {
                stack.push(Type::I32);
                Ok(())
            }
        }
        CILOp::BTrue(_) => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize => Ok(stack.push(a)),
                _ => Err(VerificationFailure::StackUnexpected(op.clone(), a)),
            }
        }
        CILOp::Call(call_info) => {
            for input in call_info.inputs().iter().cloned().map(to_stacktype) {
                let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
                if !equivalent(&input, &stack_input) {
                    return Err(VerificationFailure::StackUnexpected(
                        op.clone(),
                        stack_input,
                    ));
                }
            }
            match call_info.signature().output() {
                Type::Void => Ok(()),
                _ => {
                    stack.push(to_stacktype(call_info.signature().output().clone()));
                    Ok(())
                }
            }
        }
        CILOp::NewObj(call_info) => {
            for input in call_info
                .explicit_inputs()
                .iter()
                .cloned()
                .map(to_stacktype)
            {
                let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
                if !equivalent(&input, &stack_input) {
                    return Err(VerificationFailure::StackUnexpected(
                        op.clone(),
                        stack_input,
                    ));
                }
            }
            stack.push(call_info.class().unwrap().clone().into());
            Ok(())
        }

        CILOp::Throw => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            stack_input
                .as_dotnet()
                .ok_or(VerificationFailure::InvalidExceptionType(stack_input))?;
            Ok(())
        }
        CILOp::LDIndI8 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(stack_input, Type::USize | Type::ISize) {
                stack.push(Type::I32);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(
                    op.clone(),
                    stack_input,
                ))
            }
        }
        CILOp::LdObj(tpe) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(stack_input, Type::USize | Type::ISize) {
                stack.push(*tpe.clone());
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(
                    op.clone(),
                    stack_input,
                ))
            }
        }
        CILOp::LDIndISize => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(stack_input, Type::USize | Type::ISize) {
                stack.push(Type::ISize);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(
                    op.clone(),
                    stack_input,
                ))
            }
        }
        CILOp::LDStaticField(field_desc) => {
            stack.push(field_desc.tpe().clone());
            Ok(())
        }

        CILOp::STStaticField(field_desc) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&stack_input, field_desc.tpe()) {
                stack.push(stack_input);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(
                    op.clone(),
                    stack_input,
                ))
            }
        }
        CILOp::BEq(_) => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let b = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&a, &b) {
                return Err(VerificationFailure::OperandsMismatch(op.clone(), a, b));
            }
            Ok(())
        }
        CILOp::LDField(filed_desc)=>{
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(stack_input,Type::USize | Type::ISize | Type::ManagedReference(_)){
                stack.push(filed_desc.tpe().clone());
                Ok(())
            }
            else{
               Err(VerificationFailure::StackUnexpected(op.clone(), stack_input)) 
            }
        }
        CILOp::LDFieldAdress(filed_desc)=>{
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(stack_input,Type::USize | Type::ISize | Type::ManagedReference(_)){
                stack.push(Type::Ptr(filed_desc.tpe().clone().into()));
                Ok(())
            }
            else{
               Err(VerificationFailure::StackUnexpected(op.clone(), stack_input)) 
            }
        }
        _ => todo!("Can't verifiy op {op:?}"),
    }
}
