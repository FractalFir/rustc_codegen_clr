use std::{borrow::Cow, fmt::Write};

use crate::{
    asm::Assembly, cil_node::CILNode, cil_root::CILRoot, AsmStringContainer, DepthSetting,
    DotnetTypeRef, IlasmFlavour, Type,
};

macro_rules! un_op {
    ($out:ident,$val:ident,$depth:ident,$il_flavour:ident,$op:literal,$strings:ident) => {{
        export_node($out, $val, $depth.incremented(), $il_flavour, $strings)?;
        $depth.pad($out)?;
        write!($out, $op)
    }};
}
macro_rules! bi_op {
    ($out:ident,$a:ident,$b:ident,$depth:ident,$il_flavour:ident,$op:literal,$str_map:ident) => {{
        export_2_nodes($out, $a, $b, $depth.incremented(), $il_flavour, $str_map)?;
        $depth.pad($out)?;
        write!($out, $op)
    }};
}
/// Exports 2 nodes, checking if the `dup` instruction can be used to speed things up.
fn export_2_nodes(
    out: &mut impl Write,
    a: &CILNode,
    b: &CILNode,
    depth: DepthSetting,
    il_flavour: IlasmFlavour,
    strings: &AsmStringContainer,
) -> std::fmt::Result {
    // a and b are the same, and have no side effects, we  to use dup to save on space.
    if a == b && !a.has_side_effects() {
        export_node(out, a, depth, il_flavour, strings)?;
        depth.pad(out)?;
        write!(out, "dup")
    } else {
        export_node(out, a, depth, il_flavour, strings)?;
        export_node(out, b, depth, il_flavour, strings)
    }
}
fn export_node(
    out: &mut impl Write,
    tree: &CILNode,
    depth: DepthSetting,
    il_flavour: IlasmFlavour,
    strings: &AsmStringContainer,
) -> std::fmt::Result {
    match tree {
        CILNode::GetException => {
            depth.pad(out)?;
            write!(out, "// Internal GetException op")
        }
        CILNode::CheckedCast(inner) => {
            export_node(out, &inner.0, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "castclass {}", dotnet_type_ref_cli(&inner.1, strings))
        }
        CILNode::IsInst(inner) => {
            export_node(out, &inner.0, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "isinst {}", dotnet_type_ref_cli(&inner.1, strings))
        }
        CILNode::LDLoc(local) => {
            depth.pad(out)?;
            match local {
                0..=3 => write!(out, "ldloc.{local}"),
                4..=255 => write!(out, "ldloc.s {local}"),
                _ => write!(out, "ldloc {local}"),
            }
        }
        CILNode::LDArg(args) => {
            depth.pad(out)?;
            match args {
                0..=3 => write!(out, "ldarg.{args}"),
                4..=255 => write!(out, "ldarg.s {args}"),
                _ => write!(out, "ldarg {args}"),
            }
        }
        CILNode::LDLocA(local) => {
            depth.pad(out)?;
            match local {
                0..=255 => write!(out, "ldloca.s {local}"),
                _ => write!(out, "ldloca {local}"),
            }
        }
        CILNode::LDArgA(local) => {
            depth.pad(out)?;
            match local {
                0..=255 => write!(out, "ldarga.s {local}"),
                _ => write!(out, "ldarga {local}"),
            }
        }
        CILNode::BlackBox(_) => todo!(),
        CILNode::LDStaticField(static_field) => {
            depth.pad(out)?;
            match static_field.owner() {
                Some(_owner) => todo!("Can't load static field {static_field:?}"),
                None => write!(
                    out,
                    "ldsfld {tpe} RustModule::{name}",
                    tpe = non_void_type_cil(static_field.tpe(), strings),
                    name = static_field.name()
                ),
            }
        }
        CILNode::AddressOfStaticField(static_field) => {
            depth.pad(out)?;
            match static_field.owner() {
                Some(_owner) => todo!("Can't load static field {static_field:?}"),
                None => write!(
                    out,
                    "ldsflda {tpe} RustModule::{name}",
                    tpe = non_void_type_cil(static_field.tpe(), strings),
                    name = static_field.name()
                ),
            }
        }
        CILNode::ConvF32(val) => un_op!(out, val, depth, il_flavour, "conv.r4", strings),
        CILNode::ConvF64(val) => un_op!(out, val, depth, il_flavour, "conv.r8", strings),
        CILNode::ConvF64Un(val) => un_op!(out, val, depth, il_flavour, "conv.r.un", strings),
        CILNode::SizeOf(tpe) => {
            depth.pad(out)?;
            write!(out, "sizeof {tpe}", tpe = type_cil(tpe, strings))
        }
        CILNode::LDIndI8 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i1", strings),
        CILNode::LDIndBool { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i1", strings),
        CILNode::LDIndI16 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i2", strings),
        CILNode::LDIndI32 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i4", strings),
        CILNode::LDIndI64 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i8", strings),
        CILNode::LDIndISize { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i", strings),
        CILNode::LDIndPtr { ptr, loaded_ptr: _ } => {
            un_op!(out, ptr, depth, il_flavour, "ldind.i", strings)
        }
        CILNode::LDIndUSize { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i", strings),
        CILNode::Volatile(inner) => match inner.as_ref() {
            CILNode::LDIndI8 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.i1")
            }
            CILNode::LDIndI16 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.i2")
            }
            CILNode::LDIndI32 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.i4")
            }
            CILNode::LDIndI64 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.i8")
            }
            CILNode::LDIndISize { ptr }
            | CILNode::LDIndPtr { ptr, loaded_ptr: _ }
            | CILNode::LDIndUSize { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.i")
            }
            CILNode::LdObj { ptr, obj } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldobj {tpe}", tpe = type_cil(obj, strings))
            }

            CILNode::LDIndF32 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.r4")
            }
            CILNode::LDIndF64 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.r8")
            }
            CILNode::LDIndBool { ptr } | CILNode::LDIndU8 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.u1")
            }
            CILNode::LDIndU16 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.u2")
            }
            CILNode::LDIndU32 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.u4")
            }
            CILNode::LDIndU64 { ptr } => {
                export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
                depth.pad(out)?;
                write!(out, "volatile. ldind.u8")
            }

            //CILNode::LDFieldAdress { addr, field } => |
            //CILNode::LDField { addr, field } |
            _ => panic!("The prefix olatile. can't be applied to op {inner:?}"),
        },
        CILNode::LdObj { ptr, obj } => {
            export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "ldobj {tpe}", tpe = type_cil(obj, strings))
        }
        CILNode::UnboxAny(ptr, obj) => {
            export_node(out, ptr, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "unbox.any {tpe}", tpe = type_cil(obj, strings))
        }
        CILNode::LDIndF32 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.r4", strings),
        CILNode::LDIndF64 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.r8", strings),
        CILNode::LDFieldAdress { addr, field } => {
            export_node(out, addr, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(
                out,
                "ldflda {prefixed_type} {owner}::'{field_name}'",
                prefixed_type = non_void_type_cil(field.tpe(), strings),
                owner = type_cil(&field.owner().clone().into(), strings),
                field_name = field.name()
            )
        }
        CILNode::LDField { addr, field } => {
            export_node(out, addr, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(
                out,
                "ldfld {prefixed_type} {owner}::'{field_name}'",
                prefixed_type = non_void_type_cil(field.tpe(), strings),
                owner = type_cil(&field.owner().clone().into(), strings),
                field_name = field.name()
            )
        }
        CILNode::Add(a, b) => bi_op!(out, a, b, depth, il_flavour, "add", strings),
        CILNode::And(a, b) => bi_op!(out, a, b, depth, il_flavour, "and", strings),
        CILNode::Sub(a, b) => bi_op!(out, a, b, depth, il_flavour, "sub", strings),
        CILNode::Mul(a, b) => bi_op!(out, a, b, depth, il_flavour, "mul", strings),
        CILNode::Div(a, b) => bi_op!(out, a, b, depth, il_flavour, "div", strings),
        CILNode::DivUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "div.un", strings),
        CILNode::Rem(a, b) => bi_op!(out, a, b, depth, il_flavour, "rem", strings),
        CILNode::RemUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "rem.un", strings),
        CILNode::Or(a, b) => bi_op!(out, a, b, depth, il_flavour, "or", strings),
        CILNode::XOr(a, b) => bi_op!(out, a, b, depth, il_flavour, "xor", strings),
        CILNode::Shr(a, b) => bi_op!(out, a, b, depth, il_flavour, "shr", strings),
        CILNode::Shl(a, b) => bi_op!(out, a, b, depth, il_flavour, "shl", strings),
        CILNode::ShrUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "shr.un", strings),
        CILNode::Call(call_op_args) => {
            let site = &call_op_args.site;
            let args = &call_op_args.args;
            if site.is_nop() {
                assert_eq!(args.len(), 1);
                export_node(out, &args[0], depth.incremented(), il_flavour, strings)
            } else {
                for arg in args {
                    export_node(out, arg, depth.incremented(), il_flavour, strings)?;
                }
                depth.pad(out)?;
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg, strings));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg, strings));
                }
                let prefix = if site.is_static() { "" } else { "instance" };
                let generics = if site.generics().is_empty() {
                    String::new()
                } else {
                    assert!(
                        site.generics().len() == 1,
                        "Methods with multiple generics not supported yet!"
                    );
                    format!("<{}>", type_cil(&site.generics()[0], strings))
                };

                let owner_name = match site.class() {
                    Some(owner) => {
                        format!("{}::", type_cil(&owner.clone().into(), strings))
                    }
                    None => "RustModule::".into(),
                };
                let function_name = site.name();
                write!(out,
                    "call {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                    output = type_cil(site.signature().output(), strings)
                )
            }
        }
        CILNode::CallVirt(call_op_args) => {
            for arg in &call_op_args.args {
                export_node(out, arg, depth.incremented(), il_flavour, strings)?;
            }
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = call_op_args.site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, strings));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, strings));
            }
            let prefix = if call_op_args.site.is_static() {
                ""
            } else {
                "instance"
            };
            let generics = if call_op_args.site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    call_op_args.site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&call_op_args.site.generics()[0], strings))
            };

            let owner_name = match call_op_args.site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into(), strings),)
                }
                None => "RustModule::".into(),
            };
            let function_name = call_op_args.site.name();
            write!(out,
                "callvirt {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(call_op_args.site.signature().output(), strings)
            )
        }

        CILNode::LdcI64(value) => {
            depth.pad(out)?;
            if *value == -1 {
                write!(out, "ldc.i4.m1")
            } else if *value <= 8 && *value >= 0 {
                write!(out, "ldc.i4.{value}")
            } else if i8::try_from(*value).is_ok() {
                write!(out, "ldc.i4.s {value}\n\t")
            } else if i32::try_from(*value).is_ok() {
                write!(out, "ldc.i4 {value}")
            } else {
                write!(out, "ldc.i8 {value}")
            }
        }
        CILNode::LdcU64(value) => {
            depth.pad(out)?;
            if *value <= 8 {
                write!(out, "ldc.i4.{value}")
            } else if *value < u64::from(i8::MAX as u8) {
                write!(out, "ldc.i4.s {value}\n\t")
            } else if *value < u64::from(i32::MAX as u32) {
                write!(out, "ldc.i4 {value}")
            } else {
                write!(out, "ldc.i8 {value}")
            }
        }
        CILNode::LdcI32(value) => {
            depth.pad(out)?;
            if *value == -1 {
                write!(out, "ldc.i4.m1")
            } else if *value <= 8 && *value >= 0 {
                write!(out, "ldc.i4.{value}")
            } else if i8::try_from(*value).is_ok() {
                write!(out, "ldc.i4.s {value}")
            } else {
                write!(out, "ldc.i4 {value}")
            }
        }
        CILNode::LdcI16(value) => {
            depth.pad(out)?;
            if *value == -1 {
                write!(out, "ldc.i4.m1")
            } else if *value <= 8 && *value >= 0 {
                write!(out, "ldc.i4.{value}")
            } else if i8::try_from(*value).is_ok() {
                write!(out, "ldc.i4.s {value}")
            } else {
                write!(out, "ldc.i4 {value}")
            }
        }
        CILNode::LdcU32(value) => {
            depth.pad(out)?;
            if *value <= 8 {
                write!(out, "ldc.i4.{value}")
            } else if *value < u32::from(i8::MAX as u8) {
                write!(out, "ldc.i4.s {value}")
            } else {
                // This is intended behaviour
                #[allow(clippy::cast_possible_wrap)]
                {
                    write!(out, "ldc.i4 {value}", value = *value as i32)
                }
            }
        }
        CILNode::LdcU8(value) => {
            depth.pad(out)?;
            if *value <= 8 {
                write!(out, "ldc.i4.{value}")
            } else {
                write!(out, "ldc.i4.s {value}")
            }
        }
        CILNode::LdcU16(value) => {
            depth.pad(out)?;
            if *value <= 8 {
                write!(out, "ldc.i4.{value}")
            } else if *value < u16::from(i8::MAX as u8) {
                write!(out, "ldc.i4.s {value}")
            } else {
                write!(out, "ldc.i4 {value}", value = *value)
            }
        }
        CILNode::LdcI8(value) => {
            depth.pad(out)?;
            if *value <= 8 && *value >= 0 {
                write!(out, "ldc.i4.{value}")
            } else if *value == -1 {
                write!(out, "ldc.i4.m1")
            } else {
                write!(out, "ldc.i4.s {value}")
            }
        }
        CILNode::LdcF64(f64const) => {
            depth.pad(out)?;
            let const_literal = f64const.to_le_bytes();
            write!(
                out,
                "ldc.r8 ({:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x})",
                const_literal[0],
                const_literal[1],
                const_literal[2],
                const_literal[3],
                const_literal[4],
                const_literal[5],
                const_literal[6],
                const_literal[7]
            )
        }
        CILNode::LdcF32(f32const) => {
            depth.pad(out)?;
            let const_literal = f32const.to_le_bytes();
            write!(
                out,
                "ldc.r4 ({:02x} {:02x} {:02x} {:02x})",
                const_literal[0], const_literal[1], const_literal[2], const_literal[3]
            )
        }
        CILNode::LoadGlobalAllocPtr { alloc_id: _ } => {
            panic!("Global allocs should be resolved before export")
        }
        CILNode::ConvU8(val) => un_op!(out, val, depth, il_flavour, "conv.u1", strings),
        CILNode::ConvU16(val) => un_op!(out, val, depth, il_flavour, "conv.u2", strings),
        CILNode::ConvU32(val) => un_op!(out, val, depth, il_flavour, "conv.u4", strings),
        CILNode::ZeroExtendToU64(val) => un_op!(out, val, depth, il_flavour, "conv.u8", strings),
        CILNode::ZeroExtendToUSize(val) => un_op!(out, val, depth, il_flavour, "conv.u", strings),
        CILNode::ZeroExtendToISize(val) => un_op!(out, val, depth, il_flavour, "conv.u", strings),
        CILNode::MRefToRawPtr(val) => {
            un_op!(
                out,
                val,
                depth,
                il_flavour,
                "conv.u // mreftorawptr ",
                strings
            )
        }
        CILNode::ConvI8(val) => un_op!(out, val, depth, il_flavour, "conv.i1", strings),
        CILNode::ConvI16(val) => un_op!(out, val, depth, il_flavour, "conv.i2", strings),
        CILNode::ConvI32(val) => un_op!(out, val, depth, il_flavour, "conv.i4", strings),
        CILNode::SignExtendToI64(val) => un_op!(out, val, depth, il_flavour, "conv.i8", strings),
        CILNode::SignExtendToU64(val) => un_op!(out, val, depth, il_flavour, "conv.i8", strings),
        CILNode::SignExtendToISize(val) => un_op!(out, val, depth, il_flavour, "conv.i", strings),
        CILNode::SignExtendToUSize(val) => un_op!(out, val, depth, il_flavour, "conv.i", strings),
        CILNode::Neg(val) => un_op!(out, val, depth, il_flavour, "neg", strings),
        CILNode::Not(val) => un_op!(out, val, depth, il_flavour, "not", strings),
        CILNode::Eq(a, b) => bi_op!(out, a, b, depth, il_flavour, "ceq", strings),
        CILNode::Lt(a, b) => bi_op!(out, a, b, depth, il_flavour, "clt", strings),
        CILNode::LtUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "clt.un", strings),
        CILNode::Gt(a, b) => bi_op!(out, a, b, depth, il_flavour, "cgt", strings),
        CILNode::GtUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "cgt.un", strings),
        CILNode::TemporaryLocal(_) => {
            depth.pad(out)?;
            write!(
                out,
                "ldstr \"Unresolved tmp local at the export stage ???\""
            )?;
            depth.pad(out)?;
            write!(
                out,
                "newobj instance void [System.Runtime]System.Exception::.ctor()"
            )?;
            depth.pad(out)?;
            write!(out, "throw")
        }
        CILNode::SubTrees(_) => todo!(),
        CILNode::LoadAddresOfTMPLocal => todo!(),
        CILNode::LoadTMPLocal => todo!(),
        CILNode::LDFtn(site) => {
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, strings));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, strings));
            }
            let prefix = if site.is_static() { "" } else { "instance" };
            let generics = if site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&site.generics()[0], strings))
            };

            let owner_name = match site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into(), strings))
                }
                None => "RustModule::".into(),
            };
            let function_name = site.name();
            write!(
                out,
                "ldftn {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(site.signature().output(), strings)
            )
        }
        CILNode::LDTypeToken(tpe) => {
            depth.pad(out)?;
            write!(out, "ldtoken {tpe}", tpe = non_void_type_cil(tpe, strings))
        }
        CILNode::NewObj(call_op_args) => {
            for arg in &call_op_args.args {
                export_node(out, arg, depth.incremented(), il_flavour, strings)?;
            }
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = call_op_args.site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, strings));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, strings));
            }
            let prefix = if call_op_args.site.is_static() {
                ""
            } else {
                "instance"
            };
            let generics = if call_op_args.site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    call_op_args.site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&call_op_args.site.generics()[0], strings))
            };

            let owner_name = match call_op_args.site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into(), strings))
                }
                None => "RustModule::".into(),
            };
            let function_name = call_op_args.site.name();
            write!(
                out,
                "newobj {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(call_op_args.site.signature().output(), strings)
            )
        }
        CILNode::LdStr(string) => {
            depth.pad(out)?;
            let string: &str = string.as_ref();
            write!(out, "ldstr {string:?}")
        }
        CILNode::CallI(fn_ptr_and_sig) => {
            for arg in &fn_ptr_and_sig.2 {
                export_node(out, arg, depth.incremented(), il_flavour, strings)?;
            }
            depth.pad(out)?;
            export_node(
                out,
                &fn_ptr_and_sig.1,
                depth.incremented(),
                il_flavour,
                strings,
            )?;
            depth.pad(out)?;
            let mut inputs_iter = fn_ptr_and_sig.0.inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, strings));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, strings));
            }
            let callconv = "";
            //call native uint RustModule::check_calli_nonull(native uint)\n
            write!(
                out,
                "
            calli {callconv} {output} ({input_string})",
                output = type_cil(fn_ptr_and_sig.0.output(), strings)
            )
        }
        CILNode::LDIndU8 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u1", strings),
        CILNode::LDIndU16 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u2", strings),
        CILNode::LDIndU32 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u4", strings),
        CILNode::LDIndU64 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u8", strings),
        CILNode::LDLen { arr } => un_op!(out, arr, depth, il_flavour, "ldlen", strings),
        CILNode::LDElelemRef { arr, idx } => {
            bi_op!(out, arr, idx, depth, il_flavour, "ldelem.ref", strings)
        }
        CILNode::PointerToConstValue(_) => todo!(),

        CILNode::CastPtr { val, new_ptr: _ } => {
            export_node(out, val, depth.incremented(), il_flavour, strings)
        }
        CILNode::LdFalse => {
            depth.pad(out)?;
            write!(out, "ldc.i4.0")
        }
        CILNode::LdTrue => {
            depth.pad(out)?;
            write!(out, "ldc.i4.1")
        }
        CILNode::LocAllocAligned { tpe, align } => {
            // Alloc buff
            depth.pad(out)?;
            write!(out, "sizeof {tpe}", tpe = type_cil(tpe, strings))?;
            depth.pad(out)?;
            write!(out, "ldc.i8 {align}")?;
            depth.pad(out)?;
            write!(out, "conv.i")?;
            depth.pad(out)?;
            write!(out, "add")?;
            depth.pad(out)?;
            write!(out, "localloc")?;
            // Adjust align
            depth.pad(out)?;
            write!(out, "dup")?;
            depth.pad(out)?;
            write!(out, "ldc.i8 {align}")?;
            depth.pad(out)?;
            write!(out, "add")?;
            depth.pad(out)?;
            write!(out, "ldc.i8 {align}")?;
            depth.pad(out)?;
            write!(out, "rem")?;

            depth.pad(out)?;
            write!(out, "sub")?;
            depth.pad(out)?;
            write!(out, "ldc.i8 {align}")?;
            depth.pad(out)?;
            write!(out, "add")
        }
        CILNode::LocAlloc { size } => {
            export_node(out, size, depth, il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "localloc")
        }
    }
}

