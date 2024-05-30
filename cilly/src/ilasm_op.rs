use std::{any::type_name, borrow::Cow, fmt::Write, fs::write};

use crate::{
    cil_iter::CILIterTrait, cil_node::CILNode, cil_root::CILRoot, DotnetTypeRef, IlasmFlavour, Type,
};
#[derive(Copy, Clone)]
pub struct DepthSetting(u32);
impl DepthSetting {
    pub fn with_pading() -> Self {
        Self(0)
    }
    pub fn no_pading() -> Self {
        Self(0)
    }
    pub fn pad(&self, out: &mut impl Write) -> std::fmt::Result {
        write!(out, "\n")?;
        if self.0 == u32::MAX {
            return Ok(());
        }
        for _ in 0..self.0 {
            write!(out, " ")?;
        }
        Ok(())
    }
    pub fn incremented(self) -> Self {
        if self.0 == u32::MAX {
            return self;
        } else {
            Self(self.0 + 1)
        }
    }
}
macro_rules! un_op {
    ($out:ident,$val:ident,$depth:ident,$il_flavour:ident,$op:literal) => {{
        export_node($out, $val, $depth.incremented(), $il_flavour)?;
        $depth.pad($out)?;
        write!($out, $op)
    }};
}
macro_rules! bi_op {
    ($out:ident,$a:ident,$b:ident,$depth:ident,$il_flavour:ident,$op:literal) => {{
        export_2_nodes($out, $a, $b, $depth.incremented(), $il_flavour)?;
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
) -> std::fmt::Result {
    // a and b are the same, and have no side effects, we  to use dup to save on space.
    if a == b && !a.has_side_effects() && false {
        export_node(out, a, depth, il_flavour)?;
        depth.pad(out)?;
        write!(out, "dup")
    } else {
        export_node(out, a, depth, il_flavour)?;
        export_node(out, b, depth, il_flavour)
    }
}
fn export_node(
    out: &mut impl Write,
    tree: &CILNode,
    depth: DepthSetting,
    il_flavour: IlasmFlavour,
) -> std::fmt::Result {
    match tree {
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
                    tpe = non_void_type_cil(static_field.tpe()),
                    name = static_field.name()
                )
                .into(),
            }
        }
        CILNode::ConvF32(val) => un_op!(out, val, depth, il_flavour, "conv.r4"),
        CILNode::ConvF64(val) => un_op!(out, val, depth, il_flavour, "conv.r8"),
        CILNode::ConvF64Un(val) => un_op!(out, val, depth, il_flavour, "conv.r.un"),
        CILNode::SizeOf(tpe) => {
            depth.pad(out)?;
            write!(out, "sizeof {tpe}", tpe = type_cil(tpe))
        }
        CILNode::LDIndI8 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i1"),
        CILNode::LDIndBool { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i1"),
        CILNode::LDIndI16 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i2"),
        CILNode::LDIndI32 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i4"),
        CILNode::LDIndI64 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i8"),
        CILNode::LDIndISize { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i"),
        CILNode::LDIndPtr { ptr, loaded_ptr: _ } => un_op!(out, ptr, depth, il_flavour, "ldind.i"),
        CILNode::LDIndUSize { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.i"),
        CILNode::LdObj { ptr, obj } => {
            export_node(out, ptr, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "ldobj {tpe}", tpe = type_cil(obj))
        }
        CILNode::LDIndF32 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.r4"),
        CILNode::LDIndF64 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.r8"),
        CILNode::LDFieldAdress { addr, field } => {
            export_node(out, addr, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(
                out,
                "ldflda {prefixed_type} {owner}::'{field_name}'",
                prefixed_type = non_void_type_cil(field.tpe()),
                owner = type_cil(&field.owner().clone().into()),
                field_name = field.name()
            )
        }
        CILNode::LDField { addr, field } => {
            export_node(out, addr, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(
                out,
                "ldfld {prefixed_type} {owner}::'{field_name}'",
                prefixed_type = non_void_type_cil(field.tpe()),
                owner = type_cil(&field.owner().clone().into()),
                field_name = field.name()
            )
        }
        CILNode::Add(a, b) => bi_op!(out, a, b, depth, il_flavour, "add"),
        CILNode::And(a, b) => bi_op!(out, a, b, depth, il_flavour, "and"),
        CILNode::Sub(a, b) => bi_op!(out, a, b, depth, il_flavour, "sub"),
        CILNode::Mul(a, b) => bi_op!(out, a, b, depth, il_flavour, "mul"),
        CILNode::Div(a, b) => bi_op!(out, a, b, depth, il_flavour, "div"),
        CILNode::DivUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "div.un"),
        CILNode::Rem(a, b) => bi_op!(out, a, b, depth, il_flavour, "rem"),
        CILNode::RemUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "rem.un"),
        CILNode::Or(a, b) => bi_op!(out, a, b, depth, il_flavour, "or"),
        CILNode::XOr(a, b) => bi_op!(out, a, b, depth, il_flavour, "xor"),
        CILNode::Shr(a, b) => bi_op!(out, a, b, depth, il_flavour, "shr"),
        CILNode::Shl(a, b) => bi_op!(out, a, b, depth, il_flavour, "shl"),
        CILNode::ShrUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "shr.un"),
        CILNode::Call { args, site } => {
            if site.is_nop() {
                assert_eq!(args.len(), 1);
                export_node(out, &args[0], depth.incremented(), il_flavour)
            } else {
                for arg in args {
                    export_node(out, arg, depth.incremented(), il_flavour)?;
                }
                depth.pad(out)?;
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg));
                }
                let prefix = if site.is_static() { "" } else { "instance" };
                let generics = if site.generics().is_empty() {
                    String::new()
                } else {
                    assert!(
                        site.generics().len() == 1,
                        "Methods with multiple generics not supported yet!"
                    );
                    format!("<{}>", type_cil(&site.generics()[0]))
                };

                let owner_name = match site.class() {
                    Some(owner) => {
                        format!("{}::", type_cil(&owner.clone().into()))
                    }
                    None => "RustModule::".into(),
                };
                let function_name = site.name();
                write!(out,
                    "call {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                    output = type_cil(site.signature().output())
                )
            }
        }
        CILNode::CallVirt { args, site } => {
            for arg in args {
                export_node(out, arg, depth.incremented(), il_flavour)?;
            }
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg));
            }
            let prefix = if site.is_static() { "" } else { "instance" };
            let generics = if site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&site.generics()[0]))
            };

            let owner_name = match site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into()))
                }
                None => "RustModule::".into(),
            };
            let function_name = site.name();
            write!(out,
                "callvirt {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(site.signature().output())
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
        CILNode::LdcU32(value) => {
            depth.pad(out)?;
            if *value <= 8 {
                write!(out, "ldc.i4.{value}")
            } else if *value < u32::from(i8::MAX as u8) {
                write!(out, "ldc.i4.s {value}")
            } else {
                // This is intended behaviour
                #[allow(clippy::cast_possible_wrap)]
                write!(out, "ldc.i4 {value}", value = *value as i32)
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
            .into()
        }
        CILNode::LdcF32(f32const) => {
            depth.pad(out)?;
            let const_literal = f32const.to_le_bytes();
            write!(
                out,
                "ldc.r4 ({:02x} {:02x} {:02x} {:02x})",
                const_literal[0], const_literal[1], const_literal[2], const_literal[3]
            )
            .into()
        }
        CILNode::LoadGlobalAllocPtr { alloc_id } => todo!(),
        CILNode::ConvU8(val) => un_op!(out, val, depth, il_flavour, "conv.u1"),
        CILNode::ConvU16(val) => un_op!(out, val, depth, il_flavour, "conv.u2"),
        CILNode::ConvU32(val) => un_op!(out, val, depth, il_flavour, "conv.u4"),
        CILNode::ConvU64(val) => un_op!(out, val, depth, il_flavour, "conv.u8"),
        CILNode::ZeroExtendToUSize(val) => un_op!(out, val, depth, il_flavour, "conv.u"),
        CILNode::ZeroExtendToISize(val) => un_op!(out, val, depth, il_flavour, "conv.u"),
        CILNode::MRefToRawPtr(val) => un_op!(out, val, depth, il_flavour, "conv.u"),
        CILNode::ConvI8(val) => un_op!(out, val, depth, il_flavour, "conv.i1"),
        CILNode::ConvI16(val) => un_op!(out, val, depth, il_flavour, "conv.i2"),
        CILNode::ConvI32(val) => un_op!(out, val, depth, il_flavour, "conv.i4"),
        CILNode::ConvI64(val) => un_op!(out, val, depth, il_flavour, "conv.i8"),
        CILNode::ConvISize(val) => un_op!(out, val, depth, il_flavour, "conv.i"),
        CILNode::Neg(val) => un_op!(out, val, depth, il_flavour, "neg"),
        CILNode::Not(val) => un_op!(out, val, depth, il_flavour, "not"),
        CILNode::Eq(a, b) => bi_op!(out, a, b, depth, il_flavour, "ceq"),
        CILNode::Lt(a, b) => bi_op!(out, a, b, depth, il_flavour, "clt"),
        CILNode::LtUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "clt.un"),
        CILNode::Gt(a, b) => bi_op!(out, a, b, depth, il_flavour, "cgt"),
        CILNode::GtUn(a, b) => bi_op!(out, a, b, depth, il_flavour, "cgt.un"),
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
        CILNode::SubTrees(_, _) => todo!(),
        CILNode::LoadAddresOfTMPLocal => todo!(),
        CILNode::LoadTMPLocal => todo!(),
        CILNode::LDFtn(site) => {
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg));
            }
            let prefix = if site.is_static() { "" } else { "instance" };
            let generics = if site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&site.generics()[0]))
            };

            let owner_name = match site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into()))
                }
                None => "RustModule::".into(),
            };
            let function_name = site.name();
            write!(
                out,
                "ldftn {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(site.signature().output())
            )
        }
        CILNode::LDTypeToken(tpe) => {
            depth.pad(out)?;
            write!(out, "ldtoken {tpe}", tpe = non_void_type_cil(tpe))
        }
        CILNode::NewObj { site, args } => {
            for arg in args {
                export_node(out, arg, depth.incremented(), il_flavour)?;
            }
            depth.pad(out)?;
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = site.explicit_inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg));
            }
            let prefix = if site.is_static() { "" } else { "instance" };
            let generics = if site.generics().is_empty() {
                String::new()
            } else {
                assert!(
                    site.generics().len() == 1,
                    "Methods with multiple generics not supported yet!"
                );
                format!("<{}>", type_cil(&site.generics()[0]))
            };

            let owner_name = match site.class() {
                Some(owner) => {
                    format!("{}::", type_cil(&owner.clone().into()))
                }
                None => "RustModule::".into(),
            };
            let function_name = site.name();
            write!(
                out,
                "newobj {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(site.signature().output())
            )
        }
        CILNode::LdStr(string) => {
            depth.pad(out)?;
            write!(out, "ldstr {string:?}")
        }
        CILNode::CallI(fn_ptr_and_sig) => {
            for arg in &fn_ptr_and_sig.2 {
                export_node(out, arg, depth.incremented(), il_flavour)?;
            }
            depth.pad(out)?;
            export_node(out, &fn_ptr_and_sig.1, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            let mut inputs_iter = fn_ptr_and_sig.0.inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg));
            }
            let callconv = "";
            write!(out,"call native uint RustModule::check_calli_nonull(native uint)\ncalli {callconv} {output} ({input_string})",output = type_cil(fn_ptr_and_sig.0.output()))
        }
        CILNode::LDIndU8 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u1"),
        CILNode::LDIndU16 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u2"),
        CILNode::LDIndU32 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u4"),
        CILNode::LDIndU64 { ptr } => un_op!(out, ptr, depth, il_flavour, "ldind.u8"),
        CILNode::LDLen { arr } => un_op!(out, arr, depth, il_flavour, "ldlen"),
        CILNode::LDElelemRef { arr, idx } => bi_op!(out, arr, idx, depth, il_flavour, "ldelem.ref"),
        CILNode::PointerToConstValue(_) => todo!(),
        CILNode::GetStackTop => {
            depth.pad(out)?;
            write!(out, "// unsafe builtin GetStackTop.")
        }
        CILNode::InspectValue { val, inspect } => {
            export_node(out, val, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "dup")?;
            for root in inspect {
                export_root(out, root, depth.incremented(), il_flavour)?;
            }
            Ok(())
        }
        CILNode::TransmutePtr { val, new_ptr } => {
            export_node(out, val, depth.incremented(), il_flavour)
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
            write!(out, "sizeof {tpe}", tpe = type_cil(tpe))?;
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
    }
}

