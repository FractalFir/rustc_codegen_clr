use std::borrow::Cow;

use crate::{
    assembly_exporter::escape_class_name,
    method::Method,
    r#type::{DotnetTypeRef, Type},
};

pub fn op_cli(op: &crate::cil::CILOp, method: &Method) -> Cow<'static, str> {
    use crate::cil::CILOp;

    match op {
        CILOp::SourceFileInfo(sfi)=>format!(".line {line}:{col} '{fname}'",line = sfi.0, col = sfi.1,fname = sfi.2).into(),
        CILOp::Leave(target) => format!("leave bb_{target}_0").into(),
        CILOp::BeginTry => ".try{".into(),
        CILOp::BeginCatch=>"}catch [System.Runtime]System.Exception{".into(),
        CILOp::EndTry => "}".into(),
        CILOp::CustomLabel(label)=>format!("{label}:").into(),
        CILOp::EHClause { start, end, hstart }=>
            format!(".try bb_{start} to bb_{end}_end catch [mscorlib]System.Exception handler cb_{hstart}_{start} to END_CLEANUP").into(),
        CILOp::Break => "break".into(),
        //Control flow
        CILOp::Ret => "ret".into(),
        CILOp::Label(id,sub_id) => format!("bb_{id}_{sub_id}:").into(),
        CILOp::BlockStart(id) => format!("bb_{id}:").into(),
        CILOp::BlockEnd(id) => format!("bb_{id}_end:").into(),
        CILOp::GoTo(id,sub_id) => format!("br bb_{id}_{sub_id}").into(),
        CILOp::BEq(id,sub_id) => format!("beq bb_{id}_{sub_id}").into(),
        CILOp::BNe(id,sub_id) => format!("bne.un bb_{id}_{sub_id}").into(),
        CILOp::BGe(id,sub_id) => format!("bge bb_{id}_{sub_id}").into(),
        CILOp::BLt(id,sub_id) => format!("blt bb_{id}_{sub_id}").into(),
        CILOp::BLe(id,sub_id) => format!("ble bb_{id}_{sub_id}").into(),
        CILOp::BZero(id,sub_id) => format!("brzero bb_{id}_{sub_id}").into(),
        CILOp::BTrue(id,sub_id) => format!("brtrue bb_{id}_{sub_id}").into(),
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
                    None => "RustModule::".into(),
                };
                let function_name = call_site.name();
                let function_name = if *crate::config::ESCAPE_NAMES{
                    escape_class_name(function_name)
                }else{
                    function_name.into()
                };
                if *crate::config::TRACE_CALLS{
                    return format!(
                        "
                        ldstr \"Callin \"
                        call void [System.Console] System.Console::Write(string)
                        ldstr \"{function_name}\"
                        call void [System.Console] System.Console::Write(string)
                        ldstr \" from \"
                        call void [System.Console] System.Console::Write(string)
                        ldstr \"{caller_name}\"
                        call void [System.Console] System.Console::WriteLine(string)
                        call {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})
                        ldstr \"Returned from \"
                        call void [System.Console] System.Console::Write(string)
                        ldstr \"{function_name}\"
                        call void [System.Console] System.Console::Write(string)
                        ldstr \" to \"
                        call void [System.Console] System.Console::Write(string)
                        ldstr \"{caller_name}\"
                        call void [System.Console] System.Console::WriteLine(string)
                        ",
                        output = type_cil(call_site.signature().output()),
                        caller_name = method.name(),
                    ).into();
                }
                //TODO:Remove this `break`! It *mitigagtes* an issue with calls segafulting. I don't know *why* those calls segfault, but they SHOULD NOT DO THAT.
                format!(
                    "break\n\tcall {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})\nbreak",
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
                "
                call native uint RustModule::check_calli_nonull(native uint)
                calli {callconv} {output} ({input_string})",
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
                 None => "RustModule::".into(),
             };
             let function_name = call_site.name();
                let function_name = if *crate::config::ESCAPE_NAMES{
                    escape_class_name(function_name)
                }else{
                    function_name.into()
                };
             format!(
                 "ldftn {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
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
                    None => "RustModule::".into(),
                };
                let function_name = call_site.name();
                let function_name = if *crate::config::ESCAPE_NAMES{
                    escape_class_name(function_name)
                }else{
                    function_name.into()
                };
                format!(
                    "callvirt {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
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
        CILOp::DivUn => "div.un".into(),
        CILOp::Rem => "rem".into(),
        CILOp::RemUn => "rem.un".into(),
        CILOp::Neg => "neg".into(),
        //Bitwise
        CILOp::And => "and".into(),
        CILOp::Or => "or".into(),
        CILOp::XOr => "xor".into(),
        CILOp::Not => "not".into(),
        //Bitshifts
        CILOp::Shl => "shl".into(),
        CILOp::Shr => "shr".into(),
        CILOp::ShrUn => "shr.un".into(),
        //Comparisons
        CILOp::Gt => "cgt".into(),
        CILOp::Eq => "ceq".into(),
        CILOp::GtUn => "cgt.un".into(),
        CILOp::Lt => "clt".into(),
        CILOp::LtUn => "clt.un".into(),
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
        CILOp::LdcU32(value) => {
            if *value <= 8 {
                format!("ldc.i4.{value}").into()
            } else if *value < i8::MAX as u8 as u32 {
                format!("ldc.i4.s {value}").into()
            } else {
                format!("ldc.i4 {value}",value = *value as i32).into()
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
        CILOp::LdcU64(value) => {
            if *value <= 8{
                format!("ldc.i4.{value}").into()
            } else if *value < i8::MAX as u8 as u64 {
                format!("ldc.i4.s {value}\n\t").into()
            } else if *value < i32::MAX as u32 as u64  {
                format!("ldc.i4 {value}").into()
            } else {
                format!("ldc.i8 {value}").into()
            }
        }
        CILOp::LdNull => "ldnull".into(), 
        CILOp::LdcF32(f32const) =>{
            let const_literal = f32const.to_le_bytes();
            format!("ldc.r4 ({:02x} {:02x} {:02x} {:02x})",const_literal[0],const_literal[1],const_literal[2],const_literal[3]).into()
        }
        CILOp::LdcF64(f64const) => {
            let const_literal = f64const.to_le_bytes();
            format!("ldc.r8 ({:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x})",const_literal[0],const_literal[1],const_literal[2],const_literal[3],const_literal[4],const_literal[5],const_literal[6],const_literal[7]).into()
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
        CILOp::ConvF32=> "conv.r4".into(),
        CILOp::ConvF64 =>
            "conv.r8".into(),
        CILOp::ConvF64Un =>
            "conv.r.un".into(),
        // Pointer stuff
        CILOp::LDIndI8 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.i1".into()
        }else{"ldind.i1".into()},
        CILOp::LDIndI16 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.i2".into()
        }else{"ldind.i2".into()},
        CILOp::LDIndI32 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.i4".into()
        }else{"ldind.i4".into()},
        CILOp::LDIndI64 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.i8".into()
        }else{"ldind.i8".into()},
        CILOp::LDIndU8 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.u1".into()
        }else{"ldind.u1".into()},
        CILOp::LDIndU16 =>  if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.u2".into()
        }else{"ldind.u2".into()},
        CILOp::LDIndU32 =>  if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.u4".into()
        }else{"ldind.u4".into()},
        CILOp::LDIndU64 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.u8".into()
        }else{"ldind.u8".into()},
        CILOp::LDIndF32 =>  if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.r4".into()
        }else{"ldind.r4".into()},
        CILOp::LDIndF64 => if *crate::config::PRINT_PTRS{
            "call native uint RustModule::watch_ptr(native uint) ldind.r8".into()
        }else{"ldind.r8".into()},
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
        CILOp::ReThrow => "rethrow".into(),
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
                    "newobj {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                    function_name = call_site.name(),
                    output = type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        CILOp::Volatile=>"volatile.".into(),
        CILOp::Nop => "nop".into(),
         //CILOp::LoadGlobalAllocPtr { alloc_id } => panic!("CRITICAL INTERNAL ERROR:Allocation {alloc_id} was not resolved to a static."),
        CILOp::Pop => "pop".into(),
        CILOp::Dup => "dup".into(),
        CILOp::LDStaticField(static_field) => {
            match static_field.owner(){
                Some(_owner)=>todo!("Can't load static field {static_field:?}"),
                None=>format!("ldsfld {tpe} RustModule::{name}",tpe = non_void_type_cil(static_field.tpe()), name = static_field.name()).into(),
            }
        }
        CILOp::STStaticField(static_field) => {
            match static_field.owner(){
                Some(_owner)=>todo!("Can't load static field {static_field:?}"),
                None=>format!("stsfld {tpe} RustModule::{name}",tpe = non_void_type_cil(static_field.tpe()), name = static_field.name()).into(),
            }
        }
        CILOp::InitObj(tpe)=>format!("initobj {tpe}",tpe = non_void_type_cil(tpe)).into(),
        CILOp::LDTypeToken(tpe)=>format!("ldtoken {tpe}",tpe = non_void_type_cil(tpe)).into(),
        CILOp::LDLen=>"ldlen".into(),
        CILOp::LDElelemRef => "ldelem.ref".into(),
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
        Type::FnDef(name) => format!("valuetype 'fn_{name}'").into(),
        Type::Void => "void".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::F16 => "valuetype [System.Runtime]System.Numerics.Half".into(),
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
        Type::ManagedArray { element, dims } => {
            let dims = Into::<u8>::into(*dims);
            let arr = if dims > 0_u8 {
                (0..(dims - 1)).map(|_| ",").collect::<String>()
            } else {
                "".into()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(&element)).into()
        } //_ => todo!("Unsuported type {tpe:?}"),
        Type::MethodGenericArg(idx) => format!("!!{idx}").into(),
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
        "".into()
    };
    let name = dotnet_type.name_path();
    let name = if *crate::config::ESCAPE_NAMES {
        escape_class_name(name)
    } else {
        name.into()
    };
    let generics = generics_str(dotnet_type.generics());
    format!("{prefix} {asm}'{name}'{generics}")
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