pub fn export_root(
    out: &mut impl Write,
    root: &CILRoot,
    depth: DepthSetting,
    il_flavour: IlasmFlavour,
    asm: &Assembly,
) -> std::fmt::Result {
    let strings = asm.string_map();
    match root {
        CILRoot::STLoc { local, tree } => {
            export_node(out, tree, depth.incremented(), il_flavour, asm.string_map())?;
            depth.pad(out)?;
            match local {
                0..=3 => write!(out, "stloc.{local}"),
                4..=255 => write!(out, "stloc.s {local}"),
                _ => write!(out, "stloc {local}"),
            }
        }
        CILRoot::Volatile(inner) => {
            match inner.as_ref() {
                //CILRoot::SetField { addr, value, desc } => todo!(),
                CILRoot::CpBlk { dst, src, len } => {
                    export_node(out, dst, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, src, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, len, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. cpblk")
                }
                CILRoot::STIndI8(addr, val) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.i1")
                }
                CILRoot::STIndI16(addr, val) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.i2")
                }
                CILRoot::STIndI32(addr, val) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.i4")
                }
                CILRoot::STIndI64(addr, val) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.i8")
                }
                CILRoot::STIndISize(addr, val) | CILRoot::STIndPtr(addr, val, _) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.i")
                }
                CILRoot::STIndF64(addr, val) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.r8")
                }
                CILRoot::STIndF32(addr, val) => {
                    export_node(out, addr, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stind.r4")
                }
                CILRoot::STObj {
                    tpe,
                    addr_calc,
                    value_calc,
                } => {
                    export_node(out, addr_calc, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, value_calc, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. stobj {tpe}", tpe = type_cil(tpe, strings))
                }
                CILRoot::InitBlk { dst, val, count } => {
                    export_node(out, dst, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, val, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    export_node(out, count, depth.incremented(), il_flavour, strings)?;
                    depth.pad(out)?;
                    write!(out, "volatile. initblk")
                }
                _ => todo!("Invalid volitale prefix on instrctuion {inner:?}"),
            }
        }
        CILRoot::BTrue {
            target,
            sub_target,
            cond,
        } => {
            export_node(out, cond, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "brtrue bb_{target}_{sub_target}")
        }
        CILRoot::BFalse {
            target,
            sub_target,
            cond,
        } => {
            export_node(out, cond, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "brfalse bb_{target}_{sub_target}")
        }
        CILRoot::BEq {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "beq bb_{target}_{sub_target}")
        }
        CILRoot::BNe {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "bne bb_{target}_{sub_target}")
        }
        CILRoot::BLt {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "blt bb_{target}_{sub_target}")
        }
        CILRoot::BLtUn {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "blt.un bb_{target}_{sub_target}")
        }
        CILRoot::BGt {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "bgt bb_{target}_{sub_target}")
        }
        CILRoot::BGtUn {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "bgt.un bb_{target}_{sub_target}")
        }
        CILRoot::BLe {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "ble bb_{target}_{sub_target}")
        }
        CILRoot::BGe {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour, strings)?;
            depth.pad(out)?;
            write!(out, "bge bb_{target}_{sub_target}")
        }
        CILRoot::GoTo { target, sub_target } => {
            depth.pad(out)?;
            write!(out, "br bb_{target}_{sub_target}")
        }
        CILRoot::Call { site, args } => {
            if site.is_nop() {
                Ok(())
            } else {
                for arg in args {
                    export_node(out, arg, depth.incremented(), il_flavour, strings)?;
                }
                depth.pad(out)?;
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg, strings));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg, strings));
                }
                let prefix = if site.is_static() { "" } else { "instance" };
                let generics = if site.generics().is_empty() {
                    String::new()
                } else {
                    assert!(
                        site.generics().len() == 1,
                        "Methods with multiple generics not supported yet!"
                    );
                    format!("<{}>", type_cil(&site.generics()[0], strings))
                };

                let owner_name = match site.class() {
                    Some(owner) => {
                        format!("{}::", type_cil(&owner.clone().into(), strings))
                    }
                    None => "RustModule::".into(),
                };
                let function_name = site.name();
                write!(out,
                "call {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(site.signature().output(), strings)
            )
            }
        }
        CILRoot::SetField { addr, value, desc } => {
            export_2_nodes(
                out,
                addr,
                value,
                depth.incremented(),
                il_flavour,
                asm.string_map(),
            )?;
            depth.pad(out)?;
            write!(
                out,
                "stfld {prefixed_type} {owner}::'{field_name}'",
                prefixed_type = non_void_type_cil(desc.tpe(), asm.string_map()),
                owner = type_cil(&desc.owner().clone().into(), asm.string_map()),
                field_name = desc.name()
            )
        }
        CILRoot::SetTMPLocal { value: _ } => {
            panic!("Temporary locals should be resolved before export.")
        }
        CILRoot::CpBlk { dst, src, len } => {
            export_node(out, dst, depth, il_flavour, asm.string_map())?;
            export_node(out, src, depth, il_flavour, asm.string_map())?;
            export_node(out, len, depth, il_flavour, asm.string_map())?;
            depth.pad(out)?;
            write!(out, "cpblk")
        }
        CILRoot::STIndI8(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i1", strings),
        CILRoot::STIndI16(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i2", strings),
        CILRoot::STIndI32(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i4", strings),
        CILRoot::STIndI64(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i8", strings),
        CILRoot::STIndISize(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i", strings),
        CILRoot::STIndPtr(a, b, _) => bi_op!(out, a, b, depth, il_flavour, "stind.i", strings),
        CILRoot::STIndF64(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.r8", strings),
        CILRoot::STIndF32(a, b) => {
            bi_op!(out, a, b, depth, il_flavour, "stind.r4", strings)
        }
        CILRoot::STObj {
            tpe,
            addr_calc,
            value_calc,
        } => {
            export_2_nodes(
                out,
                addr_calc,
                value_calc,
                depth.incremented(),
                il_flavour,
                asm.string_map(),
            )?;
            depth.pad(out)?;
            write!(out, "stobj {tpe}", tpe = type_cil(tpe, asm.string_map()))
        }
        CILRoot::STArg { arg, tree } => {
            export_node(out, tree, depth.incremented(), il_flavour, asm.string_map())?;
            depth.pad(out)?;
            match arg {
                0..=255 => write!(out, "starg.s {arg}"),
                _ => write!(out, "starg {arg}"),
            }
        }
        CILRoot::Break => {
            depth.pad(out)?;
            write!(out, "break")
        }
        CILRoot::Nop => {
            depth.pad(out)?;
            write!(out, "nop")
        }
        CILRoot::InitBlk { dst, val, count } => {
            export_node(out, dst, depth, il_flavour, asm.string_map())?;
            export_node(out, val, depth, il_flavour, asm.string_map())?;
            export_node(out, count, depth, il_flavour, asm.string_map())?;
            depth.pad(out)?;
            write!(out, "initblk")
        }
        CILRoot::CallVirt { site, args } => {
            for arg in args {
                export_node(out, arg, depth.incremented(), il_flavour, asm.string_map())?;
            }
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, asm.string_map()));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, asm.string_map()));
            }
            let prefix = if site.is_static() { "" } else { "instance" };
            let generics = if site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&site.generics()[0], asm.string_map()))
            };

            let owner_name = match site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into(), asm.string_map()))
                }
                None => "RustModule::".into(),
            };
            let function_name = site.name();
            write!(out,
            "callvirt {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
            output = type_cil(site.signature().output(),asm.string_map())
        )
        }
        CILRoot::Ret { tree } => {
            export_node(out, tree, depth.incremented(), il_flavour, asm.string_map())?;
            depth.pad(out)?;
            write!(out, "ret")
        }
        CILRoot::Pop { tree } => {
            export_node(out, tree, depth.incremented(), il_flavour, asm.string_map())?;
            depth.pad(out)?;
            write!(out, "pop")
        }
        CILRoot::VoidRet => {
            depth.pad(out)?;
            write!(out, "ret")
        }
        CILRoot::Throw(tree) => {
            export_node(out, tree, depth.incremented(), il_flavour, asm.string_map())?;
            depth.pad(out)?;
            write!(out, "throw")
        }
        CILRoot::ReThrow => {
            depth.pad(out)?;
            write!(out, "rethrow")
        }
        CILRoot::CallI { sig, fn_ptr, args } => {
            for arg in args {
                export_node(out, arg, depth.incremented(), il_flavour, asm.string_map())?;
            }
            depth.pad(out)?;
            export_node(
                out,
                fn_ptr,
                depth.incremented(),
                il_flavour,
                asm.string_map(),
            )?;
            depth.pad(out)?;
            let mut inputs_iter = sig.inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, asm.string_map()));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, asm.string_map()));
            }
            let callconv = "";
            write!(out,"call native uint RustModule::check_calli_nonull(native uint)\ncalli {callconv} {output} ({input_string})",output = type_cil(sig.output(),asm.string_map()))
        }
        CILRoot::JumpingPad { source, target } => {
            depth.pad(out)?;
            write!(out, "bb_{source}_{target}:")?;
            depth.pad(out)?;
            write!(out, "leave bb_{target}_0")
        }
        CILRoot::SetStaticField { descr, value } => {
            export_node(
                out,
                value,
                depth.incremented(),
                il_flavour,
                asm.string_map(),
            )?;
            depth.pad(out)?;
            match descr.owner() {
                Some(_owner) => todo!("Can't load static field {descr:?}"),
                None => write!(
                    out,
                    "stsfld {tpe} RustModule::{name}",
                    tpe = non_void_type_cil(descr.tpe(), asm.string_map()),
                    name = descr.name()
                ),
            }
        }
        CILRoot::SourceFileInfo(sfi) => {
            depth.pad(out)?;
            if crate::debig_sfi() {
                write!(out, "{}", crate::sfi_debug_print(sfi))?;
                depth.pad(out)?;
            }
            match il_flavour {
                IlasmFlavour::Clasic => write!(
                    out,
                    ".line {line}:{col} '{fname}'",
                    line = sfi.0.start,
                    col = sfi.1.start,
                    fname = sfi.2
                ),
                IlasmFlavour::Modern => write!(
                    out,
                    ".line {ls},{le}:{cs},{ce} '{fname}'",
                    ls = sfi.0.start,
                    le = sfi.0.end,
                    cs = sfi.1.start,
                    ce = sfi.1.end,
                    fname = sfi.2
                ),
            }
        }
        CILRoot::OptimizedSourceFileInfo(lin, col, file) => {
            depth.pad(out)?;
            let fname = asm.get_string(*file);
            match il_flavour {
                IlasmFlavour::Clasic => write!(
                    out,
                    ".line {line}:{col} '{fname}'",
                    line = lin.start,
                    col = col.start,
                ),
                IlasmFlavour::Modern => write!(
                    out,
                    ".line {ls},{le}:{cs},{ce} '{fname}'",
                    ls = lin.start,
                    le = lin.end,
                    cs = col.start,
                    ce = col.end,
                ),
            }
        }
    }
}
#[test]
fn node_to_il() {
    use crate::Type;
    let node = CILNode::Add(
        Box::new(CILNode::Mul(
            Box::new(CILNode::LDLoc(0)),
            Box::new(CILNode::SizeOf(Box::new(Type::U8))),
        )),
        Box::new(CILNode::LDLoc(1)),
    );
    let mut buff = String::new();
    export_node(
        &mut buff,
        &node,
        DepthSetting(0),
        IlasmFlavour::Clasic,
        &AsmStringContainer::default(),
    )
    .unwrap();
    assert_eq!(
        buff,
        "
  ldloc.0
  sizeof uint8
 mul
 ldloc.1
add"
    );
}

