use cilly::{
    asm::Assembly,
    asm_exporter::{AssemblyExportError, AssemblyExporter},
    ilasm_exporter::ILASMExporter,
};
use lazy_static::*;
use std::path::Path;

pub fn export_assembly(
    asm: &Assembly,
    path: impl AsRef<Path>,
    is_lib: bool,
) -> Result<(), AssemblyExportError> {
    ILASMExporter::export_assembly(
        ILASMExporter::default(),
        asm,
        path.as_ref(),
        is_lib,
        *ESCAPE_NAMES,
    )
}
lazy_static! {
    #[doc = "Tells the codegen to escape class and method names."]pub static ref ESCAPE_NAMES:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(ESCAPE_NAMES){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(ESCAPE_NAMES),value),
        }).unwrap_or(false)
    };
}
