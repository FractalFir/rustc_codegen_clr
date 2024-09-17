// This exporter is WIP.
#![allow(dead_code, unused_imports, unused_variables, clippy::let_unit_value)]
use std::{collections::HashSet, io::Write};

use fxhash::{hash64, FxHashSet, FxHasher};

use crate::{
    utilis::{assert_unique, encode},
    v2::{BiMap, MethodImpl, StringIdx},
};

use super::{
    asm::MAIN_MODULE,
    cilnode::{ExtendKind, PtrCastRes},
    cilroot::BranchCond,
    int,
    method::LocalDef,
    Assembly, BinOp, CILIter, CILIterElem, CILNode, CILRoot, ClassDefIdx, ClassRef, ClassRefIdx,
    Const, Exporter, Int, MethodDef, MethodRef, NodeIdx, RootIdx, SigIdx, Type,
};

fn escape_ident(ident: &str) -> String {
    let escaped = ident.replace('.', "_");
    // Check if reserved.
    match escaped.as_str() {
        "int" => encode(hash64(&escaped)),
        _ => escaped,
    }
}
fn nonvoid_c_type(field_tpe: Type, asm: &Assembly) -> String {
    match field_tpe {
        Type::Void => "RustVoid".into(),
        _ => c_tpe(field_tpe, asm),
    }
}
fn c_tpe(field_tpe: Type, asm: &Assembly) -> String {
    match field_tpe {
        Type::Ptr(type_idx) | Type::Ref(type_idx) => format!("{}*", c_tpe(asm[type_idx], asm)),
        Type::Int(int) => match int {
            Int::U8 => "uint8_t".into(),
            Int::U16 => "uint16_t".into(),
            Int::U32 => "uint32_t".into(),
            Int::U64 => "uint64_t".into(),
            Int::U128 => "unsigned __int128".into(),
            Int::USize => "uintptr_t".into(),
            Int::I8 => "int8_t".into(),
            Int::I16 => "int16_t".into(),
            Int::I32 => "int32_t".into(),
            Int::I64 => "int64_t".into(),
            Int::I128 => "__int128".into(),
            Int::ISize => "intptr_t".into(),
        },
        Type::ClassRef(class_ref_idx) => escape_ident(&asm[asm[class_ref_idx].name()]),
        Type::Float(float) => match float {
            super::Float::F16 => todo!(),
            super::Float::F32 => "float".into(),
            super::Float::F64 => "double".into(),
            super::Float::F128 => todo!(),
        },
        Type::PlatformString => "char*".into(),
        Type::PlatformChar => "char".into(),
        Type::PlatformGeneric(_, generic_kind) => todo!(),
        Type::PlatformObject => "void*".into(),
        Type::Bool => "bool".into(),
        Type::Void => "void".into(),
        Type::PlatformArray { elem, dims } => format!(
            "{elem}{dims}",
            elem = c_tpe(asm[elem], asm),
            dims = "*".repeat(dims.get() as usize)
        )
        .into(),
        Type::FnPtr(_) => "void*".into(),
    }
}
fn method_name(class_name: &str, method_name: &str) -> String {
    if class_name == MAIN_MODULE {
        method_name.into()
    } else {
        format!("{class_name}_{method_name}")
    }
}
pub struct CExporter {
    is_lib: bool,
}
impl CExporter {
    #[must_use]
    pub fn new(is_lib: bool) -> Self {
        Self { is_lib }
    }
    fn export_method_decl(
        asm: &Assembly,
        mref: &MethodRef,
        method_decls: &mut impl Write,
    ) -> std::io::Result<()> {
        let class = &asm[mref.class()];
        let class_name = escape_ident(&asm[class.name()]);
        let mname = escape_ident(&asm[mref.name()]);
        let method_name = method_name(&class_name, &mname);
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
        inputs: &[(Type, Option<StringIdx>)],
        sig: SigIdx,
    ) -> String {
        let lhs = Self::node_to_string(lhs, asm, locals, inputs, sig);
        let rhs = Self::node_to_string(rhs, asm, locals, inputs, sig);
        match op {
            BinOp::Add => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((void*)({lhs}) + (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) + ({rhs})"),
                Type::Float(_) | Type::Int(_) => format!("({lhs}) + ({rhs})"),
                _ => todo!(),
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
                    "({tpe}*)((void*)({lhs}) - (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) - ({rhs})"),
                Type::Float(_) | Type::Int(_) => format!("({lhs}) - ({rhs})"),
                _ => todo!(),
            },
            BinOp::Mul => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((void*)({lhs}) * (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) * ({rhs})"),
                Type::Float(_) | Type::Int(_) => format!("({lhs}) * ({rhs})"),
                _ => todo!(),
            },
            BinOp::LtUn | BinOp::Lt => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) < (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) < ({rhs})"),
                Type::Bool | Type::Float(_) | Type::Int(_) => format!("({lhs}) < ({rhs})"),
                _ => todo!(),
            },
            BinOp::GtUn | BinOp::Gt => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => {
                    format!("(void*)({lhs}) > (void*)({rhs})",)
                }
                Type::FnPtr(_) => format!("({lhs}) > ({rhs})"),
                Type::Bool | Type::Float(_) | Type::Int(_) => format!("({lhs}) < ({rhs})"),
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
            BinOp::Rem => todo!(),
            BinOp::RemUn => todo!(),
            BinOp::Shl => todo!(),
            BinOp::Shr => todo!(),
            BinOp::ShrUn => todo!(),
            BinOp::DivUn | BinOp::Div => match tpe {
                Type::Ptr(type_idx) | Type::Ref(type_idx) => format!(
                    "({tpe}*)((void*)({lhs}) / (uintptr_t)({rhs}))",
                    tpe = c_tpe(asm[type_idx], asm)
                ),
                Type::FnPtr(_) => format!("({lhs}) / ({rhs})"),
                Type::Float(_) | Type::Int(_) => format!("({lhs}) / ({rhs})"),
                _ => todo!(),
            },
        }
    }
    fn node_to_string(
        node: CILNode,
        asm: &mut Assembly,
        locals: &[LocalDef],
        inputs: &[(Type, Option<StringIdx>)],
        sig: SigIdx,
    ) -> String {
        match node {
            CILNode::Const(cst) => match cst.as_ref() {
                Const::I8(v) => format!("(int8_t){v}"),
                Const::I16(v) => format!("(int16_t){v}"),
                Const::I32(v) => format!("{v}"),
                Const::I64(v) => format!("{v}L"),
                Const::ISize(v) => format!("(intptr_t){v}L"),
                Const::U8(v) => format!("(uint8_t){v}"),
                Const::U16(v) => format!("(uint16_t){v}"),
                Const::U32(v) => format!("{v}u"),
                Const::U64(v) => format!("{v}uL"),
                Const::USize(v) => format!("(uintptr_t){v}uL"),
                Const::PlatformString(string_idx) => format!("{:?}", &asm[*string_idx]),
                Const::Bool(val) => {
                    if *val {
                        "true".into()
                    } else {
                        "false".into()
                    }
                }
                Const::F32(hashable_f32) => format!("{:?}f", hashable_f32.0),
                Const::F64(hashable_f64) => format!("{:?}", hashable_f64.0),
                Const::Null(class_ref_idx) => todo!(),
            },
            CILNode::BinOp(lhs, rhs, bin_op) => {
                let tpe = node.typecheck(sig, locals, asm).unwrap();
                Self::binop_to_string(
                    asm[lhs].clone(),
                    asm[rhs].clone(),
                    bin_op,
                    tpe,
                    asm,
                    locals,
                    inputs,
                    sig,
                )
            }
            CILNode::UnOp(node_idx, un_op) => match un_op {
                super::cilnode::UnOp::Not => format!(
                    "-({})",
                    Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)
                ),
                super::cilnode::UnOp::Neg => format!(
                    "~({})",
                    Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)
                ),
            },
            CILNode::LdLoc(loc) => match locals[loc as usize].0 {
                Some(local_name) => escape_ident(&asm[local_name]),
                None => format!("L{loc}",),
            },
            CILNode::LdArg(arg) => match inputs[arg as usize].1 {
                Some(arg_name) => escape_ident(&asm[arg_name]),
                None => format!("A{arg}",),
            },
            CILNode::LdArgA(arg) => match inputs[arg as usize].1 {
                Some(arg_name) => format!("&{}", escape_ident(&asm[arg_name])),
                None => format!("&A{arg}",),
            },
            CILNode::LdLocA(loc) => match locals[loc as usize].0 {
                Some(local_name) => format!("&{}", escape_ident(&asm[local_name])),
                None => format!("&L{loc}",),
            },
            CILNode::Call(info) => {
                let (method, args) = info.as_ref();
                let method = asm[*method].clone();
                let call_args = args
                    .iter()
                    .map(|arg| {
                        format!(
                            "({})",
                            Self::node_to_string(asm[*arg].clone(), asm, locals, inputs, sig)
                        )
                    })
                    .intersperse(",".into())
                    .collect::<String>();
                let class = &asm[method.class()];
                let class_name = escape_ident(&asm[class.name()]);
                let mname = escape_ident(&asm[method.name()]);
                let method_name = method_name(&class_name, &mname);
                format!("{method_name}({call_args})")
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = Self::node_to_string(asm[input].clone(), asm, locals, inputs, sig);
                match (target, extend) {
                    (Int::U8, ExtendKind::ZeroExtend) => format!("(uint8_t)({input})"),
                    (Int::U8, ExtendKind::SignExtend) => todo!(),
                    (Int::U16, ExtendKind::ZeroExtend) => format!("(uint16_t)({input})"),
                    (Int::U16, ExtendKind::SignExtend) => todo!(),
                    (Int::U32, ExtendKind::ZeroExtend) => format!("(uint32_t)({input})"),
                    (Int::U32, ExtendKind::SignExtend) => todo!(),
                    (Int::U64, ExtendKind::ZeroExtend) => format!("(uint64_t)({input})"),
                    (Int::U64, ExtendKind::SignExtend) => format!("(uint64_t)(int64_t)({input})"),
                    (Int::U128, ExtendKind::ZeroExtend) => format!("(unsigned __int128)({input})"),
                    (Int::U128, ExtendKind::SignExtend) => todo!(),
                    (Int::USize, ExtendKind::ZeroExtend) => format!("(uintptr_t)({input})"),
                    (Int::USize, ExtendKind::SignExtend) => todo!(),
                    (Int::I8, ExtendKind::ZeroExtend) => todo!(),
                    (Int::I8, ExtendKind::SignExtend) => format!("(int8_t)({input})"),
                    (Int::I16, ExtendKind::ZeroExtend) => todo!(),
                    (Int::I16, ExtendKind::SignExtend) => format!("(int16_t)({input})"),
                    (Int::I32, ExtendKind::ZeroExtend) => format!("(int32_t)(uint32_t)({input})"),
                    (Int::I32, ExtendKind::SignExtend) => format!("(int32_t)({input})"),
                    (Int::I64, ExtendKind::ZeroExtend) => todo!(),
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
                let input = Self::node_to_string(asm[input].clone(), asm, locals, inputs, sig);
                match target {
                    super::Float::F16 => todo!(),
                    super::Float::F32 => format!("(float)({input})"),
                    super::Float::F64 => format!("(double)({input})"),
                    super::Float::F128 => todo!(),
                }
            }
            CILNode::RefToPtr(node_idx) => {
                Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)
            }
            CILNode::PtrCast(node_idx, ptr_cast_res) => {
                let node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig);
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
                let addr = Self::node_to_string(addr, asm, locals, inputs, sig);
                let field = asm[field];
                let name = &asm[field.name()];
                format!("&({addr})->{name}.f")
            }
            CILNode::LdField { addr, field } => {
                let addr = asm[addr].clone();
                let addr_tpe = addr.typecheck(sig, locals, asm).unwrap();
                let addr = Self::node_to_string(addr, asm, locals, inputs, sig);
                let field = asm[field];
                let name = &asm[field.name()];
                match addr_tpe {
                    Type::Ref(_) | Type::Ptr(_) => format!("({addr})->{name}.f"),
                    Type::ClassRef(_) => format!("({addr}).{name}.f"),
                    _ => panic!(),
                }
            }
            CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => {
                if volitale {
                    format!(
                        "*(volitale {tpe}*)({addr})",
                        tpe = c_tpe(asm[tpe], asm),
                        addr = Self::node_to_string(asm[addr].clone(), asm, locals, inputs, sig)
                    )
                } else {
                    format!(
                        "*({addr})",
                        addr = Self::node_to_string(asm[addr].clone(), asm, locals, inputs, sig)
                    )
                }
            }
            CILNode::SizeOf(type_idx) => format!("sizeof({tpe})", tpe = c_tpe(asm[type_idx], asm)),
            CILNode::GetException => todo!(),
            CILNode::IsInst(node_idx, type_idx) => todo!(),
            CILNode::CheckedCast(node_idx, type_idx) => todo!(),
            CILNode::CallI(_) => todo!(),
            CILNode::LocAlloc { size } => todo!(),
            CILNode::LdStaticField(static_field_idx) => {
                let field = asm[static_field_idx];
                let class = asm[field.owner()].clone();
                let fname = method_name(&asm[class.name()], &asm[field.name()]);
                fname.to_string()
            }
            CILNode::LdFtn(method) => {
                let method = asm[method].clone();
                let class = &asm[method.class()];
                let class_name = escape_ident(&asm[class.name()]);
                let mname = escape_ident(&asm[method.name()]);
                method_name(&class_name, &mname)
            }
            CILNode::LdTypeToken(type_idx) => todo!(),
            CILNode::LdLen(node_idx) => todo!(),
            // TODO: loc alloc aligned does not respect the aligement ATM.
            CILNode::LocAllocAlgined { tpe, align } => {
                format!("alloca(sizeof({tpe}))", tpe = c_tpe(asm[tpe], asm))
            }
            CILNode::LdElelemRef { array, index } => todo!(),
            CILNode::UnboxAny { object, tpe } => format!(
                "({object})",
                object = Self::node_to_string(asm[object].clone(), asm, locals, inputs, sig)
            ),
        }
    }
    fn root_to_string(
        root: CILRoot,
        asm: &mut Assembly,
        locals: &[LocalDef],
        inputs: &[(Type, Option<StringIdx>)],
        sig: SigIdx,
    ) -> String {
        match root {
            CILRoot::StLoc(id, node_idx) => match locals[id as usize].0 {
                Some(name) => format!(
                    "{name} = {node};",
                    node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig),
                    name = escape_ident(&asm[name]),
                ),
                None => format!(
                    "L{id} = {node};",
                    node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig),
                ),
            },
            CILRoot::StArg(arg, node_idx) =>match inputs[arg as usize].1 {
                Some(name) => format!(
                    "{name} = {node};",
                    node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig),
                    name = escape_ident(&asm[name]),
                ),
                None => format!(
                    "A{arg} = {node};",
                    node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig),
                ),
            },
            CILRoot::Ret(node_idx) => format!(
                "return {node};",
                node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)
            ),
            CILRoot::Pop(node_idx) => format!(
                "{node};",
                node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)
            ),
            CILRoot::Throw(node_idx) =>  format!(
                "eprintf(\"An error was encoutrered in %s, at %s:%d\\n\",__func__,__FILE__,__LINE__);eprintf(\"%s\\n\",{node}); abort();",
                node = Self::node_to_string(asm[node_idx].clone(), asm, locals, inputs, sig)
            ),
            CILRoot::VoidRet => "return;".into(),
            CILRoot::Break => "".into(),
            CILRoot::Nop => todo!(),
            CILRoot::Branch(binfo) => {
                let (target, _, cond) = binfo.as_ref();
                let Some(cond) = cond else {
                    return format!("goto bb{target};");
                };
                match cond {
                    BranchCond::True(node_idx) => format!(
                        "if({node}) goto bb{target};",
                        node =
                            Self::node_to_string(asm[*node_idx].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::False(node_idx) => format!(
                        "if(!({node})) goto bb{target};",
                        node =
                            Self::node_to_string(asm[*node_idx].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::Eq(lhs, rhs) => format!(
                        "if(({lhs}) == ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig),
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::Ne(lhs, rhs) => format!(
                        "if(({lhs}) != ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig),
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::Lt(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) < ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig),
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::Gt(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) > ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig),
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::Le(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) <= ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig),
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)
                    ),
                    BranchCond::Ge(lhs, rhs, _cmp_kind) => format!(
                        "if(({lhs}) >= ({rhs})) goto bb{target};",
                        lhs = Self::node_to_string(asm[*lhs].clone(), asm, locals, inputs, sig),
                        rhs = Self::node_to_string(asm[*rhs].clone(), asm, locals, inputs, sig)
                    ),
                }
            }
            CILRoot::SourceFileInfo { .. } => "".into(),
            CILRoot::SetField(info) =>{
                let (field,addr,value) = info.as_ref();
                let addr = Self::node_to_string(asm[*addr].clone(), asm, locals, inputs, sig);
                let value = Self::node_to_string(asm[*value].clone(), asm, locals, inputs, sig);
                let field = asm[*field];
                let name = &asm[field.name()];
                format!("({addr})->{name}.f = ({value});")
            }
            CILRoot::Call(info) => {
                let (method, args) = info.as_ref();
                let method = asm[*method].clone();
                let call_args = args
                    .iter()
                    .map(|arg| {
                        format!(
                            "({})",
                            Self::node_to_string(asm[*arg].clone(), asm, locals, inputs, sig)
                        )
                    })
                    .intersperse(",".into())
                    .collect::<String>();
                let class = &asm[method.class()];
                let class_name = escape_ident(&asm[class.name()]);
                let mname = escape_ident(&asm[method.name()]);
                let method_name = method_name(&class_name, &mname);
                format!("{method_name}({call_args});")
            }
            CILRoot::StInd(info) => {
                let (addr, value, tpe, is_volitle) = info.as_ref();
                let addr = Self::node_to_string(asm[*addr].clone(), asm, locals, inputs, sig);
                let value = Self::node_to_string(asm[*value].clone(), asm, locals, inputs, sig);
                if *is_volitle {
                    format!(
                        "*((volitale {tpe}*)({addr})) = ({value});",
                        tpe = c_tpe(*tpe, asm)
                    )
                } else {
                    format!("*({addr}) = ({value});")
                }
            }
            CILRoot::InitBlk(blk) => {
                let (dst, val, count) = blk.as_ref();
                let dst = Self::node_to_string(asm[*dst].clone(), asm, locals, inputs, sig);
                let val = Self::node_to_string(asm[*val].clone(), asm, locals, inputs, sig);
                let count = Self::node_to_string(asm[*count].clone(), asm, locals, inputs, sig);
                format!("memset(({dst}),({val}),({count}));")
            }
            CILRoot::CpBlk(blk) => {
                let (dst, src, len) = blk.as_ref();
                let dst = Self::node_to_string(asm[*dst].clone(), asm, locals, inputs, sig);
                let src = Self::node_to_string(asm[*src].clone(), asm, locals, inputs, sig);
                let len = Self::node_to_string(asm[*len].clone(), asm, locals, inputs, sig);
                format!("memset(({dst}),({src}),({len}));")
            }
            CILRoot::CallI(_) => todo!(),
            CILRoot::ExitSpecialRegion { target, source } => format!("goto bb{target};"),
            CILRoot::ReThrow => todo!(),
            CILRoot::SetStaticField { field, val } => {
                let field = asm[field];
                let class = asm[field.owner()].clone();
                let fname = method_name(&asm[class.name()], &asm[field.name()]);
                let val = Self::node_to_string(asm[val].clone(), asm, locals, inputs, sig);
                format!("{fname} = {val};")
            }
            CILRoot::CpObj { src, dst, tpe } => todo!(),
            CILRoot::Unreachable(string_idx) => todo!(),
        }
    }
    fn export_method_def(
        asm: &mut Assembly,
        def: &MethodDef,
        method_defs: &mut impl Write,
    ) -> std::io::Result<()> {
        let class = &asm[def.class()];
        let class_name = escape_ident(&asm[class.name()]);
        let mname = escape_ident(&asm[def.name()]);
        let method_name = method_name(&class_name, &mname);
        let output = c_tpe(def.ref_to().output(asm), asm);
        match def.resolved_implementation(asm) {
            MethodImpl::MethodBody { blocks, locals } => (),
            MethodImpl::Extern {
                lib,
                preserve_errno,
            } => return Ok(()),
            MethodImpl::Missing => {
                let inputs = def
                    .ref_to()
                    .stack_inputs(asm)
                    .iter()
                    .map(|i| nonvoid_c_type(*i, asm))
                    .intersperse(",".into())
                    .collect::<String>();
                writeln!(
                    method_defs,
                    "{output} {method_name}({inputs}){{eprintf(\"Missing method {mname}\\n\");abort();}}"
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
                    name = escape_ident(&asm[*name]),
                ),
                None => format!("{} A{idx} ", nonvoid_c_type(*tpe, asm)),
            })
            .intersperse(",".into())
            .collect::<String>();
        writeln!(method_defs, "{output} {method_name}({inputs}){{")?;
        let locals: Vec<_> = def.iter_locals(asm).copied().collect();
        for (idx, (local_name, local_type)) in locals.iter().enumerate() {
            match local_name {
                Some(local_name) => writeln!(
                    method_defs,
                    "{local_type} {local_name};",
                    local_name = escape_ident(&asm[*local_name]),
                    local_type = nonvoid_c_type(asm[*local_type], asm),
                ),
                None => writeln!(
                    method_defs,
                    "{local_type} L{idx};",
                    local_type = nonvoid_c_type(asm[*local_type], asm),
                ),
            }?;
        }
        let blocks = def.blocks(asm).unwrap().to_vec();
        for block in blocks {
            writeln!(method_defs, "bb{}:", block.block_id())?;
            for root in block.roots() {
                let root = Self::root_to_string(
                    asm[*root].clone(),
                    asm,
                    &locals[..],
                    &stack_inputs[..],
                    sig,
                );
                writeln!(method_defs, "{root}")?;
            }
        }
        writeln!(method_defs, "}}")
    }
    #[allow(clippy::too_many_arguments)]
    fn export_class(
        &self,
        asm: &mut super::Assembly,
        defid: ClassDefIdx,
        method_decls: &mut impl Write,
        method_defs: &mut impl Write,
        type_defs: &mut impl Write,
        defined_types: &mut FxHashSet<ClassDefIdx>,
        delayed_defs: &mut FxHashSet<ClassDefIdx>,
    ) -> std::io::Result<()> {
        let class = asm[defid].clone();
        // Checks if this def needs to be delayed, if one of its fields is not yet defined
        if !class
            .fields()
            .iter()
            .filter_map(|(tpe, _, _)| tpe.as_class_ref())
            .filter_map(|cref| asm.class_ref_to_def(cref))
            .all(|cdef| defined_types.contains(&cdef))
        {
            delayed_defs.insert(defid);
            return Ok(());
        }
        let class_name = escape_ident(&asm[class.name()]);
        writeln!(type_defs, "typedef union {class_name}{{")?;
        for (field_tpe, fname, offset) in class.fields() {
            let fname = escape_ident(&asm[*fname]);
            let Some(offset) = offset else {
                eprintln!(
                    "ERR: Can't export field {fname} of {class_name}, becuase it has no offset."
                );
                continue;
            };
            let field_tpe = c_tpe(*field_tpe, asm);
            writeln!(
                type_defs,
                "struct {{char pad[{offset}]; {field_tpe} f;}}{fname};"
            )?;
        }
        writeln!(type_defs, "}} {class_name};")?;
        for (sfield_tpe, sfname, is_thread_local) in class.static_fields() {
            let fname = escape_ident(&asm[*sfname]);
            let field_tpe = c_tpe(*sfield_tpe, asm);
            let fname = method_name(&class_name, &fname);
            if *is_thread_local {
                writeln!(type_defs, "static thread_local {field_tpe} {fname};")?;
            } else {
                writeln!(type_defs, "static {field_tpe} {fname};")?;
            }
        }
        for method in class.methods() {
            let mref = &asm[method.0];
            let def = asm[*method].clone();
            let is_extern = def.resolved_implementation(asm).is_extern();
            if !is_extern {
                Self::export_method_decl(asm, mref, method_decls)?;
                Self::export_method_def(asm, &def, method_defs)?;
            }
        }
        defined_types.insert(defid);
        Ok(())
    }
    fn export_to_write(&self, asm: &super::Assembly, out: &mut impl Write) -> std::io::Result<()> {
        let mut asm = asm.clone();
        let mut method_defs = Vec::new();
        let mut method_decls = Vec::new();
        let mut type_defs = Vec::new();
        let mut defined_types: FxHashSet<ClassDefIdx> = FxHashSet::default();
        let mut delayed_defs: FxHashSet<ClassDefIdx> = asm.iter_class_def_ids().cloned().collect();
        let mut delayed_defs_copy: FxHashSet<ClassDefIdx> = FxHashSet::default();
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
                )?;
            }
            delayed_defs_copy.clear();
        }
        out.write_all(include_bytes!("c_header.h"))?;
        out.write_all(b"\n")?;
        out.write_all(&type_defs)?;
        out.write_all(&method_decls)?;
        out.write_all(&method_defs)
    }
}