pub fn non_void_type_cil(tpe: &Type, strings: &AsmStringContainer) -> Cow<'static, str> {
    match tpe {
        Type::Void => "valuetype RustVoid".into(),
        _ => type_cil(tpe, strings),
    }
}
pub fn type_cil(tpe: &Type, strings: &AsmStringContainer) -> Cow<'static, str> {
    match tpe {
        Type::DelegatePtr(sig) => {
            let mut inputs_iter = sig.inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg, strings));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg, strings));
            }
            format!(
                "method {output}*({input_string})",
                output = type_cil(sig.output(), strings)
            )
            .into()
        }

        Type::Void => "void".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::F16 => "valuetype [System.Runtime]System.Half".into(),
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
        Type::Ptr(inner) => format!("{inner}*", inner = type_cil(inner, strings)).into(),
        Type::ManagedReference(inner) => {
            format!("{inner}&", inner = type_cil(inner, strings)).into()
        }
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli(dotnet_type, strings).into(),

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
                String::new()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(element, strings)).into()
        } //_ => todo!("Unsuported type {tpe:?}"),
        Type::MethodGenericArg(idx) => format!("!!{idx}").into(),
    }
}
pub fn dotnet_type_ref_cli(dotnet_type: &DotnetTypeRef, strings: &AsmStringContainer) -> String {
    let prefix = dotnet_type.tpe_prefix();
    if Some("System.Runtime") == dotnet_type.asm()
        && "System.String" == dotnet_type.name_path(strings)
        && !dotnet_type.is_valuetype()
    {
        return "string".into();
    }
    if Some("System.Runtime") == dotnet_type.asm()
        && "System.Object" == dotnet_type.name_path(strings)
        && !dotnet_type.is_valuetype()
    {
        return "object".into();
    }
    let asm = if let Some(asm_ref) = dotnet_type.asm() {
        format!("[{asm_ref}]")
    } else {
        String::new()
    };
    let name = if dotnet_type.generics().is_empty() {
        dotnet_type.name_path(strings).to_string()
    } else {
        format!(
            "{name}`{count}",
            count = dotnet_type.generics().len(),
            name = dotnet_type.name_path(strings).to_string()
        )
    };

    let generics = generics_str(dotnet_type.generics(), strings);
    format!("{prefix} {asm}'{name}'{generics}")
}
pub fn dotnet_type_ref_extends(
    dotnet_type: &DotnetTypeRef,
    strings: &AsmStringContainer,
) -> String {
    //let prefix = dotnet_type.tpe_prefix();

    let asm = if let Some(asm_ref) = dotnet_type.asm() {
        format!("[{asm_ref}]")
    } else {
        String::new()
    };
    let name = dotnet_type.name_path(strings);

    let generics = generics_str(dotnet_type.generics(), strings);
    format!("{asm}'{name}'{generics}")
}
fn generics_str(generics: &[Type], strings: &AsmStringContainer) -> Cow<'static, str> {
    if generics.is_empty() {
        "".into()
    } else {
        let mut garg_string = String::new();
        let mut generic_iter = generics.iter();
        if let Some(first_generic) = generic_iter.next() {
            garg_string.push_str(&format!(
                "{type_cil}",
                type_cil = type_cil(first_generic, strings)
            ));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(",{type_cil}", type_cil = type_cil(arg, strings)));
        }
        format!("<{garg_string}>").into()
    }
}
