use std::path::Path;

use rustc_codegen_clr::{assembly::Assembly, assembly_exporter::{AssemblyExportError, AssemblyExporter}, config::USE_CECIL_EXPORTER};

pub fn export_assembly(asm:&Assembly,path:impl AsRef<Path>,is_lib:bool)->Result<(),AssemblyExportError>{
    if *USE_CECIL_EXPORTER {
        rustc_codegen_clr::assembly_exporter::cecil_exporter::DotnetContext::export_assembly(
            asm,
            path.as_ref(),
            is_lib,
        )
     
    } else {
        rustc_codegen_clr::assembly_exporter::ilasm_exporter::ILASMExporter::export_assembly(
            asm,
            path.as_ref(),
            is_lib,
        )
      
    }
}