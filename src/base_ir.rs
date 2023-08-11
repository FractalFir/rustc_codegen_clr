use crate::{IString,VariableType,CLRMethod};
#[derive(Debug, Clone)]
pub(crate) enum BaseIR {
    LDConstI32(i32),
    STArg(u32),
    LDArg(u32),
    STLoc(u32),
    LDLoc(u32),
    Add,
    Mul,
    Shl,
    Return,
}
impl BaseIR{
    pub(crate) fn get_trivial_type(&self,parrent_method:&CLRMethod)->Option<VariableType>{
        match self{
            Self::LDConstI32(_)=>Some(VariableType::I32),
            Self::LDArg(arg_id)=>Some(parrent_method.get_arg_type(*arg_id).clone()),
            _=>None,
        }
    }
    pub(crate) fn clr_ir(&self)->IString{
        match self{
            Self::LDArg(arg)=>format!("\tldarg.{arg}\n"),
            Self::LDLoc(arg)=>format!("\tldloc.{arg}\n"),
            Self::STLoc(arg)=>format!("\tstloc.{arg}\n"),
            Self::Return=>"\tret\n".into(),
            Self::Add=>"\tadd\n".into(),
            Self::Mul=>"\tmul\n".into(),
            Self::Shl=>"\tshl\n".into(),
            Self::LDConstI32(i32const)=>format!("ldc.i4 {i32const}\t\n"),
            _=>format!("\t//Comment!\n"),
        }.into()
    }
}
