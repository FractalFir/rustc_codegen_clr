use std::path::Path;

use cilly::{
    asm::Assembly,
    asm_exporter::{AssemblyExportError, AssemblyExporter}, ilasm_exporter::ILASMExporter,
};

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
        *crate::config::ESCAPE_NAMES,
    )
}
