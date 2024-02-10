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
    InvalidCallArg {
        index: usize,
        expected: Type,
        got: Type,
    },
    StackUnderflow,
    PointerExpected(Type),
    InvalidExceptionType(Type),
    StackUnexpected(Type),
    StackUnexpected2 {
        expected: Type,
        got: Type,
    },
    OpUnsuported(Type),
    OpUnsuported2(Type, Type),
    OperandsMismatch(CILOp, Type, Type),
}
pub fn verify(method: &Method) {
    let mut stack = Vec::with_capacity(64);
    let mut errors = Vec::new();
    for op in method.get_ops().iter() {
        if let Err(error) = verify_op(op, &mut stack, method) {
            errors.push(Some(error));
        } else {
            errors.push(None);
        }
    }
    if errors.iter().any(|e| e.is_some()) {
        eprintln!(
            "Could not verify the method {name} due to {ecount} errors:",
            name = method.name(),
            ecount = errors.iter().filter(|e| e.is_some()).count()
        );
        for (op, error) in method.get_ops().iter().zip(errors.iter()) {
            match error {
                Some(err) => eprintln!("\x1b[1;31m{op:?}:{err:?}\x1b[0m"),
                None => eprintln!("{op:?}"),
            }
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
            _ => Err(VerificationFailure::StackUnexpected(tpe)),
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
        (Type::I32 | Type::U32, Type::U32 | Type::I32) => true,
        (Type::I64 | Type::U64, Type::U64 | Type::I64) => true,
        (Type::ISize | Type::USize | Type::Ptr(_), Type::ISize | Type::USize | Type::Ptr(_)) => {
            true
        }
        (Type::U128 | Type::I128, Type::DotnetType(rf))
        | (Type::DotnetType(rf), Type::U128 | Type::I128) => {
            if rf.asm() == Some("System.Runtime")
                && (rf.name_path() == "System.Int128" || rf.name_path() == "System.UInt128")
            {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
fn verify_op(
    op: &CILOp,
    stack: &mut Vec<Type>,
    method: &Method,
) -> Result<(), VerificationFailure> {
    match op {
        CILOp::LDFtn(_) => Ok(stack.push(Type::USize)),
        CILOp::Label(_) | CILOp::Comment(_) | CILOp::Break | CILOp::Nop | CILOp::Volatile => Ok(()),
        CILOp::GoTo(_) => Ok(()),
        CILOp::LDTypeToken(_) => Ok(stack.push(DotnetTypeRef::type_handle_type().into())),
        CILOp::LdcI32(_) | CILOp::SizeOf(_) => Ok(stack.push(Type::I32)),
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
                        Err(VerificationFailure::StackUnexpected2 { expected: ret, got })
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
                _ => Err(VerificationFailure::StackUnexpected(tpe)),
            }
        }
        CILOp::ConvISize(_) => conv!(stack, op, Type::ISize),
        CILOp::ConvF32(_) => conv!(stack, op, Type::F32),
        CILOp::ConvF64(_) => conv!(stack, op, Type::F64),
        CILOp::ConvI8(_) | CILOp::ConvI16(_) | CILOp::ConvI32(_) => conv!(stack, op, Type::I32),
        CILOp::ConvU8(_) | CILOp::ConvU16(_) | CILOp::ConvU32(_) => conv!(stack, op, Type::U32),
        CILOp::Dup => {
            let t = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            stack.push(t.clone());
            stack.push(t);
            Ok(())
        }
        CILOp::Add | CILOp::Sub | CILOp::Mul | CILOp::Div | CILOp::Rem => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let b = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&a, &b) {
                if (a == Type::USize && b == Type::I32) || (a == Type::I32 && b == Type::USize) {
                    return Ok(stack.push(Type::USize));
                }
                return Err(VerificationFailure::OperandsMismatch(op.clone(), a, b));
            }
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize | Type::F32 | Type::F64 => {
                    Ok(stack.push(a))
                }
                _ => Err(VerificationFailure::OpUnsuported2(a, b)),
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
                _ => Err(VerificationFailure::OpUnsuported2(a, b)),
            }
        }
        CILOp::Shl => {
            let ammount = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let value = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match ammount {
                Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => Ok(()),
                Type::USize | Type::ISize => Ok(()),
                _ => Err(VerificationFailure::OpUnsuported2(
                    value.clone(),
                    ammount.clone(),
                )),
            }?;
            match value {
                Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => {
                    Ok(stack.push(Type::I32))
                }
                Type::I64 | Type::ISize | Type::I64 | Type::U64 => Ok(stack.push(value)),
                _ => Err(VerificationFailure::OpUnsuported2(value, ammount)),
            }
        }
        CILOp::Shr => {
            let ammount = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let value = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match ammount {
                Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => Ok(()),
                Type::USize | Type::ISize => Ok(()),
                _ => Err(VerificationFailure::OpUnsuported2(
                    value.clone(),
                    ammount.clone(),
                )),
            }?;
            match value {
                Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => {
                    Ok(stack.push(Type::I32))
                }
                Type::I64 | Type::ISize | Type::I64 | Type::U64 => Ok(stack.push(value)),
                _ => Err(VerificationFailure::OpUnsuported2(value, ammount)),
            }
        }
        CILOp::ShrUn => {
            let ammount = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let value = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match ammount {
                Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => Ok(()),
                Type::USize | Type::ISize => Ok(()),
                _ => Err(VerificationFailure::OpUnsuported2(
                    value.clone(),
                    ammount.clone(),
                )),
            }?;
            match value {
                Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => {
                    Ok(stack.push(Type::I32))
                }
                Type::I64 | Type::ISize | Type::I64 | Type::U64 => Ok(stack.push(value)),
                _ => Err(VerificationFailure::OpUnsuported2(value, ammount)),
            }
        }
        CILOp::Not => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize => Ok(stack.push(a)),
                _ => Err(VerificationFailure::OpUnsuported(a)),
            }
        }
        CILOp::Neg => {
            let a = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            match a {
                Type::U8 | Type::U16 | Type::U32 => Ok(stack.push(Type::U32)),
                Type::I8 | Type::I16 | Type::I32 => Ok(stack.push(Type::I32)),
                Type::U64 | Type::I64 | Type::USize | Type::ISize | Type::F32 | Type::F64 => {
                    Ok(stack.push(a))
                }
                _ => Err(VerificationFailure::OpUnsuported(a)),
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
                _ => Err(VerificationFailure::OpUnsuported(a)),
            }
        }
        CILOp::Call(call_info) | CILOp::CallVirt(call_info) => {
            for (index, input) in call_info
                .inputs()
                .iter()
                .rev()
                .cloned()
                .map(to_stacktype)
                .enumerate()
            {
                let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
                if !equivalent(&input, &stack_input) {
                    if index == call_info.inputs().len() - 1
                        && !call_info.is_static()
                        && matches!(input, Type::USize | Type::ISize | Type::ManagedReference(_))
                    {
                        continue;
                    }
                    return Err(VerificationFailure::InvalidCallArg {
                        got: stack_input,
                        expected: input,
                        index: index,
                    });
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
        CILOp::CallI(sig) => {
            let fn_ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            for input in sig.inputs().iter().rev().cloned().map(to_stacktype) {
                let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
                if !equivalent(&input, &stack_input) {
                    return Err(VerificationFailure::StackUnexpected(stack_input));
                }
            }
            match sig.output() {
                Type::Void => Ok(()),
                _ => {
                    stack.push(to_stacktype(sig.output().clone()));
                    Ok(())
                }
            }
        }
        CILOp::NewObj(call_info) => {
            for input in call_info
                .explicit_inputs()
                .iter()
                .rev()
                .cloned()
                .map(to_stacktype)
            {
                let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
                if !equivalent(&input, &stack_input) {
                    return Err(VerificationFailure::StackUnexpected(stack_input));
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
        CILOp::LDIndI8 | CILOp::LDIndI16 | CILOp::LDIndI32 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                stack.push(Type::I32);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }

        CILOp::STIndI8 | CILOp::STIndI16 | CILOp::STIndI32 => {
            //[dst,val]
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::Ptr(_) | Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                return Err(VerificationFailure::StackUnexpected(ptr));
            }
            if matches!(stack_input, Type::U32 | Type::I32) {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::STIndI64 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::Ptr(_) | Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                return Err(VerificationFailure::StackUnexpected(ptr));
            }
            if matches!(stack_input, Type::U64 | Type::I64) {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::STIndF64 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::Ptr(_) | Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                return Err(VerificationFailure::StackUnexpected(ptr));
            }
            if matches!(stack_input, Type::F64) {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::STIndF32 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::Ptr(_) | Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                return Err(VerificationFailure::StackUnexpected(ptr));
            }
            if matches!(stack_input, Type::F32) {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::STIndISize => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::Ptr(_) | Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                return Err(VerificationFailure::PointerExpected(ptr));
            }
            if matches!(stack_input, Type::USize | Type::ISize | Type::Ptr(_)) {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::STObj(tpe) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::Ptr(_) | Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                return Err(VerificationFailure::StackUnexpected(ptr));
            }
            if stack_input == **tpe {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::LdObj(tpe) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                stack.push(*tpe.clone());
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::LDIndISize => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                stack.push(Type::ISize);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::LDIndI64 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                stack.push(Type::I64);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::LDIndF32 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                stack.push(Type::F32);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::LDIndF64 => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_)
            ) {
                stack.push(Type::F64);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        CILOp::LDStaticField(field_desc) => {
            stack.push(to_stacktype(field_desc.tpe().clone()));
            Ok(())
        }

        CILOp::STStaticField(field_desc) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !equivalent(&stack_input, field_desc.tpe()) {
                stack.push(stack_input);
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
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
        CILOp::LDField(filed_desc) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_) | Type::Ptr(_)
            ) || Some(filed_desc.owner()) == stack_input.as_dotnet().as_ref()
            {
                stack.push(to_stacktype(filed_desc.tpe().clone()));
                Ok(())
            } else {
                Err(VerificationFailure::PointerExpected(stack_input))
            }
        }
        CILOp::LDFieldAdress(filed_desc) => {
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if matches!(
                stack_input,
                Type::USize | Type::ISize | Type::ManagedReference(_) | Type::Ptr(_)
            ) || Some(filed_desc.owner()) == stack_input.as_dotnet().as_ref()
            {
                stack.push(Type::Ptr(to_stacktype(filed_desc.tpe().clone()).into()));
                Ok(())
            } else {
                Err(VerificationFailure::PointerExpected(stack_input))
            }
        }
        CILOp::STField(filed_desc) => {
            // [ptr,val]
            let stack_input = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            let ptr = stack.pop().ok_or(VerificationFailure::StackUnderflow)?;
            if !matches!(
                ptr,
                Type::USize | Type::ISize | Type::ManagedReference(_) | Type::Ptr(_)
            ) || Some(filed_desc.owner()) == stack_input.as_dotnet().as_ref()
            {
                return Err(VerificationFailure::PointerExpected(ptr));
            }
            if equivalent(&stack_input, &to_stacktype(filed_desc.tpe().clone())) {
                Ok(())
            } else {
                Err(VerificationFailure::StackUnexpected(stack_input))
            }
        }
        _ => todo!("Can't verifiy op {op:?}"),
    }
}
