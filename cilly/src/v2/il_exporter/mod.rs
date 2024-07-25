use std::io::Write;

use crate::{v2::MethodImpl, IlasmFlavour};

use super::{Assembly, ClassRefIdx, Exporter, NodeIdx, RootIdx, Type};

pub struct ILExporter {
    flavour: IlasmFlavour,
}
impl ILExporter {
    pub fn new(flavour: IlasmFlavour) -> Self {
        Self { flavour }
    }

    fn export_to_write(&self, asm: &super::Assembly, out: &mut impl Write) -> std::io::Result<()> {
        // Iterate trough all types
        for class_def in asm.iter_class_defs() {
            let vis = match class_def.access() {
                crate::v2::Access::Extern | crate::v2::Access::Public => "public",
                crate::v2::Access::Private => "private",
            };
            let sealed = if class_def.is_valuetype() {
                "sealed"
            } else {
                ""
            };
            let extends = if let Some(parrent) = class_def.extends() {
                simple_class_ref(parrent, asm)
            } else if class_def.is_valuetype() {
                "[System.Runtime]System.ValueType".into()
            } else {
                "[System.Runtime]System.Object".into()
            };
            let explicit = if class_def.has_explicit_layout() {
                "explicit"
            } else {
                ""
            };
            let name = asm.get_string(class_def.name());
            writeln!(
                out,
                ".class {vis} unicode {sealed} {explicit} '{name}' extends {extends}{{"
            )?;
            // Export all fields
            for (tpe, name, offset) in class_def.fields() {
                let name = asm.get_string(*name);
                let tpe = type_il(tpe, asm);
                if let Some(offset) = offset {
                    writeln!(out, ".field [{offset}] {tpe} '{name}'")
                } else {
                    writeln!(out, ".field {tpe} '{name}'")
                }?;
            }
            // Export all static fields
            for (tpe, name, thread_local) in class_def.static_fields() {
                let name = asm.get_string(*name);
                let tpe = type_il(tpe, asm);
                if *thread_local {
                    writeln!(out,".custom instance void [System.Runtime]System.ThreadStaticAttribute::.ctor() = (01 00 00 00)")?;
                };
                writeln!(out, ".field static {tpe} '{name}'")?;
            }
            // Export all methods
            for method in class_def.methods() {
                let method = asm.method_def(*method);
                let vis = match method.access() {
                    crate::v2::Access::Extern | crate::v2::Access::Public => "public",
                    crate::v2::Access::Private => "private",
                };
                let kind = match method.kind() {
                    crate::v2::cilnode::MethodKind::Static => "cil managed static",
                    crate::v2::cilnode::MethodKind::Instance => "cil managed instance",
                    crate::v2::cilnode::MethodKind::Virtual => "cil managed virtual instance",
                    crate::v2::cilnode::MethodKind::Constructor => {
                        "static rtspecialname specialname"
                    }
                };
                let pinvoke = if let MethodImpl::Extern {
                    lib,
                    preserve_errno,
                } = method.implementation()
                {
                    let lib = asm.get_string(*lib);
                    if *preserve_errno {
                        format!("pinvokeimpl(\"{lib}\" cdecl lasterr)")
                    } else {
                        format!("pinvokeimpl(\"{lib}\" cdecl)")
                    }
                } else {
                    "".into()
                };
                let name = asm.get_string(method.name());
                let sig = asm.get_sig(method.sig());
                let ret = type_il(sig.output(), asm);
                let inputs: String = sig
                    .input()
                    .iter()
                    .zip(method.arg_names())
                    .map(|(tpe, name)| match name {
                        Some(name) => {
                            format!("{} '{}'", type_il(tpe, asm), asm.get_string(*name))
                        }
                        None => type_il(tpe, asm),
                    })
                    .intersperse(",".to_string())
                    .collect();

                writeln!(
                    out,
                    ".method {vis} hidebysig {kind} {pinvoke} {ret} '{name}'({inputs}){{"
                )?;
                // Export the implementation
                self.export_method_imp(asm, out, method.resolved_implementation(asm), name)?;
                writeln!(out, "}}")?;
            }
            writeln!(out, "}}")?;
        }
        todo!()
    }
    fn export_method_imp(
        &self,
        asm: &super::Assembly,
        out: &mut impl Write,
        mimpl: &MethodImpl,
        name: &str,
    ) -> std::io::Result<()> {
        match  mimpl{
            MethodImpl::MethodBody { blocks, locals } => {
                let locals:String = locals.iter().map(|(name,tpe)|match name {
                    Some(name) => {
                        format!("\n  {} '{}'", type_il(asm.get_type(*tpe), asm), asm.get_string(*name))
                    }
                    None => format!("\n  {}",type_il(asm.get_type(*tpe), asm)),
                }).intersperse(",".to_owned()).collect();
                writeln!(out," .locals ({locals})")?;
                for block in blocks{
                    writeln!(out," bb_{}_{}:",block.block_id(),0)?;
                    for root in block.roots(){
                        self.export_root(asm,out,*root)?;
                    }
                }
            }
            MethodImpl::Extern { .. } => (),
            MethodImpl::AliasFor(_) => {
                panic!("resolved_implementation returned `AliasFor`")
            }
            MethodImpl::Missing =>writeln!(out,"ldstr \"missing methiod {name}\"\n newobj void [System.Runtime] System.Exception::.ctor(string)\n throw")?,
        };
        Ok(())
    }
    fn node_code(
        &self,
        asm: &super::Assembly,
        out: &mut impl Write,
        node: NodeIdx,
    ) -> std::io::Result<()> {
        let node = asm.get_node(node);
        match node {
            super::CILNode::Const(cst) => match cst.as_ref() {
                super::Const::I8(val) => match val {
                    -1 => writeln!(out, "ldc.i4.m1"),
                    0..=8 => writeln!(out, "ldc.i4.{val}"),
                    _ => writeln!(out, "ldc.i4.s {val}"),
                },
                super::Const::I16(val) => match val {
                    -1 => writeln!(out, "ldc.i4.m1"),
                    0..=8 => writeln!(out, "ldc.i4.{val}"),
                    9..=127 => writeln!(out, "ldc.i4.s {val}"),
                    _ => writeln!(out, "ldc.i4 {val}"),
                },
                super::Const::I32(val) => match val {
                    -1 => writeln!(out, "ldc.i4.m1"),
                    0..=8 => writeln!(out, "ldc.i4.{val}"),
                    9..=127 => writeln!(out, "ldc.i4.s {val}"),
                    _ => writeln!(out, "ldc.i4 {val}"),
                },
                super::Const::I64(val) => match val {
                    -1 => writeln!(out, "ldc.i4.m1 conv.i8"),
                    0..=8 => writeln!(out, "ldc.i4.{val} conv.i8"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} conv.i8"),
                    -2_147_483_648i64..0 | 128..=2_147_483_647i64 => {
                        writeln!(out, "ldc.i4 {val} conv.i8")
                    }
                    _ => writeln!(out, "ldc.i8 {val}"),
                },
                super::Const::ISize(val) => match val {
                    -1 => writeln!(out, "ldc.i4.m1 conv.i"),
                    0..=8 => writeln!(out, "ldc.i4.{val} conv.i"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} conv.i"),
                    -2_147_483_648i64..0 | 128..=2_147_483_647i64 => {
                        writeln!(out, "ldc.i4 {val} conv.i")
                    }
                    _ => writeln!(out, "ldc.i8 {val} conv.i"),
                },
                super::Const::U8(val) => match val {
                    0..=8 => writeln!(out, "ldc.i4.{val}"),
                    9..=127 => writeln!(out, "ldc.i4.s {val}"),
                    _ => writeln!(out, "ldc.i4 {val}"),
                },
                super::Const::U16(val) => match val {
                    0..=8 => writeln!(out, "ldc.i4.{val}"),
                    9..=127 => writeln!(out, "ldc.i4.s {val}"),
                    _ => writeln!(out, "ldc.i4 {val}"),
                },
                super::Const::U32(val) => match val {
                    0..=8 => writeln!(out, "ldc.i4.{val}"),
                    9..=127 => writeln!(out, "ldc.i4.s {val}"),
                    _ => writeln!(out, "ldc.i4 {val}"),
                },
                super::Const::U64(val) => match val {
                    0..=8 => writeln!(out, "ldc.i4.{val} conv.u8"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} conv.u8"),
                    128..=2_147_483_647u64 => writeln!(out, "ldc.i4 {val} conv.u8"),
                    _ => writeln!(out, "ldc.i8 {val}"),
                },
                super::Const::USize(val) => match val {
                    0..=8 => writeln!(out, "ldc.i4.{val} conv.u"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} conv.u"),
                    128..=2_147_483_647u64 => writeln!(out, "ldc.i4 {val} conv.u"),
                    _ => writeln!(out, "ldc.i8 {val} conv.u"),
                },
                super::Const::PlatformString(_) => todo!(),
                super::Const::Bool(val) => {
                    if *val {
                        writeln!(out, "ldc.i4.1")
                    } else {
                        writeln!(out, "ldc.i4.0")
                    }
                }
                super::Const::F32(float) => {
                    let const_literal = if float.is_nan() {
                        [0x00, 0x00, 0xF8, 0xFF]
                    } else {
                        float.to_le_bytes()
                    };
                    write!(
                        out,
                        "ldc.r4 ({:02x} {:02x} {:02x} {:02x})",
                        const_literal[0], const_literal[1], const_literal[2], const_literal[3]
                    )
                }
                super::Const::F64(float) => {
                    let const_literal = if float.is_nan() {
                        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0xFF]
                    } else {
                        float.to_le_bytes()
                    };
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
            },
            super::CILNode::BinOp(_, _, _) => todo!(),
            super::CILNode::UnOp(_, _) => todo!(),
            super::CILNode::LdLoc(loc) => match loc {
                0..=3 => writeln!(out, "ldloc.{loc}"),
                4..=255 => writeln!(out, "ldloc.s {loc}"),
                _ => writeln!(out, "ldloc {loc}"),
            },
            super::CILNode::LdLocA(arg) => match arg {
                0..=255 => writeln!(out, "ldloca.s {arg}"),
                _ => writeln!(out, "ldloca {arg}"),
            },
            super::CILNode::LdArg(arg) => match arg {
                0..=3 => writeln!(out, "ldarg.{arg}"),
                4..=255 => writeln!(out, "ldarg.s {arg}"),
                _ => writeln!(out, "ldarg {arg}"),
            },
            super::CILNode::LdArgA(arg) => match arg {
                0..=255 => writeln!(out, "ldarg.s {arg}"),
                _ => writeln!(out, "ldarg {arg}"),
            },
            super::CILNode::Call(call) => {
                for arg in &call.1 {
                    self.node_code(asm, out, *arg)?;
                }
                let mref = asm.get_mref(call.0);
                let call_op = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => "call",
                    crate::v2::cilnode::MethodKind::Instance => "call instance",
                    crate::v2::cilnode::MethodKind::Virtual => " callvirt instance",
                    crate::v2::cilnode::MethodKind::Constructor => "newobj instance",
                };
                let sig = asm.get_sig(mref.sig());
                let output = type_il(sig.output(), asm);
                let inputs: String = sig
                    .input()
                    .iter()
                    .map(|tpe| type_il(tpe, asm))
                    .intersperse(",".to_owned())
                    .collect();
                let name = asm.get_string(mref.name());
                let class = class_ref(mref.class(), asm);
                writeln!(out, "{call_op} {output} {class}::'{name}'({inputs})")
            }
            super::CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                self.node_code(asm, out, *input)?;
                match (target, extend) {
                    (super::Int::U8, super::cilnode::ExtendKind::ZeroExtend)
                    | (super::Int::I8, super::cilnode::ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u1")
                    }
                    (super::Int::U8, super::cilnode::ExtendKind::SignExtend)
                    | (super::Int::I8, super::cilnode::ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i1")
                    }
                    (super::Int::U16, super::cilnode::ExtendKind::ZeroExtend)
                    | (super::Int::I16, super::cilnode::ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u2")
                    }
                    (super::Int::U16, super::cilnode::ExtendKind::SignExtend)
                    | (super::Int::I16, super::cilnode::ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i2")
                    }
                    (super::Int::U32, super::cilnode::ExtendKind::ZeroExtend)
                    | (super::Int::I32, super::cilnode::ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u4")
                    }
                    (super::Int::U32, super::cilnode::ExtendKind::SignExtend)
                    | (super::Int::I32, super::cilnode::ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i4")
                    }

                    (super::Int::U64, super::cilnode::ExtendKind::ZeroExtend)
                    | (super::Int::I64, super::cilnode::ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u8")
                    }
                    (super::Int::U64, super::cilnode::ExtendKind::SignExtend)
                    | (super::Int::I64, super::cilnode::ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i8")
                    }
                    (super::Int::USize, super::cilnode::ExtendKind::SignExtend)
                    | (super::Int::ISize, super::cilnode::ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i")
                    }
                    (super::Int::USize, super::cilnode::ExtendKind::ZeroExtend)
                    | (super::Int::ISize, super::cilnode::ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u")
                    }
                    (super::Int::U128, super::cilnode::ExtendKind::ZeroExtend) => todo!(),
                    (super::Int::U128, super::cilnode::ExtendKind::SignExtend) => todo!(),
                    (super::Int::I128, super::cilnode::ExtendKind::ZeroExtend) => todo!(),
                    (super::Int::I128, super::cilnode::ExtendKind::SignExtend) => todo!(),
                }
            }
            super::CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => todo!(),
            super::CILNode::RefToPtr(_) => todo!(),
            super::CILNode::PtrCast(val, _) => self.node_code(asm, out, *val),
            super::CILNode::LdFieldAdress { addr, field } => todo!(),
            super::CILNode::LdField { addr, field } => todo!(),
            super::CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => todo!(),
            super::CILNode::SizeOf(_) => todo!(),
            super::CILNode::GetException => todo!(),
            super::CILNode::IsInst(_, _) => todo!(),
            super::CILNode::CheckedCast(_, _) => todo!(),
            super::CILNode::CallI(_) => todo!(),
            super::CILNode::LocAlloc { size } => todo!(),
            super::CILNode::LdStaticField(_) => todo!(),
            super::CILNode::LdFtn(_) => todo!(),
            super::CILNode::LdTypeToken(_) => todo!(),
            super::CILNode::LdLen(_) => todo!(),
            super::CILNode::LocAllocAlgined { tpe, align } => todo!(),
            super::CILNode::LdElelemRef { array, index } => todo!(),
            super::CILNode::UnboxAny { object, tpe } => todo!(),
        }
    }
    fn export_root(
        &self,
        asm: &super::Assembly,
        out: &mut impl Write,
        root: RootIdx,
    ) -> std::io::Result<()> {
        let root = asm.get_root(root);
        match root {
            super::CILRoot::StLoc(loc, val) => {
                self.node_code(asm, out, *val)?;
                match loc {
                    0..=3 => writeln!(out, "stloc.{loc}"),
                    4..=255 => writeln!(out, "stloc.s {loc}"),
                    _ => writeln!(out, "stloc {loc}"),
                }
            }
            super::CILRoot::StArg(loc, val) => {
                self.node_code(asm, out, *val)?;
                match loc {
                    0..=255 => writeln!(out, "starg.s {loc}"),
                    _ => writeln!(out, "starg {loc}"),
                }
            }
            super::CILRoot::Ret(val) => {
                self.node_code(asm, out, *val)?;
                writeln!(out, "ret")
            }
            super::CILRoot::Pop(val) => {
                self.node_code(asm, out, *val)?;
                writeln!(out, "pop")
            }
            super::CILRoot::Throw(val) => {
                self.node_code(asm, out, *val)?;
                writeln!(out, "throw")
            }
            super::CILRoot::VoidRet => {
                writeln!(out, "ret")
            }
            super::CILRoot::Break => {
                writeln!(out, "break")
            }
            super::CILRoot::Nop => {
                writeln!(out, "nop")
            }
            super::CILRoot::Branch(_) => todo!(),
            super::CILRoot::SourceFileInfo {
                line_start,
                line_len,
                col_start,
                col_len,
                file,
            } => {
                let col_end = *col_start as u32 + *col_len as u32;
                let line_end = *line_start + *line_len as u32;
                let file = asm.get_string(*file);
                match self.flavour {
                    IlasmFlavour::Clasic => {
                        writeln!(out, ".line {line_start}:{col_start} '{file}'")
                    }
                    IlasmFlavour::Modern => writeln!(
                        out,
                        ".line {line_start},{line_end}:{col_start},{col_end} '{file}'"
                    ),
                }
            }
            super::CILRoot::SetField(_) => todo!(),
            super::CILRoot::Call(_) => todo!(),
            super::CILRoot::StInd(stind) => {
                let addr = stind.0;
                let val = stind.1;
                let tpe = stind.2;
                let is_volitale = if stind.3 { "volatile." } else { "" };
                match tpe {
                    Type::Ptr(_) => writeln!(out, "{is_volitale} stind.i"),
                    Type::Ref(_) => todo!(),
                    Type::Int(int) => match int {
                        super::Int::U8 => writeln!(out, "{is_volitale} stind.i1"),
                        super::Int::U16 => writeln!(out, "{is_volitale} stind.i2"),
                        super::Int::U32 => writeln!(out, "{is_volitale} stind.i4"),
                        super::Int::U64 => writeln!(out, "{is_volitale} stind.i8"),
                        super::Int::U128 => todo!(),
                        super::Int::USize => writeln!(out, "{is_volitale} stind.i"),
                        super::Int::I8 => writeln!(out, "{is_volitale} stind.i1"),
                        super::Int::I16 => writeln!(out, "{is_volitale} stind.i2"),
                        super::Int::I32 => writeln!(out, "{is_volitale} stind.i4"),
                        super::Int::I64 => writeln!(out, "{is_volitale} stind.i8"),
                        super::Int::I128 => todo!(),
                        super::Int::ISize => writeln!(out, "{is_volitale} stind.i"),
                    },
                    Type::ClassRef(cref_idx) => {
                        let cref = asm.class_ref(cref_idx);
                        if cref.is_valuetype() {
                            writeln!(
                                out,
                                "{is_volitale} stobj {cref}",
                                cref = class_ref(cref_idx, asm)
                            )
                        } else {
                            writeln!(out, "{is_volitale} stind.ref")
                        }
                    }
                    Type::Float(float) => match float {
                        super::Float::F16 => todo!(),
                        super::Float::F32 => writeln!(out, "{is_volitale} stind.r4"),
                        super::Float::F64 => writeln!(out, "{is_volitale} stind.r8"),
                    },
                    Type::PlatformString => writeln!(out, "{is_volitale} stind.ref"),
                    Type::PlatformChar => writeln!(out, "{is_volitale} stind.i2"),
                    Type::PlarformGeneric(_, _) => todo!(),
                    Type::Bool => writeln!(out, "{is_volitale} stind.i1"),
                    Type::Void => todo!(),
                    Type::PlatformArray { .. } => writeln!(out, "{is_volitale} stind.ref"),
                    Type::FnPtr(_) => writeln!(out, "{is_volitale} stind.i"),
                }
            }
            super::CILRoot::InitBlk(blk) => {
                self.node_code(asm, out, blk.0)?;
                self.node_code(asm, out, blk.1)?;
                self.node_code(asm, out, blk.2)?;
                writeln!(out, "initblk")
            }
            super::CILRoot::CpBlk(_) => todo!(),
            super::CILRoot::CallI(_) => todo!(),
            super::CILRoot::ExitSpecialRegion { target, source } => todo!(),
            super::CILRoot::ReThrow => {
                writeln!(out, "rethrow")
            }
            super::CILRoot::SetStaticField { field, val } => todo!(),
        }
    }
}

