use std::io::Write;
use lazy_static::*;
use crate::v2::MethodImpl;

use super::{
    asm::{IlasmFlavour, ILASM_FLAVOUR, ILASM_PATH}, cilroot::BranchCond, int, Assembly, CILIter, CILIterElem, CILNode, ClassRefIdx, Exporter, NodeIdx, RootIdx, Type
};

pub struct ILExporter {
    flavour: IlasmFlavour,
    is_lib: bool,
}
impl ILExporter {
    pub fn new(flavour: IlasmFlavour, is_lib: bool) -> Self {
        Self { flavour, is_lib }
    }

    fn export_to_write(&self, asm: &super::Assembly, out: &mut impl Write) -> std::io::Result<()> {
        writeln!(out, ".assembly _{{}}")?;
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
                "auto"
            };
            let name = asm.get_string(class_def.name());
            writeln!(
                out,
                ".class {vis} ansi {sealed} {explicit} '{name}' extends {extends}{{"
            )?;
            // Export size
            if let Some(size) = class_def.explict_size() {
                writeln!(out, ".size {size}", size = size.get())?;
            }
            // Export all fields
            for (tpe, name, offset) in class_def.fields() {
                let name = asm.get_string(*name);
                let tpe = non_void_type_il(tpe, asm);
                if let Some(offset) = offset {
                    writeln!(out, ".field [{offset}] {tpe} '{name}'")
                } else {
                    writeln!(out, ".field {tpe} '{name}'")
                }?;
            }
            // Export all static fields
            for (tpe, name, thread_local) in class_def.static_fields() {
                let name = asm.get_string(*name);
                let tpe = non_void_type_il(tpe, asm);
                
                writeln!(out, ".field static {tpe} '{name}'")?;
                if *thread_local {
                    writeln!(out,".custom instance void [System.Runtime]System.ThreadStaticAttribute::.ctor() = (01 00 00 00)")?;
                };
            }
            // Export all methods