impl Exporter for CExporter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        // The IL file should be next to the target
        let c_path = target.with_extension("c");
        let mut c_out = std::io::BufWriter::new(std::fs::File::create(&c_path)?);
        self.export_to_write(asm, &mut c_out)?;
        // Needed to ensure the IL file is valid!
        c_out.flush().unwrap();
        drop(c_out);
        let exe_out = target;

        let mut cmd = std::process::Command::new(std::env::var("CC").unwrap_or("cc".to_owned()));
        cmd.arg(c_path)
        .arg("-o")
        .arg(exe_out)
        .arg("-g")
        .arg("-Ofast")
        // .arg("-FOLD") saves up on space, consider enabling.
        ;
        let out = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !(stderr.contains("error") || stderr.contains("fatal")),
            "stdout:{} stderr:{} cmd:{cmd:?}",
            stdout,
            String::from_utf8_lossy(&out.stderr)
        );

        Ok(())
    }
}

#[must_use]
pub fn class_to_mangled(class: &super::ClassRef, asm: &Assembly) -> String {
    let assembly = match class.asm() {
        Some(asm_idx) => asm.get_string(asm_idx).as_ref(),
        None => "",
    };
    format!("{assembly}{name}", name = asm.get_string(class.name()))
}
#[must_use]
pub fn name_sig_class_to_mangled(
    name: &str,
    sig: super::SigIdx,
    class: Option<ClassRefIdx>,
    asm: &Assembly,
) -> String {
    let class = match class {
        Some(_) => todo!(),
        None => todo!(),
    };
}
