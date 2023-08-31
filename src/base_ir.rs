use crate::{FunctionSignature, IString,types::Type};
use serde::{Deserialize, Serialize};
// An IR close, but not exactly equivalent to the CoreCLR IR.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum BaseIR {
    DebugComment(IString),
    //LDConstI8(i8),
    LDConstI32(i32),
    LDConstI64(i64),
    LDConstF32(f32),
    STIndIn(u8),
    STIndI,
    STIndR8,
    STIndR4,
    LDIndIn(u8),
    LDIndI,
    LDIndR8,
    LDIndR4,
    LDConstString(String),
    STArg(u32),
    LDArg(u32),
    LDArgA(u32),
    STLoc(u32),
    LDLoc(u32),
    LDLocA(u32),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Shl,
    Shr,
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
    Xor,
    Or,
    And,
    Not,
    Return,
    ConvI32Checked,
    ConvI8,
    ConvU8,
    ConvI16,
    ConvU16,
    ConvF32,
    ConvI32,
    ConvU32,
    ConvF64,
    ConvI64,
    ConvU64,
    ConvI16Checked,
    ConvI,
    ConvU,
    Nop,
    CallStatic {
        sig: FunctionSignature,
        function_name: IString,
    },
    Call {
        sig: FunctionSignature,
        function_name: IString,
    },
    //Not a real instruction, but a marker for a basic block.
    BBLabel {
        bb_id: u32,
    },
    BEq {
        target: u32,
    },
    GoTo {
        target: u32,
    },
    NewObj {
        ctor_fn: String,
    },
    LDField {
        field_parent: IString,
        field_name: IString,
        field_type: Type,
    },
    LDFieldAdress {
        field_parent: IString,
        field_name: IString,
        field_type: Type,
    },
    STField {
        field_parent: IString,
        field_name: IString,
        field_type: Type,
    },
    SizeOf(IString),
    LDObj(IString),
    STObj(IString),
    Throw,
    InitObj(IString),
}
impl BaseIR {
    pub(crate) fn remove_void_local(&mut self, void_locals: &[usize]) {
        match self {
            Self::LDLoc(local) => {
                if void_locals
                    .iter()
                    .any(|void_local| *void_local as u32 == *local)
                {
                    *self = BaseIR::Nop;
                }
            }
            Self::LDLocA(local) => {
                if void_locals
                    .iter()
                    .any(|void_local| *void_local as u32 == *local)
                {
                    *self = BaseIR::Nop;
                }
            }
            Self::STLoc(local) => {
                if void_locals
                    .iter()
                    .any(|void_local| *void_local as u32 == *local)
                {
                    *self = BaseIR::Nop;
                }
            }
            _ => (),
        }
    }
}