            for method_id in class_def.methods() {
                let method = asm.method_def(*method_id);
                let vis = match method.access() {
                    crate::v2::Access::Extern | crate::v2::Access::Public => "public",
                    crate::v2::Access::Private => "private",
                };
                let kind = match method.kind() {
                    crate::v2::cilnode::MethodKind::Static => "static",
                    crate::v2::cilnode::MethodKind::Instance => "instance",
                    crate::v2::cilnode::MethodKind::Virtual => "virtual instance",
                    crate::v2::cilnode::MethodKind::Constructor => {
                        "rtspecialname specialname"
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
                assert_eq!(method.arg_names().len(), sig.inputs().len(), "{name:?}");
                let inputs = match method.kind() {
                    crate::v2::cilnode::MethodKind::Static => sig.inputs(),
                    crate::v2::cilnode::MethodKind::Instance
                    | crate::v2::cilnode::MethodKind::Virtual
                    | crate::v2::cilnode::MethodKind::Constructor => &sig.inputs()[1..],
                };

                let inputs: String = inputs
                    .iter()
                    .zip(method.arg_names())
                    .map(|(tpe, name)| match name {
                        Some(name) => {
                            format!("{} '{}'", non_void_type_il(tpe, asm), asm.get_string(*name))
                        }
                        None => non_void_type_il(tpe, asm),
                    })
                    .intersperse(",".to_string())
                    .collect();
                let preservesig = if method.implementation().is_extern(){
                    "preservesig"
                }else{""};
                writeln!(
                    out,
                    ".method {vis} hidebysig {kind} {pinvoke} {ret} '{name}'({inputs}) cil managed {preservesig}{{// Method ID {method_id:?}"
                )?;
                let stack_size = match method.resolved_implementation(asm) {
                    MethodImpl::MethodBody { blocks, .. } => blocks
                        .iter()
                        .flat_map(|block| block.roots().iter())
                        .map(|root| {
                            crate::v2::CILIter::new(asm.get_root(*root).clone(), asm).count() + 4
                        })
                        .max()
                        .unwrap_or(0),
                    MethodImpl::Extern { .. } => 0,
                    MethodImpl::AliasFor(_) => todo!(),
                    MethodImpl::Missing => 3,
                };
                if stack_size > 6 {
                    writeln!(out, ".maxstack {stack_size}")?;
                }
                if **name == *"entrypoint" {
                    writeln!(out, ".entrypoint")?;
                }
                // Export the implementation
                self.export_method_imp(asm, out, method.resolved_implementation(asm), name)?;
                writeln!(out, "}}")?;
            }
            writeln!(out, "}}")?;
        }

        Ok(())
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
                        format!("\n  {} '{}'", non_void_type_il(asm.get_type(*tpe), asm), asm.get_string(*name))
                    }
                    None => format!("\n  {}",non_void_type_il(asm.get_type(*tpe), asm)),
                }).intersperse(",".to_owned()).collect();
                writeln!(out," .locals ({locals})")?;
                for block in blocks{
                    if block.handler().is_some(){
                        writeln!(out,".try{{")?;
                    }
                    writeln!(out," bb{}:",block.block_id())?;
                    for root in block.roots(){
                        self.export_root(asm,out,*root,false)?;
                    }
                    if let Some(handler) = block.handler(){
                        writeln!(out,"}} catch [System.Runtime]System.Object{{")?;
                        // Check for the GetException intrinsic. If it is not used, put a pop here.
                        if !handler.iter().flat_map(|block|block.roots()).flat_map(|root|CILIter::new(asm.get_root(*root).clone(),asm)).any(|elem|matches!(elem,CILIterElem::Node(CILNode::GetException))){
                            writeln!(out,"pop")?;
                        }
                        for hblock in handler{
                            writeln!(out," h{}_{}:",block.block_id(),hblock.block_id())?;
                            for root in hblock.roots(){
                                self.export_root(asm,out,*root,true)?;
                            }
                        }  
                    }
                      if block.handler().is_some(){
                        writeln!(out,"}}")?;
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
    fn export_node(
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
                super::Const::PlatformString(msg) => {
                    let msg = asm.get_string(*msg);
                    writeln!(out, "ldstr {msg:?}")
                }
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
                    writeln!(
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
                    writeln!(
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
            super::CILNode::BinOp(lhs, rhs, op) => {
                self.export_node(asm, out, *lhs)?;
                self.export_node(asm, out, *rhs)?;
                match op {
                    super::BinOp::Add => writeln!(out, "add"),
                    super::BinOp::Eq => writeln!(out, "ceq"),
                    super::BinOp::Sub => writeln!(out, "sub"),
                    super::BinOp::Mul => writeln!(out, "mul"),
                    super::BinOp::LtUn => writeln!(out, "clt.un"),
                    super::BinOp::Lt => writeln!(out, "clt"),
                    super::BinOp::GtUn => writeln!(out, "cgt.un"),
                    super::BinOp::Gt => writeln!(out, "cgt"),
                    super::BinOp::Or => writeln!(out, "or"),
                    super::BinOp::XOr => writeln!(out, "xor"),
                    super::BinOp::And => writeln!(out, "and"),
                    super::BinOp::Rem => writeln!(out, "rem"),
                    super::BinOp::RemUn => writeln!(out, "rem.un"),
                    super::BinOp::Shl => writeln!(out, "shl"),
                    super::BinOp::Shr => writeln!(out, "shr"),
                    super::BinOp::ShrUn => writeln!(out, "shr.un"),
                    super::BinOp::DivUn => writeln!(out, "div.un"),
                    super::BinOp::Div => writeln!(out, "div"),
                }
            }
            super::CILNode::UnOp(arg, un) => {
                self.export_node(asm, out, *arg)?;
                match un {
                    super::cilnode::UnOp::Not => writeln!(out, "not"),
                    super::cilnode::UnOp::Neg => writeln!(out, "neg"),
                }
            }
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
                0..=255 => writeln!(out, "ldarga.s {arg}"),
                _ => writeln!(out, "ldarga {arg}"),
            },
            super::CILNode::Call(call) => {
                for arg in &call.1 {
                    self.export_node(asm, out, *arg)?;
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
                let inputs = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => sig.inputs(),
                    crate::v2::cilnode::MethodKind::Instance
                    | crate::v2::cilnode::MethodKind::Virtual
                    | crate::v2::cilnode::MethodKind::Constructor => {
                        assert!(
                            !sig.inputs().is_empty(),
                            "invalid argc when calling {} of {}",
                            asm.get_string(mref.name()),
                            class_ref(mref.class(), asm)
                        );
                        &sig.inputs()[1..]
                    }
                };
                let inputs: String = inputs
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
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
                self.export_node(asm, out, *input)?;
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
            } => {
                self.export_node(asm, out, *input)?;
                match (target, is_signed) {
                    (super::Float::F16, true) => todo!(),
                    (super::Float::F16, false) => todo!(),
                    (super::Float::F32, true) => writeln!(out, "conv.r4"),
                    (super::Float::F32, false) => writeln!(out, "conv.r.un conv.r4"),
                    (super::Float::F64, true) => writeln!(out, "conv.r8"),
                    (super::Float::F64, false) => writeln!(out, "conv.r.un conv.r8"),
                }
            }
            super::CILNode::RefToPtr(inner) => {
                self.export_node(asm, out, *inner)?;
                writeln!(out, "conv.u")
            }
            super::CILNode::PtrCast(val, _) => self.export_node(asm, out, *val),
            super::CILNode::LdFieldAdress { addr, field } => {
                self.export_node(asm, out, *addr)?;
                let fld = asm.get_field(*field);
                let owner = class_ref(fld.owner(), asm);
                let name = asm.get_string(fld.name());
                let tpe = type_il(&fld.tpe(), asm);
                writeln!(out, "ldflda {tpe} {owner}::'{name}'")
            }
            super::CILNode::LdField { addr, field } => {
                self.export_node(asm, out, *addr)?;
                let fld = asm.get_field(*field);
                let owner = class_ref(fld.owner(), asm);
                let name = asm.get_string(fld.name());
                let tpe = type_il(&fld.tpe(), asm);
                writeln!(out, "ldfld {tpe} {owner}::'{name}'")
            }
            super::CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => {
                self.export_node(asm, out, *addr)?;
                let tpe = asm.get_type(*tpe);

                match (tpe, volitale) {
                    (Type::Ptr(_), true) => writeln!(out, "volatile. ldind.i"),
                    (Type::Ptr(_), false) => writeln!(out, "ldind.i"),
                    (Type::Ref(_), true) => todo!(),
                    (Type::Ref(_), false) => todo!(),
                    (Type::Int(int), volitale) => match (int, volitale) {
                        (int::Int::U8, true) => writeln!(out, "volatile. ldind.u1"),
                        (int::Int::U8, false) => writeln!(out, "ldind.u1"),
                        (int::Int::U16, true) => writeln!(out, "volatile. ldind.u2"),
                        (int::Int::U16, false) => writeln!(out, "ldind.u2"),
                        (int::Int::U32, true) => writeln!(out, "volatile. ldind.u4"),
                        (int::Int::U32, false) => writeln!(out, "ldind.u4"),
                        (int::Int::U64, true) => writeln!(out, "volatile. ldind.u8"),
                        (int::Int::U64, false) => writeln!(out, "ldind.u8"),
                        (int::Int::U128, true) => writeln!(
                            out,
                            "volatile. ldobj valuetype [System.Runtime]System.UInt128"
                        ),
                        (int::Int::U128, false) => {
                            writeln!(out, "ldobj valuetype [System.Runtime]System.UInt128")
                        }
                        (int::Int::USize, true) => writeln!(out, "volatile. ldind.i"),
                        (int::Int::USize, false) => writeln!(out, "ldind.i"),
                        (int::Int::I8, true) => writeln!(out, "volatile. ldind.i1"),
                        (int::Int::I8, false) => writeln!(out, "ldind.i1"),
                        (int::Int::I16, true) => writeln!(out, "volatile. ldind.i2"),
                        (int::Int::I16, false) => writeln!(out, "ldind.i2"),
                        (int::Int::I32, true) => writeln!(out, "volatile. ldind.i4"),
                        (int::Int::I32, false) => writeln!(out, "ldind.i4"),
                        (int::Int::I64, true) => writeln!(out, "volatile. ldind.i8"),
                        (int::Int::I64, false) => writeln!(out, "ldind.i8"),
                        (int::Int::I128, true) => writeln!(
                            out,
                            "volatile. ldobj valuetype [System.Runtime]System.Int128"
                        ),
                        (int::Int::I128, false) => {
                            writeln!(
                                out,
                                "volatile. ldobj valuetype [System.Runtime]System.Int128"
                            )
                        }
                        (int::Int::ISize, true) => writeln!(out, "volatile. ldind.i"),
                        (int::Int::ISize, false) => writeln!(out, "ldind.i"),
                    },
                    (Type::ClassRef(cref), true) => {
                        writeln!(out, "volatile. ldobj {cref}", cref = class_ref(*cref, asm))
                    }
                    (Type::ClassRef(cref), false) => {
                        writeln!(out, "ldobj {cref}", cref = class_ref(*cref, asm))
                    }
                    (Type::Float(float), volitale) => match (float, volitale) {
                        (super::Float::F16, true) => todo!(),
                        (super::Float::F16, false) => todo!(),
                        (super::Float::F32, true) => writeln!(out, "volatile. ldind.r4"),
                        (super::Float::F32, false) => writeln!(out, "ldind.r4"),
                        (super::Float::F64, true) => writeln!(out, "volatile. ldind.r8"),
                        (super::Float::F64, false) => writeln!(out, "ldind.r8"),
                    },
                    (Type::PlatformString | Type::PlatformObject, true) => {
                        writeln!(out, "volatile. ldind.ref")
                    }
                    (Type::PlatformString | Type::PlatformObject, false) => {
                        writeln!(out, "ldind.ref")
                    }
                    (Type::PlatformChar, true) => writeln!(out, "volatile. ldind.i2"),
                    (Type::PlatformChar, false) => writeln!(out, "ldind.i2"),
                    (Type::PlatformGeneric(_, _), true) => todo!(),
                    (Type::PlatformGeneric(_, _), false) => todo!(),
                    (Type::Bool, true) => writeln!(out, "volatile. ldind.i1"),
                    (Type::Bool, false) => writeln!(out, "ldind.i1"),
                    (Type::Void, true) | (Type::Void, false) => {
                        panic!("Void can't be dereferenced!")
                    }
                    (Type::PlatformArray { .. }, true) => writeln!(out, "volatile. ldind.ref"),
                    (Type::PlatformArray { .. }, false) => writeln!(out, "ldind.ref"),
                    (Type::FnPtr(_), true) => writeln!(out, "volatile. ldind.i"),
                    (Type::FnPtr(_), false) => writeln!(out, "ldind.i"),
                }
            }
            super::CILNode::SizeOf(tpe) => {
                writeln!(out, "sizeof {}", type_il(asm.get_type(*tpe), asm))
            }
            super::CILNode::GetException => Ok(()),
            super::CILNode::IsInst(val, tpe) => {
                self.export_node(asm, out, *val)?;
                writeln!(out, "isinst {tpe}", tpe = type_il(asm.get_type(*tpe), asm))
            }
            super::CILNode::CheckedCast(val, tpe) => {
                self.export_node(asm, out, *val)?;
                writeln!(
                    out,
                    "castclass {tpe}",
                    tpe = type_il(asm.get_type(*tpe), asm)
                )
            }
            super::CILNode::CallI(calli) => {
                let (fn_ptr, sig, args) = calli.as_ref();
                for arg in args {
                    self.export_node(asm, out, *arg)?;
                }
                let sig = asm.get_sig(*sig);
                let output = type_il(sig.output(), asm);
                self.export_node(asm, out, *fn_ptr)?;
                let inputs: String = sig
                    .inputs()
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_owned())
                    .collect();
                writeln!(out, "calli {output} ({inputs})")
            }
            super::CILNode::LocAlloc { size } => {
                self.export_node(asm, out, *size)?;
                writeln!(out, "localloc")
            }
            super::CILNode::LdStaticField(sfld) => {
                let sfld = asm.get_static_field(*sfld);
                let owner = class_ref(sfld.owner(), asm);
                let name = asm.get_string(sfld.name());
                let tpe = type_il(&sfld.tpe(), asm);
                writeln!(out, "ldsfld {tpe} {owner}::{name}")
            }
            super::CILNode::LdFtn(ftn) => {
                let mref = asm.get_mref(*ftn);
                let sig = asm.get_sig(mref.sig());
                let output = type_il(sig.output(), asm);
                let inputs = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => sig.inputs(),
                    crate::v2::cilnode::MethodKind::Instance
                    | crate::v2::cilnode::MethodKind::Virtual
                    | crate::v2::cilnode::MethodKind::Constructor => &sig.inputs()[1..],
                };
                let inputs: String = inputs
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_owned())
                    .collect();
                let name = asm.get_string(mref.name());
                let class = class_ref(mref.class(), asm);
                let ldftn_op = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => "ldftn",
                    crate::v2::cilnode::MethodKind::Instance => "ldftn instance",
                    crate::v2::cilnode::MethodKind::Virtual => " ldftn instance",
                    crate::v2::cilnode::MethodKind::Constructor => "ldftn instance",
                };
                writeln!(out, "{ldftn_op} {output} {class}::'{name}'({inputs})")
            }
            super::CILNode::LdTypeToken(tok) => {
                writeln!(out, "ldtoken {tok}", tok = type_il(asm.get_type(*tok), asm))
            }
            super::CILNode::LdLen(array) => {
                self.export_node(asm, out, *array)?;
                writeln!(out, "ldlen")
            }
            super::CILNode::LocAllocAlgined { tpe, align } => {
                writeln!(out, "sizeof {tpe} ldc.i8 {align} conv.i add localloc dup ldc.i8 {align} add ldc.i8 {align} rem sub ldc.i8 {align} add", tpe = type_il(asm.get_type(*tpe), asm))
            }
            super::CILNode::LdElelemRef { array, index } => {
                self.export_node(asm, out, *array)?;
                self.export_node(asm, out, *index)?;
                writeln!(out, "ldelem.ref")
            }
            super::CILNode::UnboxAny { object, tpe } => {
                self.export_node(asm, out, *object)?;
                writeln!(
                    out,
                    "unbox.any {object}",
                    object = type_il(asm.get_type(*tpe), asm)
                )
            }
        }
    }
    fn export_root(
        &self,
        asm: &super::Assembly,
        out: &mut impl Write,
        root: RootIdx,
        is_handler: bool,
    ) -> std::io::Result<()> {
        let root = asm.get_root(root);
        match root {
            super::CILRoot::StLoc(loc, val) => {
                self.export_node(asm, out, *val)?;
                match loc {
                    0..=3 => writeln!(out, "stloc.{loc}"),
                    4..=255 => writeln!(out, "stloc.s {loc}"),
                    _ => writeln!(out, "stloc {loc}"),
                }
            }
            super::CILRoot::StArg(loc, val) => {
                self.export_node(asm, out, *val)?;
                match loc {
                    0..=255 => writeln!(out, "starg.s {loc}"),
                    _ => writeln!(out, "starg {loc}"),
                }
            }
            super::CILRoot::Ret(val) => {
                self.export_node(asm, out, *val)?;
                writeln!(out, "ret")
            }
            super::CILRoot::Pop(val) => {
                self.export_node(asm, out, *val)?;
                writeln!(out, "pop")
            }
            super::CILRoot::Throw(val) => {
                self.export_node(asm, out, *val)?;
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
            super::CILRoot::Branch(branch) => match &branch.2 {
                Some(BranchCond::Eq(a, b)) => {
                    self.export_node(asm, out, *a)?;
                    self.export_node(asm, out, *b)?;
                    if branch.1 == 0 {
                        writeln!(out, "beq bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "beq h{}_{}", branch.0, branch.1)
                    }
                    else {
                        writeln!(out, "beq jp{}_{}", branch.0, branch.1)
                    }
                }
                Some(BranchCond::Ne(a, b)) => {
                    self.export_node(asm, out, *a)?;
                    self.export_node(asm, out, *b)?;
                    if branch.1 == 0 {
                        writeln!(out, "bne bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "bne h{}_{}", branch.0, branch.1)
                    }
                    else {
                        writeln!(out, "bne jp{}_{}", branch.0, branch.1)
                    }
                }
                Some(BranchCond::Lt(a, b, kind)) => {
                    self.export_node(asm, out, *a)?;
                    self.export_node(asm, out, *b)?;
                    match kind{
                        super::cilroot::CmpKind::Ordered |  super::cilroot::CmpKind::Signed =>  if branch.1 == 0 {
                            writeln!(out, "blt bb{}", branch.0)
                        } else if is_handler {
                            writeln!(out, "blt h{}_{}", branch.0, branch.1)
                        }
                        else {
                            writeln!(out, "blt jp{}_{}", branch.0, branch.1)
                        }
                        super::cilroot::CmpKind::Unordered  | super::cilroot::CmpKind::Unsigned =>  if branch.1 == 0 {
                            writeln!(out, "blt.un bb{}", branch.0)
                        } else if is_handler {
                            writeln!(out, "blt.un h{}_{}", branch.0, branch.1)
                        }
                        else {
                            writeln!(out, "blt.un jp{}_{}", branch.0, branch.1)
                        }
                    }
                }
                Some(BranchCond::Gt(a, b, kind)) => {
                    self.export_node(asm, out, *a)?;
                    self.export_node(asm, out, *b)?;
                    match kind{
                        super::cilroot::CmpKind::Ordered |  super::cilroot::CmpKind::Signed =>  if branch.1 == 0 {
                            writeln!(out, "bgt bb{}", branch.0)
                        } else if is_handler {
                            writeln!(out, "bgt h{}_{}", branch.0, branch.1)
                        }
                        else {
                            writeln!(out, "bgt jp{}_{}", branch.0, branch.1)
                        }
                        super::cilroot::CmpKind::Unordered  | super::cilroot::CmpKind::Unsigned =>  if branch.1 == 0 {
                            writeln!(out, "bgt.un bb{}", branch.0)
                        } else if is_handler {
                            writeln!(out, "bgt.un h{}_{}", branch.0, branch.1)
                        }
                        else {
                            writeln!(out, "bgt.un jp{}_{}", branch.0, branch.1)
                        }
                    }
                   
                }
                Some(BranchCond::True(cond)) => {
                    self.export_node(asm, out, *cond)?;
                    if branch.1 == 0 {
                        writeln!(out, "brtrue bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "brtrue h{}_{}", branch.0, branch.1)
                    }
                    else {
                        writeln!(out, "brtrue jp{}_{}", branch.0, branch.1)
                    }
                }
                Some(BranchCond::False(cond)) => {
                    self.export_node(asm, out, *cond)?;
                    if branch.1 == 0 {
                        writeln!(out, "brfalse bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "brfalse h{}_{}", branch.0, branch.1)
                    } else {
                        writeln!(out, "brfalse jp{}_{}", branch.0, branch.1)
                    }
                }
                None => {
                    if branch.1 == 0 {
                        writeln!(out, "br bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "br h{}_{}", branch.0, branch.1)
                    }
                    // If it is not a handler, then this is the only block in this try, then all jumps are extern, then we can just use leave
                    else {
                        writeln!(out, "leave bb{}", branch.1)
                    }
                }
            },
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
            super::CILRoot::SetField(flds) => {
                self.export_node(asm, out, flds.1)?;
                self.export_node(asm, out, flds.2)?;
                let fld = asm.get_field(flds.0);
                let owner = class_ref(fld.owner(), asm);
                let name = asm.get_string(fld.name());
                let tpe = type_il(&fld.tpe(), asm);
                writeln!(out, "stfld {tpe} {owner}::'{name}'")
            }
            super::CILRoot::Call(call) => {
                for arg in &call.1 {
                    self.export_node(asm, out, *arg)?;
                }
                let mref = asm.get_mref(call.0);
                let call_op = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => "call",
                    crate::v2::cilnode::MethodKind::Instance => "call instance",
                    crate::v2::cilnode::MethodKind::Virtual => " callvirt instance",
                    crate::v2::cilnode::MethodKind::Constructor => {
                        panic!("A constructor can't be a CIL root")
                    }
                };
                let sig = asm.get_sig(mref.sig());
                let output = type_il(sig.output(), asm);
                let inputs = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => sig.inputs(),
                    crate::v2::cilnode::MethodKind::Instance
                    | crate::v2::cilnode::MethodKind::Virtual
                    | crate::v2::cilnode::MethodKind::Constructor => &sig.inputs()[1..],
                };
                let inputs: String = inputs
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_owned())
                    .collect();
                let name = asm.get_string(mref.name());
                let class = class_ref(mref.class(), asm);
                writeln!(out, "{call_op} {output} {class}::'{name}'({inputs}) //mref:{:?}",call.0)
            }
            super::CILRoot::StInd(stind) => {
                self.export_node(asm, out, stind.0)?;
                self.export_node(asm, out, stind.1)?;
              
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
                        super::Int::U128 => {
                            writeln!(out, "{is_volitale} stobj [System.Runtime]System.UInt128")
                        }
                        super::Int::USize => writeln!(out, "{is_volitale} stind.i"),
                        super::Int::I8 => writeln!(out, "{is_volitale} stind.i1"),
                        super::Int::I16 => writeln!(out, "{is_volitale} stind.i2"),
                        super::Int::I32 => writeln!(out, "{is_volitale} stind.i4"),
                        super::Int::I64 => writeln!(out, "{is_volitale} stind.i8"),
                        super::Int::I128 => {
                            writeln!(out, "{is_volitale} stobj [System.Runtime]System.Int128")
                        }
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
                    Type::PlatformString | Type::PlatformObject => {
                        writeln!(out, "{is_volitale} stind.ref")
                    }
                    Type::PlatformChar => writeln!(out, "{is_volitale} stind.i2"),
                    Type::PlatformGeneric(_, _) => todo!(),
                    Type::Bool => writeln!(out, "{is_volitale} stind.i1"),
                    Type::Void => writeln!(out, "pop pop ldstr \"Attempted to wrtie to a zero-sized type(void).\" newobj void [System.Runtime]System.Exception::.ctor(string) throw"), // TODO: forbid this, since this is NEVER valid.
                    Type::PlatformArray { .. } => writeln!(out, "{is_volitale} stind.ref"),
                    Type::FnPtr(_) => writeln!(out, "{is_volitale} stind.i"),
                }
            }
            super::CILRoot::InitBlk(blk) => {
                self.export_node(asm, out, blk.0)?;
                self.export_node(asm, out, blk.1)?;
                self.export_node(asm, out, blk.2)?;
                writeln!(out, "initblk")
            }
            super::CILRoot::CpBlk(cpblk) => {
                self.export_node(asm, out, cpblk.0)?;
                self.export_node(asm, out, cpblk.1)?;
                self.export_node(asm, out, cpblk.2)?;
                writeln!(out, "cpblk")
            }
            super::CILRoot::CallI(calli) => {
                let (fn_ptr, sig, args) = calli.as_ref();
                for arg in args {
                    self.export_node(asm, out, *arg)?;
                }
                let sig = asm.get_sig(*sig);
                let output = type_il(sig.output(), asm);
                self.export_node(asm, out, *fn_ptr)?;
                let inputs: String = sig
                    .inputs()
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_owned())
                    .collect();
                writeln!(out, "calli {output} ({inputs})")
            }
            super::CILRoot::ExitSpecialRegion { target, source } => {
                if is_handler {
                    writeln!(out, "h{source}_{target}: leave bb{target}")
                } else {
                    writeln!(out, "jp{source}_{target}: leave bb{target}")
                }
            }
            super::CILRoot::ReThrow => {
                writeln!(out, "rethrow")
            }
            super::CILRoot::SetStaticField { field, val } => {
                self.export_node(asm, out, *val)?;
                let sfld = asm.get_static_field(*field);
                let owner = class_ref(sfld.owner(), asm);
                let name = asm.get_string(sfld.name());
                let tpe = type_il(&sfld.tpe(), asm);
                writeln!(out, "stsfld {tpe} {owner}::{name}")
            }
        }
    }
}