impl Exporter for ILExporter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        // The IL file should be next to the target
        let il_path = target.with_extension("il");
        let mut il_out = std::io::BufWriter::new(std::fs::File::create(il_path)?);
        self.export_to_write(asm, &mut il_out)?;
        todo!()
    }
}
fn simple_class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = asm.get_string(cref.name());
    if let Some(assembly) = cref.asm() {
        let assembly = asm.get_string(assembly);
        format!("[{assembly}]'{name}'")
    } else {
        format!("'{name}'")
    }
}
fn class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = asm.get_string(cref.name());
    let prefix = if cref.is_valuetype() {
        "valuetype"
    } else {
        "class"
    };
    if let Some(assembly) = cref.asm() {
        let assembly = asm.get_string(assembly);
        format!("{prefix} [{assembly}]'{name}'")
    } else {
        format!("{prefix} '{name}'")
    }
}
fn type_il(tpe: &Type, asm: &Assembly) -> String {
    match tpe {
        Type::Ptr(inner) => format!("{}*", type_il(asm.get_type(*inner), asm)),
        Type::Ref(inner) => format!("{}&", type_il(asm.get_type(*inner), asm)),
        Type::Int(int) => match int {
            super::Int::U8 => "uint8".into(),
            super::Int::U16 => "uint16".into(),
            super::Int::U32 => "uint32".into(),
            super::Int::U64 => "uin64".into(),
            super::Int::U128 => "valuetype [System.Runtime]System.UInt128".into(),
            super::Int::USize => "native uint".into(),
            super::Int::I8 => "int8".into(),
            super::Int::I16 => "int16".into(),
            super::Int::I32 => "int32".into(),
            super::Int::I64 => "int64".into(),
            super::Int::I128 => "valuetype [System.Runtime]System.Int128".into(),
            super::Int::ISize => "native int".into(),
        },
        Type::ClassRef(cref) => class_ref(*cref, asm),
        Type::Float(float) => match float {
            super::Float::F16 => todo!(),
            super::Float::F32 => "float32".into(),
            super::Float::F64 => "float64".into(),
        },
        Type::PlatformChar => "char".into(),
        Type::PlarformGeneric(_, _) => todo!(),
        Type::Bool => "bool".into(),
        Type::Void => "void".into(),
        Type::PlatformArray { elem, dims } => format!(
            "{elem}[{dims}]",
            elem = type_il(asm.get_type(*elem), asm),
            dims = (0..dims.get()).map(|_| ',').collect::<String>()
        ),
        Type::FnPtr(_) => todo!(),
        Type::PlatformString => "string".into(),
    }
}
