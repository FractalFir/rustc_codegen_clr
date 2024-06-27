use std::fmt::Write;

use crate::call_site::CallSite;
use crate::cil_node::CILNode;
use crate::cil_root::CILRoot;
use crate::ilasm_op::type_cil;
use crate::method::Method;
use crate::{c_exporter::c_tpe, Type};
use crate::{mangle, DepthSetting, FnSig, IString};

use super::escape_type_name;
pub(crate) fn arg_name(method: &Method, arg: u32) -> IString {
    if let Some(name) = &method
        .arg_names()
        .get(arg as usize)
        .into_iter()
        .flatten()
        .next()
    {
        if is_reserved(name) {
            format!("A{arg}").into()
        } else {
            (*name).clone()
        }
    } else {
        format!("A{arg}").into()
    }
}
pub(crate) fn loc_name(method: &Method, loc: u32) -> IString {
    if let Some(name) = &method.locals()[loc as usize].0 {
        if is_reserved(name) {
            format!("L{loc}").into()
        } else {
            name.clone()
        }
    } else {
        format!("L{loc}").into()
    }
}
pub(crate) fn function_decl(method: &Method, class: Option<&str>) -> String {
    let name = method.name();
    let name = escape_type_name(name);
    let output = c_tpe(method.sig().output());
    let mut inputs: String = "(".into();
    let mut input_iter = method
        .sig()
        .inputs()
        .iter()
        .enumerate()
        .filter(|(_, tpe)| **tpe != Type::Void);
    if let Some((idx, input)) = input_iter.next() {
        inputs.push_str(&format!(
            "{input} {aname}",
            input = c_tpe(input),
            aname = arg_name(method, idx as u32)
        ));
    }
    for (idx, input) in input_iter {
        inputs.push_str(&format!(
            ",{input} {aname}",
            input = c_tpe(input),
            aname = arg_name(method, idx as u32)
        ));
    }
    inputs.push(')');
    if let Some(class) = class {
        let class = escape_type_name(class);
        let name = method.name();
        let name = escape_type_name(name);
        let mangled_overloads: String = method.sig().inputs().iter().map(mangle).collect();
        format!("{output} {class}_{name}_{mangled_overloads} {inputs}")
    } else {
        format!("{output} {name} {inputs}")
    }
}
pub(crate) fn export_method(
    mut method_defs: impl Write,
    mut encoded_asm: impl Write,
    method: &Method,
    class: Option<&str>,
    ds: DepthSetting,
) -> std::fmt::Result {
    if crate::libc_fns::LIBC_FNS.contains(&method.name()) {
        return Ok(());
    }
    let fn_decl = function_decl(method, class);
    let code = method_code(method, ds.incremented());
    writeln!(method_defs, "{fn_decl};")?;
    write!(encoded_asm, "{fn_decl}{{\n{code}}}\n")
}
fn is_reserved(name: &str) -> bool {
    matches!(name, "int")
}
fn local_defs(method: &Method, ds: DepthSetting) -> String {
    let mut local_defs = String::new();
    for (id, (_name, local)) in method.locals().iter().enumerate() {
        if *local == Type::Void {
            ds.pad(&mut local_defs).unwrap();
            local_defs.push_str(&format!(
                "RustVoid {lname};",
                lname = loc_name(method, id as u32),
            ));
            continue;
        }
        ds.pad(&mut local_defs).unwrap();
        local_defs.push_str(&format!(
            "{local} {lname};",
            local = c_tpe(local),
            lname = loc_name(method, id as u32),
        ));
    }
    local_defs
}
fn method_code(method: &Method, ds: DepthSetting) -> String {
    let mut code = String::new();
    code.push_str(&local_defs(method, ds));
    for block in method.blocks() {
        block_code(method, block, &mut code, ds).unwrap();
    }
    ds.pad(&mut code).unwrap();
    code
}

