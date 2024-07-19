use crate::{
    asm::Assembly, asm_exporter::AssemblyExporter, basic_block::BasicBlock, call_site::CallSite,
    cil_node::CILNode, cil_root::CILRoot, escape_type_name, mangle, method::Method,
    AsmStringContainer, DepthSetting, IString, Type,
};
use std::fmt::Write;
#[derive(Default)]
pub struct JSExporter {
    file: String,
}
impl AssemblyExporter for JSExporter {
    fn add_type(&mut self, tpe: &crate::type_def::TypeDef, asm: &Assembly) {
        if tpe.fields().is_empty() {
            return;
        }
        if let None = tpe.explicit_offsets() {
            eprintln!("tpe {name} has no offests", name = tpe.name());
            return;
        }
        for (field_def, field_offset) in tpe
            .fields()
            .iter()
            .zip(tpe.explicit_offsets().unwrap().iter())
        {}
    }

    fn add_method(&mut self, method: &crate::method::Method, asm: &Assembly) {
        self.file
            .push_str(&function_decl(method, None, asm.string_map()));
        write!(self.file, "{{").unwrap();
        let ds = DepthSetting::with_pading();
        self.file.push_str(&local_defs(method, ds.incremented()));
        let ds = ds.incremented();
        ds.pad(&mut self.file).unwrap();
        write!(self.file, "while (true){{switch (bb_id){{").unwrap();
        ds.pad(&mut self.file).unwrap();
        for block in method.blocks() {
            export_bb(block, method, &mut self.file, ds, asm.string_map()).unwrap();
        }
        write!(self.file, "default: throw new Error(\"Invalid bb_id.\");}}").unwrap();
        ds.pad(&mut self.file).unwrap();
        write!(self.file, "}}").unwrap();
        ds.pad(&mut self.file).unwrap();
        let ds = DepthSetting::with_pading();
        write!(self.file, "}}").unwrap();
        ds.pad(&mut self.file).unwrap();
    }

    fn add_extern_method(
        &mut self,
        lib_path: &str,
        name: &str,
        sig: &crate::FnSig,
        preserve_errno: bool,
        info: &Assembly,
    ) {
        //todo!()
    }

    fn finalize(
        mut self,
        final_path: &std::path::Path,
        is_dll: bool,
    ) -> Result<(), crate::asm_exporter::AssemblyExportError> {
        self.file.push_str(include_str!("defines.js"));
        self.file.push_str("_cctor();\nentrypoint();");

        use std::io::Write;
        std::fs::File::create(final_path)
            .unwrap()
            .write_all(self.file.as_bytes())
            .unwrap();
        //todo!()
        Ok(())
    }

    fn add_extern_ref(&mut self, asm_name: &str, info: &crate::asm::AssemblyExternRef) {}

    fn add_global(&mut self, tpe: &Type, name: &str, thread_local: bool, info: &Assembly) {
        if name == "GlobalAtomicLock" {
            return;
        }
        if !matches!(tpe, Type::Ptr(_)) {
            eprintln!("Unsuported global. {name}");
            return;
        }

        writeln!(self.file, "var {name};").unwrap();
    }
}
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

