use serde::{Deserialize, Serialize};

use super::bimap::BiMapIndex;
use super::field::StaticFieldIdx;

use super::{bimap::IntoBiMapIndex, Assembly, Const, Int, MethodRefIdx, SigIdx, TypeIdx};
use super::{ClassRef, FieldIdx, Float};
use crate::cil_node::CILNode as V1Node;
use crate::v2::Type;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NodeIdx(BiMapIndex);
impl IntoBiMapIndex for NodeIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum CILNode {
    Const(Box<Const>),
    BinOp(NodeIdx, NodeIdx, BinOp),
    UnOp(NodeIdx, UnOp),
    LdLoc(u32),
    LdLocA(u32),
    LdArg(u32),
    LdArgA(u32),
    Call(Box<(MethodRefIdx, Box<[NodeIdx]>)>),
    IntCast {
        input: NodeIdx,
        target: Int,
        extend: ExtendKind,
    },
    FloatCast {
        input: NodeIdx,
        target: Float,
        is_signed: bool,
    },
    RefToPtr(NodeIdx),
    /// Changes the type of a pointer to `PtrCastRes`
    PtrCast(NodeIdx, Box<PtrCastRes>),
    /// Loads the address of a field at `addr`
    LdFieldAdress {
        addr: NodeIdx,
        field: FieldIdx,
    },
    /// Loads the value of a field at `addr`
    LdField {
        addr: NodeIdx,
        field: FieldIdx,
    },
    /// Loads a value of `tpe` at `addr`
    LdInd {
        addr: NodeIdx,
        tpe: TypeIdx,
        volitale: bool,
    },
    /// Calcualtes the size of a type.
    SizeOf(TypeIdx),
    /// Gets the currenrt exception, if it exisits. UB outside an exception handler.
    GetException,
    /// Checks if the object is an instace of a class.
    IsInst(NodeIdx, TypeIdx),
    /// Casts  the object to instace of a clsass.
    CheckedCast(NodeIdx, TypeIdx),
    /// Calls fn pointer with args
    CallI(Box<(NodeIdx, SigIdx, Box<[NodeIdx]>)>),
    /// Allocates memory from a local pool. It will get freed when this function return
    LocAlloc {
        size: NodeIdx,
    },
    /// Loads a static field at descr
    LdStaticField(StaticFieldIdx),
    /// Loads a static field at descr
    LdStaticFieldAdress(StaticFieldIdx),
    /// Loads a pointer to a function
    LdFtn(MethodRefIdx),
    /// Loads a "type token"
    LdTypeToken(TypeIdx),
    /// Gets the length of a platform array
    LdLen(NodeIdx),
    /// Allocates a local buffer sizeof type, and aligned to algin.
    LocAllocAlgined {
        tpe: TypeIdx,
        align: u64,
    },
    /// Loads a reference to array element at index.
    LdElelemRef {
        array: NodeIdx,
        index: NodeIdx,
    },
    /// Turns a managed reference to object into type
    UnboxAny {
        object: NodeIdx,
        tpe: TypeIdx,
    },
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum PtrCastRes {
    Ptr(TypeIdx),
    Ref(TypeIdx),
    FnPtr(SigIdx),
    USize,
    ISize,
}
impl PtrCastRes {
    pub fn as_type(&self) -> Type {
        match self {
            PtrCastRes::Ptr(type_idx) => Type::Ptr(*type_idx),
            PtrCastRes::Ref(type_idx) => Type::Ref(*type_idx),
            PtrCastRes::FnPtr(sig_idx) => Type::FnPtr(*sig_idx),
            PtrCastRes::USize => Type::Int(Int::USize),
            PtrCastRes::ISize => Type::Int(Int::ISize),
        }
    }
}
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]

