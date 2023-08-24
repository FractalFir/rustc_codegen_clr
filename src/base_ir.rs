use crate::{FunctionSignature, IString, VariableType};
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
    NEq,
    Gt,
    Lt,
    Ge,
    Le,
    Xor,
    Or,
    And,
    Return,
    ConvF32,
    ConvI32,
    ConvI32Checked,
    ConvI8,
    ConvI16Checked,
    ConvI,
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
        field_type: VariableType,
    },
    LDFieldAdress {
        field_parent: IString,
        field_name: IString,
        field_type: VariableType,
    },
    STField {
        field_parent: IString,
        field_name: IString,
        field_type: VariableType,
    },
    SizeOf(IString),
    LDObj(IString),
    STObj(IString),
    Throw,
    InitObj(IString),
}
impl BaseIR {
    pub(crate) fn clr_ir(&self) -> IString {
        match self {
            Self::BBLabel { bb_id } => format!("\tBB_{bb_id}:\n"),
            Self::BEq { target } => format!("\tbeq BB_{target}\n"),
            //Self::BGt{target} => format!("\tbgt BB_{target}\n"),
            Self::GoTo { target } => format!("\tbr BB_{target}\n"),
            Self::LDArg(arg) => match arg {
                0..=3 => format!("\tldarg.{arg}\n"),
                4..=255 => format!("\tldarg.s {arg}\n"),
                _ => format!("\tldarg {arg}\n"),
            },
            Self::STArg(arg) => match arg {
                0..=255 => format!("\tstarg.s {arg}\n"),
                _ => format!("\ttstarg{arg}\n"),
            },
            Self::LDArgA(arg) => {
                if *arg < 256 {
                    format!("\tldarga.s {arg}\n")
                } else {
                    format!("\tldarga {arg}\n")
                }
            }
            Self::LDLoc(arg) => match arg {
                0..=3 => format!("\tldloc.{arg}\n"),
                4..=255 => format!("\tldloc.s {arg}\n"),
                _ => format!("\tldloc {arg}\n"),
            },
            Self::STLoc(arg) => match arg {
                0..=3 => format!("\tstloc.{arg}\n"),
                4..=255 => format!("\tstloc.s {arg}\n"),
                _ => format!("\tstloc {arg}\n"),
            },
            Self::LDLocA(arg) => {
                if *arg < 256 {
                    format!("\tldloca.s {arg}\n")
                } else {
                    format!("\tldloca {arg}\n")
                }
            }
            Self::Return => "\tret\n".into(),
            Self::Add => "\tadd\n".into(),
            Self::Sub => "\tadd\n".into(),
            Self::Mul => "\tmul\n".into(),
            Self::Rem => "\trem\n".into(),
            Self::Div => "\tdiv\n".into(),
            Self::Shl => "\tshl\n".into(),
            Self::Shr => "\tshr\n".into(),
            Self::Eq => "\tceq\n".into(),
            Self::NEq => "\tceq\n\tldc.i4.0\n\tceq\n".into(),
            Self::Gt => "\tcgt\n".into(),
            Self::Lt => "\tclt\n".into(),
            Self::Ge => "\tcge\n".into(),
            Self::Le => "\tcle\n".into(),
            Self::Xor => "\txor\n".into(),
            Self::Or => "\tor\n".into(),
            Self::And => "\tand\n".into(),
            //Self::LDConstI8(i8const) => format!("\tldc.i4.s {i8const}\n"),
            Self::LDConstI32(i32const) => match i32const {
                -1 => "\tldc.i4.m1".into(),
                0..=8 => format!("\tldc.i4.{i32const}\n"),
                9..=127 => format!("\tldc.i4.s {i32const}\n"),
                _ => format!("\tldc.i4 {i32const}\n"),
            },
            Self::LDConstI64(i32const) => match i32const {
                -1 => "\tldc.i4.m1".into(),
                0..=8 => format!("\tldc.i4.{i32const}\n"),
                9..=127 => format!("\tldc.i4.s {i32const}\n"),
                127..=32_767 => format!("\tldc.i4 {i32const}\n"),
                _ => format!("\tldc.i8 {i32const}\n"),
            },
            Self::LDConstF32(f32const) => format!("\tldc.r4 {f32const}\n"),
            Self::ConvF32 => "\tconv.r4\n".into(),
            Self::ConvI => "\tconv.i\n".into(),
            Self::ConvI8 => "\tconv.i1\n".into(),
            Self::ConvI32 => "\tconv.i4\n".into(),
            Self::ConvI32Checked => "\tconv.ovf.i4\n".into(),
            Self::ConvI16Checked => "\tconv.ovf.i2\n".into(),
            Self::Nop => "\tnop\n".into(), //_=>format!("\t//Comment!\n"),
            Self::LDConstString(string) => format!("\tldstr \"{string}\"\n"),
            Self::NewObj { ctor_fn } => format!("\tnewobj instance {ctor_fn}\n"),
            Self::Throw => "\tthrow\n".into(),
            Self::STIndIn(size) => format!("\tstind.i{size}\n"),
            Self::LDIndIn(size) => format!("\tldind.i{size}\n"),
            Self::LDIndR8 => "\tldind.r8\n".into(),
            Self::LDIndR4 => "\tldind.r4\n".into(),
            Self::STIndR8 => "\tstind.r8\n".into(),
            Self::STIndR4 => "\tstind.r4\n".into(),
            Self::LDIndI => "\tldind.i\n".into(),
            Self::STIndI => "\tstind.i\n".into(),
            Self::CallStatic { sig, function_name } => {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = sig.inputs.iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&firts_arg.il_name());
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&arg.il_name());
                }
                format!(
                    "\tcall {output} {function_name}({input_string})\n",
                    output = sig.output.il_name()
                )
            }
            Self::Call { sig, function_name } => {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = sig.inputs.iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&firts_arg.il_name());
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&arg.il_name());
                }
                format!(
                    "\tcall instance {output} {function_name}({input_string})\n",
                    output = sig.output.il_name()
                )
            }
            //todo!("Can't call functions yet!")
            Self::LDField {
                field_parent,
                field_name,
                field_type,
            } => {
                format!(
                    "\tldfld {field_type} '{field_parent}'::{field_name}\n",
                    field_type = field_type.arg_name(),
                )
            }
            Self::LDFieldAdress {
                field_parent,
                field_name,
                field_type,
            } => {
                format!(
                    "\tldflda {field_type} '{field_parent}'::{field_name}\n",
                    field_type = field_type.arg_name(),
                )
            }
            Self::STField {
                field_parent,
                field_name,
                field_type,
            } => {
                format!(
                    "\tstfld {field_type} '{field_parent}'::{field_name}\n",
                    field_type = field_type.arg_name(),
                )
            }
            Self::LDObj(struct_name) => format!("\tldobj valuetype {struct_name}\n"),
            Self::STObj(struct_name) => format!("\tstobj valuetype {struct_name}\n"),
            Self::DebugComment(comment) => format!("//{comment}\n"),
            Self::InitObj(type_name) => format!("\tinitobj {type_name}\n"),
            Self::SizeOf(type_name) => format!("\tsizeof {type_name}\n"),
        }
        .into()
    }
}
