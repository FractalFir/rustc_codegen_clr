use crate::v2::MethodImpl;

use std::{io::Write, path::Path};

use super::{
    asm::{IlasmFlavour, ILASM_FLAVOUR, ILASM_PATH},
    cilnode::{ExtendKind, UnOp},
    cilroot::BranchCond,
    method::LocalDef,
    tpe::simd::SIMDElem,
    Assembly, BinOp, CILIter, CILIterElem, CILNode, ClassRefIdx, Exporter, Int, MethodDefIdx,
    NodeIdx, RootIdx, SigIdx, Type,
};

pub struct ILExporter {
    flavour: IlasmFlavour,
    is_lib: bool,
}
impl ILExporter {
    #[must_use]
    pub fn new(flavour: IlasmFlavour, is_lib: bool) -> Self {
        Self { flavour, is_lib }
    }

    fn export_to_write(&self, asm: &super::Assembly, out: &mut impl Write) -> std::io::Result<()> {
        let asm_mut = &mut asm.clone();
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
            let name = &asm[class_def.name()];
            writeln!(
                out,
                ".class {vis} ansi {sealed} {explicit} '{name}' extends {extends}{{"
            )?;
            // Export size
            if let Some(size) = class_def.explict_size() {
                writeln!(out, ".size {size}", size = size.get())?;
            }
            if let Some(align) = class_def.align() {
                writeln!(out, "//align {align}", align = align.get())?;
            }
            // Export all fields
            for (tpe, name, offset) in class_def.fields() {
                let name = &asm[*name];
                let tpe = non_void_type_il(tpe, asm);
                if let Some(offset) = offset {
                    writeln!(out, ".field [{offset}] {tpe} '{name}'")
                } else {
                    writeln!(out, ".field {tpe} '{name}'")
                }?;
            }
            crate::utilis::assert_unique(
                class_def.static_fields(),
                format!(
                    "The class {} contains a duplicate static field",
                    &asm[class_def.name()]
                ),
            );
            // Export all static fields
            for (tpe, name, thread_local) in class_def.static_fields() {
                let name = &asm[*name];
                let tpe = non_void_type_il(tpe, asm);

                writeln!(out, ".field static {tpe} '{name}'")?;
                if *thread_local {
                    writeln!(out,".custom instance void [System.Runtime]System.ThreadStaticAttribute::.ctor() = (01 00 00 00)")?;
                };
            }
            // Debug check
            let mut ensure_unqiue: std::collections::HashSet<MethodDefIdx> =
                std::collections::HashSet::new();
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
                    crate::v2::cilnode::MethodKind::Constructor => "rtspecialname specialname",
                };
                let pinvoke = if let MethodImpl::Extern {
                    lib,
                    preserve_errno,
                } = method.implementation()
                {
                    let lib = &asm[*lib];
                    if *preserve_errno {
                        format!("pinvokeimpl(\"{lib}\" cdecl lasterr)")
                    } else {
                        format!("pinvokeimpl(\"{lib}\" cdecl)")
                    }
                } else {
                    String::new()
                };
                let name = &asm[method.name()];
                let sig = &asm[method.sig()];
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
                            format!("{} '{}'", non_void_type_il(tpe, asm_mut), &asm_mut[*name])
                        }
                        None => non_void_type_il(tpe, asm_mut),
                    })
                    .intersperse(",".to_string())
                    .collect();
                let preservesig = if method.implementation().is_extern() {
                    "preservesig"
                } else {
                    ""
                };
                writeln!(
                    out,
                    ".method {vis} hidebysig {kind} {pinvoke} {ret} '{name}'({inputs}) cil managed {preservesig}{{// Method ID {method_id:?}"
                )?;
                debug_assert!(ensure_unqiue.insert(*method_id));
                let stack_size = match method.resolved_implementation(asm_mut) {
                    MethodImpl::MethodBody { blocks, .. } => blocks
                        .iter()
                        .flat_map(|block| block.roots().iter())
                        .map(|root| {
                            crate::v2::CILIter::new(asm_mut.get_root(*root).clone(), asm_mut)
                                .count()
                                + 10
                        })
                        .max()
                        .unwrap_or(0),
                    MethodImpl::Extern { .. } => 0,
                    MethodImpl::AliasFor(_) => todo!(),
                    MethodImpl::Missing => 3,
                };

                writeln!(out, ".maxstack {stack_size}")?;

                if *name == *"entrypoint" {
                    writeln!(out, ".entrypoint")?;
                }
                // Export the implementation
                let mimpl = method.resolved_implementation(asm_mut).clone();
                self.export_method_imp(asm_mut, out, &mimpl, name, method.sig())?;
                writeln!(out, "}}")?;
            }
            writeln!(out, "}}")?;
        }

        Ok(())
    }
    fn export_method_imp(
        &self,
        asm: &mut super::Assembly,
        out: &mut impl Write,
        mimpl: &MethodImpl,
        name: &str,
        sig: SigIdx,
    ) -> std::io::Result<()> {
        //assert_ne!(name,"stack_addr", "The builtin 'stack_addr' cilly function must always be inlined, and can't be exported otherwise.");
        match  mimpl{
            MethodImpl::MethodBody { blocks, locals } => {
                let locals_string:String = locals.iter().map(|(name,tpe)|match name {
                    Some(name) => {
                        format!("\n  {} '{}'", non_void_type_il(&asm[*tpe], asm), &asm[*name])
                    }
                    None => format!("\n  {}",non_void_type_il(&asm[*tpe], asm)),
                }).intersperse(",".to_owned()).collect();
                writeln!(out," .locals ({locals_string})")?;
                let mut blocks_iter = blocks.iter().peekable();
                //let mut is_in_multiblock_handler = false;
                while let Some(block) = blocks_iter.next(){
                    if block.handler().is_some() { //&& !is_in_multiblock_handler
                        writeln!(out,".try{{")?;
                    }
                    //DEBUG REMOVE THIS
                    writeln!(out,"// targets:{}",block.targets(asm).count())?;
                    writeln!(out," bb{}:",block.block_id())?;
                    for root in block.roots(){
                        self.export_root(asm,out,*root,false, block.handler().is_some(),sig,locals)?;
                    }
                    if let Some(handler) = block.handler(){
                        if Some(handler) == blocks_iter.peek().and_then(|block|block.handler()){
                            eprintln!("Multiblock handler candiate");
                        }
                        writeln!(out,"}} catch [System.Runtime]System.Object{{")?;
                        // Check for the GetException intrinsic. If it is not used, put a pop here.
                        if !handler.iter().flat_map(super::basic_block::BasicBlock::roots).flat_map(|root|CILIter::new(asm.get_root(*root).clone(),asm)).any(|elem|matches!(elem,CILIterElem::Node(CILNode::GetException))){
                            writeln!(out,"pop")?;
                        }
                        for hblock in handler{
                            writeln!(out," h{}_{}:",block.block_id(),hblock.block_id())?;
                            for root in hblock.roots(){
                                self.export_root(asm,out,*root,true,false,sig,locals)?;
                            }
                        }
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
    #[allow(clippy::only_used_in_recursion)] // Futrue proffing. The IL exporter will need this in the future.
    fn export_node(
        &self,
        asm: &mut super::Assembly,
        out: &mut impl Write,
        node: NodeIdx,
        sig: SigIdx,
        locals: &[LocalDef],
    ) -> std::io::Result<()> {
        let node = asm.get_node(node).clone();
        match node {
            CILNode::Const(cst) => match cst.as_ref() {
                super::Const::Null(_) => writeln!(out, "ldnull"),
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
                super::Const::I128(val) => match val {
                    -1 => writeln!(out, "ldc.i4.m1 call valuetype [System.Runtime]System.Int128 [System.Runtime]System.Int128::op_Implicit(int32)"),
                    0..=8 => writeln!(out, "ldc.i4.{val} call valuetype [System.Runtime]System.Int128 [System.Runtime]System.Int128::op_Implicit(int32)"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} call valuetype [System.Runtime]System.Int128 [System.Runtime]System.Int128::op_Implicit(int32)"),
                    -2_147_483_648i128..0 | 128..=2_147_483_647i128 => {
                        writeln!(out, "ldc.i4 {val} call valuetype [System.Runtime]System.Int128 [System.Runtime]System.Int128::op_Implicit(int32)")
                    }
                    -9_223_372_036_854_775_808_i128..-2_147_483_648i128 | 2_147_483_648i128..=9_223_372_036_854_775_807i128 => {
                        writeln!(out, "ldc.i8 {val} call valuetype [System.Runtime]System.Int128 [System.Runtime]System.Int128::op_Implicit(int64)")
                    }
                    _ => {
                        let low =  u64::try_from((*val as u128) & u128::from(u64::MAX)).expect("trucating cast error");
                        let high = ((*val as u128) >> 64) as u64;
                        writeln!(out, "ldc.i8 {high} ldc.i8 {low} newobj instance void valuetype [System.Runtime]System.Int128::.ctor(uint64,uint64)")
                    },
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
                    128..=4_294_967_295u64 => writeln!(out, "ldc.i4 {val} conv.u8"),
                    _ => writeln!(out, "ldc.i8 {val}"),
                },
                super::Const::USize(val) => match val {
                    0..=8 => writeln!(out, "ldc.i4.{val} conv.u"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} conv.u"),
                    128..=2_147_483_647u64 => writeln!(out, "ldc.i4 {val} conv.u"),
                    _ => writeln!(out, "ldc.i8 {val} conv.u"),
                },
                super::Const::U128(val)=>match val {
                    0..=8 => writeln!(out, "ldc.i4.{val} call valuetype [System.Runtime]System.UInt128 [System.Runtime]System.UInt128::op_Implicit(uint32)"),
                    9..=127 => writeln!(out, "ldc.i4.s {val} call valuetype [System.Runtime]System.UInt128 [System.Runtime]System.UInt128::op_Implicit(uint32)"),
                    128..=4_294_967_295u128 => writeln!(out, "ldc.i4 {val} call valuetype [System.Runtime]System.UInt128 [System.Runtime]System.UInt128::op_Implicit(uint32)"),
                    4_294_967_296u128..=18_446_744_073_709_551_615u128 => writeln!(out, "ldc.i8 {val} call valuetype [System.Runtime]System.UInt128 [System.Runtime]System.UInt128::op_Implicit(uint64)"),
                    _ => {
                        let low =  u64::try_from({ *val } & u128::from(u64::MAX)).expect("trucating cast error");
                        let high = ({ *val } >> 64) as u64;
                        writeln!(out, "ldc.i8 {high} ldc.i8 {low} newobj instance void valuetype [System.Runtime]System.UInt128::.ctor(uint64,uint64)")
                    },
                }
                super::Const::PlatformString(msg) => {
                    let msg = &asm[*msg];
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
                    let const_literal = float.to_le_bytes();
                    writeln!(
                        out,
                        "ldc.r4 ({:02x} {:02x} {:02x} {:02x})",
                        const_literal[0], const_literal[1], const_literal[2], const_literal[3]
                    )
                }
                super::Const::F64(float) => {
                    let const_literal = float.to_le_bytes();
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
            CILNode::BinOp(lhs, rhs, op) => {
                self.export_node(asm, out, lhs, sig, locals)?;
                self.export_node(asm, out, rhs, sig, locals)?;
                match op {
                    BinOp::Add => writeln!(out, "add"),
                    BinOp::Eq => writeln!(out, "ceq"),
                    BinOp::Sub => writeln!(out, "sub"),
                    BinOp::Mul => writeln!(out, "mul"),
                    BinOp::LtUn => writeln!(out, "clt.un"),
                    BinOp::Lt => writeln!(out, "clt"),
                    BinOp::GtUn => writeln!(out, "cgt.un"),
                    BinOp::Gt => writeln!(out, "cgt"),
                    BinOp::Or => writeln!(out, "or"),
                    BinOp::XOr => writeln!(out, "xor"),
                    BinOp::And => writeln!(out, "and"),
                    BinOp::Rem => writeln!(out, "rem"),
                    BinOp::RemUn => writeln!(out, "rem.un"),
                    BinOp::Shl => writeln!(out, "shl"),
                    BinOp::Shr => writeln!(out, "shr"),
                    BinOp::ShrUn => writeln!(out, "shr.un"),
                    BinOp::DivUn => writeln!(out, "div.un"),
                    BinOp::Div => writeln!(out, "div"),
                }
            }
            CILNode::UnOp(arg, un) => {
                self.export_node(asm, out, arg, sig, locals)?;
                match un {
                    UnOp::Not => writeln!(out, "not"),
                    UnOp::Neg => writeln!(out, "neg"),
                }
            }
            CILNode::LdLoc(loc) => match loc {
                0..=3 => writeln!(out, "ldloc.{loc}"),
                4..=255 => writeln!(out, "ldloc.s {loc}"),
                _ => writeln!(out, "ldloc {loc}"),
            },
            CILNode::LdLocA(arg) => match arg {
                0..=255 => writeln!(out, "ldloca.s {arg}"),
                _ => writeln!(out, "ldloca {arg}"),
            },
            CILNode::LdArg(arg) => match arg {
                0..=3 => writeln!(out, "ldarg.{arg}"),
                4..=255 => writeln!(out, "ldarg.s {arg}"),
                _ => writeln!(out, "ldarg {arg}"),
            },
            CILNode::LdArgA(arg) => match arg {
                0..=255 => writeln!(out, "ldarga.s {arg}"),
                _ => writeln!(out, "ldarga {arg}"),
            },
            CILNode::Call(call) => {
                for arg in &call.1 {
                    self.export_node(asm, out, *arg, sig, locals)?;
                }
                let mref = &asm[call.0];
                let call_op = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => "call",
                    crate::v2::cilnode::MethodKind::Instance => "call instance",
                    crate::v2::cilnode::MethodKind::Virtual => " callvirt instance",
                    crate::v2::cilnode::MethodKind::Constructor => "newobj instance",
                };
                let sig = &asm[mref.sig()];
                let output = type_il(sig.output(), asm);
                let inputs = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => sig.inputs(),
                    crate::v2::cilnode::MethodKind::Instance
                    | crate::v2::cilnode::MethodKind::Virtual
                    | crate::v2::cilnode::MethodKind::Constructor => {
                        assert!(
                            !sig.inputs().is_empty(),
                            "invalid argc when calling {} of {}",
                            &asm[mref.name()],
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
                let generic = if mref.generics().is_empty() {
                    "".to_string()
                } else {
                    let generic_list: String = mref
                        .generics()
                        .iter()
                        .map(|tpe| type_il(tpe, asm))
                        .intersperse(",".to_owned())
                        .collect();
                    format!("<{generic_list}>")
                };
                let name = &asm[mref.name()];
                let class = class_ref(mref.class(), asm);
                writeln!(
                    out,
                    "{call_op} {output} {class}::'{name}'{generic}({inputs})"
                )
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                self.export_node(asm, out, input, sig, locals)?;
                match (target, extend) {
                    (super::Int::U8 | super::Int::I8, ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u1")
                    }
                    (super::Int::U8 | super::Int::I8, ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i1")
                    }
                    (super::Int::U16 | super::Int::I16, ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u2")
                    }
                    (super::Int::U16 | super::Int::I16, ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i2")
                    }
                    (super::Int::U32 | super::Int::I32, ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u4")
                    }
                    (super::Int::U32 | super::Int::I32, ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i4")
                    }

                    (super::Int::U64 | super::Int::I64, ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u8")
                    }
                    (super::Int::U64 | super::Int::I64, ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i8")
                    }
                    (super::Int::USize | super::Int::ISize, ExtendKind::SignExtend) => {
                        writeln!(out, "conv.i")
                    }
                    (super::Int::USize | super::Int::ISize, ExtendKind::ZeroExtend) => {
                        writeln!(out, "conv.u")
                    }
                    (super::Int::U128, ExtendKind::ZeroExtend) => todo!(),
                    (super::Int::U128, ExtendKind::SignExtend) => todo!(),
                    (super::Int::I128, ExtendKind::ZeroExtend) => todo!(),
                    (super::Int::I128, ExtendKind::SignExtend) => todo!(),
                }
            }
            CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => {
                self.export_node(asm, out, input, sig, locals)?;
                match (target, is_signed) {
                    (super::Float::F16, true) => todo!(),
                    (super::Float::F16, false) => todo!(),
                    (super::Float::F32, true) => writeln!(out, "conv.r4"),
                    (super::Float::F32, false) => writeln!(out, "conv.r.un conv.r4"),
                    (super::Float::F64, true) => writeln!(out, "conv.r8"),
                    (super::Float::F64, false) => writeln!(out, "conv.r.un conv.r8"),
                    (super::Float::F128, true) => todo!(),
                    (super::Float::F128, false) => todo!(),
                }
            }
            CILNode::RefToPtr(inner) => {
                self.export_node(asm, out, inner, sig, locals)?;
                writeln!(out, "conv.u//rtp")
            }
            CILNode::PtrCast(val, _) => self.export_node(asm, out, val, sig, locals),
            CILNode::LdFieldAdress { addr, field } => {
                self.export_node(asm, out, addr, sig, locals)?;
                let fld = asm.get_field(field);
                let owner = class_ref(fld.owner(), asm);
                let name = &asm[fld.name()];
                let tpe = type_il(&fld.tpe(), asm);
                writeln!(out, "ldflda {tpe} {owner}::'{name}'")
            }
            CILNode::LdField { addr, field } => {
                self.export_node(asm, out, addr, sig, locals)?;
                let fld = asm.get_field(field);
                let owner = class_ref(fld.owner(), asm);
                let name = &asm[fld.name()];
                let tpe = type_il(&fld.tpe(), asm);
                writeln!(out, "ldfld {tpe} {owner}::'{name}'")
            }
            CILNode::LdInd {
                addr,
                tpe,
                volatile: volitale,
            } => {
                self.export_node(asm, out, addr, sig, locals)?;
                let tpe = asm[tpe];

                match (tpe, volitale) {
                    (Type::Ptr(_), true) => writeln!(out, "volatile. ldind.i"),
                    (Type::Ptr(_), false) => writeln!(out, "ldind.i"),
                    (Type::Ref(_), true) => todo!(),
                    (Type::Ref(_), false) => todo!(),
                    (Type::Int(int), volitale) => match (int, volitale) {
                        (Int::U8, true) => writeln!(out, "volatile. ldind.u1"),
                        (Int::U8, false) => writeln!(out, "ldind.u1"),
                        (Int::U16, true) => writeln!(out, "volatile. ldind.u2"),
                        (Int::U16, false) => writeln!(out, "ldind.u2"),
                        (Int::U32, true) => writeln!(out, "volatile. ldind.u4"),
                        (Int::U32, false) => writeln!(out, "ldind.u4"),
                        (Int::U64, true) => writeln!(out, "volatile. ldind.u8"),
                        (Int::U64, false) => writeln!(out, "ldind.u8"),
                        (Int::U128, true) => writeln!(
                            out,
                            "volatile. ldobj valuetype [System.Runtime]System.UInt128"
                        ),
                        (Int::U128, false) => {
                            writeln!(out, "ldobj valuetype [System.Runtime]System.UInt128")
                        }
                        (Int::USize, true) => writeln!(out, "volatile. ldind.i"),
                        (Int::USize, false) => writeln!(out, "ldind.i"),
                        (Int::I8, true) => writeln!(out, "volatile. ldind.i1"),
                        (Int::I8, false) => writeln!(out, "ldind.i1"),
                        (Int::I16, true) => writeln!(out, "volatile. ldind.i2"),
                        (Int::I16, false) => writeln!(out, "ldind.i2"),
                        (Int::I32, true) => writeln!(out, "volatile. ldind.i4"),
                        (Int::I32, false) => writeln!(out, "ldind.i4"),
                        (Int::I64, true) => writeln!(out, "volatile. ldind.i8"),
                        (Int::I64, false) => writeln!(out, "ldind.i8"),
                        (Int::I128, true) => writeln!(
                            out,
                            "volatile. ldobj valuetype [System.Runtime]System.Int128"
                        ),
                        (Int::I128, false) => {
                            writeln!(
                                out,
                                "ldobj valuetype [System.Runtime]System.Int128"
                            )
                        }
                        (Int::ISize, true) => writeln!(out, "volatile. ldind.i"),
                        (Int::ISize, false) => writeln!(out, "ldind.i"),
                    },
                    (Type::ClassRef(cref), true) => {
                        writeln!(out, "volatile. ldobj {cref}", cref = class_ref(cref, asm))
                    }
                    (Type::ClassRef(cref), false) => {
                        writeln!(out, "ldobj {cref}", cref = class_ref(cref, asm))
                    }
                    (Type::Float(float), volitale) => match (float, volitale) {
                        (super::Float::F16, true) => {
                            writeln!(out, "volatile. ldobj [System.Runtime]System.Half")
                        }
                        (super::Float::F16, false) => {
                            writeln!(out, "ldobj [System.Runtime]System.Half")
                        }
                        (super::Float::F32, true) => writeln!(out, "volatile. ldind.r4"),
                        (super::Float::F32, false) => writeln!(out, "ldind.r4"),
                        (super::Float::F64, true) => writeln!(out, "volatile. ldind.r8"),
                        (super::Float::F64, false) => writeln!(out, "ldind.r8"),
                        (super::Float::F128, true) => {
                            writeln!(out, "volatile. ldobj {}", type_il(&tpe, asm))
                        }
                        (super::Float::F128, false) => {
                            writeln!(out, "ldobj {}", type_il(&tpe, asm))
                        }
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
                    (Type::Void, true | false) => {
                        panic!("Void can't be dereferenced!")
                    }
                    (Type::PlatformArray { .. }, true) => writeln!(out, "volatile. ldind.ref"),
                    (Type::PlatformArray { .. }, false) => writeln!(out, "ldind.ref"),
                    (Type::FnPtr(_), true) => writeln!(out, "volatile. ldind.i"),
                    (Type::FnPtr(_), false) => writeln!(out, "ldind.i"),
                    (Type::SIMDVector(_), true) => {
                        writeln!(out, "volatile. ldobj {}", type_il(&tpe, asm))
                    }
                    (Type::SIMDVector(_), false) => {
                        writeln!(out, "ldobj {}", type_il(&tpe, asm))
                    }
                }
            }
            CILNode::SizeOf(tpe) => {
                let tpe = asm[tpe];
                if tpe == Type::Void{
                    eprintln!("WARNING: attempted to calc size_of(void). This is UB: not all targets support ZSTs. Please use Const::I32(0) instead. Continuing anyway.");
                    writeln!(out, "ldc.i4.0")
                }
                else{
                    writeln!(out, "sizeof {}", type_il(&tpe, asm))
                }
            }
            CILNode::GetException => Ok(()),
            CILNode::IsInst(val, tpe) => {
                self.export_node(asm, out, val, sig, locals)?;
                writeln!(out, "isinst {tpe}", tpe = type_il(&asm[tpe], asm))
            }
            CILNode::CheckedCast(val, tpe) => {
                self.export_node(asm, out, val, sig, locals)?;
                writeln!(out, "castclass {tpe}", tpe = type_il(&asm[tpe], asm))
            }
            CILNode::CallI(calli) => {
                let (fn_ptr, fn_sig, args) = calli.as_ref();
                for arg in args {
                    self.export_node(asm, out, *arg, sig, locals)?;
                }
                let fn_sig = asm[*fn_sig].clone();
                let output = type_il(fn_sig.output(), asm);
                self.export_node(asm, out, *fn_ptr, sig, locals)?;
                let inputs: String = fn_sig
                    .inputs()
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_owned())
                    .collect();
                writeln!(out, "calli {output} ({inputs})")
            }
            CILNode::LocAlloc { size } => {
                self.export_node(asm, out, size, sig, locals)?;
                writeln!(out, "localloc")
            }
            CILNode::LdStaticField(sfld) => {
                let sfld = asm.get_static_field(sfld);
                let owner = class_ref(sfld.owner(), asm);
                let name = &asm[sfld.name()];
                let tpe = type_il(&sfld.tpe(), asm);
                writeln!(out, "ldsfld {tpe} {owner}::{name}")
            }
            CILNode::LdStaticFieldAdress(sfld) => {
                let sfld = asm.get_static_field(sfld);
                let owner = class_ref(sfld.owner(), asm);
                let name = &asm[sfld.name()];
                let tpe = type_il(&sfld.tpe(), asm);
                writeln!(out, "ldsflda {tpe} {owner}::{name}")
            }
            CILNode::LdFtn(ftn) => {
                let mref = &asm[ftn];
                let sig = &asm[mref.sig()];
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
                let name = &asm[mref.name()];
                let class = class_ref(mref.class(), asm);
                let ldftn_op = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => "ldftn",
                    crate::v2::cilnode::MethodKind::Instance => "ldftn instance",
                    crate::v2::cilnode::MethodKind::Virtual => " ldftn instance",
                    crate::v2::cilnode::MethodKind::Constructor => "ldftn instance",
                };
                writeln!(
                    out,
                    "{ldftn_op} {output} {class}::'{name}'({inputs}) //{ftn:?}"
                )
            }
            CILNode::LdTypeToken(tok) => {
                writeln!(out, "ldtoken {tok}", tok = type_il(&asm[tok], asm))
            }
            CILNode::LdLen(array) => {
                self.export_node(asm, out, array, sig, locals)?;
                writeln!(out, "ldlen")
            }
            CILNode::LocAllocAlgined { tpe, align } => {
                writeln!(out, "sizeof {tpe} ldc.i8 {align} conv.i add localloc dup ldc.i8 {align} add ldc.i8 {align} rem sub ldc.i8 {align} add conv.u", tpe = type_il(&asm[tpe], asm))
            }
            CILNode::LdElelemRef { array, index } => {
                self.export_node(asm, out, array, sig, locals)?;
                self.export_node(asm, out, index, sig, locals)?;
                writeln!(out, "ldelem.ref")
            }
            CILNode::UnboxAny { object, tpe } => {
                self.export_node(asm, out, object, sig, locals)?;
                writeln!(out, "unbox.any {object}", object = type_il(&asm[tpe], asm))
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn export_root(
        &self,
        asm: &mut super::Assembly,
        out: &mut impl Write,
        root: RootIdx,
        is_handler: bool,
        has_handler: bool,
        sig: SigIdx,
        locals: &[LocalDef],
    ) -> std::io::Result<()> {
        let root = asm.get_root(root).clone();
        match root {
            super::CILRoot::StLoc(loc, val) => {
                self.export_node(asm, out, val, sig, locals)?;
                match loc {
                    0..=3 => writeln!(out, "stloc.{loc}"),
                    4..=255 => writeln!(out, "stloc.s {loc}"),
                    _ => writeln!(out, "stloc {loc}"),
                }
            }
            super::CILRoot::StArg(loc, val) => {
                self.export_node(asm, out, val, sig, locals)?;
                match loc {
                    0..=255 => writeln!(out, "starg.s {loc}"),
                    _ => writeln!(out, "starg {loc}"),
                }
            }
            super::CILRoot::Ret(val) => {
                self.export_node(asm, out, val, sig, locals)?;
                writeln!(out, "ret")
            }
            super::CILRoot::Pop(val) => {
                self.export_node(asm, out, val, sig, locals)?;
                writeln!(out, "pop")
            }
            super::CILRoot::Throw(val) => {
                self.export_node(asm, out, val, sig, locals)?;
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
                    self.export_node(asm, out, *a, sig, locals)?;
                    self.export_node(asm, out, *b, sig, locals)?;
                    if branch.1 == 0 {
                        writeln!(out, "beq bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "beq h{}_{}", branch.0, branch.1)
                    } else if has_handler {
                        writeln!(out, "beq jp{}_{}", branch.0, branch.1)
                    }
                    // If the handler was removed, we can just blt.un
                    else {
                        writeln!(out, "beq bb{}", branch.1)
                    }
                }
                Some(BranchCond::Ne(a, b)) => {
                    self.export_node(asm, out, *a, sig, locals)?;
                    self.export_node(asm, out, *b, sig, locals)?;
                    if branch.1 == 0 {
                        writeln!(out, "bne.un bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "bne.un h{}_{}", branch.0, branch.1)
                    } else if has_handler {
                        writeln!(out, "bne.un jp{}_{}", branch.0, branch.1)
                    }
                    // If the handler was removed, we can just blt.un
                    else {
                        writeln!(out, "bne.un bb{}", branch.1)
                    }
                }
                Some(BranchCond::Lt(a, b, kind)) => {
                    self.export_node(asm, out, *a, sig, locals)?;
                    self.export_node(asm, out, *b, sig, locals)?;
                    match kind {
                        super::cilroot::CmpKind::Ordered | super::cilroot::CmpKind::Signed => {
                            if branch.1 == 0 {
                                writeln!(out, "blt bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "blt h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "blt jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just blt
                            else {
                                writeln!(out, "blt bb{}", branch.1)
                            }
                        }
                        super::cilroot::CmpKind::Unordered | super::cilroot::CmpKind::Unsigned => {
                            if branch.1 == 0 {
                                writeln!(out, "blt.un bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "blt.un h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "blt.un jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just blt.un
                            else {
                                writeln!(out, "blt.un bb{}", branch.1)
                            }
                        }
                    }
                }
                Some(BranchCond::Gt(a, b, kind)) => {
                    self.export_node(asm, out, *a, sig, locals)?;
                    self.export_node(asm, out, *b, sig, locals)?;
                    match kind {
                        super::cilroot::CmpKind::Ordered | super::cilroot::CmpKind::Signed => {
                            if branch.1 == 0 {
                                writeln!(out, "bgt bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "bgt h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "bgt jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just bgt
                            else {
                                writeln!(out, "bgt bb{}", branch.1)
                            }
                        }
                        super::cilroot::CmpKind::Unordered | super::cilroot::CmpKind::Unsigned => {
                            if branch.1 == 0 {
                                writeln!(out, "bgt.un bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "bgt.un h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "bgt.un jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just bgt.un
                            else {
                                writeln!(out, "bgt.un bb{}", branch.1)
                            }
                        }
                    }
                }
                Some(BranchCond::Le(a, b, kind)) => {
                    self.export_node(asm, out, *a, sig, locals)?;
                    self.export_node(asm, out, *b, sig, locals)?;
                    match kind {
                        super::cilroot::CmpKind::Ordered | super::cilroot::CmpKind::Signed => {
                            if branch.1 == 0 {
                                writeln!(out, "ble bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "ble h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "ble jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just ble
                            else {
                                writeln!(out, "ble bb{}", branch.1)
                            }
                        }
                        super::cilroot::CmpKind::Unordered | super::cilroot::CmpKind::Unsigned => {
                            if branch.1 == 0 {
                                writeln!(out, "ble.un bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "ble.un h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "ble.un jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just ble.un
                            else {
                                writeln!(out, "ble.un bb{}", branch.1)
                            }
                        }
                    }
                }
                Some(BranchCond::Ge(a, b, kind)) => {
                    self.export_node(asm, out, *a, sig, locals)?;
                    self.export_node(asm, out, *b, sig, locals)?;
                    match kind {
                        super::cilroot::CmpKind::Ordered | super::cilroot::CmpKind::Signed => {
                            if branch.1 == 0 {
                                writeln!(out, "bge bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "bge h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "bge jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just bge
                            else {
                                writeln!(out, "bge bb{}", branch.1)
                            }
                        }
                        super::cilroot::CmpKind::Unordered | super::cilroot::CmpKind::Unsigned => {
                            if branch.1 == 0 {
                                writeln!(out, "bge.un bb{}", branch.0)
                            } else if is_handler {
                                writeln!(out, "bge.un h{}_{}", branch.0, branch.1)
                            } else if has_handler {
                                writeln!(out, "bge.un jp{}_{}", branch.0, branch.1)
                            }
                            // If the handler was removed, we can just bge.un
                            else {
                                writeln!(out, "bge.un bb{}", branch.1)
                            }
                        }
                    }
                }
                Some(BranchCond::True(cond)) => {
                    self.export_node(asm, out, *cond, sig, locals)?;
                    if branch.1 == 0 {
                        writeln!(out, "brtrue bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "brtrue h{}_{}", branch.0, branch.1)
                    } else if has_handler {
                        writeln!(out, "brtrue jp{}_{}", branch.0, branch.1)
                    }
                    // If the handler was removed, we can just brtrue
                    else {
                        writeln!(out, "brtrue bb{}", branch.1)
                    }
                }
                Some(BranchCond::False(cond)) => {
                    self.export_node(asm, out, *cond, sig, locals)?;
                    if branch.1 == 0 {
                        writeln!(out, "brfalse bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "brfalse h{}_{}", branch.0, branch.1)
                    } else if has_handler {
                        writeln!(out, "brfalse jp{}_{}", branch.0, branch.1)
                    }
                    // If the handler was removed, we can just brfalse
                    else {
                        writeln!(out, "brfalse bb{}", branch.1)
                    }
                }
                None => {
                    if branch.1 == 0 {
                        writeln!(out, "br bb{}", branch.0)
                    } else if is_handler {
                        writeln!(out, "br h{}_{}", branch.0, branch.1)
                    }
                    // If it is not a handler, then this is the only block in this try, then all jumps are extern, then we can just use leave
                    else if has_handler {
                        writeln!(out, "leave bb{}", branch.1)
                    }
                    // If the handler was removed, we can just br
                    else {
                        writeln!(out, "br bb{}", branch.1)
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
                let col_end = u32::from(col_start) + u32::from(col_len);
                let line_end = line_start + u32::from(line_len);
                let file = &asm[file];
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
                self.export_node(asm, out, flds.1, sig, locals)?;
                self.export_node(asm, out, flds.2, sig, locals)?;
                let fld = asm.get_field(flds.0);
                let owner = class_ref(fld.owner(), asm);
                let name = &asm[fld.name()];
                let tpe = type_il(&fld.tpe(), asm);
                writeln!(out, "stfld {tpe} {owner}::'{name}'")
            }
            super::CILRoot::Call(call) => {
                for arg in &call.1 {
                    self.export_node(asm, out, *arg, sig, locals)?;
                }
                let mref = &asm[call.0];
                let call_op = match mref.kind() {
                    crate::v2::cilnode::MethodKind::Static => "call",
                    crate::v2::cilnode::MethodKind::Instance => "call instance",
                    crate::v2::cilnode::MethodKind::Virtual => " callvirt instance",
                    crate::v2::cilnode::MethodKind::Constructor => {
                        panic!("A constructor can't be a CIL root")
                    }
                };
                let sig = &asm[mref.sig()];
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
                let name = &asm[mref.name()];
                let class = class_ref(mref.class(), asm);

                writeln!(
                    out,
                    "{call_op} {output} {class}::'{name}'({inputs}) //mref:{:?}",
                    call.0
                )
            }
            super::CILRoot::CpObj { src, dst, tpe } => {
                self.export_node(asm, out, src, sig, locals)?;
                self.export_node(asm, out, dst, sig, locals)?;
                let tpe = type_il(&asm[tpe], asm);
                writeln!(out, "cpobj {tpe}")
            }
            super::CILRoot::StInd(stind) => {
                self.export_node(asm, out, stind.0, sig, locals)?;
                self.export_node(asm, out, stind.1, sig, locals)?;

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
                        super::Float::F128 => writeln!(out, "stobj {}", type_il(&tpe, asm)),
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
                    Type::SIMDVector(_)=>writeln!(out, "stobj {}", type_il(&tpe, asm)),
                }
            }
            super::CILRoot::InitBlk(blk) => {
                self.export_node(asm, out, blk.0, sig, locals)?;
                self.export_node(asm, out, blk.1, sig, locals)?;
                self.export_node(asm, out, blk.2, sig, locals)?;
                writeln!(out, "initblk")
            }
            super::CILRoot::CpBlk(cpblk) => {
                self.export_node(asm, out, cpblk.0, sig, locals)?;
                self.export_node(asm, out, cpblk.1, sig, locals)?;
                self.export_node(asm, out, cpblk.2, sig, locals)?;
                writeln!(out, "cpblk")
            }
            super::CILRoot::CallI(calli) => {
                let (fn_ptr, fn_sig, args) = calli.as_ref();
                for arg in args {
                    self.export_node(asm, out, *arg, sig, locals)?;
                }
                let fn_sig = asm[*fn_sig].clone();
                let output = type_il(fn_sig.output(), asm);
                self.export_node(asm, out, *fn_ptr, sig, locals)?;
                let inputs: String = fn_sig
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
                } else if has_handler {
                    writeln!(out, "jp{source}_{target}: leave bb{target}")
                } else {
                    Ok(())
                }
            }
            super::CILRoot::ReThrow => {
                writeln!(out, "rethrow")
            }
            super::CILRoot::SetStaticField { field, val } => {
                self.export_node(asm, out, val, sig, locals)?;
                let sfld = asm[field];
                let owner = class_ref(sfld.owner(), asm);
                let name = &asm[sfld.name()];
                let tpe = type_il(&sfld.tpe(), asm);
                writeln!(out, "stsfld {tpe} {owner}::{name}")
            }
            super::CILRoot::Unreachable(msg) => {
                writeln!(
                    out,
                    "ldstr {:?} newobj void [System.Runtime]System.Exception::.ctor(string) throw",
                    &asm[msg]
                )
            }
        }
    }
}
#[cfg(not(target_os = "windows"))]
fn assemble_file(exe_out: &Path, il_path: &Path, is_lib: bool) {
    let asm_type = if is_lib { "-dll" } else { "-exe" };
    let mut cmd = std::process::Command::new(ILASM_PATH.clone());
    cmd.arg(il_path)
    .arg(format!("-output:{exe_out}", exe_out = exe_out.to_string_lossy()))
    .arg("-debug")
    .arg("-OPTIMIZE")
    .arg(asm_type)
    // .arg("-FOLD") saves up on space, consider enabling.
    ;
    if *ILASM_FLAVOUR == IlasmFlavour::Clasic {
        // Limit the memory usage of mono
        cmd.env("MONO_GC_PARAMS", "soft-heap-limit=500m");
    }
    let out = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !(stderr.contains("\nError\n") || stderr.contains("FAILURE") || stdout.contains("FAILURE")),
        "stdout:{} stderr:{} cmd:{cmd:?}",
        stdout,
        String::from_utf8_lossy(&out.stderr)
    );
}
#[cfg(target_os = "windows")]
fn assemble_file(exe_out: &Path, il_path: &Path, is_lib: bool) {
    let asm_type = if is_lib { "-dll" } else { "-exe" };
    let mut cmd = std::process::Command::new(ILASM_PATH.clone());
    cmd.arg(il_path)
    .arg(format!("-output:{exe_out}", exe_out = exe_out.to_string_lossy()))
    .arg("-OPTIMIZE")
    .arg(asm_type)
    // .arg("-FOLD") saves up on space, consider enabling.
    ;
    if *ILASM_FLAVOUR == IlasmFlavour::Clasic {
        // Limit the memory usage of mono
        cmd.env("MONO_GC_PARAMS", "soft-heap-limit=500m");
    }
    let out = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !(stderr.contains("\nError\n") || stderr.contains("FAILURE") || stdout.contains("FAILURE")),
        "stdout:{} stderr:{} cmd:{cmd:?}",
        stdout,
        String::from_utf8_lossy(&out.stderr)
    );
    let asm_type = if is_lib { "-dll" } else { "-exe" };
    let mut cmd = std::process::Command::new(ILASM_PATH.clone());
    cmd.arg(il_path)
    .arg(format!("-output:{exe_out}", exe_out = exe_out.to_string_lossy()))
    .arg("-debug")
    .arg("-OPTIMIZE")
    .arg(asm_type)
    // .arg("-FOLD") saves up on space, consider enabling.
    ;
    if *ILASM_FLAVOUR == IlasmFlavour::Clasic {
        // Limit the memory usage of mono
        cmd.env("MONO_GC_PARAMS", "soft-heap-limit=500m");
    }
    let out = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !(stderr.contains("\nError\n") || stderr.contains("FAILURE") || stdout.contains("FAILURE")),
        "stdout:{} stderr:{} cmd:{cmd:?}",
        stdout,
        String::from_utf8_lossy(&out.stderr)
    );
}
impl Exporter for ILExporter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        // The IL file should be next to the target
        let il_path = target.with_extension("il");

        if let Err(err) = std::fs::remove_file(&il_path) {
            match err.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => {
                    panic!("Could not remove tmp file because {err:?}")
                }
            }
        };
        let mut il_out = std::io::BufWriter::new(std::fs::File::create(&il_path)?);
        self.export_to_write(asm, &mut il_out)?;
        // Needed to ensure the IL file is valid!
        il_out.flush().unwrap();
        drop(il_out);
        let exe_out = std::path::absolute(target.with_extension("exe")).unwrap();
        if let Err(err) = std::fs::remove_file(&exe_out) {
            match err.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => {
                    panic!("Could not remove tmp file because {err:?}")
                }
            }
        };
        assemble_file(&exe_out, &il_path, self.is_lib);

        Ok(())
    }
}
fn simple_class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = &asm[cref.name()];
    if let Some(assembly) = cref.asm() {
        format!("[{assembly}]'{name}'", assembly = &asm[assembly])
    } else {
        format!("'{name}'")
    }
}
pub(crate) fn class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = &asm[cref.name()];
    let prefix = if cref.is_valuetype() {
        "valuetype"
    } else {
        "class"
    };
    let generic_list = if cref.generics().is_empty() {
        String::new()
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
        String::new()
    } else {
        format!("`{}", cref.generics().len())
    };
    if let Some(assembly) = cref.asm() {
        format!(
            "{prefix} [{assembly}]'{name}{generic_postfix}'{generic_list}",
            assembly = &asm[assembly]
        )
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
        Type::SIMDVector(simdvec) => {
            let vec_bits = simdvec.bits();
            assert!(
                vec_bits == 64 || vec_bits == 128 || vec_bits == 256 || vec_bits == 512,
                "Unusported SIMD vector size"
            );
            let elem = match simdvec.elem() {
                SIMDElem::Int(int) => type_il(&Type::Int(int), asm),
                SIMDElem::Float(float) => type_il(&Type::Float(float), asm),
            };
            format!("valuetype [System.Runtime.Intrinsics]System.Runtime.Intrinsics.Vector{vec_bits}`1<{elem}>")
        }
        Type::Ptr(inner) => format!("{}*", type_il(&asm[*inner], asm)),
        Type::Ref(inner) => format!("{}&", type_il(&asm[*inner], asm)),
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
            super::Float::F16 => "valuetype [System.Runtime]System.Half".into(),
            super::Float::F32 => "float32".into(),
            super::Float::F64 => "float64".into(),

            super::Float::F128 => "valuetype f128".into(),
        },
        Type::PlatformChar => "char".into(),
        Type::PlatformGeneric(arg, generic) => match generic {
            super::tpe::GenericKind::MethodGeneric => format!("!{arg}"),
            super::tpe::GenericKind::CallGeneric => format!("!!{arg}"),
            super::tpe::GenericKind::TypeGeneric => format!("!{arg}"),
        },
        Type::Bool => "bool".into(),
        Type::Void => "void".into(),
        Type::PlatformArray { elem, dims } => format!(
            "{elem}[{dims}]",
            elem = type_il(&asm[*elem], asm),
            dims = (1..(dims.get())).map(|_| ',').collect::<String>()
        ),
        Type::FnPtr(sig) => {
            let sig = asm[*sig].clone();
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

/// Cached runtime configuration string, obtained from calling the .NET runtime.
#[must_use]
pub fn get_runtime_config() -> &'static str {
    RUNTIME_CONFIG.as_ref()
}

/// Cached runtime configuration file, obtained from calling the .NET runtime.
static RUNTIME_CONFIG: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
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
          \"tfm\": \"net8.0\",
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
});
