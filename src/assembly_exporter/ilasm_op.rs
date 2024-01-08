use std::borrow::Cow;

use crate::r#type::{DotnetTypeRef, Type};

pub fn op_cli(op: &crate::cil::CILOp) -> Cow<'static, str> {
    use crate::cil::CILOp;
    match op {
        //Control flow
        CILOp::Ret => "ret".into(),
        CILOp::Label(id) => format!("bb_{id}:").into(),
        CILOp::GoTo(id) => format!("br bb_{id}").into(),
        CILOp::BEq(id) => format!("beq bb_{id}").into(),
        CILOp::BNe(id) => format!("bne.un bb_{id}").into(),
        CILOp::BGe(id) => format!("bge bb_{id}").into(),
        CILOp::BLt(id) => format!("blt bb_{id}").into(),
        CILOp::BLe(id) => format!("ble bb_{id}").into(),
        CILOp::BZero(id) => format!("brzero bb_{id}").into(),
        CILOp::BTrue(id) => format!("brtrue bb_{id}").into(),

        CILOp::Call(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let generics = if call_site.generics().is_empty(){
                    "".into()
                }else{
                    assert!(call_site.generics().len() == 1,"Methods with multiple generics not supported yet!");
                    format!("<{}>",type_cil(&call_site.generics()[0]))
                };
                let owner_name = match call_site.class() {
                    Some(owner) => {
                        format!("{}::", type_cil(&owner.clone().into()))
                    }
                    None => String::new(),
                };
                format!(
                    "call {prefix} {output} {owner_name}{function_name}{generics}({input_string})",
                    function_name = call_site.name(),
                    output = type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        CILOp::CallI(sig)=>{
            let mut inputs_iter = sig.inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg));
                }
                let callconv = "";
            format!(
                "calli {callconv} {output} ({input_string})",
                output = type_cil(sig.output())
            ).into()
        }
        CILOp::LDFtn(call_site)=>{
             //assert!(sig.inputs.is_empty());
             let mut inputs_iter = call_site.explicit_inputs().iter();
             let mut input_string = String::new();
             if let Some(firts_arg) = inputs_iter.next() {
                 input_string.push_str(&non_void_type_cil(firts_arg));
             }
             for arg in inputs_iter {
                 input_string.push(',');
                 input_string.push_str(&non_void_type_cil(arg));
             }
             let prefix = if call_site.is_static() {
                 ""
             } else {
                 "instance"
             };
             let generics = if call_site.generics().is_empty(){
                "".into()
            }else{
                assert!(call_site.generics().len() == 1,"Methods with multiple generics not supported yet!");
                format!("<{}>",type_cil(&call_site.generics()[0]))
            };
             let owner_name = match call_site.class() {
                 Some(owner) => {
                     format!("{}::", type_cil(&owner.clone().into()))
                 }
                 None => String::new(),
             };
             format!(
                 "ldftn {prefix} {output} {owner_name} {function_name}{generics}({input_string})",
                 function_name = call_site.name(),
                 output = type_cil(call_site.signature().output())
             )
             .into()
        }
        CILOp::CallVirt(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let generics = if call_site.generics().is_empty(){
                    "".into()
                }else{
                    assert!(call_site.generics().len() == 1,"Methods with multiple generics not supported yet!");
                    format!("<{}>",type_cil(&call_site.generics()[0]))
                };
                let owner_name = match call_site.class() {
                    Some(owner) => {
                        format!("{}::", type_cil(&owner.clone().into()))
                    }
                    None => String::new(),
                };
                format!(
                    "callvirt {prefix} {output} {owner_name} {function_name}{generics}({input_string})",
                    function_name = call_site.name(),
                    output = type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        //Arthmetics
        CILOp::Add => "add".into(),
        CILOp::AddOvf => "add.ovf".into(),
        CILOp::AddOvfUn => "add.ovf.un".into(),
        CILOp::Sub => "sub".into(),
        CILOp::SubOvf => "sub.ovf".into(),
        CILOp::SubOvfUn => "sub.ovf.un".into(),
        CILOp::Mul => "mul".into(),
        CILOp::MulOvf => "mul.ovf".into(),
        CILOp::Div => "div".into(),
        CILOp::Rem => "rem".into(),
        CILOp::Neg => "neg".into(),
        //Bitwise
        CILOp::And => "and".into(),
        CILOp::Or => "or".into(),
        CILOp::XOr => "xor".into(),
        CILOp::Not => "not".into(),
        //Bitshifts
        CILOp::Shl => "shl".into(),
        CILOp::Shr => "shr".into(),
        //Comparisons
        CILOp::Gt => "cgt".into(),
        CILOp::Eq => "ceq".into(),
        CILOp::Lt => "clt".into(),
        CILOp::InitBlk => "initblk".into(),
        //Arguments
        CILOp::LDArg(argnum) => {
            if *argnum < 4 {
                format!("ldarg.{argnum}").into()
            } else if u8::try_from(*argnum).is_ok() {
                format!("ldarg.s {argnum}").into()
            } else {
                format!("ldarg {argnum}").into()
            }
        }
        CILOp::LDArgA(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("ldarga.s {argnum}").into()
            } else {
                format!("ldarga {argnum}").into()
            }
        }
        CILOp::STArg(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("starg.s {argnum}").into()
            } else {
                format!("starg {argnum}").into()
            }
        }
        //Locals
        CILOp::LDLoc(argnum) => {
            if *argnum < 4 {
                format!("ldloc.{argnum}").into()
            } else if u8::try_from(*argnum).is_ok() {
                format!("ldloc.s {argnum}").into()
            } else {
                format!("ldloc {argnum}").into()
            }
        }
        CILOp::LDLocA(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("ldloca.s {argnum}").into()
            } else {
                format!("ldloca {argnum}").into()
            }
        }
        CILOp::STLoc(argnum) => {
            if *argnum < 4 {
                format!("stloc.{argnum}").into()
            } else if u8::try_from(*argnum).is_ok() {
                format!("stloc.s {argnum}").into()
            } else {
                format!("stloc {argnum}").into()
            }
        }
        //Constant
        CILOp::LdcI32(value) => {
            if *value == -1 {
                "ldc.i4.m1".into()
            } else if *value <= 8 && *value >= 0 {
                format!("ldc.i4.{value}").into()
            } else if i8::try_from(*value).is_ok() {
                format!("ldc.i4.s {value}").into()
            } else {
                format!("ldc.i4 {value}").into()
            }
        }
        CILOp::LdcI64(value) => {
            if *value == -1 {
                "ldc.i4.m1".into()
            } else if *value <= 8 && *value >= 0 {
                format!("ldc.i4.{value}").into()
            } else if i8::try_from(*value).is_ok() {
                format!("ldc.i4.s {value}\n\t").into()
            } else if i32::try_from(*value).is_ok() {
                format!("ldc.i4 {value}").into()
            } else {
                format!("ldc.i8 {value}").into()
            }
        }
        CILOp::LdNull => "ldnull".into(), 
        CILOp::LdcF32(f32const) =>{
            if f32const.is_finite() && format!("{f32const}").len() < 14{
                format!("ldc.r4 {f32const}").into()
            }
            else{
                let const_literal = f32const.to_le_bytes();
                format!("ldc.r4 ({:02x} {:02x} {:02x} {:02x})",const_literal[0],const_literal[1],const_literal[2],const_literal[3]).into()
            }
        }
        CILOp::LdcF64(f64const) => {
            if f64const.is_finite() && format!("{f64const}").len() < 26{
                format!("ldc.r8 {f64const}").into()
            }
            else{
                let const_literal = f64const.to_le_bytes();
                format!("ldc.r8 ({:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x})",const_literal[0],const_literal[1],const_literal[2],const_literal[3],const_literal[4],const_literal[5],const_literal[6],const_literal[7]).into()
            }
        }
        //Debug
        CILOp::Comment(comment) => format!("//{comment}").into(),
        //Convertions
        CILOp::ConvISize(checked) => {
            if *checked {
                "conv.ovf.i".into()
            } else {
                "conv.i".into()
            }
        }
        CILOp::ConvI8(checked) => {
            if *checked {
                "conv.ovf.i1".into()
            } else {
                "conv.i1".into()
            }
        }
        CILOp::ConvI16(checked) => {
            if *checked {
                "conv.ovf.i2".into()
            } else {
                "conv.i2".into()
            }
        }
        CILOp::ConvI32(checked) => {
            if *checked {
                "conv.ovf.i4".into()
            } else {
                "conv.i4".into()
            }
        }
        CILOp::ConvI64(checked) => {
            if *checked {
                "conv.ovf.i8".into()
            } else {
                "conv.i8".into()
            }
        }
        CILOp::ConvUSize(checked) => {
            if *checked {
                "conv.ovf.u".into()
            } else {
                "conv.u".into()
            }
        }
        CILOp::ConvU8(checked) => {
            if *checked {
                "conv.ovf.u1".into()
            } else {
                "conv.u1".into()
            }
        }
        CILOp::ConvU16(checked) => {
            if *checked {
                "conv.ovf.u2".into()
            } else {
                "conv.u2".into()
            }
        }
        CILOp::ConvU32(checked) => {
            if *checked {
                "conv.ovf.u4".into()
            } else {
                "conv.u4".into()
            }
        }
        CILOp::ConvU64(checked) => {
            if *checked {
                "conv.ovf.u8".into()
            } else {
                "conv.u8".into()
            }
        }
        CILOp::ConvF32(checked) => {
            if *checked {
                "conv.ovf.r4".into()
            } else {
                "conv.r4".into()
            }
        }
        CILOp::ConvF64(checked) => {
            if *checked {
                "conv.ovf.r8".into()
            } else {
                "conv.r8".into()
            }
        }
        // Pointer stuff
        CILOp::LDIndI8 => "ldind.i1".into(),
        CILOp::LDIndI16 => "ldind.i2".into(),
        CILOp::LDIndI32 => "ldind.i4".into(),
        CILOp::LDIndI64 => "ldind.i8".into(),
        CILOp::LDIndF32 => "ldind.r4".into(),
        CILOp::LDIndF64 => "ldind.r8".into(),
        CILOp::LDIndRef => "ldind.ref".into(),
        CILOp::STIndI8 => "stind.i1".into(),
        CILOp::STIndI16 => "stind.i2".into(),
        CILOp::STIndI32 => "stind.i4".into(),
        CILOp::STIndI64 => "stind.i8".into(),
        CILOp::STIndF32 => "stind.r4".into(),
        CILOp::STIndF64 => "stind.r8".into(),
        CILOp::LDIndISize => "ldind.i".into(),
        CILOp::STIndISize => "stind.i".into(),
        CILOp::LocAlloc => "localloc".into(),
        //OOP
        CILOp::SizeOf(tpe) => format!("sizeof {tpe}", tpe = type_cil(tpe)).into(),
        CILOp::Throw => "throw".into(),
        CILOp::Rethrow => "rethrow".into(),
        CILOp::LdStr(str) => format!("ldstr {str:?}").replace('\'',"\\\'").into(),
        CILOp::LdObj(obj) => format!(
            "ldobj {tpe}",
            tpe = type_cil(&obj.as_ref().clone())
        )
        .into(),
        CILOp::STObj(obj) => format!(
            "stobj {tpe}",
            tpe = type_cil(&obj.as_ref().clone())
        )
        .into(),
        CILOp::LDField(descr) => format!(
            "ldfld {prefixed_type} {owner}::'{field_name}'",
            prefixed_type = non_void_type_cil(descr.tpe()),
            owner = type_cil(&descr.owner().clone().into()),
            field_name = descr.name()
        )
        .into(),
        CILOp::LDFieldAdress(descr) => format!(
            "ldflda {prefixed_type} {owner}::'{field_name}'",
            prefixed_type = non_void_type_cil(descr.tpe()),
            owner = type_cil(&descr.owner().clone().into()),
            field_name = descr.name()
        )
        .into(),
        CILOp::STField(descr) => format!(
            "stfld {prefixed_type} {owner}::'{field_name}'",
            prefixed_type = non_void_type_cil(descr.tpe()),
            owner = type_cil(&descr.owner().clone().into()),
            field_name = descr.name()
        )
        .into(),
        CILOp::CpBlk=>"cpblk".into(),
        CILOp::NewObj(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let generics = if call_site.generics().is_empty(){
                    "".into()
                }else{
                    assert!(call_site.generics().len() == 1,"Methods with multiple generics not supported yet!");
                    format!("<{}>",type_cil(&call_site.generics()[0]))
                };
                let owner_name = match &call_site.class() {
                    Some(owner) => format!("{}::", dotnet_type_ref_cli(owner)),
                    None => String::new(),
                };
                format!(
                    "newobj {prefix} {output} {owner_name}{function_name}{generics}({input_string})",
                    function_name = call_site.name(),
                    output = type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        CILOp::Volatile=>"volatile.".into(),
        CILOp::Nop => "nop".into(),
        CILOp::NewTMPLocal(_) | CILOp::FreeTMPLocal | CILOp::LoadAddresOfTMPLocal | CILOp::SetTMPLocal | CILOp::LoadTMPLocal | CILOp::LoadUnderTMPLocal(_) | CILOp::LoadAdressUnderTMPLocal(_) =>
         panic!("CRITICAL INTERNAL ERROR: OP '{op:?}' is syntetic(internal only) and should have been substituted before being emmited!"),
         CILOp::LoadGlobalAllocPtr { alloc_id } => panic!("CRITICAL INTERNAL ERROR:Allocation {alloc_id} was not resolved to a static."),
        CILOp::Pop => "pop".into(),
        CILOp::Dup => "dup".into(),
        CILOp::LDStaticField(static_field) => {
            match static_field.owner(){
                Some(_owner)=>todo!("Can't load static field {static_field:?}"),
                None=>format!("ldsfld {tpe} {name}",tpe = non_void_type_cil(static_field.tpe()), name = static_field.name()).into(),
            }
        }
        CILOp::STStaticField(static_field) => {
            match static_field.owner(){
                Some(_owner)=>todo!("Can't load static field {static_field:?}"),
                None=>format!("stsfld {tpe} {name}",tpe = non_void_type_cil(static_field.tpe()), name = static_field.name()).into(),
            }
        }
        CILOp::InitObj(tpe)=>format!("initobj {tpe}",tpe = non_void_type_cil(tpe)).into(),
    }
}
pub fn non_void_type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "valuetype RustVoid".into(),
        _ => type_cil(tpe),
    }
}
pub fn type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::DelegatePtr(sig) => {
            let mut inputs_iter = sig.inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg));
            }
            format!(
                "method {output}*({input_string})",
                output = type_cil(sig.output())
            )
            .into()
        }
        Type::FnDef(name) => format!("valuetype fn_{name}").into(),
        Type::Void => "void".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::I16 => "int16".into(),
        Type::U16 => "uint16".into(),
        Type::F32 => "float32".into(),
        Type::I32 => "int32".into(),
        Type::U32 => "uint32".into(),
        Type::F64 => "float64".into(),
        Type::I64 => "int64".into(),
        Type::U64 => "uint64".into(),
        Type::I128 => "valuetype [System.Runtime]System.Int128".into(),
        Type::U128 => "valuetype [System.Runtime]System.UInt128".into(),
        Type::ISize => "native int".into(),
        Type::USize => "native uint".into(),
        Type::Ptr(inner) => format!("{inner}*", inner = type_cil(inner)).into(),
        Type::ManagedReference(inner) => format!("{inner}&", inner = type_cil(inner)).into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli(dotnet_type).into(),
        //Special type
        Type::Unresolved => "valuetype Unresolved".into(),
        Type::Bool => "bool".into(),
        Type::DotnetChar => "char".into(),
        Type::GenericArg(idx) => format!("!{idx}").into(),
        Type::CallGenericArg(idx) => format!("!!{idx}").into(),
        Type::Foreign => "valuetype Foreign".into(),
        Type::DotnetArray(array) => {
            let arr = if array.dimensions > 0 {
                (0..(array.dimensions - 1)).map(|_| ",").collect::<String>()
            } else {
                "".into()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(&array.element)).into()
        } //_ => todo!("Unsuported type {tpe:?}"),
    }
}
pub fn dotnet_type_ref_cli(dotnet_type: &DotnetTypeRef) -> String {
    let prefix = dotnet_type.tpe_prefix();
    if Some("System.Runtime") == dotnet_type.asm()
        && "System.String" == dotnet_type.name_path()
        && !dotnet_type.is_valuetype()
    {
        return "string".into();
    }
    let asm = if let Some(asm_ref) = dotnet_type.asm() {
        format!("[{asm_ref}]")
    } else {
        String::new()
    };
    let name = dotnet_type.name_path();
    let generics = generics_str(dotnet_type.generics());
    format!("{prefix} {asm}{name}{generics}")
}
fn generics_str(generics: &[Type]) -> Cow<'static, str> {
    if generics.is_empty() {
        "".into()
    } else {
        let mut garg_string = String::new();
        let mut generic_iter = generics.iter();
        if let Some(first_generic) = generic_iter.next() {
            garg_string.push_str(&format!("{type_cil}", type_cil = type_cil(first_generic)));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(",{type_cil}", type_cil = type_cil(arg)));
        }
        format!("<{garg_string}>").into()
    }
}
