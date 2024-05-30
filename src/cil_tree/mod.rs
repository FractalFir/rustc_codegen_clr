/*
pub(crate) fn validate(&self, method: &crate::method::Method) -> Result<(), String> {
    match self {
        CILRoot::STIndI8(addr, val) => {
            let addr = addr.validate(method)?;
            let val = val.validate(method)?;
            match &addr {
                Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                    Type::I8 | Type::U8 => (),
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                        ))
                    }
                },
                _ => {
                    return Err(format!(
                        "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                    ))
                }
            }
            match val{
                Type::I8 | Type::U8 => Ok(()),
                _=>Err(format!("Can't indirectly set a valur of type i8/u8 because the provided value is {val:?}")),
            }
        }

        CILRoot::STIndISize(addr, val) => {
            let addr = addr.validate(method)?;
            let val = val.validate(method)?;
            match &addr {
                Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                    Type::Ptr(_) | Type::ManagedReference(_) | Type::USize | Type::ISize => (),
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                        ))
                    }
                },
                _ => {
                    return Err(format!(
                        "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                    ))
                }
            }
            match val{
                Type::I8 | Type::U8 => Ok(()),
                _=>Err(format!("Can't indirectly set a valur of type i8/u8 because the provided value is {val:?}")),
            }
        }
        CILRoot::Break => Ok(()),
        CILRoot::JumpingPad { ops: _ } => Ok(()),
        CILRoot::BTrue {
            target: _,
            sub_target: _,
            cond,
        } => {
            // Just check that `cond` is a boolean.
            let cond = cond.validate(method)?;
            if cond != Type::Bool {
                Err(format!(
                    "BTrue must have a boolean argument. cond is:{cond:?}"
                ))
            } else {
                Ok(())
            }
        }
        CILRoot::GoTo {
            target: _,
            sub_target: _,
        } => Ok(()),
        CILRoot::STLoc { local, tree } => {
            let expected_tpe = if let Some(loc) = method.locals().get(*local as usize) {
                loc
            } else {
                return Err(format!("Local out of range! Local{local:?}"));
            };
            let got = tree.validate(method)?;
            if expected_tpe.1 != got {
                Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting local {local:?}"))
            } else {
                Ok(())
            }
        }
        CILRoot::STArg { arg, tree } => {
            let expected_tpe = if let Some(arg) = method.locals().get(*arg as usize) {
                arg
            } else {
                return Err(format!("Arg out of range! Arg {arg:?}"));
            };
            let got = tree.validate(method)?;
            if expected_tpe.1 != got {
                Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting local {arg:?}"))
            } else {
                Ok(())
            }
        }
        CILRoot::Call { site, args } => {
            if site.inputs().len() != args.len() {
                return Err(format!(
                    "Expected {} arguments, got {}",
                    site.explicit_inputs().len(),
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
            Ok(())
        }
        CILRoot::CallI { args, sig, fn_ptr } => {
            let _ptr = fn_ptr.validate(method)?;
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
            Ok(())
        }
        CILRoot::VoidRet => Ok(()),
        CILRoot::SourceFileInfo(_) => Ok(()),
        CILRoot::Nop => Ok(()),
        CILRoot::Throw(execption) => {
            let tpe = execption.validate(method)?;
            if tpe.as_dotnet().is_some() {
                Ok(())
            } else {
                Err("`throw` instruction suplied with a non-object type.".into())
            }
        }
        CILRoot::Ret { tree } => {
            let expected = method.sig().output();
            let got = tree.validate(method)?;
            if got != *expected {
                Err(format!(
                    "Mismatched return type. Expected {expected:?} got {got:?}"
                ))
            } else {
                Ok(())
            }
        }
        CILRoot::SetField { addr, value, desc } => {
            let addr = addr.validate(method)?;
            let value = value.validate(method)?;
            if *desc.tpe() != value {
                return Err(format!(
                    "Mismatched field type. Expected {expected:?} got {value:?}",
                    expected = desc.tpe(),
                ));
            }
            match addr {
                Type::ManagedReference(tpe) | Type::Ptr(tpe) => {
                    if tpe.as_dotnet() != Some(desc.owner().clone()) {
                        return Err(format!(
                            "Mismatched pointer type. Expected {desc:?} got {tpe:?}"
                        ));
                    }
                }
                _ => (),
            }
            Ok(())
        }
        _ => todo!("Can't check the type safety of cil root {self:?}"),
    }
}*/
