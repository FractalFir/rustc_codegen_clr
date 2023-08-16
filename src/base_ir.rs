use crate::IString;
use serde::{Deserialize, Serialize};
// An IR close, but not exactly equivalent to the CoreCLR IR.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum BaseIR {
    LDConstI8(i8),
    LDConstI32(i32),
    LDConstI64(i64),
    STArg(u32),
    LDArg(u32),
    STLoc(u32),
    LDLoc(u32),
    Add,
    Sub,
    Mul,
    Rem,
    Shl,
    Shr,
    Eq,
    Gt,
    Return,
    ConvF32,
    ConvI32,
    ConvI32Checked,
    ConvI8,
    Nop,
    //Not a real instruction, but a marker for a basic block.
    BBLabel { bb_id: u32 },
    BEq { target: u32 },
    GoTo { target: u32 },
}
impl BaseIR {
    pub(crate) fn clr_ir(&self) -> IString {
        match self {
            Self::BBLabel { bb_id } => format!("\tBB_{bb_id}:\n"),
            Self::BEq { target } => format!("\tbeq BB_{target}\n"),
            //Self::BGt{target} => format!("\tbgt BB_{target}\n"),
            Self::GoTo { target } => format!("\tbr BB_{target}\n"),
            Self::LDArg(arg) => format!("\tldarg {arg}\n"),
            Self::STArg(arg) => format!("\tstarg {arg}\n"),
            Self::LDLoc(arg) => format!("\tldloc {arg}\n"),
            Self::STLoc(arg) => format!("\tstloc {arg}\n"),
            Self::Return => "\tret\n".into(),
            Self::Add => "\tadd\n".into(),
            Self::Sub => "\tadd\n".into(),
            Self::Mul => "\tmul\n".into(),
            Self::Rem => "\trem\n".into(),
            Self::Shl => "\tshl\n".into(),
            Self::Shr => "\tshr\n".into(),
            Self::Eq => "\tceq\n".into(),
            Self::Gt => "\tcgt\n".into(),
            Self::LDConstI8(i8const) => format!("\tldc.i4.s {i8const}\n"),
            Self::LDConstI32(i32const) => format!("\tldc.i4 {i32const}\n"),
            Self::LDConstI64(i64const) => format!("\tldc.i8 {i64const}\n"),
            Self::ConvF32 => "\tconv.r4\n".into(),
            Self::ConvI8 => "\tconv.i1\n".into(),
            Self::ConvI32 => "\tconv.i4\n".into(),
            Self::ConvI32Checked => "\tconv.ovf.i4\n".into(),
            Self::Nop => "\tnop\n".into(), //_=>format!("\t//Comment!\n"),
        }
        .into()
    }
}