impl Exporter for ILExporter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        // The IL file should be next to the target
        let il_path = target.with_extension("il");
        let mut il_out = std::io::BufWriter::new(std::fs::File::create(&il_path)?);
        self.export_to_write(asm, &mut il_out)?;
        // Needed to ensure the IL file is valid!
        il_out.flush().unwrap();
        drop(il_out);
        let exe_out = target.with_extension("exe");
        let asm_type = if self.is_lib { "-dll" } else { "-exe" };
        let mut cmd = std::process::Command::new(ILASM_PATH.clone());
        cmd.arg(il_path)
        .arg(format!("-output:{exe_out}", exe_out = exe_out.clone().to_string_lossy()))
        .arg("-debug")
        .arg("-OPTIMIZE")
        .arg(asm_type)
        // .arg("-FOLD") saves up on space, consider enabling.
        ;
        if *ILASM_FLAVOUR == IlasmFlavour::Clasic{
            // Limit the memory usage of mono
            cmd.env("MONO_GC_PARAMS","soft-heap-limit=500m");
        }
        let out = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("\nError\n") || stderr.contains("FAILURE") || stdout.contains("FAILURE")
        {
            panic!(
                "stdout:{} stderr:{} cmd:{cmd:?}",
                stdout,
                String::from_utf8_lossy(&out.stderr)
            );
        }

        Ok(())
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
pub(crate) fn class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = asm.get_string(cref.name());
    let prefix = if cref.is_valuetype() {
        "valuetype"
    } else {
        "class"
    };
    let generic_list = if cref.generics().is_empty() {
        "".into()
    } else {
        format!(
            "<{generics}>",
            generics = cref
                .generics()
                .iter()
                .map(|tpe| type_il(tpe, asm))
                .intersperse(",".to_string())
                .collect::<String>()
        )
    };
    let generic_postfix = if cref.generics().is_empty() {
        "".into()
    } else {
        format!("`{}", cref.generics().len())
    };
    if let Some(assembly) = cref.asm() {
        let assembly = asm.get_string(assembly);
        format!("{prefix} [{assembly}]'{name}{generic_postfix}'{generic_list}")
    } else {
        format!("{prefix} '{name}{generic_postfix}'{generic_list}")
    }
}
fn non_void_type_il(tpe: &Type, asm: &Assembly) -> String {
    match tpe {
        Type::Void => "valuetype RustVoid".into(),
        _ => type_il(tpe, asm),
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
            super::Int::U64 => "uint64".into(),
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
        Type::PlatformGeneric(arg, generic) => match generic {
            super::tpe::GenericKind::MethodGeneric => todo!(),
            super::tpe::GenericKind::CallGeneric => format!("!!{arg}"),
            super::tpe::GenericKind::TypeGeneric => format!("!{arg}"),
        },
        Type::Bool => "bool".into(),
        Type::Void => "void".into(),
        Type::PlatformArray { elem, dims } => format!(
            "{elem}[{dims}]",
            elem = type_il(asm.get_type(*elem), asm),
            dims = (1..(dims.get())).map(|_| ',').collect::<String>()
        ),
        Type::FnPtr(sig) => {
            let sig = asm.get_sig(*sig);
            format!(
                "method {output}*({inputs})",
                output = type_il(sig.output(), asm),
                inputs = sig
                    .inputs()
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_string())
                    .collect::<String>(),
            )
        }
        Type::PlatformString => "string".into(),
        Type::PlatformObject => "object".into(),
    }
}
/*
compile_test::aligned::stable::debug
    compile_test::aligned::stable::release
    compile_test::any::stable::debug
    compile_test::any::stable::release
    compile_test::arg_test::stable::debug
    compile_test::arg_test::stable::release
    compile_test::assert::stable::debug
    compile_test::assert::stable::release
    compile_test::assign::stable::debug
    compile_test::assign::stable::release
    compile_test::binops::stable::debug
    compile_test::binops::stable::release
    compile_test::branches::stable::debug
    compile_test::branches::stable::release
    compile_test::caller_location::stable::debug
    compile_test::caller_location::stable::release
    compile_test::calls::stable::debug
    compile_test::calls::stable::release
    compile_test::casts::stable::debug
    compile_test::casts::stable::release
    compile_test::catch::stable::debug
    compile_test::catch::stable::release
    compile_test::closure::stable::debug
    compile_test::closure::stable::release
    compile_test::cmp::stable::debug
    compile_test::cmp::stable::release
    compile_test::copy_nonoverlaping::stable::debug
    compile_test::copy_nonoverlaping::stable::release
    compile_test::dyns::stable::debug
    compile_test::dyns::stable::release
    compile_test::empty_string_slice::stable::debug
    compile_test::empty_string_slice::stable::release
    compile_test::fn_ptr::stable::debug
    compile_test::fn_ptr::stable::release
    compile_test::fold::stable::debug
    compile_test::fold::stable::release
    compile_test::fuzz100::stable::debug
    compile_test::fuzz16::stable::debug
    compile_test::fuzz16::stable::release
    compile_test::fuzz43::stable::debug
    compile_test::fuzz4::stable::debug
    compile_test::fuzz4::stable::release
    compile_test::fuzz67::stable::debug
    compile_test::fuzz67::stable::release
    compile_test::fuzz80::stable::debug
    compile_test::fuzz88::stable::debug
    compile_test::fuzz88::stable::release
    compile_test::fuzz94::stable::debug
    compile_test::fuzz94::stable::release
    compile_test::fuzz9::stable::debug
    compile_test::fuzz9::stable::release
    compile_test::identity::stable::debug
    compile_test::identity::stable::release
    compile_test::interop::stable::debug
    compile_test::interop::stable::release
    compile_test::mutithreading::stable::debug
    compile_test::mutithreading::stable::release
    compile_test::recursive::stable::debug
    compile_test::recursive::stable::release
    compile_test::references::stable::debug
    compile_test::references::stable::release
    compile_test::slice::stable::debug
    compile_test::slice::stable::release
    compile_test::slice_from_end::stable::debug
    compile_test::slice_from_end::stable::release
    compile_test::slice_index_ref::stable::debug
    compile_test::slice_index_ref::stable::release
    compile_test::tlocal_key_test::stable::debug
    compile_test::tlocal_key_test::stable::release
    compile_test::tuple::stable::debug
    compile_test::tuple::stable::release
    compile_test::type_id::stable::debug
    compile_test::type_id::stable::release
    compile_test::types::stable::debug
    compile_test::types::stable::release
    compile_test::vec::stable::debug
    compile_test::vec::stable::release

*/
/// Cached runtime configuration string, obtained from calling the .NET runtime.
#[must_use]
pub fn get_runtime_config() -> &'static str {
    RUNTIME_CONFIG.as_ref()
}
lazy_static! {
  /// Cached runtime configuration file, obtained from calling the .NET runtime.
  static ref RUNTIME_CONFIG: String = {
    let info = std::process::Command::new("dotnet")
        .arg("--info")
        .output()
        .expect("Could not run `dotnet --info`");
    if !info.stderr.is_empty() {
        let stderr = std::str::from_utf8(&info.stderr).expect("Error message not utf8");
        panic!("dotnet --info panicked with {stderr}")
    }
    let info = std::str::from_utf8(&info.stdout).expect("Error message not utf8");
    let version_start = info.find("Host:").unwrap_or_default();
    let version_start = version_start + info[version_start..].find("Version:").unwrap();
    let version_start = version_start + "Version:".len();
    let version_end = info.find("Architecture:").unwrap();
    let version = &info[version_start..version_end].trim();
    format!(
        "{{
        \"runtimeOptions\": {{
          \"tfm\": \"netcoreapp3.1\",
          \"framework\": {{
            \"name\": \"Microsoft.NETCore.App\",
            \"version\": \"{version}\"
          }},
          \"configProperties\": {{
            \"System.Threading.ThreadPool.MinThreads\": 4,
            \"System.Threading.ThreadPool.MaxThreads\": 25
          }}
        }}
      }}"
    )
    };
}