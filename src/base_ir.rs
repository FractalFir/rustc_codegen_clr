use crate::{FunctionSignature, IString, VariableType};
use serde::{Deserialize, Serialize};
// An IR close, but not exactly equivalent to the CoreCLR IR.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum BaseIR {
    LDConstI8(i8),
    LDConstI32(i32),
    LDConstI64(i64),
    LDConstF32(f32),
    STIndIn(u8),
    STIndI,
    LDIndIn(u8),
    LDIndI,
    LDConstString(String),
    STArg(u32),
    LDArg(u32),
    STLoc(u32),
    LDLoc(u32),
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
    Xor,
    Or,
    And,
    Return,
    ConvF32,
    ConvI32,
    ConvI32Checked,
    ConvI8,
    Nop,
    CallStatic {
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
    STField {
        field_parent: IString,
        field_name: IString,
        field_type: VariableType,
    },
    LDObj(IString),
    STObj(IString),
    Throw,
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
            Self::Div => "\tdiv\n".into(),
            Self::Shl => "\tshl\n".into(),
            Self::Shr => "\tshr\n".into(),
            Self::Eq => "\tceq\n".into(),
            Self::NEq => "\tceq\n\tldc.i4.0\n\tceq\n".into(),
            Self::Gt => "\tcgt\n".into(),
            Self::Xor => "\txor\n".into(),
            Self::Or => "\tor\n".into(),
            Self::And => "\tand\n".into(),
            Self::LDConstI8(i8const) => format!("\tldc.i4.s {i8const}\n"),
            Self::LDConstI32(i32const) => format!("\tldc.i4 {i32const}\n"),
            Self::LDConstI64(i64const) => format!("\tldc.i8 {i64const}\n"),
            Self::LDConstF32(f32const) => format!("\tldc.r4 {f32const}\n"),
            Self::ConvF32 => "\tconv.r4\n".into(),
            Self::ConvI8 => "\tconv.i1\n".into(),
            Self::ConvI32 => "\tconv.i4\n".into(),
            Self::ConvI32Checked => "\tconv.ovf.i4\n".into(),
            Self::Nop => "\tnop\n".into(), //_=>format!("\t//Comment!\n"),
            Self::LDConstString(string) => format!("\tldstr \"{string}\"\n"),
            Self::NewObj { ctor_fn } => format!("\tnewobj instance {ctor_fn}\n"),
            Self::Throw => "\tthrow\n".into(),
            Self::STIndIn(size) => format!("\tstind.i{size}\n"),
            Self::LDIndIn(size) => format!("\tldind.i{size}\n"),
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
            } //todo!("Can't call functions yet!")
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
        }
        .into()
    }
}