pub enum ExtendKind {
    ZeroExtend,
    SignExtend,
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MethodKind {
    Static,
    Instance,
    Virtual,
    Constructor,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum UnOp {
    Not,
    Neg,
}
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]

pub enum BinOp {
    Add,
    Eq,
    Sub,
    Mul,
    LtUn,
    Lt,
    GtUn,
    Gt,
    Or,
    XOr,
    And,
    Rem,
    RemUn,
    Shl,
    Shr,
    ShrUn,
    DivUn,
    Div,
}
impl BinOp {
    /// All binary operations, including signed and unsinged varaiants. For only one varaint, use  [`Self::ALL_DISTINCT_OPS`]
    pub const ALL_OPS: [Self; 18] = [
        BinOp::Add,
        BinOp::Eq,
        BinOp::Sub,
        BinOp::Mul,
        BinOp::LtUn,
        BinOp::Lt,
        BinOp::GtUn,
        BinOp::Gt,
        BinOp::Or,
        BinOp::XOr,
        BinOp::And,
        BinOp::Rem,
        BinOp::RemUn,
        BinOp::Shl,
        BinOp::Shr,
        BinOp::ShrUn,
        BinOp::DivUn,
        BinOp::Div,
    ];
    /// [`Self::ALL_OPS`], not including the unsinged variants.
    pub const ALL_DISTINCT_OPS: [Self; 13] = [
        BinOp::Add,
        BinOp::Eq,
        BinOp::Sub,
        BinOp::Mul,
        BinOp::Lt,
        BinOp::Gt,
        BinOp::Or,
        BinOp::XOr,
        BinOp::And,
        BinOp::Rem,
        BinOp::Shl,
        BinOp::Shr,
        BinOp::Div,
    ];
    /// Returns a short name descirbing this operation.
    /// WARNING: this function will return the same name for signed and unsinged variants!
    /// ```
    /// # use cilly::v2::BinOp;
    /// assert_eq!(BinOp::Eq.name(),"eq");
    /// assert_eq!(BinOp::Add.name(),"add");
    /// // Signed and unsinged variant have the same name!
    /// assert_eq!(BinOp::Lt.name(),BinOp::LtUn.name());
    /// assert_eq!(BinOp::Lt.name(),"lt");
    /// assert_eq!(BinOp::LtUn.name(),"lt");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            BinOp::Add => "add",
            BinOp::Eq => "eq",
            BinOp::Sub => "sub",
            BinOp::Mul => "mul",
            BinOp::LtUn | BinOp::Lt => "lt",
            BinOp::GtUn | BinOp::Gt => "gt",
            BinOp::Or => "or",
            BinOp::XOr => "xor",
            BinOp::And => "and",
            BinOp::Rem | BinOp::RemUn => "mod",
            BinOp::Shl => "shl",
            BinOp::Shr | BinOp::ShrUn => "shr",
            BinOp::DivUn | BinOp::Div => "div",
        }
    }
    /// Returns the name of the .NET operation this binop corresponds to.
    /// ```
    /// # use cilly::v2::BinOp;
    /// assert_eq!(BinOp::Eq.dotnet_name(),"op_Equality");
    /// assert_eq!(BinOp::Add.dotnet_name(),"op_Addition");
    /// // Signed and unsinged variant have the same name!
    /// assert_eq!(BinOp::Lt.dotnet_name(),BinOp::LtUn.dotnet_name());
    /// assert_eq!(BinOp::Lt.dotnet_name(),"op_LessThan");
    /// assert_eq!(BinOp::LtUn.dotnet_name(),"op_LessThan");
    /// ```
    pub fn dotnet_name(&self) -> &str {
        match self {
            BinOp::Add => "op_Addition",
            BinOp::Eq => "op_Equality",
            BinOp::Sub => "op_Subtraction",
            BinOp::Mul => "op_Multiply",
            BinOp::LtUn | BinOp::Lt => "op_LessThan",
            BinOp::GtUn | BinOp::Gt => "op_GreaterThan",
            BinOp::Or => "op_BitwiseOr",
            BinOp::XOr => "op_ExclusiveOr",
            BinOp::And => "op_BitwiseAnd",
            BinOp::Rem | BinOp::RemUn => "op_Modulus",
            BinOp::Shl => "op_LeftShift",
            BinOp::Shr | BinOp::ShrUn => "op_RightShift",
            BinOp::DivUn | BinOp::Div => "op_Division",
        }
    }
}
impl CILNode {
    /// Returns all the nodes this node references.
    /// ```
    /// # use cilly::v2::*;
    /// # let mut asm = Assembly::default();
    /// let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
    /// let ldloc_1 = asm.alloc_node(CILNode::LdLoc(1));
    /// let binop = CILNode::BinOp(ldarg_0,ldloc_1,BinOp::Add);
    /// // Two child nodes - ldarg_0 and ldloc_1
    /// assert_eq!(binop.child_nodes(),vec![ldarg_0,ldloc_1]);
    /// ```
    pub fn child_nodes(&self) -> Vec<NodeIdx> {
        match self {
            CILNode::Const(_)
            | CILNode::LdLoc(_)
            | CILNode::LdLocA(_)
            | CILNode::LdArg(_)
            | CILNode::LdArgA(_)
            | CILNode::SizeOf(_)
            | CILNode::LocAllocAlgined { .. }
            | CILNode::LdFtn(_)
            | CILNode::LdTypeToken(_)
            | CILNode::LdStaticField(_)
            | CILNode::LdStaticFieldAdress(_)
            | CILNode::GetException => vec![],
            CILNode::UnOp(node_idx, _)
            | CILNode::RefToPtr(node_idx)
            | CILNode::PtrCast(node_idx, _)
            | CILNode::LdLen(node_idx)
            | CILNode::LdFieldAdress { addr: node_idx, .. }
            | CILNode::LdField { addr: node_idx, .. }
            | CILNode::LdInd { addr: node_idx, .. }
            | CILNode::LocAlloc { size: node_idx }
            | CILNode::IsInst(node_idx, _)
            | CILNode::CheckedCast(node_idx, _)
            | CILNode::IntCast {
                input: node_idx, ..
            }
            | CILNode::FloatCast {
                input: node_idx, ..
            }
            | CILNode::LdElelemRef {
                array: node_idx, ..
            }
            | CILNode::UnboxAny {
                object: node_idx, ..
            } => vec![*node_idx],
            CILNode::BinOp(lhs, rhs, _) => vec![*lhs, *rhs],
            CILNode::Call(info) => {
                let (_, args) = info.as_ref();
                args.to_vec()
            }
            CILNode::CallI(info) => {
                let (fnptr, _, args) = info.as_ref();
                let mut res = vec![*fnptr];
                res.extend(args);
                res
            }
        }
    }
    /// Turns a native object handle into a special handle of type [`Int::ISize`]
    #[must_use]
    pub fn ref_to_handle(&self, asm: &mut Assembly) -> Self {
        let gc_handle = ClassRef::gc_handle(asm);
        let alloc = asm.alloc_string("Alloc");
        let alloc = asm.class_ref(gc_handle).clone().static_mref(
            &[Type::PlatformObject],
            Type::ClassRef(gc_handle),
            alloc,
            asm,
        );
        let op_explict = asm.alloc_string("op_Explicit");
        let op_explict = asm.class_ref(gc_handle).clone().static_mref(
            &[Type::ClassRef(gc_handle)],
            Type::Int(Int::ISize),
            op_explict,
            asm,
        );
        let arg = asm.alloc_node(self.clone());
        let alloc = asm.alloc_node(CILNode::Call(Box::new((alloc, [arg].into()))));
        CILNode::Call(Box::new((op_explict, [alloc].into())))
    }