fn block_code(
    method: &Method,
    block: &crate::basic_block::BasicBlock,
    code: &mut String,
    ds: DepthSetting,
) -> std::fmt::Result {
    ds.pad(code)?;
    code.push_str(&format!(
        "bb_{target}_{sub_target}:",
        target = block.id(),
        sub_target = 0
    ));
    for tree in block.trees() {
        tree_code(method, tree.root(), code, ds.incremented())?;
    }
    Ok(())
}

fn tree_code(
    method: &Method,
    root: &CILRoot,
    code: &mut String,
    ds: DepthSetting,
) -> std::fmt::Result {
    match root {
        CILRoot::Nop => {
            ds.pad(code)?;
            write!(code, "//nop")
        }
        CILRoot::VoidRet => {
            ds.pad(code)?;
            write!(code, "return;")
        }
        CILRoot::SetField { addr, value, desc } => {
            ds.pad(code)?;
            write!(
                code,
                "({addr})->{name}.f = ({value});",
                addr = node_code(method, addr),
                name = desc.name(),
                value = node_code(method, value)
            )
        }

        CILRoot::BEq {
            target,
            sub_target,
            a,
            b,
        } => {
            ds.pad(code)?;
            write!(
                code,
                "if(({a}) == ({b})) goto bb_{target}_{sub_target};",
                a = node_code(method, a),
                b = node_code(method, b)
            )
        }
        CILRoot::BFalse {
            target,
            sub_target,
            cond,
        } => {
            ds.pad(code)?;
            write!(
                code,
                "if(({cond}) == false) goto bb_{target}_{sub_target};",
                cond = node_code(method, cond),
            )
        }
        CILRoot::BTrue {
            target,
            sub_target,
            cond,
        } => {
            ds.pad(code)?;
            write!(
                code,
                "if({cond}) goto bb_{target}_{sub_target};",
                cond = node_code(method, cond),
            )
        }
        CILRoot::Ret { tree } => {
            ds.pad(code)?;
            write!(code, "return {node};", node = node_code(method, tree))
        }
        CILRoot::STLoc { tree, local } => {
            ds.pad(code)?;
            write!(
                code,
                "{local} = {node};",
                node = node_code(method, tree),
                local = loc_name(method, *local)
            )
        }
        CILRoot::STArg { tree, arg } => {
            ds.pad(code)?;
            write!(
                code,
                "{local} = {node};",
                node = node_code(method, tree),
                local = arg_name(method, *arg)
            )
        }
        CILRoot::SetStaticField { descr, value } => {
            ds.pad(code)?;
            write!(
                code,
                "{name} = {node};",
                node = node_code(method, value),
                name = descr.name(),
            )
        }
        CILRoot::Pop { tree } => {
            ds.pad(code)?;
            write!(code, "{node};", node = node_code(method, tree))
        }
        CILRoot::Throw(inner) => {
            ds.pad(code)?;
            write!(
                code,
                "fprintf(stderr,\"%s\",{inner});",
                inner = node_code(method, inner)
            )?;
            ds.pad(code)?;
            write!(code, "abort();")
        }
        CILRoot::Break => Ok(()),
        CILRoot::GoTo { target, sub_target } => {
            ds.pad(code)?;
            write!(code, "goto bb_{target}_{sub_target};")
        }
        CILRoot::STIndI8(addr_calc, value_calc)
        | CILRoot::STIndISize(addr_calc, value_calc)
        | CILRoot::STIndF64(addr_calc, value_calc)
        | CILRoot::STIndF32(addr_calc, value_calc) => {
            ds.pad(code)?;
            write!(
                code,
                "*({addr_calc}) = {value_calc};",
                addr_calc = node_code(method, addr_calc),
                value_calc = node_code(method, value_calc)
            )
        }
        CILRoot::STObj {
            tpe: _,
            addr_calc,
            value_calc,
        } => {
            ds.pad(code)?;
            write!(
                code,
                "*({addr_calc}) = {value_calc};",
                addr_calc = node_code(method, addr_calc),
                value_calc = node_code(method, value_calc)
            )
        }
        // SFI is not implementable in C, and does nothing.
        CILRoot::SourceFileInfo(_) => Ok(()),
        CILRoot::Call { site, args } | CILRoot::CallVirt { site, args } => {
            let name = call_site_to_name(site);
            let mut arg_iter = args.iter();
            let mut call_inner = String::new();
            if let Some(arg) = arg_iter.next() {
                if arg.validate(method.vctx(), None).unwrap() != Type::Void {
                    call_inner.push_str(&node_code(method, arg));
                }
            }
            for arg in arg_iter {
                if arg.validate(method.vctx(), None).unwrap() == Type::Void {
                    continue;
                }
                call_inner.push(',');
                call_inner.push_str(&node_code(method, arg));
            }
            ds.pad(code)?;
            write!(code, "{name}({call_inner});")
        }
        CILRoot::CallI { sig, fn_ptr, args } => {
            let ptr_ty = fn_ptr_ty(sig);
            let mut arg_iter = args.iter();
            let mut call_inner = String::new();
            if let Some(arg) = arg_iter.next() {
                if arg.validate(method.vctx(), None).unwrap() != Type::Void {
                    call_inner.push_str(&node_code(method, arg));
                }
            }
            for arg in arg_iter {
                if arg.validate(method.vctx(), None).unwrap() == Type::Void {
                    continue;
                }
                call_inner.push(',');
                call_inner.push_str(&node_code(method, arg));
            }
            let ptr = node_code(method, fn_ptr);
            write!(code, "(({ptr_ty}){ptr})({call_inner})")
        }
        _ => {
            ds.pad(code)?;
            eprintln!("Can't yet export the CIL root {root:?}");
            write!(code, "// Can't yet export the CIL root {root:?}")
        }
    }
}
fn call_site_to_name(call_site: &CallSite) -> String {
    if let Some(class) = call_site.class() {
        let class = escape_type_name(class.name_path());
        let name = call_site.name();
        let name = escape_type_name(name);
        let mangled_overloads: String = call_site.signature().inputs().iter().map(mangle).collect();
        format!("{class}_{name}_{mangled_overloads}")
    } else {
        escape_type_name(call_site.name())
    }
}
fn node_code(method: &Method, node: &crate::cil_node::CILNode) -> IString {
    match node {
        CILNode::LdStr(string) => format!("{string:?}").into(),
        CILNode::LdTrue => "true".into(),
        CILNode::LdFalse => "false".into(),
        CILNode::LDArg(arg) => arg_name(method, *arg),
        CILNode::LDArgA(loc) => format!("&{name}", name = arg_name(method, *loc)).into(),
        CILNode::LDLoc(loc) => loc_name(method, *loc),
        CILNode::LDLocA(loc) => format!("&{name}", name = loc_name(method, *loc)).into(),
        CILNode::LdcU64(val) => format!("{val}UL").into(),
        CILNode::LdcI64(val) => format!("{val}L").into(),
        CILNode::LdcU32(val) => format!("{val}U").into(),
        CILNode::LdcI32(val) => format!("{val}").into(),
        CILNode::LdcU16(val) => format!("(uint16_t){val}").into(),
        CILNode::LdcI16(val) => format!("(int16_t){val}").into(),
        CILNode::LdcU8(val) => format!("(uint8_t){val}").into(),
        CILNode::LdcI8(val) => format!("(int8_t){val}").into(),
        CILNode::LdcF32(val) => format!("{val:?}f").into(),
        CILNode::LdcF64(val) => format!("{val:?}").into(),
        CILNode::SizeOf(tpe) => format!("sizeof({tpe})", tpe = c_tpe(tpe)).into(),
        CILNode::LDStaticField(sfd) => sfd.name().to_string().into(),
        CILNode::LDLen { arr } => {
            let tpe = arr.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let tpe = mangle(&tpe);
            let arr = node_code(method, arr);
            format!("({tpe}_getLength({arr}))").into()
        }
        CILNode::LDIndUSize { ptr }
        | CILNode::LDIndU8 { ptr }
        | CILNode::LDIndU16 { ptr }
        | CILNode::LDIndI32 { ptr }
        | CILNode::LDIndU32 { ptr }
        | CILNode::LDIndU64 { ptr }
        | CILNode::LDIndI64 { ptr }
        | CILNode::LDIndBool { ptr }
        | CILNode::LDIndISize { ptr }
        | CILNode::LDIndI8 { ptr }
        | CILNode::LDIndI16 { ptr }
        | CILNode::LDIndF64 { ptr }
        | CILNode::LDIndF32 { ptr } => format!("(*({ptr}))", ptr = node_code(method, ptr)).into(),
        CILNode::LdObj { ptr, obj: _ } => {
            format!("(*({ptr}))", ptr = node_code(method, ptr)).into()
        }
        CILNode::ConvF64Un(val) => format!("(double)({val})", val = node_code(method, val)).into(),
        CILNode::ConvU8(val) => format!("(uint8_t)({val})", val = node_code(method, val)).into(),
        CILNode::ConvU16(val) => {
            format!("((uint16_t)({val}))", val = node_code(method, val)).into()
        }
        CILNode::ConvU32(val) => {
            format!("((uint32_t)({val}))", val = node_code(method, val)).into()
        }
        CILNode::ConvI8(val) => format!("((int8_t)({val}))", val = node_code(method, val)).into(),
        CILNode::ConvI16(val) => format!("((int16_t)({val}))", val = node_code(method, val)).into(),
        CILNode::ConvI32(val) => {
            format!("((uint32_t)({val}))", val = node_code(method, val)).into()
        }
        CILNode::ConvF32(val) => format!("((float)({val}))", val = node_code(method, val)).into(),
        CILNode::ConvF64(val) => format!("((double)({val}))", val = node_code(method, val)).into(),
        CILNode::LDFtn(call_site) => {
            format!("(void*)&{name}", name = call_site_to_name(call_site)).into()
        }
        CILNode::SignExtendToISize(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                Type::I64
                | Type::I32
                | Type::I16
                | Type::I8
                | Type::Bool
                | Type::F32
                | Type::F64 => format!("((intptr_t)({val}))").into(),
                Type::U64 => format!("((intptr_t)(int64_t)({val}))").into(),
                Type::U32 => format!("((intptr_t)(int32_t)({val}))").into(),
                Type::ISize => format!("{val}").into(),
                Type::USize | Type::Ptr(_) => format!("(intptr_t)({val})").into(),
                _ => todo!("Can't yet `SignExtendToISize` {tpe:?}"),
            }
        }
        CILNode::SignExtendToUSize(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                Type::I64 | Type::I32 | Type::I16 | Type::I8 | Type::Bool => {
                    format!("(uintptr_t)(intptr_t)({val})").into()
                }
                Type::U64 => format!("(uintptr_t)(intptr_t)({val})").into(),
                Type::ISize => format!("(uintptr_t){val}").into(),
                Type::USize => val,
                _ => todo!("Can't yet `SignExtendToUSize` {tpe:?}"),
            }
        }
        CILNode::SignExtendToI64(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                Type::I64 => format!("{val}").into(),
                Type::U64
                | Type::F64
                | Type::F32
                | Type::ISize
                | Type::I8
                | Type::I32
                | Type::Bool => format!("((int64_t)({val}))").into(),
                _ => todo!("Can't yet `SignExtendToI64` {tpe:?}"),
            }
        }
        CILNode::SignExtendToU64(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                Type::I64 => format!("(uint64_t){val}").into(),
                Type::U64
                | Type::F64
                | Type::F32
                | Type::ISize
                | Type::I8
                | Type::I16
                | Type::I32
                | Type::Bool => format!("(uint64_t)(int64_t)({val})").into(),
                _ => todo!("Can't yet `SignExtendToU64` {tpe:?}"),
            }
        }
        CILNode::ZeroExtendToISize(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                _ => todo!("Can't yet `ZeroExtendToISize` {tpe:?}"),
            }
        }
        CILNode::ZeroExtendToUSize(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                Type::U64
                | Type::U32
                | Type::U16
                | Type::Bool
                | Type::U8
                | Type::F32
                | Type::F64 => format!("(uintptr_t)({val})").into(),
                Type::Ptr(_) | Type::ISize => format!("(uintptr_t)({val})").into(),

                Type::I32 => format!("(uintptr_t)((uint32_t)({val}))").into(),

                Type::I64 => format!("(uintptr_t)((uint64_t)({val}))").into(),
                Type::USize => val,
                _ => todo!("Can't yet `ZeroExtendToUSize` {tpe:?}"),
            }
        }
        CILNode::ZeroExtendToU64(val) => {
            let tpe = val.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let val = node_code(method, val);
            match tpe {
                Type::U32 | Type::U16 | Type::U8 => format!("(uint64_t)({val})").into(),
                Type::USize => format!("(uint64_t)({val})").into(),
                Type::ISize => format!("(uint64_t)(uintptr_t)({val})").into(),
                Type::U64 => val,
                Type::I32 => format!("((uint64_t)((uint32_t)({val})))").into(),
                Type::I64 | Type::F64 | Type::F32 => format!("((uint64_t)({val}))").into(),
                _ => todo!("Can't yet `ZeroExtendToU64` {tpe:?}"),
            }
        }
        CILNode::TransmutePtr { val, new_ptr } => format!(
            "({new_ptr})({val})",
            new_ptr = c_tpe(new_ptr),
            val = node_code(method, val)
        )
        .into(),
        CILNode::MRefToRawPtr(raw) => node_code(method, raw),
        CILNode::LDField { addr, field } => {
            let addr_tpe = addr.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            match addr_tpe {
                Type::Ptr(_) | Type::ManagedReference(_) => format!(
                    "({addr})->{name}.f",
                    addr = node_code(method, addr),
                    name = field.name()
                )
                .into(),
                Type::DotnetType(_) => format!(
                    "({addr}).{name}.f",
                    addr = node_code(method, addr),
                    name = field.name()
                )
                .into(),
                _ => panic!("{addr_tpe:?} is *not a pointer or struct*!"),
            }
        }
        CILNode::LDFieldAdress { addr, field } => {
            let addr_tpe = addr.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            match addr_tpe {
                Type::Ptr(_) | Type::ManagedReference(_) => format!(
                    "&(({addr})->{name}.f)",
                    addr = node_code(method, addr),
                    name = field.name()
                )
                .into(),
                _ => panic!("{addr_tpe:?} is *not a pointer*!"),
            }
        }
        CILNode::Neg(a) => format!("-({a})", a = node_code(method, a),).into(),
        CILNode::Not(a) => format!("!({a})", a = node_code(method, a),).into(),
        CILNode::Eq(a, b) => format!(
            "({a}) == ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Or(a, b) => format!(
            "({a}) | ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::XOr(a, b) => format!(
            "({a}) ^ ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Shl(a, b) => format!(
            "({a}) << ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Shr(a, b) | CILNode::ShrUn(a, b) => format!(
            "({a}) << ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::And(a, b) => format!(
            "({a}) & ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::LtUn(a, b) | CILNode::Lt(a, b) => format!(
            "({a}) < ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::GtUn(a, b) | CILNode::Gt(a, b) => format!(
            "({a}) > ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Mul(a, b) => format!(
            "({a}) * ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Div(a, b) | CILNode::DivUn(a, b) => format!(
            "({a}) / ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Rem(a, b) | CILNode::RemUn(a, b) => format!(
            "({a}) % ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Sub(a, b) => format!(
            "({a}) - ({b})",
            a = node_code(method, a),
            b = node_code(method, b)
        )
        .into(),
        CILNode::Add(a, b) => {
            let a_type = a.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            let b_type = a.validate(method.vctx(), None).expect("ERROR: type info is necceary for exporting C code, but the type checker could not validate a node.");
            match (&a_type, &b_type) {
                (Type::Ptr(_), Type::Ptr(_)) => format!(
                    "({a}) + ({b})",
                    a = node_code(method, a),
                    b = node_code(method, b)
                )
                .into(),
                (Type::Ptr(_), _) => format!(
                    "({a_type})((uintptr_t){a}) + ((uintptr_t){b})",
                    a = node_code(method, a),
                    b = node_code(method, b),
                    a_type = type_cil(&a_type),
                )
                .into(),
                (_, Type::Ptr(_)) => todo!("Can't perform pointer arithmetics yet!"),
                _ => format!(
                    "({a}) + ({b})",
                    a = node_code(method, a),
                    b = node_code(method, b)
                )
                .into(),
            }
        }
        CILNode::CallI(sig_ptr_args) => {
            let ptr_ty = fn_ptr_ty(&sig_ptr_args.0);
            let mut arg_iter = sig_ptr_args.2.iter();
            let mut call_inner = String::new();
            if let Some(arg) = arg_iter.next() {
                if arg.validate(method.vctx(), None).unwrap() != Type::Void {
                    call_inner.push_str(&node_code(method, arg));
                }
            }
            for arg in arg_iter {
                if arg.validate(method.vctx(), None).unwrap() == Type::Void {
                    continue;
                }
                call_inner.push(',');
                call_inner.push_str(&node_code(method, arg));
            }
            let ptr = node_code(method, &sig_ptr_args.1);
            format!("(({ptr_ty}){ptr})({call_inner})").into()
        }
        CILNode::Call(call_op_args)
        | CILNode::NewObj(call_op_args)
        | CILNode::CallVirt(call_op_args) => {
            let name = call_site_to_name(&call_op_args.site);
            let mut arg_iter = call_op_args.args.iter();
            let mut call_inner = String::new();
            if let Some(arg) = arg_iter.next() {
                if arg.validate(method.vctx(), None).unwrap() != Type::Void {
                    call_inner.push_str(&node_code(method, arg));
                }
            }
            for arg in arg_iter {
                if arg.validate(method.vctx(), None).unwrap() == Type::Void {
                    continue;
                }
                call_inner.push(',');
                call_inner.push_str(&node_code(method, arg));
            }
            format!("{name}({call_inner})").into()
        }
        CILNode::LDElelemRef { arr, idx } => format!(
            "&({arr})[{idx}]",
            arr = node_code(method, arr),
            idx = node_code(method, idx)
        )
        .into(),
        CILNode::LocAlloc { size } => {
            format!("alloca({size})", size = node_code(method, size)).into()
        }
        _ => todo!("Can't yet export the CILNode node {node:?}"),
    }
}
pub(crate) fn fn_ptr_ty(sig: &FnSig) -> String {
    let output = c_tpe(sig.output());
    let mut inputs: String = "(".into();
    let mut input_iter = sig
        .inputs()
        .iter()
        .enumerate()
        .filter(|(_, tpe)| **tpe != Type::Void);
    if let Some((idx, input)) = input_iter.next() {
        inputs.push_str(&format!("{input}", input = c_tpe(input),));
    }
    for (idx, input) in input_iter {
        inputs.push_str(&format!(",{input}", input = c_tpe(input),));
    }
    inputs.push(')');

    format!("{output}(*){inputs}")
}