fn is_reserved(name: &str) -> bool {
    false
}
pub(crate) fn function_decl(
    method: &Method,
    class: Option<&str>,
    strings: &AsmStringContainer,
) -> String {
    let name = method.name();
    let name = escape_type_name(name);

    let mut inputs: String = "(".into();
    let mut input_iter = method
        .sig()
        .inputs()
        .iter()
        .enumerate()
        .filter(|(_, tpe)| **tpe != Type::Void);
    if let Some((idx, _input)) = input_iter.next() {
        inputs.push_str(&format!("{aname}", aname = arg_name(method, idx as u32)));
    }
    for (idx, _input) in input_iter {
        inputs.push_str(&format!(",{aname}", aname = arg_name(method, idx as u32)));
    }
    inputs.push(')');
    if let Some(class) = class {
        let class = escape_type_name(class);
        let name = method.name();
        let name = escape_type_name(name);
        let mangled_overloads: String = method
            .sig()
            .inputs()
            .iter()
            .map(|inp| mangle(inp, strings))
            .collect();
        format!("function {class}_{name}_{mangled_overloads} {inputs}")
    } else {
        format!("function {name} {inputs}")
    }
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
            "var {lname};",
            lname = loc_name(method, id as u32),
        ));
    }
    ds.pad(&mut local_defs).unwrap();
    local_defs.push_str("var bb_id = 0;");
    local_defs
}
fn export_bb(
    block: &BasicBlock,
    method: &Method,
    out: &mut impl Write,
    ds: DepthSetting,
    strings: &AsmStringContainer,
) -> std::fmt::Result {
    ds.pad(out)?;
    write!(out, "case {id}:", id = block.id())?;
    for tree in block.trees() {
        tree_code(method, tree.root(), out, ds.incremented(), strings)?;
    }
    ds.pad(out)
}
fn tree_code(
    method: &Method,
    root: &CILRoot,
    code: &mut impl Write,
    ds: DepthSetting,
    strings: &AsmStringContainer,
) -> std::fmt::Result {
    ds.pad(code)?;
    match root {
        CILRoot::SetStaticField { descr, value } => {
            if descr.name() == "GlobalAtomicLock" {
                return Ok(());
            }
            write!(
                code,
                "{name} = {value};",
                name = descr.name(),
                value = node_code(method, value, strings)
            )?;
            ds.pad(code)
        }
        CILRoot::STLoc { tree, local } => {
            ds.pad(code)?;
            write!(
                code,
                "{local} = {node};",
                node = node_code(method, tree, strings),
                local = loc_name(method, *local)
            )
        }
        CILRoot::Ret { tree } => {
            ds.pad(code)?;
            write!(
                code,
                "return {node};",
                node = node_code(method, tree, strings),
            )
        }
        CILRoot::GoTo { target, sub_target } => {
            if *sub_target != 0 {
                ds.pad(code)?;
                write!(code, "bb_id = {sub_target};")?;
                write!(code, "break;")
            } else {
                ds.pad(code)?;
                write!(code, "bb_id = {target};")?;
                write!(code, "break;")
            }
        }
        CILRoot::STIndI8(addr, val) => {
            ds.pad(code)?;
            let vtpe = val.validate(method.vctx(strings), None).unwrap();
            let val = node_code(method, val, strings);
            let addr = node_code(method, addr, strings);
            match vtpe {
                Type::I8 => write!(code, "globalMemory.setInt8(Number({addr}),({val}))"),
                Type::U8 | Type::Bool => {
                    write!(code, "globalMemory.setUint8(Number({addr}),({val}))")
                }
                _ => panic!("can't STIndI8 {vtpe:?}"),
            }
        }
        CILRoot::STIndISize(addr, val) => {
            ds.pad(code)?;
            let vtpe = val.validate(method.vctx(strings), None).unwrap();
            let val = node_code(method, val, strings);
            let addr = node_code(method, addr, strings);
            write!(code, "globalMemory.setBigInt64(Number({addr}),({val}))")
        }
        _ => {
            eprintln!("Unsuported CIL root {root:?}");
            write!(
                code,
                "throw new Error({:?});",
                format!("Unsuported CIL root {root:?}")
            )
        }
    }
}
fn local_is_byaddress(method: &Method, local: usize) -> bool {
    false
}
fn node_code(method: &Method, node: &CILNode, strings: &AsmStringContainer) -> IString {
    match node {
        CILNode::LdcI64(val) => format!("{val}n").into(),
        CILNode::LdcU64(val) => format!("{val}n").into(),
        CILNode::LdcI32(val) => format!("{val}").into(),
        CILNode::LdcU32(val) => format!("{val}").into(),
        CILNode::LdcI16(val) => format!("{val}").into(),
        CILNode::LdcU16(val) => format!("{val}").into(),
        CILNode::LdcI8(val) => format!("{val}").into(),
        CILNode::LdcU8(val) => format!("{val}").into(),
        CILNode::LDIndU8 { ptr } => format!(
            "globalMemory.getUint8(Number({ptr}))",
            ptr = node_code(method, ptr, strings)
        )
        .into(),
        CILNode::SizeOf(tpe) => match tpe.as_ref() {
            Type::USize | Type::ISize | Type::Ptr(_) | Type::U64 | Type::I64 => "8".into(),
            Type::U32 | Type::I32 => "4".into(),
            _ => todo!("Can't get sizeof({tpe:?})"),
        },
        CILNode::Call(call_op_args)
        | CILNode::NewObj(call_op_args)
        | CILNode::CallVirt(call_op_args) => {
            let name = call_site_to_name(&call_op_args.site, strings);
            let mut arg_iter = call_op_args.args.iter();
            let mut call_inner = String::new();
            if let Some(arg) = arg_iter.next() {
                if arg.validate(method.vctx(strings), None).unwrap() != Type::Void {
                    call_inner.push_str(&node_code(method, arg, strings));
                }
            }
            for arg in arg_iter {
                if arg.validate(method.vctx(strings), None).unwrap() == Type::Void {
                    continue;
                }
                call_inner.push(',');
                call_inner.push_str(&node_code(method, arg, strings));
            }
            format!("{name}({call_inner})").into()
        }
        // Transmute Ptr does nothing.
        CILNode::TransmutePtr { val, new_ptr } => format!(
            "(/*TransmutePtr to {new_ptr}*/{val})",
            new_ptr = mangle(new_ptr, strings),
            val = node_code(method, val, strings)
        )
        .into(),
        CILNode::ConvI32(val) => {
            let vtpe = val.validate(method.vctx(strings), None).unwrap();
            match vtpe {
                Type::I32 | Type::I16 | Type::I8 | Type::F32 | Type::F64 => {
                    node_code(method, val, strings)
                }

                _ => todo!("Can't yet cast {vtpe:?} to i32."),
            }
        }
        CILNode::ZeroExtendToUSize(val) => {
            let vtpe = val.validate(method.vctx(strings), None).unwrap();
            match vtpe {
                Type::U64 => format!(
                    "({val}) & {max}n",
                    val = node_code(method, val, strings),
                    max = u64::MAX
                )
                .into(),
                Type::U32 => format!(
                    "BigInt(({val} & {max}))",
                    val = node_code(method, val, strings),
                    max = u32::MAX
                )
                .into(),
                Type::F32 => {
                    format!("BigInt({val})", val = node_code(method, val, strings),).into()
                }
                Type::F64 => {
                    format!("BigInt({val})", val = node_code(method, val, strings),).into()
                }
                Type::I32 => format!(
                    "BigInt(i32_to_u32({val}))",
                    val = node_code(method, val, strings),
                )
                .into(),
                _ => todo!("Can't yet cast {vtpe:?} to usize."),
            }
        }
        CILNode::LDArg(arg) => arg_name(method, *arg),
        CILNode::Mul(a, b) => {
            let atpe = a.validate(method.vctx(strings), None).unwrap();
            let btpe = b.validate(method.vctx(strings), None).unwrap();
            match (&atpe, &btpe) {
                (Type::Ptr(_) | Type::USize, Type::USize) => format!(
                    "({a}) * ({b})",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                _ => todo!("Can't yet mul {atpe:?} and {btpe:?}"),
            }
        }
        CILNode::Add(a, b) => {
            let atpe = a.validate(method.vctx(strings), None).unwrap();
            let btpe = b.validate(method.vctx(strings), None).unwrap();
            match (&atpe, &btpe) {
                (Type::Ptr(_) | Type::USize, Type::USize) => format!(
                    "({a}) + ({b})",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::I8, Type::I8) => format!(
                    "add_i8(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::I16, Type::I16) => format!(
                    "add_i16(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::I32, Type::I32) => format!(
                    "add_i32(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::I64, Type::I64) => format!(
                    "add_i64(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::F32, Type::F32) => format!(
                    "add_f32(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::F64, Type::F64) => format!(
                    "add_f64(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                (Type::U8, Type::U8) => format!(
                    "add_u8(({a}),({b}))",
                    a = node_code(method, a, strings),
                    b = node_code(method, b, strings)
                )
                .into(),
                _ => todo!("Can't yet add {atpe:?} and {btpe:?}"),
            }
        }
        //CILNode::LDArgA(loc) => format!("&{name}", name = arg_name(method, *loc)).into(),
        CILNode::LDLoc(loc) => loc_name(method, *loc),
        //CILNode::LDLocA(loc) => format!("&{name}", name = loc_name(method, *loc)).into(),
        CILNode::LDLen { arr } => {
            format!("({arr}).length", arr = node_code(method, arr, strings)).into()
        }
        _ => {
            eprintln!("Unsuported CIL node {node:?}");
            format!(
                "(() => {{throw new Error({:?})}})() ",
                format!("Unsuported CIL node {node:?}")
            )
            .into()
        }
    }
}
fn call_site_to_name(call_site: &CallSite, strings: &AsmStringContainer) -> String {
    if let Some(class) = call_site.class() {
        let class = escape_type_name(class.name_path(strings));
        let name = call_site.name();
        let name = escape_type_name(name);
        let mangled_overloads: String = call_site
            .signature()
            .inputs()
            .iter()
            .map(|input| mangle(input, strings))
            .collect();
        format!("{class}_{name}_{mangled_overloads}")
    } else {
        escape_type_name(call_site.name())
    }
}