    // This function has to be complex
    #[allow(clippy::too_many_lines)]
    pub fn from_v1(v1: &V1Node, asm: &mut Assembly) -> Self {
        match v1 {
            // Varaible access
            V1Node::LDArg(arg) => CILNode::LdArg(*arg),
            V1Node::LDLoc(arg) => CILNode::LdLoc(*arg),
            V1Node::LDArgA(arg) => CILNode::LdArgA(*arg),
            V1Node::LDLocA(arg) => CILNode::LdLocA(*arg),
            // Ptr deref
            V1Node::LDIndBool { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Bool),
                    volitale: false,
                }
            }
            V1Node::LDIndU8 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::U8)),
                    volitale: false,
                }
            }
            V1Node::LDIndU16 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::U16)),
                    volitale: false,
                }
            }
            V1Node::LDIndU32 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::U32)),
                    volitale: false,
                }
            }
            V1Node::LDIndU64 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::U64)),
                    volitale: false,
                }
            }
            V1Node::LDIndUSize { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::USize)),
                    volitale: false,
                }
            }
            V1Node::LDIndI8 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::I8)),
                    volitale: false,
                }
            }
            V1Node::LDIndI16 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::I16)),
                    volitale: false,
                }
            }
            V1Node::LDIndI32 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::I32)),
                    volitale: false,
                }
            }
            V1Node::LDIndI64 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::I64)),
                    volitale: false,
                }
            }
            V1Node::LDIndISize { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Int(Int::ISize)),
                    volitale: false,
                }
            }
            V1Node::LDIndF32 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Float(Float::F32)),
                    volitale: false,
                }
            }
            V1Node::LDIndF64 { ptr } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(Type::Float(Float::F64)),
                    volitale: false,
                }
            }
            V1Node::LdObj { ptr, obj } => {
                let ptr = Self::from_v1(ptr, asm);
                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(*obj.as_ref()),
                    volitale: false,
                }
            }
            V1Node::LDIndPtr { ptr, loaded_ptr } => {
                let ptr = Self::from_v1(ptr, asm);

                Self::LdInd {
                    addr: asm.alloc_node(ptr),
                    tpe: asm.alloc_type(*loaded_ptr.as_ref()),
                    volitale: false,
                }
            }
            // Casts
            V1Node::ZeroExtendToU64(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::U64,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::SignExtendToU64(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::U64,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ZeroExtendToUSize(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::USize,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::SignExtendToUSize(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::USize,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ConvU8(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::U8,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::ConvU16(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::U16,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::ConvU32(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::U32,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::SignExtendToI64(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::I64,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ZeroExtendToISize(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::ISize,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::SignExtendToISize(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::ISize,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ConvI8(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::I8,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ConvI16(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::I16,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ConvI32(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.alloc_node(node),
                    target: Int::I32,
                    extend: ExtendKind::SignExtend,
                }
            }
            V1Node::ConvF32(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::FloatCast {
                    input: asm.alloc_node(node),
                    target: Float::F32,
                    is_signed: true,
                }
            }
            V1Node::ConvF64(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::FloatCast {
                    input: asm.alloc_node(node),
                    target: Float::F64,
                    is_signed: true,
                }
            }
            V1Node::ConvF64Un(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::FloatCast {
                    input: asm.alloc_node(node),
                    target: Float::F64,
                    is_signed: false,
                }
            }
            V1Node::MRefToRawPtr(inner) => {
                let raw = Self::from_v1(inner, asm);
                CILNode::RefToPtr(asm.alloc_node(raw))
            }
            V1Node::CastPtr { val, new_ptr } => {
                let val = Self::from_v1(val, asm);

                let ptr = match &**new_ptr {
                    Type::Int(Int::USize) => PtrCastRes::USize,
                    Type::Int(Int::ISize) => PtrCastRes::ISize,
                    Type::Ptr(inner) => PtrCastRes::Ptr(*inner),
                    Type::Ref(inner) => PtrCastRes::Ref(*inner),
                    Type::FnPtr(sig) => PtrCastRes::FnPtr(*sig),
                    _ => panic!("Type {new_ptr:?} is not a pointer."),
                };
                CILNode::PtrCast(asm.alloc_node(val), Box::new(ptr))
            }
            // Binops
            V1Node::Add(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Add)
            }
            V1Node::Sub(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Sub)
            }
            V1Node::Mul(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Mul)
            }
            V1Node::Eq(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Eq)
            }
            V1Node::Or(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Or)
            }
            V1Node::XOr(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::XOr)
            }
            V1Node::And(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::And)
            }
            V1Node::LtUn(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::LtUn)
            }
            V1Node::Lt(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Lt)
            }
            V1Node::GtUn(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::GtUn)
            }
            V1Node::Gt(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Gt)
            }
            V1Node::Rem(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Rem)
            }
            V1Node::RemUn(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::RemUn)
            }
            V1Node::Shl(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Shl)
            }
            V1Node::Shr(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Shr)
            }
            V1Node::ShrUn(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::ShrUn)
            }
            V1Node::Div(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::Div)
            }
            V1Node::DivUn(lhs, rhs) => {
                let lhs = Self::from_v1(lhs, asm);
                let rhs = Self::from_v1(rhs, asm);
                Self::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), BinOp::DivUn)
            }
            // Unops
            V1Node::Not(val) => {
                let val = Self::from_v1(val, asm);
                Self::UnOp(asm.alloc_node(val), UnOp::Not)
            }
            V1Node::Neg(val) => {
                let val = Self::from_v1(val, asm);
                Self::UnOp(asm.alloc_node(val), UnOp::Neg)
            }
            // Field access
            V1Node::LDField { addr, field } => {
                let addr = Self::from_v1(addr, asm);
                Self::LdField {
                    addr: asm.alloc_node(addr),
                    field: *field,
                }
            }
            V1Node::LDFieldAdress { addr, field } => {
                let addr = Self::from_v1(addr, asm);
                Self::LdFieldAdress {
                    addr: asm.alloc_node(addr),
                    field: *field,
                }
            }
            // Calls
            V1Node::Call(callargs) => {
                let args: Box<[_]> = callargs
                    .args
                    .iter()
                    .map(|arg| {
                        let node = Self::from_v1(arg, asm);
                        asm.alloc_node(node)
                    })
                    .collect();
                Self::Call(Box::new((callargs.site, args)))
            }
            V1Node::CallVirt(callargs) => {
                let args: Box<[_]> = callargs
                    .args
                    .iter()
                    .map(|arg| {
                        let node = Self::from_v1(arg, asm);
                        asm.alloc_node(node)
                    })
                    .collect();

                Self::Call(Box::new((callargs.site, args)))
            }
            V1Node::NewObj(callargs) => {
                let args: Box<[_]> = callargs
                    .args
                    .iter()
                    .map(|arg| {
                        let node = Self::from_v1(arg, asm);
                        asm.alloc_node(node)
                    })
                    .collect();
                Self::Call(Box::new((callargs.site, args)))
            }
            // Special
            V1Node::GetException => Self::GetException,
            // Consts
            V1Node::LdStr(string) => {
                let string = asm.alloc_string(string.clone());
                Const::PlatformString(string).into()
            }
            V1Node::SizeOf(tpe) => Self::SizeOf(asm.alloc_type(*tpe.as_ref())),
            V1Node::LDTypeToken(tpe) => Self::LdTypeToken(asm.alloc_type(*tpe.as_ref())),
            V1Node::LdcU64(val) => Const::U64(*val).into(),
            V1Node::LdcU32(val) => Const::U32(*val).into(),
            V1Node::LdcU16(val) => Const::U16(*val).into(),
            V1Node::LdcU8(val) => Const::U8(*val).into(),
            V1Node::LdcI64(val) => Const::I64(*val).into(),
            V1Node::LdcI32(val) => Const::I32(*val).into(),
            V1Node::LdcI16(val) => Const::I16(*val).into(),
            V1Node::LdcI8(val) => Const::I8(*val).into(),
            V1Node::LdFalse => Const::Bool(false).into(),
            V1Node::LdTrue => Const::Bool(true).into(),
            V1Node::LdcF64(val) => Const::F64(*val).into(),
            V1Node::LdcF32(val) => Const::F32(*val).into(),
            // Special
            V1Node::IsInst(combined) => {
                let (val, tpe) = combined.as_ref();

                let tpe = asm.alloc_type(Type::ClassRef(*tpe));
                let val = Self::from_v1(val, asm);

                Self::IsInst(asm.alloc_node(val), tpe)
            }
            V1Node::CheckedCast(combined) => {
                let (val, tpe) = combined.as_ref();

                let val = Self::from_v1(val, asm);
                let tpe = asm.alloc_type(Type::ClassRef(*tpe));
                Self::CheckedCast(asm.alloc_node(val), tpe)
            }
            V1Node::CallI(sig_ptr_args) => {
                let sig = asm.alloc_sig(sig_ptr_args.0.clone());
                let ptr = Self::from_v1(&sig_ptr_args.1, asm);
                let ptr = asm.alloc_node(ptr);
                let args: Box<[_]> = sig_ptr_args
                    .2
                    .iter()
                    .map(|arg| {
                        let arg = Self::from_v1(arg, asm);
                        asm.alloc_node(arg)
                    })
                    .collect();
                Self::CallI(Box::new((ptr, sig, args)))
            }
            V1Node::LocAlloc { size } => {
                let size = Self::from_v1(size, asm);
                let size = asm.alloc_node(size);
                CILNode::LocAlloc { size }
            }
            V1Node::LocAllocAligned { tpe, align } => {
                let tpe = asm.alloc_type(*tpe.as_ref());
                CILNode::LocAllocAlgined { tpe, align: *align }
            }
            V1Node::AddressOfStaticField(sfld) => Self::LdStaticFieldAdress(asm.alloc_sfld(**sfld)),
            V1Node::LDStaticField(sfld) => Self::LdStaticField(asm.alloc_sfld(**sfld)),
            V1Node::LDFtn(method_ref) => Self::LdFtn(*method_ref),
            V1Node::Volatile(inner) => {
                let mut tmp = Self::from_v1(inner, asm);
                if let Self::LdInd { volitale, .. } = &mut tmp {
                    *volitale = true;
                } else {
                    panic!()
                }
                tmp
            }
            V1Node::LDLen { arr } => {
                let arr = Self::from_v1(arr, asm);
                let arr = asm.alloc_node(arr);
                Self::LdLen(arr)
            }
            V1Node::LDElelemRef { arr, idx } => {
                let arr = Self::from_v1(arr, asm);
                let array = asm.alloc_node(arr);
                let idx = Self::from_v1(idx, asm);
                let index = asm.alloc_node(idx);
                Self::LdElelemRef { array, index }
            }
            V1Node::UnboxAny(object, tpe) => {
                let object = Self::from_v1(object, asm);
                let object = asm.alloc_node(object);
                let tpe = asm.alloc_type(*tpe.as_ref());
                Self::UnboxAny { object, tpe }
            }
            V1Node::LdNull(tpe) => Self::Const(Box::new(Const::Null(*tpe))),
            _ => todo!("v1:{v1:?}"),
        }
    }
}
impl CILNode {
    /// Changes the node by applying the `map` closure to each node. This process is
    // The complexity of this function is unavoidable.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn map(
        self,
        asm: &mut Assembly,
        map: &mut impl FnMut(Self, &mut Assembly) -> Self,
    ) -> Self {
        match self {
            CILNode::Const(_)
            | CILNode::LdLoc(_)
            | CILNode::LdLocA(_)
            | CILNode::LdArg(_)
            | CILNode::LdArgA(_)
            | CILNode::SizeOf(_)
            | CILNode::GetException
            | CILNode::LocAllocAlgined { .. }
            | CILNode::LdStaticField(_)
            | CILNode::LdStaticFieldAdress(_)
            | CILNode::LdFtn(_)
            | CILNode::LdTypeToken(_) => map(self, asm),
            CILNode::BinOp(lhs, rhs, op) => {
                let lhs = asm.get_node(lhs).clone().map(asm, map);
                let rhs = asm.get_node(rhs).clone().map(asm, map);
                let node = CILNode::BinOp(asm.alloc_node(lhs), asm.alloc_node(rhs), op);
                map(node, asm)
            }
            CILNode::UnOp(lhs, op) => {
                let lhs = asm.get_node(lhs).clone().map(asm, map);
                let node = CILNode::UnOp(asm.alloc_node(lhs), op);
                map(node, asm)
            }
            CILNode::Call(call_info) => {
                let (method_id, args) = *call_info;
                let args = args
                    .iter()
                    .map(|arg| {
                        let node = asm.get_node(*arg).clone().map(asm, map);
                        asm.alloc_node(node)
                    })
                    .collect();

                let node = CILNode::Call(Box::new((method_id, args)));
                map(node, asm)
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = asm.get_node(input).clone().map(asm, map);
                let node = CILNode::IntCast {
                    input: asm.alloc_node(input),
                    target,
                    extend,
                };
                map(node, asm)
            }
            CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => {
                let input = asm.get_node(input).clone().map(asm, map);
                let node = CILNode::FloatCast {
                    input: asm.alloc_node(input),
                    target,
                    is_signed,
                };
                map(node, asm)
            }
            CILNode::RefToPtr(input) => {
                let input = asm.get_node(input).clone().map(asm, map);
                let node = CILNode::RefToPtr(asm.alloc_node(input));
                map(node, asm)
            }
            CILNode::PtrCast(input, tpe) => {
                let input = asm.get_node(input).clone().map(asm, map);
                let node = CILNode::PtrCast(asm.alloc_node(input), tpe);
                map(node, asm)
            }
            CILNode::LdFieldAdress { addr, field } => {
                let addr = asm.get_node(addr).clone().map(asm, map);
                let node = CILNode::LdFieldAdress {
                    addr: asm.alloc_node(addr),
                    field,
                };
                map(node, asm)
            }
            CILNode::LdField { addr, field } => {
                let addr = asm.get_node(addr).clone().map(asm, map);
                let node = CILNode::LdField {
                    addr: asm.alloc_node(addr),
                    field,
                };
                map(node, asm)
            }
            CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => {
                let addr = asm.get_node(addr).clone().map(asm, map);
                let node = CILNode::LdInd {
                    addr: asm.alloc_node(addr),
                    tpe,
                    volitale,
                };
                map(node, asm)
            }
            CILNode::IsInst(object, tpe) => {
                let object = asm.get_node(object).clone().map(asm, map);
                let node = CILNode::IsInst(asm.alloc_node(object), tpe);
                map(node, asm)
            }
            CILNode::CheckedCast(object, tpe) => {
                let object = asm.get_node(object).clone().map(asm, map);
                let node = CILNode::CheckedCast(asm.alloc_node(object), tpe);
                map(node, asm)
            }
            CILNode::CallI(call_info) => {
                let (ptr, sig, args) = *call_info;
                let args = args
                    .iter()
                    .map(|arg| {
                        let node = asm.get_node(*arg).clone().map(asm, map);
                        asm.alloc_node(node)
                    })
                    .collect();
                let ptr = asm.get_node(ptr).clone().map(asm, map);
                let node = CILNode::CallI(Box::new((asm.alloc_node(ptr), sig, args)));
                map(node, asm)
            }
            CILNode::LocAlloc { size } => {
                let size = asm.get_node(size).clone().map(asm, map);
                let node = CILNode::LocAlloc {
                    size: asm.alloc_node(size),
                };
                map(node, asm)
            }

            CILNode::LdLen(input) => {
                let input = asm.get_node(input).clone().map(asm, map);
                let node = CILNode::LdLen(asm.alloc_node(input));
                map(node, asm)
            }

            CILNode::LdElelemRef { array, index } => {
                let array = asm.get_node(array).clone().map(asm, map);
                let index = asm.get_node(index).clone().map(asm, map);
                let node = CILNode::LdElelemRef {
                    array: asm.alloc_node(array),
                    index: asm.alloc_node(index),
                };
                map(node, asm)
            }
            CILNode::UnboxAny { object, tpe } => {
                let object = asm.get_node(object).clone().map(asm, map);
                let node = CILNode::UnboxAny {
                    object: asm.alloc_node(object),
                    tpe,
                };
                map(node, asm)
            }
        }
    }
}
