// This exporter is WIP.
#![allow(dead_code, unused_imports, unused_variables, clippy::let_unit_value)]
use fxhash::{hash64, FxHashSet, FxHasher};
use std::{collections::HashSet, io::Write, num::NonZero, path::Path};

use crate::{
    asm::LINKER_RECOVER,
    config, typecheck,
    utilis::{assert_unique, encode},
    BiMap, IString, MethodImpl,
};

config!(NO_SFI, bool, false);
config!(ANSI_C, bool, false);
config!(NO_OPT, bool, false);
config!(NO_DEBUG, bool, false);

config!(UB_CHECKS, bool, true);
config!(SHORT_TYPENAMES, bool, false);
config!(PARTS, u32, 1);
config!(ASCII_IDENTS, bool, false);
mod utilis;
use super::{
    basic_block::BlockId,
    bimap::{Interned, IntoBiMapIndex},
    cilnode::{ExtendKind, PtrCastRes},
    cilroot::BranchCond,
    class::{ClassDefIdx, StaticFieldDef},
    method::LocalDef,
    typecheck::TypeCheckError,
    Assembly, BinOp, CILNode, CILRoot, ClassRef, Const, Exporter, FnSig, Int, MethodDef, MethodRef,
    Type,
};
use utilis::*;

pub struct CExporter {
    is_lib: bool,
    libs: Vec<String>,
    dirs: Vec<String>,
    metadata: Vec<u8>,
    curr_fname: Interned<IString>,
}
impl CExporter {
    pub fn c_compiler() -> String {
        std::env::var("CC").unwrap_or("cc".to_owned())
    }
    pub fn is_tcc() -> bool {
        Self::c_compiler().contains("tcc")
    }
    pub fn set_metadata(&mut self, metadata: impl Into<Vec<u8>>) {
        self.metadata = metadata.into();
    }
    /// Retrives correctly formatted .rustc section
    pub fn get_dot_rustc(&self) -> Vec<u8> {
        // Insert magic
        let mut section = b"rust".to_vec();
        // Insert version
        section.extend(9_u32.to_be_bytes());
        // Insert section length
        section.extend((self.metadata.len() as u64).to_le_bytes());
        // Insert metadata
        section.extend(&self.metadata);
        section
    }
    #[must_use]
    pub fn new(is_lib: bool, mut libs: Vec<String>, mut dirs: Vec<String>) -> Self {
        // TODO: fix LLVM not found bug.
        libs.retain(|l| !(l.contains("LLVM") || l.contains("gcc_s")));
        if Self::is_tcc() {
            dirs = vec![];
        }
        Self {
            is_lib,
            libs,
            dirs,
            metadata: vec![],
            curr_fname: Interned::from_index(NonZero::new(1).unwrap()),
        }
    }
    fn export_method_decl(
        asm: &Assembly,
        mref: &MethodRef,
        method_decls: &mut impl Write,
    ) -> std::io::Result<()> {
        let method_name = mref_to_name(mref, asm);
        if method_name == "malloc" || method_name == "realloc" || method_name == "free" {
            return Ok(());
        }
        let output = c_tpe(mref.output(asm), asm);
        let inputs = mref
            .stack_inputs(asm)
            .iter()
            .map(|i| nonvoid_c_type(*i, asm))
            .intersperse(",".into())
            .collect::<String>();

        writeln!(method_decls, "{output} {method_name}({inputs});")
    }
    #[allow(clippy::too_many_arguments)]
    fn binop_to_string(
        lhs: CILNode,
        rhs: CILNode,
        op: BinOp,
        tpe: Type,
        asm: &mut Assembly,
        locals: &[LocalDef],
        inputs: &[(Type, Option<Interned<IString>>)],
        sig: Interned<FnSig>,
    ) -> Result<String, TypeCheckError> {
        let lhs = Self::node_to_string(lhs, asm, locals, inputs, sig)?;
        let rhs = Self::node_to_string(rhs, asm, locals, inputs, sig)?;
        Ok(match op {
            BinOp::Add => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((uint8_t*)({lhs}) + (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) + ({rhs})"),
                Type::Float(_) => format!("({lhs}) + ({rhs})"),
                Type::Int(Int::ISize) => {
                    format!("(intptr_t)((uintptr_t)({lhs}) + (uintptr_t)({rhs}))")
                }
                Type::Int(Int::I128) => {
                    format!("(__int128)((__uint128_t)({lhs}) + (__uint128_t)({rhs}))")
                }
                Type::Int(Int::I64) => format!("(int64_t)((uint64_t)({lhs}) + (uint64_t)({rhs}))"),
                Type::Int(Int::I32) => format!("(int32_t)((uint32_t)({lhs}) + (uint32_t)({rhs}))"),
                Type::Int(Int::I16) => format!("(int16_t)((uint16_t)({lhs}) + (uint16_t)({rhs}))"),
                Type::Int(Int::I8) => format!("(int8_t)((uint8_t)({lhs}) + (uint8_t)({rhs}))"),

                Type::Int(_) => format!("({lhs}) + ({rhs})"),
                _ => todo!("can't add {}", tpe.mangle(asm)),
            },
            BinOp::Eq => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) == (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) == ({rhs})"),
                Type::Bool | Type::Float(_) | Type::Int(_) => format!("({lhs}) == ({rhs})"),
                _ => todo!(),
            },
            BinOp::Sub => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((uint8_t*)({lhs}) - (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) - ({rhs})"),
                Type::Int(Int::I128) => {
                    format!("(__int128)((__uint128_t)({lhs}) - (__uint128_t)({rhs}))")
                }
                Type::Int(Int::I64) => {
                    format!("(int64_t)((uint64_t)({lhs}) - (uint64_t)({rhs}))")
                }
                Type::Int(Int::I32) => {
                    format!("(int32_t)((uint32_t)({lhs}) - (uint32_t)({rhs}))")
                }
                Type::Int(Int::I16) => {
                    format!("(int16_t)((uint16_t)({lhs}) - (uint16_t)({rhs}))")
                }
                Type::Int(Int::I8) => {
                    format!("(int8_t)((uint8_t)({lhs}) - (uint8_t)({rhs}))")
                }
                Type::Int(Int::ISize) => {
                    format!("(intptr_t)((uintptr_t)({lhs}) - (uintptr_t)({rhs}))")
                }
                Type::Float(_) | Type::Int(_) => format!("({lhs}) - ({rhs})"),
                _ => todo!(),
            },
            BinOp::Mul => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((uint8_t*)({lhs}) * (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) * ({rhs})"),
                Type::Float(_) => format!("({lhs}) * ({rhs})"),
                Type::Int(int) => match int {
                    // Signed multiply is seemingly equivalent to unsigned multiply, looking at the assembly: TODO: check this.
                    Int::I8 => format!("(int8_t)((uint8_t)({lhs}) * (uint8_t)({rhs}))"),
                    Int::I16 => {
                        format!("(int16_t)(uint16_t)(((uint32_t)({lhs})) * ((uint32_t)({rhs})))")
                    }
                    Int::I32 => format!("(int32_t)((uint32_t)({lhs}) * (uint32_t)({rhs}))"),
                    Int::I64 => format!("(int64_t)((uint64_t)({lhs}) * (uint64_t)({rhs}))"),
                    Int::I128 => format!("(__int128)((__uint128_t)({lhs}) * (__uint128_t)({rhs}))"),
                    Int::ISize => format!("(intptr_t)((uintptr_t)({lhs}) * (uintptr_t)({rhs}))"),
                    Int::U16 => format!("(uint16_t)(((uint32_t)({lhs})) * ((uint32_t)({rhs})))"),
                    _ => format!("({lhs}) * ({rhs})"),
                },
                _ => todo!(),
            },
            BinOp::Lt => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) < (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) < ({rhs})"),
                Type::Bool | Type::Float(_) | Type::Int(_) => format!("({lhs}) < ({rhs})"),
                _ => todo!(),
            },
            BinOp::LtUn => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) < (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) < ({rhs})"),
                Type::Bool | Type::Int(_) => format!("({lhs}) < ({rhs})"),
                Type::Float(_) => format!("!(({lhs}) >= ({rhs}))"),
                _ => todo!(),
            },
            BinOp::Gt => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) > (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) > ({rhs})"),
                Type::Bool | Type::Float(_) | Type::Int(_) => format!("({lhs}) > ({rhs})"),
                _ => todo!(),
            },
            BinOp::GtUn => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) > (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) > ({rhs})"),
                Type::Bool | Type::Int(_) => format!("({lhs}) > ({rhs})"),
                Type::Float(_) => format!("!(({lhs}) <= ({rhs}))"),
                _ => todo!(),
            },
            BinOp::Or => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) | (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) | ({rhs})"),
                Type::Int(_) => format!("({lhs}) | ({rhs})"),
                Type::Bool => format!("({lhs}) || ({rhs})"),
                _ => todo!(),
            },
            BinOp::XOr => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) ^ (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) ^ ({rhs})"),
                Type::Int(_) => format!("({lhs}) ^ ({rhs})"),
                Type::Bool => format!("({lhs}) != ({rhs})"),
                _ => todo!(),
            },
            BinOp::And => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) & (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) & ({rhs})"),
                Type::Int(_) => format!("({lhs}) & ({rhs})"),
                Type::Bool => format!("({lhs}) && ({rhs})"),
                _ => todo!(),
            },
            BinOp::Rem | BinOp::RemUn => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((void*)({lhs}) % (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) % ({rhs})"),
                Type::Int(_) => format!("({lhs}) % ({rhs})"),
                Type::Float(flt) => match flt {
                    super::Float::F16 => todo!(),
                    super::Float::F32 => format!("(float)fmod((double)({lhs}),((double)({rhs}))"),
                    super::Float::F64 => format!("fmod(({lhs}),({rhs}))"),
                    super::Float::F128 => todo!(),
                },
                // TODO: reminder of a bool can only be false or a segfault. Is this a valid operation?
                Type::Bool => "false".into(),

                _ => todo!("can't rem {tpe:?}"),
            },
            BinOp::Shl => match tpe {
                // Signed shift is equivalent to Rust unsinged shift, but it is well defined in C
                Type::Int(Int::I8) => format!("(int8_t)((uint8_t)({lhs}) << ({rhs}))"),
                Type::Int(Int::I16) => format!("(int16_t)((uint16_t)({lhs}) << ({rhs}))"),
                Type::Int(Int::I32) => format!("(int32_t)((uint32_t)({lhs}) << ({rhs}))"),
                Type::Int(Int::I64) => format!("(int64_t)((uint64_t)({lhs}) << ({rhs}))"),
                Type::Int(Int::I128) => format!("(__int128)((__uint128_t)({lhs}) << ({rhs}))"),
                Type::Int(Int::ISize) => format!("(intptr_t)((uintptr_t)({lhs}) << ({rhs}))"),
                Type::Int(_) => format!("({lhs}) << ({rhs})"),
                _ => todo!("can't shl {tpe:?}"),
            },
            BinOp::Shr | BinOp::ShrUn => match tpe {
                Type::Int(_) => format!("({lhs}) >> ({rhs})"),
                _ => todo!("can't shr {tpe:?}"),
            },
            BinOp::DivUn | BinOp::Div => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((void*)({lhs}) / (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) / ({rhs})"),
                Type::Float(_) | Type::Int(_) => format!("({lhs}) / ({rhs})"),
                _ => todo!(),
            },
        })
    }
    /// Equivalent to `node_to_string`, except prsent in an enviroment where types can be infered.
    /// Eg. In a function call like this `foo((uintptr_t)5)` the cast can be safely ommited.
    fn node_to_string_implict(
        node: CILNode,
        asm: &mut Assembly,
        locals: &[LocalDef],
        inputs: &[(Type, Option<Interned<IString>>)],
        sig: Interned<FnSig>,
    ) -> Result<String, TypeCheckError> {
        match node {
            CILNode::PtrCast(ref src_node, ref target)
                if matches!(target.as_ref(), PtrCastRes::Ptr(_)) =>
            {
                let Type::Ptr(ptr) = asm[*src_node].clone().typecheck(sig, locals, asm)? else {
                    return Self::node_to_string(node, asm, locals, inputs, sig);
                };
                let Type::Void = asm[ptr] else {
                    return Self::node_to_string(node, asm, locals, inputs, sig);
                };
                Self::node_to_string(asm[*src_node].clone(), asm, locals, inputs, sig)
            }
            CILNode::Const(cst) => Ok(match cst.as_ref() {
                Const::ByteBuffer { data, tpe: _ } => {
                    format!("c_{}", encode(data.inner() as u64))
                }
                Const::I8(v) => format!("{v}"),
                Const::I16(v) => format!("{v}"),
                Const::I32(v) => format!("{v}"),
                Const::I64(v) => format!("((int64_t)0x{v:x}L)"),
                Const::I128(v) => {
                    let low = *v as u128 as u64;
                    let high = ((*v as u128) >> 64) as u64;
                    format!("(__int128)((unsigned __int128)(0x{low:x}) | ((unsigned __int128)(0x{high:x}) << 64))")
                }
                Const::ISize(v) => format!("(intptr_t)0x{v:x}L"),
                // For u8 and u16, using hex makes no sense(uses more chars)
                Const::U8(v) => format!("{v}"),
                Const::U16(v) => format!("{v}"),
                Const::U32(v) => format!("{v}"),
                Const::U64(v) => format!("0x{v:x}uL"),
                Const::U128(v) => {
                    let low = *v as u64;
                    let high = ({ *v } >> 64) as u64;
                    format!("((unsigned __int128)(0x{low:x}) | ((unsigned __int128)(0x{high:x}) << 64))")
                }
                Const::USize(v) => {
                    if *v < u32::MAX as u64 {
                        format!("{v}")
                    } else {
                        format!("0x{v:x}uL")
                    }
                }
                Const::PlatformString(string_idx) => format!("{:?}", &asm[*string_idx]),
                Const::Bool(val) => {
                    if *val {
                        "true".into()
                    } else {
                        "false".into()
                    }
                }
                Const::F32(hashable_f32) => {
                    if !hashable_f32.0.is_nan() {
                        format!("{:?}f", hashable_f32.0)
                    } else {
                        "NAN".into()
                    }
                }
                Const::F64(hashable_f64) => {
                    if !hashable_f64.0.is_nan() {
                        format!("{:?}", hashable_f64.0)
                    } else {
                        "NAN".into()
                    }
                }
                Const::Null(class_ref_idx) => todo!(),
            }),
            _ => Self::node_to_string(node, asm, locals, inputs, sig),
        }
    }
    fn node_to_string(
        node: CILNode,
        asm: &mut Assembly,
        locals: &[LocalDef],
        inputs: &[(Type, Option<Interned<IString>>)],
        sig: Interned<FnSig>,
    ) -> Result<String, TypeCheckError> {
        Ok(match node {
            CILNode::Const(cst) => match cst.as_ref() {
                Const::ByteBuffer { data, tpe } => format!(
                    "(({tpe}*)c_{})",
                    encode(data.inner() as u64),
                    tpe = c_tpe(asm[*tpe], asm)
                ),
                Const::I8(v) => format!("(int8_t)0x{v:x}"),
                Const::I16(v) => format!("(int16_t)0x{v:x}"),
                Const::I32(v) => format!("((int32_t)0x{v:x})"),
                Const::I64(v) => format!("((int64_t)0x{v:x}L)"),
                Const::I128(v) => {
                    let low = *v as u128 as u64;
                    let high = ((*v as u128) >> 64) as u64;
                    format!("(__int128)((unsigned __int128)(0x{low:x}) | ((unsigned __int128)(0x{high:x}) << 64))")
                }
                Const::ISize(v) => format!("(intptr_t)0x{v:x}L"),
                // For u8 and u16, using hex makes no sense(uses more chars)
                Const::U8(v) => format!("(uint8_t){v}"),
                Const::U16(v) => format!("(uint16_t){v}"),
                Const::U32(v) => format!("{v}u"),
                Const::U64(v) => format!("0x{v:x}uL"),
                Const::U128(v) => {
                    let low = *v as u64;
                    let high = ({ *v } >> 64) as u64;
                    format!("((unsigned __int128)(0x{low:x}) | ((unsigned __int128)(0x{high:x}) << 64))")
                }
                Const::USize(v) => format!("(uintptr_t)0x{v:x}uL"),
                Const::PlatformString(string_idx) => format!("{:?}", &asm[*string_idx]),
                Const::Bool(val) => {
                    if *val {
                        "true".into()
                    } else {
                        "false".into()
                    }
                }
                Const::F32(hashable_f32) => {
                    if !hashable_f32.0.is_nan() {
                        format!("{:?}f", hashable_f32.0)
                    } else {
                        "NAN".into()
                    }
                }
                Const::F64(hashable_f64) => {
                    if !hashable_f64.0.is_nan() {
                        format!("{:?}", hashable_f64.0)
                    } else {
                        "NAN".into()
                    }
                }
                Const::Null(class_ref_idx) => todo!(),
            },
            CILNode::BinOp(lhs, rhs, bin_op) => {
                let tpe = asm[lhs].clone().typecheck(sig, locals, asm)?;
                Self::binop_to_string(
                    asm[lhs].clone(),
                    asm[rhs].clone(),
                    bin_op,
                    tpe,
                    asm,
                    locals,
                    inputs,
                    sig,
                )?
            }
            CILNode::UnOp(node_idx, ref un_op) => match un_op {
                super::cilnode::UnOp::Not => format!(
                    "~({})",
                    Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                ),
                super::cilnode::UnOp::Neg => {
                    let tpe = node.typecheck(sig, locals, asm)?;
                    match tpe {
                        Type::Ptr(_) | Type::Ref(_) => format!(
                            "-({})",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::FnPtr(_) => format!(
                            "-({})",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Float(_) => format!(
                            "-({})",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(Int::I8) => format!(
                            "(int8_t)(0 - ((uint8_t)({})))",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(Int::I16) => format!(
                            "(int16_t)(0 - ((uint16_t)({})))",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(Int::I32) => format!(
                            "(int32_t)(0 - ((uint32_t)({})))",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(Int::I64) => format!(
                            "(int64_t)(0 - ((uint64_t)({})))",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(Int::I128) => format!(
                            "(__int128_t)(0 - ((__uint128_t)({})))",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(Int::ISize) => format!(
                            "(intptr_t)(0 - ((uintptr_t)({})))",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        Type::Int(_) => format!(
                            "-({})",
                            Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
                        ),
                        _ => todo!("can't neg {}", tpe.mangle(asm)),
                    }
                }
            },
            CILNode::LdLoc(loc) => local_name(locals, asm, loc),
            CILNode::LdArg(arg) => match inputs[arg as usize].1 {
                Some(arg_name) => escape_nonfn_name(&asm[arg_name]),
                None => format!("A{arg}",),
            },
            CILNode::LdArgA(arg) => match inputs[arg as usize].1 {
                Some(arg_name) => format!("&{}", escape_nonfn_name(&asm[arg_name])),
                None => format!("&A{arg}",),
            },
            CILNode::LdLocA(loc) => format!("&{}", local_name(locals, asm, loc),),
            CILNode::Call(info) => {
                let (method, args, _is_pure) = info.as_ref();
                let method = asm[*method].clone();
                let call_args = args
                    .iter()
                    .map(|arg| {
                        format!(
                            "{}",
                            Self::node_to_string_implict(
                                asm[*arg].clone(),
                                asm,
                                locals,
                                inputs,
                                sig
                            )
                            .unwrap()
                        )
                    })
                    .intersperse(",".into())
                    .collect::<String>();
                let class = &asm[method.class()];
                let class_name = escape_ident(&asm[class.name()]);
                let mname = escape_ident(&asm[method.name()]);
                let method_name = mref_to_name(&method, asm);
                format!("{method_name}({call_args})")
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = Self::node_to_string(asm[input].clone(), asm, locals, inputs, sig)?;
                match (target, extend) {
                    (Int::U8, ExtendKind::ZeroExtend) => format!("(uint8_t)({input})"),
                    (Int::U8, ExtendKind::SignExtend) => todo!(),
                    (Int::U16, ExtendKind::ZeroExtend) => format!("(uint16_t)({input})"),
                    (Int::U16, ExtendKind::SignExtend) => todo!(),
                    (Int::U32, ExtendKind::ZeroExtend) => format!("(uint32_t)({input})"),
                    (Int::U32, ExtendKind::SignExtend) => format!("(uint32_t)(int32_t)({input})"),
                    (Int::U64, ExtendKind::ZeroExtend) => format!("(uint64_t)({input})"),
                    (Int::U64, ExtendKind::SignExtend) => format!("(uint64_t)(int64_t)({input})"),
                    (Int::U128, ExtendKind::ZeroExtend) => format!("(__uint128_t)({input})"),
                    (Int::U128, ExtendKind::SignExtend) => todo!(),
                    (Int::USize, ExtendKind::ZeroExtend) => format!("(uintptr_t)({input})"),
                    (Int::USize, ExtendKind::SignExtend) => {
                        format!("(uintptr_t)(intptr_t)({input})")
                    }
                    (Int::I8, ExtendKind::ZeroExtend) => format!("(int8_t)({input})"),
                    (Int::I8, ExtendKind::SignExtend) => format!("(int8_t)({input})"),
                    (Int::I16, ExtendKind::ZeroExtend) => todo!(),
                    (Int::I16, ExtendKind::SignExtend) => format!("(int16_t)({input})"),
                    (Int::I32, ExtendKind::ZeroExtend) => format!("(int32_t)(uint32_t)({input})"),
                    (Int::I32, ExtendKind::SignExtend) => format!("(int32_t)({input})"),
                    (Int::I64, ExtendKind::ZeroExtend) => format!("(int64_t)(uint64_t)({input})"),
                    (Int::I64, ExtendKind::SignExtend) => format!("(int64_t)({input})"),
                    (Int::I128, ExtendKind::ZeroExtend) => todo!(),
                    (Int::I128, ExtendKind::SignExtend) => todo!(),
                    (Int::ISize, ExtendKind::ZeroExtend) => {
                        format!("(intptr_t)(uintptr_t)({input})")
                    }
                    (Int::ISize, ExtendKind::SignExtend) => format!("(intptr_t)({input})"),
                }
            }
            CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => {
                let input = Self::node_to_string(asm[input].clone(), asm, locals, inputs, sig)?;
                match target {
                    super::Float::F16 => todo!(),
                    super::Float::F32 => format!("(float)({input})"),
                    super::Float::F64 => format!("(double)({input})"),
                    super::Float::F128 => todo!(),
                }
            }
            CILNode::RefToPtr(node_idx) => {
                Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
            }
            CILNode::PtrCast(node_idx, ptr_cast_res) => {
                let node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?;
                match ptr_cast_res.as_ref() {
                    PtrCastRes::Ptr(type_idx) | PtrCastRes::Ref(type_idx) => {
                        format!("({tpe}*)({node})", tpe = c_tpe(asm[*type_idx], asm),)
                    }
                    PtrCastRes::FnPtr(_) => format!("(void*)({node})"),
                    PtrCastRes::USize => format!("(uintptr_t)({node})"),
                    PtrCastRes::ISize => format!("(intptr_t)({node})"),
                }
            }
            CILNode::LdFieldAdress { addr, field } => {
                let addr = asm[addr].clone();
                let addr = Self::node_to_string(addr, asm, locals, inputs, sig)?;
                let field = asm[field];
                let name = escape_nonfn_name(&asm[field.name()]);
                if asm
                    .class_ref_to_def(field.owner())
                    .is_some_and(|tpe| asm[tpe].has_nonveralpping_layout())
                {
                    format!("&({addr})->{name}")
                } else {
                    format!("&({addr})->{name}.f")
                }
            }
            CILNode::LdField { addr, field } => {
                let addr_node = asm[addr].clone();
                let addr_tpe = addr_node.typecheck(sig, locals, asm)?;
                let addr_str = Self::node_to_string(addr_node, asm, locals, inputs, sig)?;
                let field = asm[field];
                let name = escape_nonfn_name(&asm[field.name()]);
                match addr_tpe {
                    Type::Ref(_) | Type::Ptr(_) => {
                        if asm
                            .class_ref_to_def(field.owner())
                            .is_some_and(|tpe| asm[tpe].has_nonveralpping_layout())
                        {
                            format!("({addr_str})->{name}")
                        } else {
                            format!("({addr_str})->{name}.f")
                        }
                    }
                    Type::ClassRef(_) => {
                        if asm
                            .class_ref_to_def(field.owner())
                            .is_some_and(|tpe| asm[tpe].has_nonveralpping_layout())
                        {
                            match asm[addr] {
                                CILNode::LdLoc(loc) => {
                                    format!("{loc}.{name}", loc = local_name(locals, asm, loc))
                                }
                                CILNode::LdField { .. } => format!("{addr_str}.{name}"),
                                _ => format!("({addr_str}).{name}"),
                            }
                        } else {
                            format!("({addr_str}).{name}.f")
                        }
                    }
                    _ => panic!(),
                }
            }
            CILNode::LdInd {
                addr,
                tpe,
                volatile,
            } => {
                if volatile {
                    if matches!(asm[tpe], Type::Ptr(_) | Type::Ref(_)) {
                        format!(
                            "(({tpe})*(volatile size_t*)({addr}))",
                            tpe = c_tpe(asm[tpe], asm),
                            addr =
                                Self::node_to_string(asm[addr].clone(), asm, locals, inputs, sig)?
                        )
                    } else {
                        format!(
                            "*(volatile {tpe}*)({addr})",
                            tpe = c_tpe(asm[tpe], asm),
                            addr =
                                Self::node_to_string(asm[addr].clone(), asm, locals, inputs, sig)?
                        )
                    }
                } else {
                    format!(
                        "*({addr})",
                        addr = Self::node_to_string(asm[addr].clone(), asm, locals, inputs, sig)?
                    )
                }
            }
            CILNode::SizeOf(type_idx) => format!("sizeof({tpe})", tpe = c_tpe(asm[type_idx], asm)),
            CILNode::GetException => todo!(),
            CILNode::IsInst(node_idx, type_idx) => todo!(),
            CILNode::CheckedCast(node_idx, type_idx) => todo!(),
            CILNode::CallI(info) => {
                let (fn_ptr, fn_ptr_sig, args) = info.as_ref();
                let fn_ptr_sig = asm[*fn_ptr_sig].clone();
                let call_args = args
                    .iter()
                    .map(|arg| {
                        format!(
                            "{}",
                            Self::node_to_string_implict(
                                asm[*arg].clone(),
                                asm,
                                locals,
                                inputs,
                                sig
                            )
                            .unwrap()
                        )
                    })
                    .intersperse(",".into())
                    .collect::<String>();
                let ret = c_tpe(*fn_ptr_sig.output(), asm);
                let args = fn_ptr_sig
                    .inputs()
                    .iter()
                    .map(|i| nonvoid_c_type(*i, asm))
                    .intersperse(",".into())
                    .collect::<String>();
                let fn_ptr = Self::node_to_string(asm[*fn_ptr].clone(), asm, locals, inputs, sig)?;
                format!("((*({ret}(*)({args}))({fn_ptr})))({call_args})")
            }
            CILNode::LocAlloc { size } => format!(
                "((uint8_t*)alloca({}))",
                Self::node_to_string(asm[size].clone(), asm, locals, inputs, sig)?
            ),
            CILNode::LdStaticField(static_field_idx) => {
                let field = asm[static_field_idx];
                let class = asm[field.owner()].clone();
                let fname = class_member_name(&asm[class.name()], &asm[field.name()]);
                fname.to_string()
            }
            CILNode::LdStaticFieldAdress(static_field_idx) => {
                let field = asm[static_field_idx];
                let class = asm[field.owner()].clone();
                let fname = class_member_name(&asm[class.name()], &asm[field.name()]);
                format!("&{}", fname)
            }
            CILNode::LdFtn(method) => mref_to_name(&asm[method], asm),
            CILNode::LdTypeToken(type_idx) => format!("{}", type_idx.as_bimap_index()),
            //TODO: ld len is not really supported in C, and is only there due to the argc emulation.
            CILNode::LdLen(node_idx) => format!(
                "ld_len({arr})",
                arr = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
            ),
            // TODO: loc alloc aligned does not respect the aligement ATM.
            CILNode::LocAllocAlgined { tpe, align } => {
                format!(
                    "({tpe}*)(alloca(sizeof({tpe})))",
                    tpe = c_tpe(asm[tpe], asm)
                )
            }
            //TODO: ld elem ref is not really supported in C, and is only there due to the argc emulation.
            CILNode::LdElelemRef { array, index } => {
                let tpe = node.typecheck(sig, locals, asm)?;
                let array = Self::node_to_string(asm[array].clone(), asm, locals, inputs, sig)?;
                let index = Self::node_to_string(asm[index].clone(), asm, locals, inputs, sig)?;
                format!("({array})[{index}]")
            }
            CILNode::UnboxAny { object, tpe } => format!(
                "({object})",
                object = Self::node_to_string(asm[object].clone(), asm, locals, inputs, sig)?
            ),
        })
    }
    fn root_to_string(
        &mut self,
        root: CILRoot,
        asm: &mut Assembly,
        locals: &[LocalDef],
        inputs: &[(Type, Option<Interned<IString>>)],
        sig: Interned<FnSig>,
        next: Option<BlockId>,
    ) -> Result<String, TypeCheckError> {
        Ok(match root {
            CILRoot::StLoc(id, node_idx) => {
                let name = local_name(locals, asm, id);
                if let CILNode::LocAllocAlgined { tpe, align } = asm[node_idx]{
                    return Ok(format!("loc_alloc_aligned({name},{tpe},{align},t{hash})",tpe = c_tpe(asm[tpe],asm),hash = encode(asm.alloc_root(root).as_bimap_index().get() as u64)));
                }
                return Ok(format!("{name} = {node};", node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?,));
            },
            CILRoot::StArg(arg, node_idx) =>match inputs[arg as usize].1 {
                Some(name) => format!(
                    "{name} = {node};",
                    node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?,
                    name = escape_nonfn_name(&asm[name]),
                ),
                None => format!(
                    "A{arg} = {node};",
                    node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?,
                ),
            },
            CILRoot::Ret(node_idx) => format!(
                "return {node};",
                node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
            ),
            CILRoot::Pop(node_idx) => format!(
                "{node};",
                node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
            ),
            CILRoot::Throw(node_idx) =>  format!(
                "eprintf(\"An error was encoutrered in %s, at %s:%d\\n\",__func__,__FILE__,__LINE__);eprintf(\"%s\\n\",{node}); abort();",
                node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)?
            ),
            CILRoot::VoidRet => "return;".into(),
            CILRoot::Break => "".into(),
            CILRoot::Nop => "".into(),
            CILRoot::Branch(binfo) => {
                let (target, sub_target, cond) = binfo.as_ref();
                let target = if *sub_target != 0{
                    sub_target
                }else {target};
                let Some(cond) = cond else {
                    if next == Some(*target) && *sub_target == 0{
                        return Ok("".into());
                    }
                    else{
                        return Ok(format!("goto bb{target};"));
                    }
                };
                match cond {
                    BranchCond::True(node_idx) => format!(
                        "if({node}) goto bb{target};",
                        node =
                            Self::node_to_string(asm[*node_idx].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::False(node_idx) => format!(
                        "if(!({node})) goto bb{target};",
                        node =
                            Self::node_to_string(asm[*node_idx].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::Eq(lhs, rhs) => format!(
                        "if(({lhs}) == ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig)?,
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::Ne(lhs, rhs) => format!(
                        "if(({lhs}) != ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig)?,
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::Lt(lhs, rhs, cmp_kind) => format!(
                        "if(({lhs}) < ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig)?,
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::Gt(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) > ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig)?,
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::Le(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) <= ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig)?,
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)?
                    ),
                    BranchCond::Ge(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) >= ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig)?,
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)?
                    ),
                }
            }
            CILRoot::SourceFileInfo { line_start, line_len, col_start, col_len, file  } =>{
                if !*NO_SFI{
                    self.set_sfi(line_start, file,asm)
                }else{
                    "".into()
                }
            },
            CILRoot::SetField(info) =>{
                let (field,addr,value) = info.as_ref();
                let addr_str = Self::node_to_string(asm[*addr].clone(), asm, locals, inputs, sig)?;
                let value = Self::node_to_string(asm[*value].clone(), asm, locals, inputs, sig)?;
                let field = asm[*field];
                let name = escape_nonfn_name(&asm[field.name()]);
                if asm.class_ref_to_def(field.owner()).is_some_and(|tpe|asm[tpe].has_nonveralpping_layout()){
                    match asm[*addr]{
                        CILNode::LdLocA(loc)=>format!("{loc}.{name} = ({value});",loc = local_name(locals, asm, loc)),
                        _=>format!("({addr_str})->{name} = ({value});"),
                    }
                }
                else{
                    format!("({addr_str})->{name}.f = ({value});")
                }
            }
            CILRoot::Call(info) => {
                let (method, args,_is_pure) = info.as_ref();
                let method = asm[*method].clone();
                let call_args = args
                    .iter()
                    .map(|arg| {
                        format!(
                            "{}",
                            Self::node_to_string_implict(asm[*arg].clone(), asm, locals, inputs, sig).unwrap()
                        )
})
                    .intersperse(",".into())
                    .collect::<String>();
                let method_name = mref_to_name(&method, asm);
                format!("{method_name}({call_args});")
            }
            CILRoot::InitObj(addr,tpe) => {
                let addr = Self::node_to_string(asm[addr].clone(), asm, locals, inputs, sig)?;
                    format!(
                        "memset({addr},0,sizeof({tpe}));",
                        tpe = c_tpe(asm[tpe], asm)
                    )
            }
            CILRoot::StInd(info) => {
                let (addr, value, tpe, is_volitle) = info.as_ref();
                let addr = Self::node_to_string(asm[*addr].clone(), asm, locals, inputs, sig)?;
                let value = Self::node_to_string(asm[*value].clone(), asm, locals, inputs, sig)?;
                if *is_volitle {
                    format!(
                        "*((volatile {tpe}*)({addr})) = ({value});",
                        tpe = c_tpe(*tpe, asm)
                    )
                } else {
                    format!("*({addr}) = ({value});")
                }
            }
            CILRoot::InitBlk(blk) => {
                let (dst, val, count) = blk.as_ref();
                let dst = Self::node_to_string(asm[*dst].clone(), asm, locals, inputs, sig)?;
                let val = Self::node_to_string(asm[*val].clone(), asm, locals, inputs, sig)?;
                let count = Self::node_to_string(asm[*count].clone(), asm, locals, inputs, sig)?;
                format!("memset(({dst}),({val}),({count}));")
            }
            CILRoot::CpBlk(blk) => {
                let (dst, src, len) = blk.as_ref();
                let dst = Self::node_to_string(asm[*dst].clone(), asm, locals, inputs, sig)?;
                let src = Self::node_to_string(asm[*src].clone(), asm, locals, inputs, sig)?;
                let len = Self::node_to_string(asm[*len].clone(), asm, locals, inputs, sig)?;
                format!("memcpy(({dst}),({src}),({len}));")
            }
            CILRoot::CallI(info) => {
                let (fn_ptr, fn_ptr_sig, args) = info.as_ref();
                let fn_ptr_sig = asm[*fn_ptr_sig].clone();
                let call_args = args
                    .iter()
                    .map(|arg| {
                        format!(
                            "{}",
                            Self::node_to_string_implict(asm[*arg].clone(), asm, locals, inputs, sig).unwrap()
                        )
                    })
                    .intersperse(",".into())
                    .collect::<String>();
                let ret = c_tpe(*fn_ptr_sig.output(), asm);
                let args = fn_ptr_sig
                    .inputs()
                    .iter()
                    .map(|i| nonvoid_c_type(*i, asm))
                    .intersperse(",".into())
                    .collect::<String>();
                let fn_ptr = Self::node_to_string(asm[*fn_ptr].clone(), asm, locals, inputs, sig)?;
                format!("((*({ret}(*)({args}))({fn_ptr})))({call_args});")
            }
            CILRoot::ExitSpecialRegion { target, source } => format!("goto bb{target};"),
            CILRoot::ReThrow => todo!(),
            CILRoot::SetStaticField { field, val } => {
                let field = asm[field];
                let class = asm[field.owner()].clone();
                let fname = class_member_name(&asm[class.name()], &asm[field.name()]);
                let val = Self::node_to_string(asm[val].clone(), asm, locals, inputs, sig)?;
                format!("{fname} = {val};")
            }
            CILRoot::CpObj { src, dst, tpe } => todo!(),
            CILRoot::Unreachable(string_idx) => format!("\neprintf({:?});\nabort();\n",&asm[string_idx]),
        })
    }
    fn export_method_def(
        &mut self,
        asm: &mut Assembly,
        def: &MethodDef,
        method_defs: &mut impl Write,
        method_decls: &mut impl Write,
    ) -> std::io::Result<()> {
        let class = &asm[def.class()];
        let class_name = escape_nonfn_name(&asm[class.name()]);
        let mname = escape_ident(&asm[def.name()]);
        // Workaround for `get_environ` - a .NET specific function, irrelevant to our use case.
        if mname == "get_environ" || mname == "malloc" || mname == "realloc" || mname == "free" {
            return Ok(());
        }
        let method_name = mref_to_name(&def.ref_to(), asm);
        let output = c_tpe(def.ref_to().output(asm), asm);
        match def.resolved_implementation(asm) {
            MethodImpl::MethodBody { blocks, locals } => (),
            MethodImpl::Extern {
                lib,
                preserve_errno,
            } => match mname.as_str() {
                "printf"
                | "puts"
                | "memcmp"
                | "memcpy"
                | "strlen"
                | "rename"
                | "realpath"
                | "unsetenv"
                | "setenv"
                | "getenv"
                | "syscall"
                | "fcntl"
                | "execvp"
                | "pthread_create_wrapper"
                | "pthread_getattr_np"
                | "ioctl"
                | "pthread_attr_destroy"
                | "pthread_attr_init"
                | "pthread_attr_getstack"
                | "sched_getaffinity"
                | "sigemptyset"
                | "sigaction"
                | "sigaltstack"
                | "poll" => return Ok(()),
                _ => {
                    let inputs = def
                        .ref_to()
                        .stack_inputs(asm)
                        .iter()
                        .map(|i| nonvoid_c_type(*i, asm))
                        .intersperse(",".into())
                        .collect::<String>();
                    writeln!(method_decls, "{output} {method_name}({inputs});")?;
                    return Ok(());
                }
            },
            MethodImpl::Missing => {
                let inputs = def
                    .ref_to()
                    .stack_inputs(asm)
                    .iter()
                    .enumerate()
                    .map(|(idx, i)| format!("{} A{}", nonvoid_c_type(*i, asm), idx))
                    .intersperse(",".into())
                    .collect::<String>();
                writeln!(
                    method_defs,
                    "{output} {method_name}({inputs}){{eprintf(\"Missing method {method_name}\\n\");abort();}}"
                )?;
                return Ok(());
            }
            MethodImpl::AliasFor(method_ref_idx) => panic!("Impossible: unrechable reached."),
        }
        let sig = def.sig();
        let stack_inputs = def.stack_inputs(asm);
        let inputs = stack_inputs
            .iter()
            .enumerate()
            .map(|(idx, (tpe, name))| match name {
                Some(name) => format!(
                    "{} {name}",
                    nonvoid_c_type(*tpe, asm),
                    name = escape_nonfn_name(&asm[*name]),
                ),
                None => format!("{} A{idx} ", nonvoid_c_type(*tpe, asm)),
            })
            .intersperse(",".into())
            .collect::<String>();
        writeln!(method_defs, "{output} {method_name}({inputs}){{")?;
        let locals: Vec<_> = def.iter_locals(asm).copied().collect();
        for (idx, (lname, local_type)) in locals.iter().enumerate() {
            // If the name of this local is found multiple times, use the L form.

            writeln!(
                method_defs,
                "{local_type} {lname};",
                lname = local_name(&locals, asm, idx as u32),
                local_type = nonvoid_c_type(asm[*local_type], asm),
            )?;
        }
        let blocks = def.blocks(asm).unwrap().to_vec();
        // Prepare allocas, if needed.
        for root in blocks[0].roots() {
            let CILRoot::StLoc(loc, node) = asm[*root] else {
                continue;
            };
            let CILNode::LocAllocAlgined { tpe, align } = asm[node] else {
                continue;
            };
            writeln!(
                method_defs,
                "register_alloca_aligned(t{hash}, {tpe},{align});",
                hash = encode(root.as_bimap_index().get() as u64),
                tpe = c_tpe(asm[tpe], asm),
            )?;
        }
        let mut block_iter = blocks.iter().peekable();
        while let Some(block) = block_iter.next() {
            writeln!(method_defs, "bb{}:", block.block_id())?;
            let mut root_iter = block.roots().iter().peekable();
            while let Some(root_idx) = root_iter.next() {
                if let Err(err) = asm[*root_idx].clone().typecheck(sig, &locals, asm) {
                    eprintln!("Typecheck error:{err:?}");
                    writeln!(method_defs, "fprintf(stderr,\"Attempted to execute a statement which failed to compile.\" {err:?}); abort();",err = format!("{err:?}"))?;
                    continue;
                }

                let root = self.root_to_string(
                    asm[*root_idx].clone(),
                    asm,
                    &locals[..],
                    &stack_inputs[..],
                    sig,
                    // If this is not the last block, but it is the last root of this block, check if the following block is our target. If so, use more optimized branching.
                    block_iter
                        .peek()
                        .map(|block| block.block_id())
                        .filter(|_| root_iter.peek().is_none()),
                );

                match root {
                    Ok(root) => {
                        if root.is_empty() {
                            continue;
                        }
                        writeln!(method_defs, "{root}")?
                    }
                    Err(err) => {
                        eprintln!("Typecheck error:{err:?}");
                        writeln!(method_defs, "fprintf(stderr,\"Attempted to execute a statement which failed to compile.\" {err:?}); abort();",err = format!("{err:?}"))?
                    }
                }
            }
        }
        writeln!(method_defs, "}}")
    }
    #[allow(clippy::too_many_arguments)]
    fn export_class(
        &mut self,
        asm: &mut super::Assembly,
        defid: ClassDefIdx,
        method_decls: &mut impl Write,
        method_defs: &mut impl Write,
        type_defs: &mut impl Write,
        defined_types: &mut FxHashSet<ClassDefIdx>,
        delayed_defs: &mut FxHashSet<ClassDefIdx>,
        extrn: bool,
    ) -> std::io::Result<()> {
        let class = asm[defid].clone();
        // Checks if this def needs to be delayed, if one of its fields is not yet defined
        if !class
            .fields()
            .iter()
            .filter_map(|(tpe, _, _)| tpe.as_class_ref())
            .chain(
                class
                    .static_fields()
                    .iter()
                    .filter_map(|fld| fld.tpe.as_class_ref()),
            )
            .filter_map(|cref| asm.class_ref_to_def(cref))
            .all(|cdef| defined_types.contains(&cdef))
        {
            delayed_defs.insert(defid);
            return Ok(());
        }
        let class_name = escape_nonfn_name(&asm[class.name()]);
        if class.has_nonveralpping_layout() && class.explict_size().is_some() {
            writeln!(type_defs, "typedef struct {class_name}{{")?;
            let mut fields = class.fields().to_vec();
            fields.sort_by(|(_, _, a_offset), (_, _, b_offset)| {
                a_offset.unwrap().cmp(&b_offset.unwrap())
            });
            let mut last_offset = 0;
            let mut pad_count = 0;
            for (field_tpe, fname, offset) in &fields {
                let fname = escape_nonfn_name(&asm[*fname]);
                let offset = offset.unwrap();
                if offset != last_offset {
                    assert!(offset >= last_offset,"Type {class_name} has overlapping fields. offset:{offset},last_offset:{last_offset}\nfields:{fields:?}",fields = fields.iter().map(|(tpe,name,offset)| format!("{offset:?} {} {}\n",&asm[*name], tpe.mangle(asm))).collect::<String>());
                    writeln!(
                        type_defs,
                        "uint8_t pad_{pad_count}[{}];\n",
                        offset - last_offset
                    )?;
                    pad_count += 1;
                }
                last_offset = offset + asm.sizeof_type(*field_tpe);
                let field_tpe = c_tpe(*field_tpe, asm);
                writeln!(type_defs, "{field_tpe} {fname};")?;
            }
            if last_offset != class.explict_size().unwrap().get() {
                let size = class.explict_size().unwrap().get();
                if let Some((tpe, name, _)) = fields.last() {
                    //assert!(size >= last_offset, "Type {class_name} has field offset {last_offset} larger than {size}. {} {}",tpe.mangle(asm),&asm[*name]);
                    writeln!(
                        type_defs,
                        "uint8_t pad_{pad_count}[{}];\n",
                        size.checked_sub(last_offset).unwrap_or_else(||{
                            eprintln!("Type {class_name} has field offset {last_offset} larger than {size}. {} {}",tpe.mangle(asm),&asm[*name]);
                            0
                        })
                    )?;
                }
            }
            writeln!(type_defs, "}} {class_name};")?;
        } else {
            writeln!(type_defs, "typedef union {class_name}{{")?;
            for (field_tpe, fname, offset) in class.fields() {
                let fname = escape_nonfn_name(&asm[*fname]);
                let Some(offset) = offset else {
                    eprintln!(
                        "ERR: Can't export field {fname} of {class_name}, becuase it has no offset."
                    );
                    continue;
                };
                let field_tpe = c_tpe(*field_tpe, asm);
                let pad = if *offset != 0 {
                    format!("char pad[{offset}];")
                } else {
                    "".into()
                };
                writeln!(type_defs, "struct {{{pad} {field_tpe} f;}}{fname};")?;
            }
            if let Some(size) = class.explict_size() {
                writeln!(type_defs, "char force_size[{size}];", size = size.get())?;
            }
            if class.fields().is_empty() {
                writeln!(type_defs, "FORCE_NOT_ZST")?;
            }
            writeln!(type_defs, "}} {class_name};")?;
        }
        if !class.static_fields().is_empty() {
            writeln!(type_defs, "\n/*START OF STATCIDEFS*/\n")?;
        }
        for sttic in class.static_fields() {
            let StaticFieldDef {
                tpe: sfield_tpe,
                name: sfname,
                is_tls: is_thread_local,
                is_const,
                default_value,
            } = sttic;
            let fname = escape_nonfn_name(&asm[*sfname]);
            let field_tpe = nonvoid_c_type(*sfield_tpe, asm);
            let fname = class_member_name(&class_name, &fname);
            let extrn = if extrn { "extern" } else { "" };
            if let Some(default) = default_value {
                assert!(!*is_thread_local);

                let val = match default {
                    Const::Bool(b) => format!("{b}"),
                    Const::U64(v) => {
                        if *v < u32::MAX as u64 {
                            format!("{v}")
                        } else {
                            format!("0x{v:x}ul")
                        }
                    }
                    Const::U32(v) => {
                        format!("{v}")
                    }
                    Const::U16(v) => {
                        format!("{v}")
                    }
                    Const::U8(v) => {
                        format!("{v}")
                    }
                    _ => todo!("Unsupported default {default:?}"),
                };
                writeln!(type_defs, "{extrn} {field_tpe} {fname} = {val};")?;
                continue;
            }
            if *is_thread_local {
                writeln!(type_defs, "{extrn} _Thread_local {field_tpe} {fname};")?;
            } else {
                writeln!(type_defs, "{extrn} {field_tpe} {fname};")?;
            }
            //writeln!(type_defs, "// {sttic:?}")?;
        }
        if !class.static_fields().is_empty() {
            writeln!(type_defs, "\n/*END OF STATCIDEFS*/\n")?;
        }

        for method in class.methods() {
            let mref = &asm[method.0].clone();
            let def = asm[*method].clone();
            let is_extern = def.resolved_implementation(asm).is_extern();
            self.export_method_def(asm, &def, method_defs, method_decls)?;
            if !is_extern {
                Self::export_method_decl(asm, mref, method_decls)?;
            }
        }
        defined_types.insert(defid);
        Ok(())
    }
    fn export_to_write(
        &mut self,
        asm: &super::Assembly,
        out: &mut impl Write,
        lib: bool,
        extrn: bool,
    ) -> std::io::Result<()> {
        let mut asm = asm.clone();

        let mut method_defs = Vec::new();
        let mut method_decls = Vec::new();
        let mut type_defs = Vec::new();
        let mut defined_types: FxHashSet<ClassDefIdx> = FxHashSet::default();
        let mut delayed_defs: FxHashSet<ClassDefIdx> = asm.iter_class_def_ids().cloned().collect();
        let mut delayed_defs_copy: FxHashSet<ClassDefIdx> = FxHashSet::default();
        for (const_data, idx) in asm.const_data.1.iter() {
            let data: String = const_data
                .iter()
                .map(|u| format!("{u}"))
                .intersperse(",".into())
                .collect();
            let encoded = encode(idx.inner() as u64);
            writeln!(type_defs, "uint8_t c_{encoded}[] = {{{data}}};")?;
        }
        // Ensure RustVoid present
        let rust_void = asm.rust_void();
        self.export_class(
            &mut asm,
            rust_void,
            &mut method_decls,
            &mut method_defs,
            &mut type_defs,
            &mut defined_types,
            &mut delayed_defs,
            extrn,
        )?;
        delayed_defs.remove(&rust_void);
        while !delayed_defs.is_empty() {
            std::mem::swap(&mut delayed_defs, &mut delayed_defs_copy);
            for class_def in &delayed_defs_copy {
                self.export_class(
                    &mut asm,
                    *class_def,
                    &mut method_decls,
                    &mut method_defs,
                    &mut type_defs,
                    &mut defined_types,
                    &mut delayed_defs,
                    extrn,
                )?;
            }
            delayed_defs_copy.clear();
        }
        let mut header: String = include_str!("c_header.h").into();
        if !asm.has_tcctor() {
            header = header.replace("void _tcctor();", "");
            header = header.replace("_tcctor();", "");
        }

        out.write_all(header.as_bytes())?;
        out.write_all(b"\n/*END OF BUILTIN HEADER*/\n")?;
        out.write_all(&type_defs)?;
        out.write_all(b"\n/*END OF TYPEDEFS*/\n")?;
        out.write_all(&method_decls)?;
        out.write_all(b"\n/*END OF METHODECLS*/\n")?;
        out.write_all(&method_defs)?;
        if !lib {
            call_entry(out, &asm)?;
        }
        Ok(())
    }

    fn set_sfi(
        &mut self,
        line_start: u32,
        file: super::bimap::Interned<IString>,
        asm: &Assembly,
    ) -> String {
        if file == self.curr_fname {
            format!("#line {line_start} ")
        } else {
            self.curr_fname = file;
            format!("#line {line_start} {file:?}", file = &asm[file])
        }
    }
}
fn call_entry(out: &mut impl Write, asm: &Assembly) -> Result<(), std::io::Error> {
    let cctor_call = if asm.has_cctor() { "_cctor();" } else { "" };

    writeln!(out,"int main(int argc_input, char** argv_input){{\n#ifndef __SDCC\nargc = argc_input;if(argc < 1)abort();\nargv = argv_input;if(argv == (char**)0)abort();\n#endif\n{cctor_call}entrypoint();\nreturn 0;}}")?;
    Ok(())
}
impl CExporter {
    fn export_to_file(
        &mut self,
        c_path: &Path,
        asm: &Assembly,
        target: &Path,
        lib: bool,
        extrn: bool,
        is_main: bool,
    ) -> Result<(), std::io::Error> {
        let mut c_out = std::io::BufWriter::new(std::fs::File::create(c_path)?);
        println!("Exporting {c_path:?}");

        self.export_to_write(asm, &mut c_out, lib, extrn)?;
        println!("Exported {c_path:?}");
        // Needed to ensure the IL file is valid!
        c_out.flush().unwrap();
        drop(c_out);

        let mut cmd = std::process::Command::new(Self::c_compiler());
        if is_main {
            cmd.args(["-DMAIN_FILE"]);
        }

        cmd.arg("-fPIC");
        cmd.arg(c_path).arg("-o").arg(target);
        if !*NO_DEBUG {
            cmd.arg("-g");
        }
        if *UB_CHECKS && *PARTS == 1 {
            cmd.args([
                "-fsanitize=undefined,alignment",
                "-fno-sanitize=leak",
                "-fno-sanitize-recover",
                "-O0",
            ]);
        } else if !*NO_OPT {
            cmd.arg("-Ofast");
        } else {
            cmd.arg("-O0");
        };
        if lib {
            cmd.arg("-c");
        } else {
            cmd.arg("-lm");
            cmd.args(&self.dirs);
            cmd.args(&self.libs);
        }
        if *ANSI_C {
            cmd.arg("-std=c89");
        }
        println!("Compiling {c_path:?} with {cmd:?}");
        let out = cmd.output().unwrap();
        println!("Compiled {c_path:?}");
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !*LINKER_RECOVER {
            assert!(
                !(stderr.contains("error:")
                    || stderr.contains("fatal")
                    || stderr.contains("failed")),
                "stdout:{} stderr:{} cmd:{cmd:?}",
                stdout,
                String::from_utf8_lossy(&out.stderr)
            );
            println!(
                "linker success. stdout:{} stderr:{} cmd:{cmd:?}",
                stdout,
                String::from_utf8_lossy(&out.stderr)
            );
        }

        Ok(())
    }
}
impl Exporter for CExporter {
    type Error = std::io::Error;

    fn export(
        &mut self,
        asm: &super::Assembly,
        target: &std::path::Path,
    ) -> Result<(), Self::Error> {
        if *PARTS == 1 {
            // The IL file should be next to the target
            let c_path = target.with_extension("c");
            self.export_to_file(&c_path, asm, target, self.is_lib, false, true)
        } else {
            let mut parts = vec![];
            for (id, part) in asm.split_to_parts(*PARTS).enumerate() {
                let name = target.file_stem().unwrap().to_string_lossy().into_owned();
                let target = target
                    .with_file_name(format!("{name}_{id}"))
                    .with_extension("o");
                let c_path = target.with_extension("c");
                self.export_to_file(&c_path, &part, &target, true, true, id == 0)?;
                parts.push(target);
            }

            let mut cmd = std::process::Command::new(Self::c_compiler());
            cmd.args(&self.dirs);

            cmd.args(&self.libs);
            cmd.args(parts);
            cmd.arg("-o").arg(target).arg("-g").arg("-lm");

            let c_path = target.with_extension("c");
            let only_statics = asm.only_statics();

            self.export_to_file(&c_path, &only_statics, target, true, false, false)?;
            let mut option = std::fs::OpenOptions::new();
            option.read(true);
            option.append(true);
            if !self.is_lib {
                let mut c_file = option.open(&c_path).unwrap();
                call_entry(&mut c_file, asm).unwrap();
            } else if target
                .extension()
                .is_some_and(|s| s.as_encoded_bytes() == b"so")
            {
                cmd.arg("-shared");
            } else {
                cmd.arg("-c");
            }
            cmd.arg(c_path);

            println!("Linking {target:?}");
            let out = cmd.output().unwrap();
            println!("Linked {target:?}");
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !*LINKER_RECOVER {
                assert!(
                    !(stderr.contains("error") || stderr.contains("fatal")),
                    "stdout:{} stderr:{} cmd:{cmd:?}",
                    stdout,
                    String::from_utf8_lossy(&out.stderr)
                );
            }
            if let Some(rustc) = asm.get_section(".rustc") {
                // .rustc section data
                let meta_path = target.with_extension("meta");
                // Write section data.
                std::fs::File::create(&meta_path)
                    .unwrap()
                    .write_all(rustc)
                    .unwrap();
                let copy = std::process::Command::new("objcopy")
                    .arg(target)
                    .arg("--add-section")
                    .arg(format!(
                        ".rustc={meta_path}",
                        meta_path = meta_path.to_str().unwrap()
                    ))
                    .output()
                    .unwrap();
                assert!(copy.status.success());
            }
            Ok(())
        }
    }
}

#[must_use]
pub fn class_to_mangled(class: &super::ClassRef, asm: &Assembly) -> String {
    let assembly = match class.asm() {
        Some(asm_idx) => &asm[asm_idx],
        None => "",
    };
    format!("{assembly}{name}", name = escape_ident(&asm[class.name()]))
}
#[must_use]
pub fn name_sig_class_to_mangled(
    name: &str,
    sig: super::Interned<FnSig>,
    class: Option<Interned<ClassRef>>,
    asm: &Assembly,
) -> String {
    let class = match class {
        Some(_) => todo!(),
        None => todo!(),
    };
}