pub fn export_root(
    out: &mut impl Write,
    root: &CILRoot,
    depth: DepthSetting,
    il_flavour: IlasmFlavour,
) -> std::fmt::Result {
    match root {
        CILRoot::STLoc { local, tree } => {
            export_node(out, tree, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            match local {
                0..=3 => write!(out, "stloc.{local}"),
                4..=255 => write!(out, "stloc.s {local}"),
                _ => write!(out, "stloc {local}"),
            }
        }
        CILRoot::BTrue {
            target,
            sub_target,
            cond,
        } => {
            export_node(out, cond, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "brtrue bb_{target}_{sub_target}")
        }
        CILRoot::BFalse {
            target,
            sub_target,
            cond,
        } => {
            export_node(out, cond, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "brfalse bb_{target}_{sub_target}")
        }
        CILRoot::BEq {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "beq bb_{target}_{sub_target}")
        }
        CILRoot::BNe {
            target,
            sub_target,
            a,
            b,
        } => {
            export_2_nodes(out, a, b, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "bne bb_{target}_{sub_target}")
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
                    export_node(out, arg, depth.incremented(), il_flavour)?;
                }
                depth.pad(out)?;
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&non_void_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&non_void_type_cil(arg));
                }
                let prefix = if site.is_static() { "" } else { "instance" };
                let generics = if site.generics().is_empty() {
                    String::new()
                } else {
                    assert!(
                        site.generics().len() == 1,
                        "Methods with multiple generics not supported yet!"
                    );
                    format!("<{}>", type_cil(&site.generics()[0]))
                };

                let owner_name = match site.class() {
                    Some(owner) => {
                        format!("{}::", type_cil(&owner.clone().into()))
                    }
                    None => "RustModule::".into(),
                };
                let function_name = site.name();
                write!(out,
                "call {prefix} {output} {owner_name}'{function_name}'{generics}({input_string})",
                output = type_cil(site.signature().output())
            )
            }
        }
        CILRoot::SetField { addr, value, desc } => {
            export_2_nodes(out, addr, value, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(
                out,
                "stfld {prefixed_type} {owner}::'{field_name}'",
                prefixed_type = non_void_type_cil(desc.tpe()),
                owner = type_cil(&desc.owner().clone().into()),
                field_name = desc.name()
            )
        }
        CILRoot::SetTMPLocal { value } => todo!(),
        CILRoot::CpBlk { dst, src, len } => {
            export_node(out, dst, depth, il_flavour)?;
            export_node(out, src, depth, il_flavour)?;
            export_node(out, len, depth, il_flavour)?;
            depth.pad(out)?;
            write!(out, "cpblk")
        }
        CILRoot::STIndI8(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i1"),
        CILRoot::STIndI16(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i2"),
        CILRoot::STIndI32(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i4"),
        CILRoot::STIndI64(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i8"),
        CILRoot::STIndISize(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.i"),
        CILRoot::STIndF64(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.r8"),
        CILRoot::STIndF32(a, b) => bi_op!(out, a, b, depth, il_flavour, "stind.r4"),
        CILRoot::STObj {
            tpe,
            addr_calc,
            value_calc,
        } => {
            export_2_nodes(out, addr_calc, value_calc, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "stobj {tpe}", tpe = type_cil(tpe))
        }
        CILRoot::STArg { arg, tree } => {
            export_node(out, tree, depth.incremented(), il_flavour)?;
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
            export_node(out, dst, depth, il_flavour)?;
            export_node(out, val, depth, il_flavour)?;
            export_node(out, count, depth, il_flavour)?;
            depth.pad(out)?;
            write!(out, "initblk")
        }
        CILRoot::CallVirt { site, args } => todo!(),
        CILRoot::Ret { tree } => {
            export_node(out, tree, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "ret")
        }
        CILRoot::Pop { tree } => {
            export_node(out, tree, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "pop")
        }
        CILRoot::VoidRet => {
            depth.pad(out)?;
            write!(out, "ret")
        }
        CILRoot::Throw(tree) => {
            export_node(out, tree, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            write!(out, "throw")
        }
        CILRoot::ReThrow => {
            depth.pad(out)?;
            write!(out, "rethrow")
        }
        CILRoot::CallI { sig, fn_ptr, args } => {
            for arg in args {
                export_node(out, arg, depth.incremented(), il_flavour)?;
            }
            depth.pad(out)?;
            export_node(out, fn_ptr, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
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
            write!(out,"call native uint RustModule::check_calli_nonull(native uint)\ncalli {callconv} {output} ({input_string})",output = type_cil(sig.output()))
        }
        CILRoot::JumpingPad { source, target } => {
            depth.pad(out)?;
            write!(out, "bb_{source}_{target}:")?;
            depth.pad(out)?;
            write!(out, "leave bb_{target}_0")
        }
        CILRoot::SetStaticField { descr, value } => {
            export_node(out, value, depth.incremented(), il_flavour)?;
            depth.pad(out)?;
            match descr.owner() {
                Some(_owner) => todo!("Can't load static field {descr:?}"),
                None => write!(
                    out,
                    "stsfld {tpe} RustModule::{name}",
                    tpe = non_void_type_cil(descr.tpe()),
                    name = descr.name()
                )
                .into(),
            }
        }
        CILRoot::SourceFileInfo(sfi) => {
            depth.pad(out)?;
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
    }
}
#[test]
fn node_to_il() {
    use crate::{call_site::CallSite, FnSig, Type};
    let node = CILNode::Add(
        Box::new(CILNode::Mul(
            Box::new(CILNode::LDLoc(0)),
            Box::new(CILNode::SizeOf(Box::new(Type::U8))),
        )),
        Box::new(CILNode::LDLoc(1)),
    );
    let mut buff = String::new();
    export_node(&mut buff, &node, DepthSetting(0), IlasmFlavour::Clasic).unwrap();
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
                String::new()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(element)).into()
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
        String::new()
    };
    let name = dotnet_type.name_path();

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
